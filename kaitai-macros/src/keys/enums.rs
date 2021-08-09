use crate::{
    utils::{assert_pattern, get_attribute, sc_to_ucc, MacroError},
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
    let enums = match get_attribute!(map; "enums" as Yaml::Hash(m) => m; "get_enums") {
        Ok(e) => e,
        Err(MacroError::RequiredAttrNotFound(..)) => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut result = Vec::new();

    for (enum_ident, variants) in enums {
        let enum_ident = assert_pattern!(
            enum_ident;
            Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site());
            attr: "enum ident", st: "get_enums";
        );

        let variants = assert_pattern!(
            variants;
            Yaml::Hash(m) => m;
            attr: "variant map", st: "get_enums";
        );

        let mut values = Vec::new();

        for (variant_value, variant_ident) in variants {
            let variant_ident = assert_pattern!(
                variant_ident;
                Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site());
                attr: "variant ident", st: "get_enums";
            );
            let variant_value = assert_pattern!(
                variant_value;
                Yaml::String(s) => s.parse().unwrap();
                attr: "variant value", st: "get_enums";
            );
            values.push((variant_ident, variant_value));
        }

        result.push(EnumSpec {
            ident: enum_ident,
            values,
        });
    }

    Ok(result)
}
