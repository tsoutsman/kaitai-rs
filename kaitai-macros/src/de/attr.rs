use crate::de::{
    ty::Type,
    util::{bool_false, bool_true},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Attr {
    id: String,
    doc: Option<String>,
    doc_ref: Option<String>,
    contents: Option<String>,
    #[serde(rename = "type")]
    ty: Option<Type>,
    repeat: Option<Repeat>,
    repeat_expr: Option<u64>,
    repeat_until: Option<String>,
    #[serde(rename = "if")]
    if_expr: Option<String>,
    size: Option<u64>,
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
    pos: Option<u64>,
    io: Option<String>,
    // value: String,
}

#[derive(serde::Deserialize)]
pub enum Repeat {
    Expr,
    Eos,
    Until,
}
