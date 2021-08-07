use crate::utils::get_attribute;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;
use yaml_rust::Yaml;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct SeqItem {
    pub id: Ident,
    pub ks_type: String,
}

impl SeqItem {
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

pub(crate) fn parse_seq(seq: &[Yaml]) -> Vec<SeqItem> {
    seq.iter()
        .map(|item| match item {
            Yaml::Hash(h) => SeqItem {
                id:
                // TODO actually give the correct span
                    get_attribute!(h | "id" as Yaml::String(s) => Ident::new(s, Span::call_site()))
                        .expect("error fetching seq > id: "),
                ks_type: get_attribute!(h | "type" as Yaml::String(s) => s.clone())
                    .expect("error fetching seq > type: "),
            },
            _ => panic!(),
        })
        .collect()
}
