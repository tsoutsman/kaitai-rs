use crate::de::{self, doc::Doc};

use std::convert::TryFrom;

pub use crate::de::{attr::Repeat, data::IntegerValue};

pub struct Attr {
    id: Option<String>,
    doc: Doc,
    logic: Logic,
}

pub enum Logic {
    FixedContents(Vec<u8>),
    Type(Type),
    Switch {
        on: String,
        cases: Vec<(Pattern, Type)>,
    },
    Repeat {
        ty: Repeat,
        num: Option<IntegerValue>,
    },
    // TODO: if logic
    Size(Size),
    // TODO: probably don't use string
    Process(String),
}

// TODO: pad-right
// TODO: pos
// TODO: io
// TODO: value

impl std::convert::TryFrom<de::attr::Attr> for Attr {
    type Error = ();

    fn try_from(a: de::attr::Attr) -> Result<Self, Self::Error> {
        let id = a.id;
        let doc = a.doc;
        let logic = {
            if let Some(contents) = a.contents {
                Logic::FixedContents(contents)
            } else if let Some(ty) = a.ty {
                match ty {
                    de::attr::AttrType::TypeRef(type_ref) => {
                        Logic::Type(Type::from((type_ref, a.en)))
                    }
                    de::attr::AttrType::Switch {
                        switch_on: on,
                        cases,
                    } => Logic::Switch {
                        on,
                        cases: cases.into_iter().map(|(_k, _v)| todo!()).collect(),
                    },
                }
            } else {
                todo!();
            }
        };

        Ok(Self { id, doc, logic })
    }
}

pub enum Type {
    UserDefined(String),
    BuiltIn { ty: BuiltInType, en: Option<String> },
}

// TODO: cow?
impl std::convert::From<(String, Option<String>)> for Type {
    fn from((type_ref, en): (String, Option<String>)) -> Self {
        if let Ok(built_in) = BuiltInType::try_from(type_ref.as_ref()) {
            Type::BuiltIn { ty: built_in, en }
        } else {
            Type::UserDefined(type_ref)
        }
    }
}

pub enum BuiltInType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl std::convert::TryFrom<&str> for BuiltInType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "u1" => BuiltInType::U8,
            "u2" => BuiltInType::U16,
            "u4" => BuiltInType::U32,
            "u8" => BuiltInType::U64,
            "s1" => BuiltInType::I8,
            "s2" => BuiltInType::I16,
            "s4" => BuiltInType::I32,
            "s8" => BuiltInType::I64,
            "f4" => BuiltInType::F32,
            "f8" => BuiltInType::F64,
            _ => return Err(()),
        })
    }
}

// TODO: Encoding field on String type
// TODO: terminator for String or Byte array

pub enum Pattern {
    Enum(String),
    Int(u64),
}

pub enum Size {
    Fixed(IntegerValue),
    Eos,
}
