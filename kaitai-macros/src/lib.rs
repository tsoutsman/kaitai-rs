#![feature(proc_macro_span)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]

mod meta;
mod parse;
mod seq;
mod utils;

use meta::parse_meta;
use parse::get_ks_source;
use seq::parse_seq;
use utils::get_attribute;

use std::path::Path;

use yaml_rust::Yaml;

// Since it gets re-exported in kaitai, crate-level refers to kaitai not kaitai-macros.
/// See crate-level documentation on how to use macro.
#[tarpaulin::skip]
#[proc_macro_derive(KaitaiStruct, attributes(kaitai_source))]
pub fn derive_kaitai_struct(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);

    let filename = match get_ks_source(&ast) {
        Some(f) => f,
        None => panic!("no source file specified"),
    };

    // Span::call_site() is a nightly feature.
    let mut source_file_path = proc_macro::Span::call_site().source_file().path();
    source_file_path.pop();
    let file_path = source_file_path.join(Path::new(&filename));

    let file_contents = std::fs::read_to_string(file_path).expect("error reading file: ");
    let structure =
        &yaml_rust::YamlLoader::load_from_str(&file_contents).expect("error parsing file: ")[0];

    let map = match structure {
        Yaml::Hash(hm) => hm,
        _ => panic!("file does not have the correct structure"),
    };

    let meta = get_attribute!(map | "meta" as Yaml::Hash(s) => s).expect("could not fetch meta: ");
    let seq = get_attribute!(map | "seq" as Yaml::Array(a) => a).expect("could not fetch seq: ");

    let _parsed_meta = parse_meta(meta);
    let _parsed_seq = parse_seq(seq);

    proc_macro::TokenStream::new()
}
