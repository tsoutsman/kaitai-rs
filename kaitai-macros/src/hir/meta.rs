pub use crate::de::meta::Endianness;

pub struct Meta {
    pub endianness: Endianness,
}

impl From<Endianness> for &'static str {
    fn from(e: Endianness) -> Self {
        match e {
            Endianness::Le => "le",
            Endianness::Be => "be",
        }
    }
}
