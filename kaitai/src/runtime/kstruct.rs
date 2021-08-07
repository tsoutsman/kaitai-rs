// The contents of this file are **heavily** inspired by https://github.com/kaitai-io/kaitai_struct_rust_runtime.
// Although this file is not a copy-paste, without their work this would have been much harder.
use crate::{runtime::KaitaiStream, Result};

// TODO fixed the Sizable issue :)

/// The trait that is implemented by all structs created from a ksy file.
pub trait KaitaiStruct {
    /// Create a KaitaitStruct from a file, relative to the root of the project.
    fn from_file(path: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let mut f = std::fs::File::open(path)?;
        Self::from(&mut f, None, None)
    }

    /// Create a KaitaiStruct from a file, relative to the root of the project.
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let mut b = std::io::Cursor::new(bytes);
        Self::from(&mut b, None, None)
    }

    #[doc(hidden)]
    fn from<S: KaitaiStream>(
        stream: &mut S,
        parent: Option<&dyn KaitaiStruct>,
        root: Option<&dyn KaitaiStruct>,
    ) -> Result<Self>
    where
        Self: Sized;

    #[doc(hidden)]
    fn read<S: KaitaiStream>(
        &mut self,
        stream: &mut S,
        parent: Option<&dyn KaitaiStruct>,
        root: Option<&dyn KaitaiStruct>,
    ) -> Result<()>
    where
        Self: Sized;
}
