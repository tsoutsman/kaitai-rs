//! Module containing the traits implemented by the [`kaitai_source`](kaitai_macros::kaitai_source) macro.

mod kstruct;
mod stream;

pub use kstruct::KaitaiStruct;
pub use stream::KaitaiStream;
