//! This crate is still very much a work in progress; it does not work.
//!
//! A macro for compiling Kaitai Struct into Rust.
//!
//! # Example
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
//! The rust code to read a `basic_be` buffer would look something like this.
//! ```
//! # use kaitai::{Result, kaitai_source, runtime::KaitaiStruct};
//! #[kaitai_source("../tests/formats/basic_be.ksy")]
//! struct BasicBigEndian;
//!
//! fn main() -> Result<()> {
//!     let file = BasicBigEndian::from_file("tests/files/example.basic")?;
//!
//!     println!("head: {}", file.body);
//!     println!("body: {}", file.body);
//!     println!("tail: {}", file.body);
//!     # Ok(())
//! }
//! ```
//! # Semantics
//! The filepath provided to `kaitai_source` is taken relative to the current file, similarly to how
//! modules are found. However, the filepath provided to `from_file` is taken relative to the root
//! of the directory, like [`std::fs::File::open`].
#![feature(extend_one)]
#![feature(seek_stream_len)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
#![allow(dead_code)]

mod errors;

//
// Public exports
//

pub use errors::{KaitaiError, Result};

#[doc(inline)]
pub use kaitai_macros::kaitai_source;

pub mod runtime;
