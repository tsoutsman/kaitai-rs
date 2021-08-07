use crate::utils::{get_attribute, MacroError, Result};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;
use yaml_rust::{yaml, Yaml};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Attribute {
    pub id: Ident,
    pub ks_type: String,
}

impl Attribute {
    #[allow(dead_code)]
    pub fn rust_type(&self) -> TokenStream {
        match &self.ks_type[..] {
            "u1" => quote! {u8},
            "u2" => quote! {u16},
            "u4" => quote! {u32},
            "u8" => quote! {u64},
            "s1" => quote! {i8},
            "s2" => quote! {i16},
            "s4" => quote! {i32},
            "s8" => quote! {i64},
            "f4" => quote! {f32},
            "f8" => quote! {f64},
            &_ => panic!(),
        }
    }
}

pub fn get_seq(map: &yaml::Hash) -> Result<Vec<Attribute>> {
    let seq = get_attribute!(map | "seq" as Yaml::Array(a) => a)?;
    let mut result = Vec::new();

    for item in seq {
        result.push(match item {
            Yaml::Hash(h) => Attribute {
                id: get_attribute!(h | "id" as Yaml::String(s) => Ident::new(s, Span::call_site()))?,
                ks_type: get_attribute!(h | "type" as Yaml::String(s) => s.clone())?,
            },
            _ => return Err(MacroError::InvalidAttribute(item.clone())),
        });
    }

    Ok(result)
}
