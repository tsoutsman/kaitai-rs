//! A macro for compiling Kaitai Struct into Rust.
//! # Syntax
//! ```
//! # use kaitai::include_kaitai;
//! include_kaitai!("filename");
//! ```

#[proc_macro]
pub fn include_kaitai(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}