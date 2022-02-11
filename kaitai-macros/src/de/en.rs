use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Enum(HashMap<u64, EnumValue>);

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EnumValue {
    id: String,
    doc: Option<String>,
    doc_ref: Option<String>,
}
