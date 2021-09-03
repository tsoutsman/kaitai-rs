use crate::{
    error::Error,
    keys::{
        doc::gen_doc_comment,
        meta::{get_meta, MetaSpec},
        seq::{gen_field_assignments, gen_field_defs},
    },
    util::{get_attr, sc_to_ucc},
};

use anyhow::{Context, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use yaml_rust::{yaml, Yaml};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeInfo<'a> {
    /// The hash map containing the necessary data to generate the type.
    pub map: &'a yaml::Hash,
    /// The ident that should be used for the generated struct.
    pub ident: proc_macro2::Ident,
    /// The attributes that should be applied to the generated struct.
    pub attrs: Vec<syn::Attribute>,
    /// The desired visibility of the generated struct.
    pub visibility: syn::Visibility,
    /// The meta information that applies to the type. The field is always set to
    /// [`Some`](Option::Some) if the [`TypeOptions`] is describing a user-defined type (i.e. any
    /// type specified in the `TypesSpec` of the ksy file).
    pub inherited_meta: Option<MetaSpec>,
}

pub fn get_types(info: &TypeInfo<'_>) -> Result<Vec<(Ident, yaml::Hash)>> {
    let map = info.map;
    let types = match get_attr!(map; "types" as Yaml::Hash(h) => h).context("get_types")? {
        Some(t) => t,
        None => return Ok(Vec::new()),
    };
    let mut result = Vec::new();

    for (name, ty) in types {
        let ident = match name {
            Yaml::String(s) => Ident::new(&sc_to_ucc(s), Span::call_site()),
            _ => {
                return Err(Error::InvalidAttrType {
                    attr: "type name".to_owned(),
                    pat: "Yaml::String(s)".to_owned(),
                    actual: name.clone(),
                })
                .context("get_types")
            }
        };
        match ty {
            Yaml::Hash(h) => result.push((ident, h.clone())),
            _ => {
                return Err(Error::InvalidAttrType {
                    attr: "type".to_owned(),
                    pat: "Yaml::Hash(h)".to_owned(),
                    actual: ty.clone(),
                })
                .context("get_types")
            }
        }
    }

    Ok(result)
}

pub fn gen_type_defs(info: &TypeInfo<'_>) -> Result<Vec<TokenStream>> {
    let types = get_types(info).context("get_type_defs")?;
    let mut result = Vec::new();

    for (ident, ty) in types {
        let meta = get_meta(info).context("gen_type_defs")?;
        let child_info = TypeInfo {
            map: &ty,
            ident,
            attrs: Vec::new(),
            visibility: info.visibility.clone(),
            inherited_meta: Some(meta),
        };

        result.push(gen_type(&child_info).context("gen_type_defs")?);
    }

    Ok(result)
}

/// Function that generates a Rust struct definition from a `yaml::Hash` in the format of a KSY
/// `TypeSpec`. The function is called recursively on all the types of the given `TypeSpec`. The
/// `struct`s generated by the function are public.
pub fn gen_type(info: &TypeInfo<'_>) -> Result<TokenStream> {
    let map = info.map;
    let type_defs = gen_type_defs(&TypeInfo {
        map,
        // The child inherits the parents `MetaSpec`.
        inherited_meta: Some(
            info.inherited_meta
                .clone()
                .unwrap_or(get_meta(info).context("gen_type")?),
        ),
        ..info.clone()
    })
    .context("gen_type")?;

    let doc_comment = gen_doc_comment(map).context("gen_type")?;

    let attrs: Vec<proc_macro2::TokenStream> =
        info.attrs.iter().map(|a| a.to_token_stream()).collect();

    let visibility = &info.visibility;

    let ident = &info.ident;

    let field_defs = gen_field_defs(map).context("gen_type")?;

    let field_assignments = gen_field_assignments(info).context("gen_type")?;

    let result = quote! {
        #(#type_defs)*

        #doc_comment
        #(#attrs)*
        #visibility struct #ident {
            #(#field_defs,)*
        }

        // The `automatically_derived` attribute is applied to implementations generated by derive
        // macros. Since this `kaitai_source` isn't a derive macro, the `automatically_derived`
        // attributed must be applied manually.
        #[automatically_derived]
        impl ::kaitai::runtime::KaitaiStruct for #ident {
            fn new<S: ::kaitai::runtime::KaitaiStream>(buf: &mut S) -> ::kaitai::Result<Self> {
                Ok(#ident {
                    #(#field_assignments,)*
                })
            }
            fn read<S: ::kaitai::runtime::KaitaiStream>(&mut self, _: &mut S) -> ::kaitai::Result<()> {
                todo!();
            }
        }
    };
    Ok(result)
}
