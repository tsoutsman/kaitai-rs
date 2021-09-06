// The contents of this file are **heavily** inspired by https://github.com/kaitai-io/kaitai_struct_rust_runtime.
// Although this file is not a copy-paste, without their work this would have been much harder.
use crate::{error::Result, runtime::KaitaiStream};

/// The trait that is implemented by the [kaitai_source](crate::kaitai_source) macro.
pub trait KaitaiStruct
where
    Self: Sized,
{
    // TODO fix the documentation for this function and `from_bytes`. I can't find the correct
    // terms to use.
    /// Create an instance of a `KaitaiStruct` format from a file, relative to the root
    /// of the project.
    fn from_file(path: &str) -> Result<Self> {
        let mut f = std::fs::File::open(path)?;
        Self::new(&mut f)
    }

    /// Create an instance of a `KaitaiStruct` format from an array of bytes.
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut b = std::io::Cursor::new(bytes);
        Self::new(&mut b)
    }

    #[doc(hidden)]
    fn new<S: KaitaiStream>(stream: &mut S) -> Result<Self>;

    #[doc(hidden)]
    fn read<S: KaitaiStream>(&mut self, stream: &mut S) -> Result<()>;
}
