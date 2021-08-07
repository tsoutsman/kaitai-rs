use std::convert::TryFrom;

use crate::{get_attribute, utils::MacroError};
use yaml_rust::{yaml, Yaml};

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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "le" => Ok(Endianness::Le),
            "be" => Ok(Endianness::Be),
            _ => Err(MacroError::InvalidEndianness),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct MetaSpec {
    pub id: String,
    pub endianness: Endianness,
}

pub(crate) fn parse_meta(meta: &yaml::Hash) -> Result<MetaSpec, MacroError> {
    let id = get_attribute!(meta | "id" as Yaml::String(s) => s.clone())?;
    let endianness: Endianness =
        Endianness::try_from(get_attribute!(meta | "endian" as Yaml::String(s) => s)?.as_ref())?;
    Ok(MetaSpec { id, endianness })
}
