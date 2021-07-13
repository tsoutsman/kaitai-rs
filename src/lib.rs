//! This crate is still very much a work in progress; it does not work.
//!
//! A macro for compiling Kaitai Struct into Rust.
//!
//! # Syntax
//! ```no_run
//! # use kaitai::include_kaitai;
//! include_kaitai!("filepath");
//! ```
//! # Semantics
//! The filepath is taken relative to the project's root directory.
#![deny(warnings)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![warn(missing_docs)]
#![warn(unused)]

/// See crate-level documentation on how to use macro.
#[proc_macro]
pub fn include_kaitai(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let cloned = item.clone();
    let ast = syn::parse_macro_input!(item as syn::Expr);

    let filename = match ast {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(ref s),
            ..
        }) => s.value(),
        _ => panic!("invalid input"),
    };

    let file_contents = std::fs::read_to_string(filename).expect("error reading file: ");
    let _structure =
        &yaml_rust::YamlLoader::load_from_str(&file_contents).expect("error parsing file: ")[0];

    cloned
}
