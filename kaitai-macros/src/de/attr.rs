use crate::de::{data::IntegerValue, doc::Doc};

use std::collections::HashMap;

use serde::{de, Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Attr {
    pub id: Option<String>,
    #[serde(flatten)]
    pub doc: Doc,
    #[serde(deserialize_with = "deserialize_contents")]
    pub contents: Option<Vec<u8>>,
    #[serde(rename = "type")]
    pub ty: Option<AttrType>,
    pub repeat: Option<Repeat>,
    pub repeat_expr: Option<IntegerValue>,
    pub repeat_until: Option<String>,
    #[serde(rename = "if")]
    pub if_expr: Option<String>,
    pub size: Option<IntegerValue>,
    pub size_eos: bool,
    pub process: Option<String>,
    #[serde(rename = "enum")]
    pub en: Option<String>,
    pub encoding: Option<String>,
    pub pad_right: Option<u64>,
    pub terminator: Option<u64>,
    pub consume: bool,
    pub include: bool,
    pub eos_error: bool,
    pub pos: Option<IntegerValue>,
    pub io: Option<String>,
    pub value: Option<String>,
}

impl Default for Attr {
    fn default() -> Self {
        Self {
            id: None,
            doc: Doc::default(),
            contents: None,
            ty: None,
            repeat: None,
            repeat_expr: None,
            repeat_until: None,
            if_expr: None,
            size: None,
            size_eos: false,
            process: None,
            en: None,
            encoding: None,
            pad_right: None,
            terminator: None,
            consume: true,
            include: false,
            eos_error: true,
            pos: None,
            io: None,
            value: None,
        }
    }
}

fn deserialize_contents<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ContentsVisitor;

    impl<'de> de::Visitor<'de> for ContentsVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("string or byte array")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.as_bytes().to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.into_bytes())
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    // The function is only called if contents variable is non null.
    Ok(Some(deserializer.deserialize_any(ContentsVisitor)?))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum AttrType {
    TypeRef(String),
    #[serde(rename_all = "kebab-case")]
    Switch {
        switch_on: String,
        cases: HashMap<String, String>,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Repeat {
    Expr,
    Eos,
    Until,
}
