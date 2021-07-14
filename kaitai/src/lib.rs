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
//! The file is located relative to the current file (similarly to how modules are found).
// missing_docs,
#![feature(extend_one)]
#![feature(seek_stream_len)]
#![warn(
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
#![allow(dead_code)]

mod errors;
mod runtime;

//
// Public exports
//

#[doc(inline)]
pub use kaitai_macros::include_kaitai;

pub(crate) use errors::{KaitaiError, Result};
