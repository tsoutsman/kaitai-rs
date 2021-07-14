// The contents of this file are **heavily** inspired by https://github.com/kaitai-io/kaitai_struct_rust_runtime.
// Although this file is not a copy-paste, without their work this would have been much harder.

use std::io::{self, Read, Seek, SeekFrom};

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
        // #[doc = concat!(" Reads in a little endian ", stringify!($rust_type), " (KS: ", stringify!($letter), stringify!($size), ")")]
        fn [<read_ $letter $size le>](&mut self) -> ::std::io::Result<$rust_type> {
            use ::byteorder::ReadBytesExt;
            self.[<read_ $rust_type>]::<::byteorder::LittleEndian>()
        }
        // This doc comment becomes stable in Rust 1.54: 2021-07-29
        // #[doc = concat!(" Reads in a big endian ", stringify!($rust_type), " (KS: ", stringify!($letter), stringify!($size), ")")]
        fn [<read_ $letter $size be>](&mut self) -> ::std::io::Result<$rust_type> {
            use ::byteorder::ReadBytesExt;
            self.[<read_ $rust_type>]::<::byteorder::BigEndian>()
        }
        )*
    }
    };
}

pub(crate) trait KaitaiStream: Read + Seek {
    // The trait doesn't require a close method as buffers are automatically closed on drop.
    // The trait doesn't require a seek method as it is already implemented by std::io::Seek.

    fn is_eof(&mut self) -> io::Result<bool> {
        // TODO: benchmark against:
        // let pos = self.pos()?;
        // let size = self.seek(SeekFrom::End(0))?;
        // self.seek(SeekFrom::Start(pos))?;
        // Ok(pos >= size)
        let mut buf = [0u8; 1];
        let result = self.read(&mut buf).map(|n| n == 0);
        self.seek(SeekFrom::Current(-1))?;
        result
    }

    fn pos(&mut self) -> io::Result<u64> {
        self.stream_position()
    }

    fn size(&mut self) -> io::Result<u64> {
        // TODO: unstable feature:
        // #![feature(seek_stream_len)]
        // self.stream_len()
        let pos = self.pos()?;
        let size = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(pos))?;
        Ok(size)
    }

    fn read_bytes(&mut self, count: usize) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(count);
        self.read_exact(&mut buffer[..]).map(|_| buffer)
    }

    fn read_bytes_full(&mut self) -> io::Result<Vec<u8>> {
        // TODO: benchmark against:
        // let mut buffer = vec![0; 0];
        let mut buffer = Vec::with_capacity(self.size()? as usize);
        self.read_to_end(&mut buffer).map(|_| buffer)
    }

    /// Reads bytes up to a terminator.
    ///
    /// If include_term is true then the terminator will be included in the returned bytes. If
    /// consume_term is true then the current position in the buffer will be set to the next byte
    /// after the terminator, otherwise it will be set to the terminator.
    fn read_bytes_term(
        &mut self,
        term: char,
        include_term: bool,
        consume_term: bool,
    ) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let temp_buffer = [0u8; 1];

        while !(temp_buffer[0] as char == term) {
            let mut temp_buffer = [0u8; 1];
            let bytes_read = self.read(&mut temp_buffer)?;

            if bytes_read == 0 {
                if eos_error {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("end of stream reached, but no terminator {} found", term),
                    ));
                }
            }
            // TODO: unstable feature:
            // #![feature(extend_one)]
            // buffer.extend_one(temp_buffer[0]);
            buffer.extend_from_slice(&temp_buffer);
        }
        if include_term {
            // TODO: same as above
            buffer.extend_from_slice(&temp_buffer);
        }
        if !consume_term {
            self.seek(SeekFrom::Current(-1))?;
        }
        Ok(buffer)
    }

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
