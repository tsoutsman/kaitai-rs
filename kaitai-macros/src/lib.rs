//! Please see the main [kaitai](https://www.crates.io/crates/kaitai) crate.
#![deny(
    non_ascii_idents,
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
#![feature(proc_macro_span, register_tool)]
#![register_tool(tarpaulin)]

mod error;
mod keys;
mod util;

use keys::*;

use std::path::Path;

use syn::parse_macro_input;
use yaml_rust::Yaml;

// Since it gets re-exported in kaitai, crate-level refers to kaitai not kaitai-macros.
/// See crate-level documentation for information on how to use this macro.
#[tarpaulin::skip]
#[proc_macro_attribute]
pub fn kaitai_source(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ks_source_path = parse_macro_input!(args as syn::LitStr);
    let item_ast = parse_macro_input!(item as syn::Item);

    let struct_item = match item_ast {
        syn::Item::Struct(s) => s,
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

    let result = match structure {
        Yaml::Hash(map) => types::gen_type(&types::TypeInfo {
            map,
            ident: struct_item.ident,
            attrs: struct_item.attrs,
            visibility: struct_item.vis,
            inherited_meta: None,
        }),
        _ => panic!("file does not have the correct structure"),
    };

    match result {
        Ok(t) => t.into(),
        Err(e) => panic!("{:?}", e),
    }
}
