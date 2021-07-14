use crate::{runtime::stream::KaitaiStream, Result};

pub trait KaitaiFormat {
    /// Create a KaitaiObject from a file, relative to the root of the project.
    fn from_file(path: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let mut f = std::fs::File::open(path)?;
        Self::new(&mut f, None, None)
    }

    /// Create a KaitaiObject from a file, relative to the root of the project.
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let mut b = std::io::Cursor::new(bytes);
        Self::new(&mut b, None, None)
    }

    fn new<S: KaitaiStream>(
        stream: &mut S,
        parent: Option<&dyn KaitaiFormat>,
        root: Option<&dyn KaitaiFormat>,
    ) -> Result<Self>
    where
        Self: Sized;

    fn read<S: KaitaiStream>(
        &mut self,
        stream: &mut S,
        parent: Option<&dyn KaitaiFormat>,
        root: Option<&dyn KaitaiFormat>,
    ) -> Result<()>
    where
        Self: Sized;
}
