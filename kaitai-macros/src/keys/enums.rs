use crate::{
    utils::{get_attribute, sc_to_ucc, MacroError},
    Result,
};

use proc_macro2::{Ident, Span, TokenStream};
use yaml_rust::{yaml, Yaml};

#[allow(dead_code)]
pub struct EnumSpec {
    pub ident: Ident,
    pub values: Vec<(Ident, TokenStream)>,
}

fn _get_enums(map: &yaml::Hash) -> Result<Vec<EnumSpec>> {
    let enums = match get_attribute!(map | "enums" as Yaml::Hash(m) => m) {
        Ok(e) => e,
        Err(MacroError::RequiredAttrNotFound(_)) => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut result = Vec::new();

    for (enum_ident, variants) in enums {
        let enum_ident = match enum_ident {
            Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site()),
            _ => {
                return Err(MacroError::InvalidAttrType {
                    attr: "enum name".to_owned(),
                    pat: "Yaml::String(s)".to_owned(),
                    actual: enum_ident.clone(),
                })
            }
        };

        let variants = match variants {
            Yaml::Hash(m) => m,
            _ => {
                return Err(MacroError::InvalidAttrType {
                    attr: "enum variants".to_owned(),
                    pat: "Yaml::Hash(m)".to_owned(),
                    actual: variants.clone(),
                })
            }
        };

        let mut values = Vec::new();

        for (variant_value, variant_ident) in variants {
            let variant_ident = match variant_ident {
                Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site()),
                _ => {
                    return Err(MacroError::InvalidAttrType {
                        attr: "enum variant name".to_owned(),
                        pat: "Yaml::String(s)".to_owned(),
                        actual: variant_ident.clone(),
                    })
                }
            };
            let variant_value = match variant_value {
                Yaml::String(s) => s.parse().unwrap(),
                _ => {
                    return Err(MacroError::InvalidAttrType {
                        attr: "enum variant value".to_owned(),
                        pat: "Yaml::String(s)".to_owned(),
                        actual: variant_value.clone(),
                    })
                }
            };
            values.push((variant_ident, variant_value));
        }

        result.push(EnumSpec {
            ident: enum_ident,
            values,
        });
    }

    Ok(result)
}
