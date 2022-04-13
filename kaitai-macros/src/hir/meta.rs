pub use crate::de::meta::Endianness;

impl From<Endianness> for &'static str {
    fn from(e: Endianness) -> Self {
        match e {
            Endianness::Le => "le",
            Endianness::Be => "be",
        }
    }
}
