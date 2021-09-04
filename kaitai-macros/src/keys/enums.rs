use crate::util::{assert_pattern, get_attr, sc_to_ucc};

use anyhow::{Context, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use yaml_rust::{yaml, Yaml};

#[allow(dead_code)]
pub struct EnumSpec {
    pub ident: Ident,
    pub variants: Vec<(Ident, TokenStream)>,
}

fn get_enums(map: &yaml::Hash) -> Result<Vec<EnumSpec>> {
    let enums = match get_attr!(map; "enums" as Yaml::Hash(m) => m).context("get_enums")? {
        Some(e) => e,
        None => return Ok(Vec::new()),
    };

    let mut result = Vec::new();

    for (enum_ident, variants) in enums {
        let enum_ident = assert_pattern!(
            enum_ident;
            Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site());
            attr: "enum ident";
        )
        .context("get_enums")?;

        let variants_yaml = assert_pattern!(
            variants;
            Yaml::Hash(m) => m;
            attr: "enum variants";
        )
        .context("get_enums")?;

        let mut variants = Vec::with_capacity(variants_yaml.len());

        for (variant_value, variant_ident) in variants_yaml {
            let variant_ident = assert_pattern!(
                variant_ident;
                Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site());
                attr: "variant ident";
            )
            .context("get_enums")?;
            let variant_value = assert_pattern!(
                variant_value;
                // TODO remove to_string call
                Yaml::Integer(i) => i.to_string().parse().unwrap();
                attr: "variant value";
            )
            .context("get_enums")?;
            variants.push((variant_ident, variant_value));
        }

        result.push(EnumSpec {
            ident: enum_ident,
            variants,
        });
    }

    Ok(result)
}

pub fn gen_enum_defs(map: &yaml::Hash) -> Result<Vec<TokenStream>> {
    let enums = get_enums(map).context("gen_enum_defs")?;
    let mut result = Vec::with_capacity(enums.len());

    for EnumSpec { ident, variants } in enums {
        let variant_defs = variants.iter().map(|(ident, value)| {
            quote! {#ident = #value}
        });

        let n_matches = variants.iter().map(|(ident, value)| {
            quote! { #value => ::std::option::Option::Some(Self::#ident) }
        });

        result.push(quote! {
            pub enum #ident {
                #(#variant_defs),*
            }

            impl #ident {
                // For some reason using an Into bound on N doesn't work so I have to use
                // this weird where clause.
                pub fn n<N>(n: N) -> Option<Self> where i128: From<N> {
                    match i128::from(n) {
                        #(#n_matches),*,
                        _ => None,
                    }
                }
            }
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust::YamlLoader;

    // #[test]
    fn _test_gen_enum_defs() {
        let map = &YamlLoader::load_from_str(
            "
enums:
  chunk_type:
    0x4E4F534A: json # JSON
    0x004E4942: bin  # BIN
  another_enum:
    0xDEADBEEF: json # JSON
    0x3: bin  # BIN\0",
        )
        .unwrap()[0];
        let map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        let result = gen_enum_defs(map).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0].to_string(),
            "pub enum ChunkType { Json = 1313821514 , Bin = 5130562 }"
        );
        assert_eq!(
            result[1].to_string(),
            "pub enum AnotherEnum { Json = 3735928559 , Bin = 3 }"
        );
    }
}
