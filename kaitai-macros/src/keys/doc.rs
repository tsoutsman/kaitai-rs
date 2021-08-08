use crate::utils::{get_attribute, MacroError, Result};

use proc_macro2::TokenStream;
use quote::quote;
use yaml_rust::{yaml, Yaml};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DocSpec {
    /// A description of a user-defined type. It is used as documentation for the Rust struct
    /// representing the type.
    pub description: Option<String>,
    /// A reference to the original documentation (if the ksy file is an implementation of some
    /// documented format).
    pub reference: Option<String>,
}

fn get_doc(map: &yaml::Hash) -> Result<DocSpec> {
    let description = match get_attribute!(map | "doc" as Yaml::String(s) => s) {
        Ok(d) => Some(d.clone()),
        Err(e) => match e {
            MacroError::InvalidAttrType { .. } => return Err(e),
            MacroError::RequiredAttrNotFound(_) => None,
            _ => unreachable!(),
        },
    };

    let reference = match get_attribute!(map | "doc-ref" as Yaml::String(s) => s) {
        Ok(d) => Some(d.clone()),
        Err(e) => match e {
            MacroError::InvalidAttrType { .. } => return Err(e),
            MacroError::RequiredAttrNotFound(_) => None,
            _ => unreachable!(),
        },
    };

    Ok(DocSpec {
        description,
        reference,
    })
}

pub fn gen_doc_comment(map: &yaml::Hash) -> Result<TokenStream> {
    let doc = get_doc(map)?;

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
