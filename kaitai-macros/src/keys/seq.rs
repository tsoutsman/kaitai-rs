use crate::{
    error::Error,
    keys::{meta::get_meta, types::TypeInfo},
    util::{get_attr, get_required_attr, sc_to_ucc},
};

use anyhow::{Context, Result};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Ident;
use yaml_rust::{yaml, Yaml};

use super::meta::MetaSpec;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Attribute {
    pub id: Ident,
    pub ks_type: String,
    pub enum_ident: Option<String>,
}

#[derive(Clone, Debug)]
pub enum TypeDef {
    BuiltIn(TokenStream),
    UserDefined(TokenStream),
    Enum(TokenStream),
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            TypeDef::BuiltIn(t) => tokens.extend(std::iter::once(t.clone())),
            TypeDef::UserDefined(t) => tokens.extend(std::iter::once(t.clone())),
            TypeDef::Enum(t) => tokens.extend(std::iter::once(t.clone())),
        };
    }
}

impl Attribute {
    pub fn rust_type(&self) -> TypeDef {
        if let Some(i) = &self.enum_ident {
            return TypeDef::Enum(Ident::new(&sc_to_ucc(i), Span::call_site()).to_token_stream());
        }

        match &self.ks_type[..] {
            "u1" => TypeDef::BuiltIn(quote! { u8 }),
            "u2" => TypeDef::BuiltIn(quote! { u16 }),
            "u4" => TypeDef::BuiltIn(quote! { u32 }),
            "u8" => TypeDef::BuiltIn(quote! { u64 }),
            "s1" => TypeDef::BuiltIn(quote! { i8 }),
            "s2" => TypeDef::BuiltIn(quote! { i16 }),
            "s4" => TypeDef::BuiltIn(quote! { i32 }),
            "s8" => TypeDef::BuiltIn(quote! { i64 }),
            "f4" => TypeDef::BuiltIn(quote! { f32 }),
            "f8" => TypeDef::BuiltIn(quote! { f64 }),
            // The type is a user-defined type, meaning a struct has been generated somewhere with
            // the name in ucc.
            &_ => TypeDef::UserDefined(
                Ident::new(&sc_to_ucc(&self.ks_type), Span::call_site()).to_token_stream(),
            ),
        }
    }

    pub fn definition(&self) -> TokenStream {
        let id = &self.id;
        let ty = self.rust_type();
        quote! { pub #id: #ty }
    }

    fn endianness(&self, meta: &MetaSpec) -> String {
        match &self.ks_type[..] {
            "u1" | "s1" => "".to_owned(),
            _ => meta.endianness.to_string(),
        }
    }

    pub fn assignment(&self, meta: &MetaSpec) -> TokenStream {
        let mut assignment = format!("{}: ", self.id);

        match self.rust_type() {
            TypeDef::BuiltIn(_) => {
                assignment += &format!("buf.read_{}{}()?", self.ks_type, self.endianness(meta));
            }
            TypeDef::UserDefined(t) => {
                // Generates something like: "CustomType::new(buf)?"
                // We are banking on the fact that this type is defined as a subtype
                // in the ksy file and that its name will be the same.
                assignment += &format!("{}::new(buf)?", t);
            }
            TypeDef::Enum(t) => {
                assignment += &format!(
                    "{}::n(buf.read_{}{}()?).ok_or(::kaitai::error::Error::NoEnumMatch)?",
                    t,
                    self.ks_type,
                    self.endianness(meta)
                );
            }
        }

        // TODO handle unwrap
        assignment.parse().unwrap()
    }
}

fn get_seq(map: &yaml::Hash) -> Result<Vec<Attribute>> {
    let seq = get_required_attr!(map; "seq" as Yaml::Array(a) => a).context("get_seq")?;
    let mut result = Vec::new();

    for item in seq {
        result.push(match item {
            Yaml::Hash(h) => Attribute {
                id: get_required_attr!(h; "id" as Yaml::String(s) => Ident::new(s, Span::call_site()))
                    .context("get_seq")?,
                ks_type: get_required_attr!(h; "type" as Yaml::String(s) => s.clone()).context("get_seq")?,
                enum_ident: get_attr!(h; "enum" as Yaml::String(s) => s.clone()).context("get_seq")?,
            },
            _ => {
                return Err(Error::InvalidAttribute(
                    item.clone(),
                )).context("get_seq")
            }
        });
    }

    Ok(result)
}

pub fn gen_field_defs(map: &yaml::Hash) -> Result<Vec<TokenStream>> {
    Ok(get_seq(map)
        .context("gen_field_defs")?
        .iter()
        .map(|a| a.definition())
        .collect())
}

pub fn gen_field_assignments(info: &TypeInfo<'_>) -> Result<Vec<TokenStream>> {
    let meta = get_meta(info)?;
    Ok(get_seq(info.map)?
        .iter()
        .map(|a| a.assignment(&meta))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_pattern;
    use yaml_rust::YamlLoader;

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
            result.unwrap_err().downcast_ref::<Error>().unwrap(),
            &Error::RequiredAttrNotFound("seq".to_owned())
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
            result.unwrap_err().downcast_ref::<Error>().unwrap(),
            &Error::RequiredAttrNotFound("type".to_owned())
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
            result.unwrap_err().downcast_ref::<Error>().unwrap(),
            &Error::InvalidAttrType {
                attr: "id".to_owned(),
                pat: "Yaml::String(s)".to_owned(),
                actual: Yaml::Hash(yaml::Hash::new()),
            }
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
            result.unwrap(),
            vec![
                Attribute {
                    id: Ident::new("example_id", Span::call_site()),
                    ks_type: "example_type".to_owned(),
                    enum_ident: None
                };
                2
            ]
        );
    }

    #[test]
    fn attribute_enum() {
        let map = &YamlLoader::load_from_str(
            "
seq:
  - id: protocol
    enum: ip_protocol
  - id: another_thing
    enum: enum_id\0",
        )
        .unwrap()[0];
        let _map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        // let result = gen_field_assignments(map).unwrap();
        // eprintln!("RESULT: {:?}", result);
    }
}
