#![feature(proc_macro_span, register_tool)]
#![register_tool(tarpaulin)]

mod meta;
mod seq;
mod utils;

use meta::parse_meta;
use seq::parse_seq;
use utils::get_attribute;

use std::path::Path;

use quote::quote;
use syn::parse_macro_input;
use yaml_rust::Yaml;

// Since it gets re-exported in kaitai, crate-level refers to kaitai not kaitai-macros.
/// See crate-level documentation on how to use macro.
#[tarpaulin::skip]
#[proc_macro_attribute]
pub fn kaitai_source(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ks_source_path = parse_macro_input!(args as syn::LitStr);
    let item_ast = parse_macro_input!(item as syn::Item);

    let struct_item = match item_ast {
        syn::Item::Struct(ref s) => s,
        _ => {
            // TODO
            panic!("attribute not on struct");
        }
    };

    match &struct_item.fields {
        syn::Fields::Unit => {}
        _ => {
            // TODO
            panic!("struct has fields");
        }
    }

    // // Span::call_site() is a nightly feature.
    let mut source_file_path = proc_macro::Span::call_site().source_file().path();
    source_file_path.pop();
    let file_path = source_file_path.join(Path::new(&ks_source_path.value()));

    let file_contents = std::fs::read_to_string(file_path).expect("error reading file: ");
    let structure =
        &yaml_rust::YamlLoader::load_from_str(&file_contents).expect("error parsing file: ")[0];

    let map = match structure {
        Yaml::Hash(hm) => hm,
        _ => panic!("file does not have the correct structure"),
    };

    let meta = get_attribute!(map | "meta" as Yaml::Hash(s) => s).expect("could not fetch meta: ");
    let seq = get_attribute!(map | "seq" as Yaml::Array(a) => a).expect("could not fetch seq: ");

    let parsed_meta = parse_meta(meta).expect("error parsing meta: ");
    let parsed_seq = parse_seq(seq);

    let fields: Vec<proc_macro2::TokenStream> = parsed_seq
        .iter()
        .map(|field| {
            let id = &field.id;
            quote! { #id }
        })
        .collect();
    let types: Vec<proc_macro2::TokenStream> = parsed_seq
        .iter()
        .map(|field| {
            let ty = field.rust_type();
            quote! { #ty }
        })
        .collect();
    let read_functions: Vec<proc_macro2::TokenStream> = parsed_seq
        .iter()
        .map(|field| {
            let mut func_name = String::new();

            func_name.push_str("read_");
            func_name.push_str(&field.ks_type);
            func_name.push_str(&parsed_meta.endianness.to_string());

            let func_ident = proc_macro2::Ident::new(&func_name, proc_macro2::Span::call_site());
            quote! { #func_ident }
        })
        .collect();

    let struct_ident = &struct_item.ident;
    // TODO carry over the visibility of the struct
    let result = quote! {
        struct #struct_ident {
            #(pub #fields: #types,)*
        }

        impl ::kaitai::runtime::KaitaiStruct for #struct_ident {
            fn from<S: ::kaitai::runtime::KaitaiStream>(buf: &mut S, _: Option<&dyn ::kaitai::runtime::KaitaiStruct>, _: Option<&dyn ::kaitai::runtime::KaitaiStruct>) -> ::kaitai::Result<Self> {
                Ok(#struct_ident {
                    #(#fields: buf.#read_functions()?,)*
                })
            }
            fn read<S: ::kaitai::runtime::KaitaiStream>(&mut self, _: &mut S, _: Option<&dyn ::kaitai::runtime::KaitaiStruct>, _: Option<&dyn ::kaitai::runtime::KaitaiStruct>) -> ::kaitai::Result<()> {
                todo!();
            }
        }
    };

    result.into()
}
