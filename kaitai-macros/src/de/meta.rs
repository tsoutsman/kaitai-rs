use crate::de::data::deserialize_string_or_seq;

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Meta {
    pub id: Option<String>,
    #[serde(flatten)]
    pub doc: MetaDoc,
    pub ks_version: Option<String>,
    pub ks_debug: bool,
    pub ks_opaque_types: bool,
    pub imports: Vec<String>,
    pub encoding: Option<String>,
    #[serde(rename = "endian")]
    pub endianness: Option<Endianness>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct MetaDoc {
    pub title: String,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub application: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub file_extension: Vec<String>,
    pub xref: Xref,
    pub license: String,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Endianness {
    Le,
    Be,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Xref {
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub forensicswiki: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub iso: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub justsolve: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub loc: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub mime: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub pronom: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub rfc: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub wikidata: Vec<String>,
}
