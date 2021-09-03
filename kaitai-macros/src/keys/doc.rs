use crate::util::get_attr;

use anyhow::{Context, Result};
use proc_macro2::TokenStream;
use quote::quote;
use yaml_rust::{yaml, Yaml};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct DocSpec {
    /// A description of a user-defined type. It is used as documentation for the Rust struct
    /// representing the type.
    pub description: Option<String>,
    /// A reference to the original documentation (if the ksy file is an implementation of some
    /// documented format).
    pub reference: Option<String>,
}

fn get_doc(map: &yaml::Hash) -> Result<DocSpec> {
    let description = get_attr!(map; "doc" as Yaml::String(s) => s)
        .context("get_doc")?
        .cloned();

    let reference = get_attr!(map; "doc-ref" as Yaml::String(s) => s)
        .context("get_doc")?
        .cloned();

    Ok(DocSpec {
        description,
        reference,
    })
}

pub fn gen_doc_comment(map: &yaml::Hash) -> Result<TokenStream> {
    let doc = get_doc(map).context("gen_doc_comment")?;

    if doc.description.is_none() && doc.reference.is_none() {
        return Ok(TokenStream::new());
    }

    let description = match doc.description {
        Some(d) => d,
        None => "".to_owned(),
    };
    let reference = match doc.reference {
        Some(d) => {
            let mut result = String::new();
            result.push_str("\n### Reference\n");
            result.push_str(&d);
            result
        }
        None => "".to_owned(),
    };

    Ok(quote! {
        #[doc = concat!(#description, #reference)]
    })
}
