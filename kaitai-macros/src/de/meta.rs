use crate::de::{data::deserialize_string_or_seq, util::bool_false};

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Meta {
    id: String,
    title: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    application: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    file_extension: Option<Vec<String>>,
    xref: Option<Xref>,
    license: Option<String>,
    ks_version: Option<String>,
    #[serde(default = "bool_false")]
    ks_debug: bool,
    #[serde(default = "bool_false")]
    ks_opaque_types: bool,
    #[serde(default)]
    imports: Vec<String>,
    encoding: Option<String>,
    endian: Endianness,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Endianness {
    Le,
    Be,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct Xref {
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    forensicswiki: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    iso: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    justsolve: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    loc: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    mime: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pronom: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    rfc: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    wikidata: Option<Vec<String>>,
}
