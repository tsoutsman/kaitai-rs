#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Param {
    id: String,
    #[serde(rename = "type")]
    ty: String,
    doc: Option<String>,
    doc_ref: Option<String>,
    #[serde(rename = "enum")]
    en: Option<String>,
}
