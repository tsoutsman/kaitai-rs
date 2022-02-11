use crate::de::{attr::Attr, en::Enum, meta::Meta, param::Param};

use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Type {
    meta: Meta,
    doc: Option<String>,
    doc_ref: Option<String>,
    params: Vec<Param>,
    seq: Vec<Attr>,
    types: HashMap<String, Type>,
    instances: HashMap<String, Attr>,
    enums: HashMap<String, Enum>,
}
