use crate::de::util::bool_false;

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Meta {
    id: String,
    title: Option<String>,
    application: Option<String>,
    file_extension: Option<String>,
    xref: (),
    license: Option<String>,
    ks_version: Option<String>,
    #[serde(default = "bool_false")]
    ks_debug: bool,
    #[serde(default = "bool_false")]
    ks_opaque_types: bool,
    imports: Vec<String>,
    encoding: Option<String>,
    endian: Endianness,
}

#[derive(serde::Deserialize)]
pub enum Endianness {
    Le,
    Be,
}
