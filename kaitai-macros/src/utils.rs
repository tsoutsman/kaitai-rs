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
            None => Err(crate::utils::MacroError::AttrNotFound($attr.to_owned())),
        }
    };
}
pub(crate) use get_attribute;

#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MacroError {
    #[error("endianness must be `le` or `be`")]
    InvalidEndianness,
    #[error("{attr} cannot be converted using {pat}")]
    InvalidAttrType { attr: String, pat: String },
    #[error("{0} not found")]
    AttrNotFound(String),
}
