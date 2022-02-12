use crate::de::data::deserialize_string_or_seq;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Param {
    id: String,
    #[serde(rename = "type")]
    ty: String,
    #[serde(default)]
    doc: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    doc_ref: Vec<String>,
    #[serde(rename = "enum")]
    en: Option<String>,
}
