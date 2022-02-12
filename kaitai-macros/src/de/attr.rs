use crate::de::{
    data::{deserialize_string_or_seq, IntegerValue},
    util::{bool_false, bool_true},
};

use std::collections::HashMap;

use serde::{de, Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Attr {
    id: Option<String>,
    #[serde(default)]
    doc: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    doc_ref: Vec<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_contents")]
    contents: Option<Vec<u8>>,
    #[serde(rename = "type")]
    ty: Option<AttrType>,
    repeat: Option<Repeat>,
    repeat_expr: Option<IntegerValue>,
    repeat_until: Option<String>,
    #[serde(rename = "if")]
    if_expr: Option<String>,
    size: Option<IntegerValue>,
    #[serde(default = "bool_false")]
    size_eos: bool,
    process: Option<String>,
    #[serde(rename = "enum")]
    en: Option<String>,
    encoding: Option<String>,
    pad_right: Option<u64>,
    terminator: Option<u64>,
    #[serde(default = "bool_true")]
    consume: bool,
    #[serde(default = "bool_false")]
    include: bool,
    #[serde(default = "bool_true")]
    eos_error: bool,
    pos: Option<IntegerValue>,
    io: Option<String>,
    value: Option<String>,
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
