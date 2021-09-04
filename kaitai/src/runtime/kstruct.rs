// The contents of this file are **heavily** inspired by https://github.com/kaitai-io/kaitai_struct_rust_runtime.
// Although this file is not a copy-paste, without their work this would have been much harder.
use crate::{error::Result, runtime::KaitaiStream};

// TODO fixed the Sizable issue :)

/// The trait that is implemented by all structs created from a ksy file.
pub trait KaitaiStruct
where
    Self: Sized,
{
    /// Create a KaitaitStruct from a file, relative to the root of the project.
    fn from_file(path: &str) -> Result<Self> {
        let mut f = std::fs::File::open(path)?;
        Self::new(&mut f)
    }

    /// Create a KaitaiStruct from a file, relative to the root of the project.
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut b = std::io::Cursor::new(bytes);
        Self::new(&mut b)
    }

    #[doc(hidden)]
    fn new<S: KaitaiStream>(stream: &mut S) -> Result<Self>;

    #[doc(hidden)]
    fn read<S: KaitaiStream>(&mut self, stream: &mut S) -> Result<()>;
}
