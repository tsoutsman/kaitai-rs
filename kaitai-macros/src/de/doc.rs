use crate::de::data::deserialize_string_or_seq;

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Doc {
    pub doc: String,
    #[serde(deserialize_with = "deserialize_string_or_seq")]
    pub doc_ref: Vec<String>,
}
