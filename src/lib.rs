//! This crate is still very much a work in progress; it does not work.
//!
//! A macro for compiling Kaitai Struct into Rust.
//! # Syntax
//! ```
//! # use kaitai::include_kaitai;
//! include_kaitai!("filename");
//! ```
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs;

#[proc_macro]
pub fn include_kaitai(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(item as syn::Expr);

    let filename = match ast {
     syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(ref s) , ..}) => s.value(),
        _ => panic!("invalid input"),
    };
    eprintln!("{:#?}", std::env::current_dir().unwrap());

    let file_contents = fs::read_to_string(filename).expect("error reading file: ");
    let structure = &YamlLoader::load_from_str(&file_contents).expect("error parsing file: ")[0];

    eprintln!("{:#?}", structure);
    todo!()
}