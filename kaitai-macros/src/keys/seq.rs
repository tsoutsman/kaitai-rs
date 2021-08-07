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
    pub fn rust_type(&self) -> TokenStream {
        match &self.ks_type[..] {
            "u1" => quote! { u8 },
            "u2" => quote! { u16 },
            "u4" => quote! { u32 },
            "u8" => quote! { u64 },
            "s1" => quote! { i8 },
            "s2" => quote! { i16 },
            "s4" => quote! { i32 },
            "s8" => quote! { i64 },
            "f4" => quote! { f32 },
            "f8" => quote! { f64 },
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ys {
        ($lit:literal) => {
            Yaml::String($lit.to_owned())
        };
    }

    #[test]
    fn no_seq() {
        let input = yaml::Hash::new();

        let result = get_seq(&input);

        assert_eq!(result, Err(MacroError::AttrNotFound("seq".to_owned())));
    }

    #[test]
    fn missing_type() {
        let mut attr = yaml::Hash::new();
        attr.insert(ys!("id"), ys!("example_id"));

        let seq = vec![Yaml::Hash(attr)];

        let mut input = yaml::Hash::new();
        input.insert(ys!("seq"), Yaml::Array(seq));

        let result = get_seq(&input);

        assert_eq!(result, Err(MacroError::AttrNotFound("type".to_owned())));
    }

    #[test]
    fn wrong_id_type() {
        let mut attr = yaml::Hash::new();
        attr.insert(ys!("id"), Yaml::Hash(yaml::Hash::new()));
        attr.insert(ys!("type"), ys!("example_type"));

        let seq = vec![Yaml::Hash(attr)];

        let mut input = yaml::Hash::new();
        input.insert(ys!("seq"), Yaml::Array(seq));

        let result = get_seq(&input);

        assert_eq!(
            result,
            Err(MacroError::InvalidAttrType {
                attr: "id".to_owned(),
                pat: "Yaml::String(s)".to_owned()
            })
        );
    }

    #[test]
    fn all_attributes() {
        let mut attr = yaml::Hash::new();
        attr.insert(ys!("id"), ys!("example_id"));
        attr.insert(ys!("type"), ys!("example_type"));

        let seq = vec![Yaml::Hash(attr); 2];

        let mut input = yaml::Hash::new();
        input.insert(ys!("seq"), Yaml::Array(seq));

        let result = get_seq(&input);

        assert_eq!(
            result,
            Ok(vec![
                Attribute {
                    id: Ident::new("example_id", Span::call_site()),
                    ks_type: "example_type".to_owned()
                };
                2
            ])
        );
    }
}
