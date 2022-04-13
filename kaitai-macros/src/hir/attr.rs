use crate::{
    de,
    hir::{doc::Doc, meta::Meta},
    util::sc_to_ucc,
};

use std::convert::TryFrom;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

pub use crate::de::data::IntegerValue;

pub struct Attribute {
    id: Ident,
    doc: Doc,
    repeat: Option<Repeat>,
    logic: Logic,
}

impl Attribute {
    pub fn field_definition(&self) -> TokenStream {
        let mut ty = match &self.logic {
            Logic::FixedContents(_) => return TokenStream::new(),
            Logic::Type(ty) => ty.ty(),
            Logic::Switch { .. } => todo!(),
            Logic::Size(_) => quote! { ::std::vec::Vec<u8> },
            Logic::Process(_) => todo!(),
        };
        if self.repeat.is_some() {
            ty = quote! { ::std::vec::Vec<#ty> };
        }

        let doc = &self.doc;
        let id = &self.id;
        quote! {
            #doc
            pub #id: #ty
        }
    }

    pub fn variable_assignment(&self, meta: &Meta) -> TokenStream {
        let mut expr = match &self.logic {
            Logic::FixedContents(c) => {
                let contents = c.iter().map(|i| quote! { #i });
                return quote! { buf.ensure_fixed_contents(&[#(#contents),*])?; };
            }
            Logic::Type(ty) => ty.expr(meta),
            Logic::Switch { .. } => todo!(),
            Logic::Size(size) => match size {
                Size::Fixed(count) => quote! { buf.read_bytes(#count)? },
                Size::Eos => quote! { buf.read_bytes_full()? },
            },
            Logic::Process(_) => todo!(),
        };

        if let Some(repeat) = &self.repeat {
            expr = match repeat {
                Repeat::Eos => {
                    quote! {
                        {
                            let mut result = Vec::new();
                            while !buf.is_eof()? {
                                result.push(#expr);
                            }
                            result
                        }
                    }
                }
                Repeat::Expr(_) => todo!(),
                Repeat::Until(_) => todo!(),
            }
        }

        let id = &self.id;
        quote! { let #id = #expr; }
    }
}

pub enum Logic {
    FixedContents(Vec<u8>),
    Type(Type),
    Switch {
        on: String,
        cases: Vec<(Pattern, Type)>,
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

impl std::convert::TryFrom<(Option<de::meta::Meta>, de::attr::Attr)> for Attribute {
    type Error = ();

    fn try_from(
        (meta, attr): (Option<de::meta::Meta>, de::attr::Attr),
    ) -> Result<Self, Self::Error> {
        let id = Ident::new(&sc_to_ucc(&attr.id.unwrap()), Span::call_site());
        let doc = (meta.map(|m| m.doc), attr.doc).into();
        let repeat = match attr.repeat {
            Some(repeat) => Some(match repeat {
                de::attr::Repeat::Eos => Repeat::Eos,
                de::attr::Repeat::Expr => Repeat::Expr(attr.repeat_expr.unwrap()),
                de::attr::Repeat::Until => Repeat::Until(attr.repeat_until.unwrap()),
            }),
            None => None,
        };
        let logic = {
            if let Some(contents) = attr.contents {
                Logic::FixedContents(contents)
            } else if let Some(size) = attr.size {
                Logic::Size(Size::Fixed(size))
            } else if attr.size_eos {
                Logic::Size(Size::Eos)
            } else {
                match attr.ty.unwrap() {
                    de::attr::AttrType::TypeRef(type_ref) => {
                        Logic::Type(Type::from((type_ref, attr.en)))
                    }
                    de::attr::AttrType::Switch {
                        switch_on: on,
                        cases,
                    } => Logic::Switch {
                        on,
                        cases: cases.into_iter().map(|(_k, _v)| todo!()).collect(),
                    },
                }
            }
        };

        Ok(Self {
            id,
            doc,
            repeat,
            logic,
        })
    }
}

pub enum Type {
    UserDefined(String),
    BuiltIn { ty: BuiltInType, en: Option<String> },
}

impl Type {
    fn ty(&self) -> TokenStream {
        match self {
            Type::UserDefined(id) => {
                Ident::new(&sc_to_ucc(id), Span::call_site()).into_token_stream()
            }
            Type::BuiltIn { ty, en } => {
                if let Some(enum_id) = en {
                    Ident::new(&sc_to_ucc(enum_id), Span::call_site()).into_token_stream()
                } else {
                    ty.to_token_stream()
                }
            }
        }
    }

    fn expr(&self, meta: &Meta) -> TokenStream {
        match self {
            Type::UserDefined(_) => todo!(),
            Type::BuiltIn { ty, en } => {
                let read_call = format!("buf.read_{}{}()?", ty.ks_type(), ty.endianness(meta));
                if let Some(enum_ident) = en {
                    format!(
                        "{}::n({}).ok_or(::kaitai::error::Error::NoEnumMatch)?",
                        enum_ident, read_call
                    )
                } else {
                    read_call
                }
            }
        }
        .parse()
        .unwrap()
    }
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

impl BuiltInType {
    fn ks_type(&self) -> &'static str {
        match self {
            BuiltInType::U8 => "u1",
            BuiltInType::U16 => "u2",
            BuiltInType::U32 => "u4",
            BuiltInType::U64 => "u8",
            BuiltInType::I8 => "s1",
            BuiltInType::I16 => "s2",
            BuiltInType::I32 => "s4",
            BuiltInType::I64 => "s8",
            BuiltInType::F32 => "f4",
            BuiltInType::F64 => "f8",
        }
    }

    /// Returns a [`String`] describing the endianness of the `VariableContents`.
    ///
    /// Little-endian contents return "le". Big-endian contents return "be".
    ///
    /// If the contents are of KS type `u1` or `s1`, the function will return an empty string.
    fn endianness(&self, meta: &Meta) -> &'static str {
        match &self {
            BuiltInType::U8 | BuiltInType::I8 => "",
            _ => meta.endianness.into(),
        }
    }
}

impl ToTokens for BuiltInType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BuiltInType::U8 => quote! { u8 },
            BuiltInType::U16 => quote! { u16 },
            BuiltInType::U32 => quote! { u32 },
            BuiltInType::U64 => quote! { u64 },
            BuiltInType::I8 => quote! { i8 },
            BuiltInType::I16 => quote! { i16 },
            BuiltInType::I32 => quote! { i32 },
            BuiltInType::I64 => quote! { i64 },
            BuiltInType::F32 => quote! { f32 },
            BuiltInType::F64 => quote! { f64 },
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

pub enum Repeat {
    Eos,
    Expr(IntegerValue),
    Until(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_field_definitions() {
        let docs = (0..5).map(|_| Doc::new());
        let repeats = vec![
            Some(Repeat::Eos),
            None,
            None,
            Some(Repeat::Eos),
            Some(Repeat::Eos),
        ];
        let logics = vec![
            Logic::FixedContents(vec![0, 1]),
            Logic::Type(Type::UserDefined("my_type".to_owned())),
            Logic::Type(Type::BuiltIn {
                ty: BuiltInType::U16,
                en: None,
            }),
            Logic::Type(Type::BuiltIn {
                ty: BuiltInType::U16,
                en: Some("my_enum".to_owned()),
            }),
            Logic::Size(Size::Eos),
        ];

        let expected = vec![
            quote! {},
            quote! {
                #[doc = ""]
                pub dont: MyType
            },
            quote! {
                #[doc = ""]
                pub kill: u16
            },
            quote! {
                #[doc = ""]
                pub my: ::std::vec::Vec<MyEnum>
            },
            quote! {
                #[doc = ""]
                // Yes the space has to be there. No I don't know why.
                pub vibe: ::std::vec::Vec<::std::vec::Vec<u8> >
            },
        ];
        vec!["bitch", "dont", "kill", "my", "vibe"]
            .iter()
            .map(|id| Ident::new(id, Span::call_site()))
            .zip(docs)
            .zip(repeats)
            .zip(logics)
            .map(|(((id, doc), repeat), logic)| {
                Attribute {
                    id,
                    doc,
                    repeat,
                    logic,
                }
                .field_definition()
            })
            .zip(expected)
            .for_each(|(def, expected)| assert_eq!(def.to_string(), expected.to_string()));
    }
}
