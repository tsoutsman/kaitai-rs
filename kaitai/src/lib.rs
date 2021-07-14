//! This crate is still very much a work in progress; it does not work.
//!
//! A macro for compiling Kaitai Struct into Rust.
//!
//! # Syntax
//! ```ignore
//! # use kaitai::include_kaitai;
//! include_kaitai!("filepath");
//! ```
//! # Semantics
//! The filepath is taken relative to the project's root directory.
// missing_docs,
#![warn(
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]

mod errors;
mod runtime;

//
// Public exports
//

pub use kaitai_macros::include_kaitai;

#[doc(hidden)]
pub use errors::{KaitaiError, Result};
#[doc(hidden)]
pub use runtime::stream::KaitaiStream;
