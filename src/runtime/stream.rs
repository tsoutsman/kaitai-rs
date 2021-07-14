// The contents of this file are **heavily** inspired by https://github.com/kaitai-io/kaitai_struct_rust_runtime.
// Although this file is not a copy-paste, without their work this would have been much harder.

use std::io::{self, Read, Seek};

use byteorder::ReadBytesExt;

/// A macro that generates functions to read Kaitai Struct specified integers and convert
/// them into Rust types.
/// # Use
/// ```
/// # trait Example: std::io::Read + std::io::Seek {
/// // s is the letter used by Kaitai Struct, [2, 4] are the numbers used by Kaitai Struct,
/// // and [i32, i64] are the Rust types that the Kaitai Struct types (i.e. s2, s4) map to.
/// generate_read_functions!(s; [2, 4] => [i32, i64]);
/// # }
/// ```
macro_rules! generate_read_functions {
    ($letter:ident; [$($size:literal),+$(,)?] => [$($rust_type:ty),+$(,)?]) => {
        ::paste::paste! {
        $(
        // This doc comment becomes stable in Rust 1.54: 2021-07-29
        // #[doc = concat!(" Read in a little endian ", stringify!($rust_type), " (KS: ", stringify!($letter), stringify!($size), ")")]
        fn [<read_ $letter $size le>](&mut self) -> ::std::io::Result<$rust_type> {
            use ::byteorder::ReadBytesExt;
            self.[<read_ $rust_type>]::<::byteorder::LittleEndian>()
        }
        // This doc comment becomes stable in Rust 1.54: 2021-07-29
        // #[doc = concat!(" Read in a big endian ", stringify!($rust_type), " (KS: ", stringify!($letter), stringify!($size), ")")]
        fn [<read_ $letter $size be>](&mut self) -> ::std::io::Result<$rust_type> {
            use ::byteorder::ReadBytesExt;
            self.[<read_ $rust_type>]::<::byteorder::BigEndian>()
        }
        )*
    }
    };
}

pub(crate) trait KaitaiStream: Read + Seek {
    // generate_read_functions can't generate u1 => u8 and s1 => i8 as they don't have an Endian
    // generic. Guess this works as additional documentation for how the macro works :)
    /// Read in a u8 (KS: u1)
    fn read_u1(&mut self) -> io::Result<u8> {
        self.read_u8()
    }
    /// Read in an i8 (KS: s1)
    fn read_s1(&mut self) -> io::Result<i8> {
        self.read_i8()
    }
    generate_read_functions!(u; [2, 4, 8] => [u16, u32, u64]);
    generate_read_functions!(s; [2, 4, 8] => [i16, i32, i64]);
    generate_read_functions!(f; [4, 8] => [f32, f64]);
}
