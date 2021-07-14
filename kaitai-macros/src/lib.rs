#![feature(proc_macro_span)]
use std::path::Path;

// Since it gets re-exported in kaitai, crate-level refers to kaitai not kaitai-macros.
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

    // Span::call_site() is a nightly feature.
    let mut source_file_path = proc_macro::Span::call_site().source_file().path();
    source_file_path.pop();
    let file_path = source_file_path.join(Path::new(&filename));

    let file_contents = std::fs::read_to_string(file_path).expect("error reading file: ");
    let _structure =
        &yaml_rust::YamlLoader::load_from_str(&file_contents).expect("error parsing file: ")[0];

    cloned
}
