#[derive(thiserror::Error, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    #[error("attribute in seq invalid: {0:?}")]
    InvalidAttribute(yaml_rust::Yaml),
    #[error("endianness must be `le` or `be`")]
    InvalidEndianness,
    #[error("{attr} does not match {pat} actual value {actual:?}")]
    InvalidAttrType {
        attr: String,
        pat: String,
        actual: yaml_rust::Yaml,
    },
    #[error("{0} not found")]
    RequiredAttrNotFound(String),
}
