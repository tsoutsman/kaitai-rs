use crate::util::{assert_pattern, get_attr, sc_to_ucc};

use std::convert::TryFrom;

use anyhow::{Context, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use yaml_rust::{yaml, Yaml};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct EnumsSpec(Vec<EnumSpec>);

impl ToTokens for EnumsSpec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for e in &self.0 {
            tokens.extend(quote! { #e });
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct EnumSpec {
    pub ident: Ident,
    pub variants: Vec<(Ident, usize)>,
}

impl ToTokens for EnumSpec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let EnumSpec { ident, variants } = self;

        let variant_defs = variants.iter().map(|(ident, value)| {
            quote! {#ident = #value}
        });

        let n_matches = variants.iter().map(|(ident, value)| {
            quote! { #value => ::std::option::Option::Some(Self::#ident) }
        });

        tokens.extend(quote! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            // TODO is this repr ok?
            #[repr(usize)]
            pub enum #ident {
                #(#variant_defs),*
            }

            impl #ident {
                // For some reason using an Into bound on N doesn't work so I have to use
                // this weird where clause.
                // TODO add doc comment for this function.
                pub fn n<N>(n: N) -> Option<Self> where usize: From<N> {
                    match usize::from(n) {
                        #(#n_matches),*,
                        _ => None,
                    }
                }
            }
        });
    }
}

// TODO rewrite
pub fn enums(map: &yaml::Hash) -> Result<EnumsSpec> {
    let enums = match get_attr!(map; "enums" as Yaml::Hash(m) => m)
        .context("enums: enums is not a hashmap")?
    {
        Some(e) => e,
        None => return Ok(EnumsSpec(Vec::new())),
    };

    let mut result = Vec::new();

    for (enum_ident, variants) in enums {
        let enum_ident = assert_pattern!(
            enum_ident;
            Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site());
            attr: "enum ident";
        )
        .context("enums: enum ident is not a string")?;

        let variants_yaml = assert_pattern!(
            variants;
            Yaml::Hash(m) => m;
            attr: "enum variants";
        )
        .context("enums: enum variants is not a hashmap")?;

        let mut variants = Vec::with_capacity(variants_yaml.len());

        for (variant_value, variant_ident) in variants_yaml {
            let variant_ident = assert_pattern!(
                variant_ident;
                Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site());
                attr: "enum variant ident";
            )
            .context("enums: enum variant ident is not a string")?;
            let variant_value = assert_pattern!(
                variant_value;
                // TODO handle this unwrap
                // TODO can KS enums be negative?
                Yaml::Integer(i) => usize::try_from(*i).unwrap();
                attr: "enum variant value";
            )
            .context("enums: enum variant value is not an integer")?;
            variants.push((variant_ident, variant_value));
        }

        result.push(EnumSpec {
            ident: enum_ident,
            variants,
        });
    }

    Ok(EnumsSpec(result))
}
