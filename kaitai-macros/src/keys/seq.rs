use crate::{
    keys::meta::get_meta,
    utils::{get_attribute, sc_to_ucc, MacroError, Result},
};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Ident;
use yaml_rust::{yaml, Yaml};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Attribute {
    pub id: Ident,
    pub ks_type: String,
}

pub enum TypeDef {
    Inbuilt(TokenStream),
    Custom(TokenStream),
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            TypeDef::Inbuilt(t) => tokens.extend(std::iter::once(t.clone())),
            TypeDef::Custom(t) => tokens.extend(std::iter::once(t.clone())),
        };
    }
}

impl Attribute {
    pub fn rust_type(&self) -> TypeDef {
        match &self.ks_type[..] {
            "u1" => TypeDef::Inbuilt(quote! { u8 }),
            "u2" => TypeDef::Inbuilt(quote! { u16 }),
            "u4" => TypeDef::Inbuilt(quote! { u32 }),
            "u8" => TypeDef::Inbuilt(quote! { u64 }),
            "s1" => TypeDef::Inbuilt(quote! { i8 }),
            "s2" => TypeDef::Inbuilt(quote! { i16 }),
            "s4" => TypeDef::Inbuilt(quote! { i32 }),
            "s8" => TypeDef::Inbuilt(quote! { i64 }),
            "f4" => TypeDef::Inbuilt(quote! { f32 }),
            "f8" => TypeDef::Inbuilt(quote! { f64 }),
            // The type is a user-defined type, meaning a struct has been generated somewhere with
            // the name in ucc.
            &_ => TypeDef::Custom(
                Ident::new(&sc_to_ucc(&self.ks_type), Span::call_site()).to_token_stream(),
            ),
        }
    }
}

fn get_seq(map: &yaml::Hash) -> Result<Vec<Attribute>> {
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

pub fn gen_field_defs(map: &yaml::Hash) -> Result<Vec<TokenStream>> {
    let seq = get_seq(map)?;
    let mut result = Vec::new();

    for attr in seq {
        let id = &attr.id;
        let ty = attr.rust_type();
        result.push(quote! { pub #id: #ty });
    }

    Ok(result)
}

pub fn gen_field_assignments(map: &yaml::Hash) -> Result<Vec<TokenStream>> {
    let meta = get_meta(map)?;
    let seq = get_seq(map)?;
    let mut result = Vec::new();

    for attr in seq {
        let mut func_name = String::new();

        func_name.push_str(&attr.id.to_string());
        func_name.push_str(": ");
        match attr.rust_type() {
            TypeDef::Inbuilt(_) => {
                func_name.push_str("buf.read_");
                func_name.push_str(&attr.ks_type);
                func_name.push_str(&meta.endianness.to_string());
                func_name.push_str("()?");
            }
            TypeDef::Custom(_) => {
                func_name.push_str(&sc_to_ucc(&attr.ks_type));
                func_name.push_str("::from(buf)?");
            }
        }

        // TODO get rid of unwrap
        result.push(func_name.parse().unwrap());
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

        assert_eq!(
            result,
            Err(MacroError::RequiredAttrNotFound("seq".to_owned()))
        );
    }

    #[test]
    fn missing_type() {
        let mut attr = yaml::Hash::new();
        attr.insert(ys!("id"), ys!("example_id"));

        let seq = vec![Yaml::Hash(attr)];

        let mut input = yaml::Hash::new();
        input.insert(ys!("seq"), Yaml::Array(seq));

        let result = get_seq(&input);

        assert_eq!(
            result,
            Err(MacroError::RequiredAttrNotFound("type".to_owned()))
        );
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
                pat: "Yaml::String(s)".to_owned(),
                actual: Yaml::Hash(yaml::Hash::new())
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
