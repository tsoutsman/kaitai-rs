//! This crate is still very much a work in progress; it has a **very** limited feature set.
//!
//! A macro for compiling Kaitai Struct into Rust.
//!
//! # Example
//!
//! Given the following file `basic_be.ksy`:
//! ```yaml
//! meta:
//!   id: basic
//!   endian: be
//! seq:
//!   - id: header
//!     type: u2
//!   - id: body
//!     type: s8
//!   - id: tail
//!     type: u4
//! ```
//! The rust code to read a `basic_be` buffer would look something like this:
//! ```
//! # use kaitai::{kaitai_source, KaitaiStruct, error::Result};
//! #[kaitai_source("../tests/formats/basic_be.ksy")]
//! struct BasicBigEndian;
//!
//! fn main() -> Result<()> {
//!     let file = BasicBigEndian::from_file("tests/files/example.basic")?;
//!
//!     println!("header: {}", file.header);
//!     println!("body: {}", file.body);
//!     println!("tail: {}", file.tail);
//! #   Ok(())
//! }
//! ```
//!
//! # Semantics
//!
//! The filepath provided to [`kaitai_source`] is taken relative to the current file, similarly to how
//! modules are found. However, the filepath provided to [`from_file`](KaitaiStruct::from_file) is taken relative to the root
//! of the project, like [`std::fs::File::open`].
#![feature(extend_one, seek_stream_len)]
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

pub mod error;

#[doc(inline)]
pub use kaitai_macros::kaitai_source;

mod runtime;
pub use runtime::KaitaiStruct;

#[doc(hidden)]
pub mod __private {
    pub use crate::runtime::KaitaiStream;
}
