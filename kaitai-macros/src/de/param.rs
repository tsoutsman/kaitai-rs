use crate::de::doc::Doc;

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Param {
    id: String,
    #[serde(rename = "type")]
    ty: String,
    #[serde(flatten)]
    doc: Doc,
    #[serde(rename = "enum")]
    en: Option<String>,
}
