use crate::util::get_attr;

use anyhow::{Context, Result};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use yaml_rust::{yaml, Yaml};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DocSpec {
    /// A description of a user-defined type. It is used as documentation for the Rust struct
    /// representing the type.
    pub description: Option<String>,
    /// A reference to the original documentation (if the ksy file is an implementation of some
    /// documented format).
    pub reference: Option<String>,
}

impl ToTokens for DocSpec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.description.is_none() && self.reference.is_none() {
            return;
        }

        // surely there's a better way
        let empty = String::new();

        let description = match self.description {
            Some(ref d) => d,
            None => &empty,
        };
        let reference = match self.reference {
            Some(ref d) => format!("\n### Reference\n{}", d),
            None => "".to_owned(),
        };

        tokens.extend(quote! {
            #[doc = concat!(#description, #reference)]
        });
    }
}

pub fn doc(map: &yaml::Hash) -> Result<DocSpec> {
    let description = get_attr!(map; "doc" as Yaml::String(s) => s)
        .context("doc: doc is not a string")?
        .cloned();

    let reference = get_attr!(map; "doc-ref" as Yaml::String(s) => s)
        .context("doc: doc-ref is not a string")?
        .cloned();

    Ok(DocSpec {
        description,
        reference,
    })
}
