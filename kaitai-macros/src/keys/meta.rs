use std::convert::TryFrom;

use crate::{
    get_attribute,
    types::TypeInfo,
    utils::{MacroError, Result},
};
use yaml_rust::Yaml;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Endianness {
    Le,
    Be,
}

impl std::fmt::Display for Endianness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            Endianness::Le => "le",
            Endianness::Be => "be",
        };
        write!(f, "{}", result)
    }
}

impl std::convert::TryFrom<&str> for Endianness {
    type Error = MacroError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "le" => Ok(Endianness::Le),
            "be" => Ok(Endianness::Be),
            _ => Err(MacroError::InvalidEndianness),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MetaSpec {
    pub id: String,
    pub endianness: Endianness,
}

pub fn get_meta(info: &TypeInfo) -> Result<MetaSpec> {
    let map = info.map;
    let meta = match get_attribute!(map | "meta" as Yaml::Hash(h) => h) {
        // The type has a `MetaSpec`. It is assumed that the provided `MetaSpec` overwrites the
        // inherited one.
        Ok(m) => m,
        Err(MacroError::RequiredAttrNotFound(a)) => {
            if let Some(m) = info.inherited_meta.clone() {
                // The type doesn't have a `MetaSpec` but it inherits one.
                return Ok(m);
            } else {
                // The type doesn't have a `MetaSpec` and does not inherit any.
                return Err(MacroError::RequiredAttrNotFound(a));
            }
        }
        Err(e) => return Err(e),
    };

    let id = get_attribute!(meta | "id" as Yaml::String(s) => s.clone())?;
    let endianness: Endianness =
        Endianness::try_from(get_attribute!(meta | "endian" as Yaml::String(s) => s)?.as_ref())?;

    Ok(MetaSpec { id, endianness })
}
