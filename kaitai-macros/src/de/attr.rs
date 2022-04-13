use crate::de::{data::IntegerValue, doc::Doc};

use std::collections::HashMap;

use serde::{de, Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Attr {
    pub id: String,
    #[serde(flatten, default)]
    pub doc: Doc,
    #[serde(deserialize_with = "deserialize_contents", default)]
    pub contents: Option<Vec<u8>>,
    #[serde(rename = "type")]
    pub ty: AttrType,
    #[serde(default)]
    pub repeat: Option<Repeat>,
    #[serde(default)]
    pub repeat_expr: Option<IntegerValue>,
    #[serde(default)]
    pub repeat_until: Option<String>,
    #[serde(rename = "if", default)]
    pub if_expr: Option<String>,
    #[serde(default)]
    pub size: Option<IntegerValue>,
    #[serde(default)]
    pub size_eos: bool,
    #[serde(default)]
    pub process: Option<String>,
    #[serde(rename = "enum", default)]
    pub en: Option<String>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub pad_right: Option<u64>,
    #[serde(default)]
    pub terminator: Option<u64>,
    #[serde(default = "default_true")]
    pub consume: bool,
    #[serde(default)]
    pub include: bool,
    #[serde(default = "default_true")]
    pub eos_error: bool,
    #[serde(default)]
    pub pos: Option<IntegerValue>,
    #[serde(default)]
    pub io: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

const fn default_true() -> bool {
    true
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
