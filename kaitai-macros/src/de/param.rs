use crate::de::data::deserialize_string_or_seq;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Param {
    id: String,
    #[serde(rename = "type")]
    ty: String,
    doc: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    doc_ref: Option<Vec<String>>,
    #[serde(rename = "enum")]
    en: Option<String>,
}
