use crate::{
    error::Error,
    types::TypeInfo,
    util::{get_attr, get_required_attr},
};

use std::convert::TryFrom;

use anyhow::{Context, Result};
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
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "le" => Ok(Endianness::Le),
            "be" => Ok(Endianness::Be),
            _ => Err(Error::InvalidEndianness),
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
    let meta = match get_attr!(map; "meta" as Yaml::Hash(h) => h).context("get_meta")? {
        // The type has a `MetaSpec`. It is assumed that the provided `MetaSpec` overwrites the
        // inherited one.
        Some(m) => m,
        None => {
            if let Some(m) = info.inherited_meta.clone() {
                // The type doesn't have a `MetaSpec` but it inherits one.
                return Ok(m);
            } else {
                // The type doesn't have a `MetaSpec` and does not inherit any.
                let e = Error::RequiredAttrNotFound("meta".to_owned());
                return Err(e).context("get_meta");
            }
        }
    };

    let id = get_required_attr!(meta; "id" as Yaml::String(s) => s.clone()).context("get_meta")?;
    let endianness = Endianness::try_from(
        get_required_attr!(meta; "endian" as Yaml::String(s) => s)
            .context("get_meta")?
            .as_ref(),
    )
    .context("get_meta")?;

    Ok(MetaSpec { id, endianness })
}
