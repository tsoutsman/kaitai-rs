use thiserror::Error;

macro_rules! get_attribute {
    ($data:ident | $attr:literal as $pat:pat => $e:expr) => {
        match $data.get(&Yaml::String($attr.to_owned())) {
            Some(s) => match s {
                $pat => Ok($e),
                _ => Err(crate::utils::MacroError::InvalidAttrType {
                    attr: $attr.to_owned(),
                    pat: stringify!($pat).to_owned(),
                }),
            },
            None => Err(crate::utils::MacroError::RequiredAttrNotFound(
                $attr.to_owned(),
            )),
        }
    };
}
pub(crate) use get_attribute;

pub type Result<T> = std::result::Result<T, MacroError>;

#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MacroError {
    #[error("attribute in seq invalid: {0:?}")]
    InvalidAttribute(yaml_rust::Yaml),
    #[error("endianness must be `le` or `be`")]
    InvalidEndianness,
    #[error("{attr} does not match {pat}")]
    InvalidAttrType { attr: String, pat: String },
    #[error("{0} not found")]
    RequiredAttrNotFound(String),
}
