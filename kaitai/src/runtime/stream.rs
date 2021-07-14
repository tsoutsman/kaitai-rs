// The contents of this file are **heavily** inspired by https://github.com/kaitai-io/kaitai_struct_rust_runtime.
// Although this file is not a copy-paste, without their work this would have been much harder.
use crate::{KaitaiError, Result};

use std::io::{Read, Seek, SeekFrom};

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
        // NIGHTLY FEATURE
         #[doc = concat!(" Reads in a little endian ", stringify!($rust_type), " (KS: ", stringify!($letter), stringify!($size), ")")]
        fn [<read_ $letter $size le>](&mut self) -> $crate::Result<$rust_type> {
            use ::byteorder::ReadBytesExt;
            self.[<read_ $rust_type>]::<::byteorder::LittleEndian>().map_err(|e| e.into())
        }
        // NIGHTLY FEATURE
        #[doc = concat!(" Reads in a big endian ", stringify!($rust_type), " (KS: ", stringify!($letter), stringify!($size), ")")]
        fn [<read_ $letter $size be>](&mut self) -> $crate::Result<$rust_type> {
            use ::byteorder::ReadBytesExt;
            self.[<read_ $rust_type>]::<::byteorder::BigEndian>().map_err(|e| e.into())
        }
        )*
    }
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TerminatorFlags {
    Include,
    Consume,
}

pub trait KaitaiStream: Read + Seek {
    // The trait doesn't require a close method as buffers are automatically closed on drop.
    // The trait doesn't require a seek method as it is already implemented by std::io::Seek.

    fn is_eof(&mut self) -> Result<bool> {
        // TODO: benchmark against:
        // let pos = self.pos()?;
        // let size = self.seek(SeekFrom::End(0))?;
        // self.seek(SeekFrom::Start(pos))?;
        // Ok(pos >= size)
        let mut buf = [0u8; 1];
        let result = self.read(&mut buf).map(|n| n == 0);
        self.seek(SeekFrom::Current(-1))?;
        result.map_err(|e| e.into())
    }

    fn pos(&mut self) -> Result<u64> {
        self.stream_position().map_err(|e| e.into())
    }

    fn size(&mut self) -> Result<u64> {
        // let pos = self.pos()?;
        // let size = self.seek(SeekFrom::End(0))?;
        // self.seek(SeekFrom::Start(pos))?;
        // Ok(size)
        // NIGHTLY FEATURE
        self.stream_len().map_err(|e| e.into())
    }

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; count];

        match self.read_exact(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e.into()),
        }
    }

    /// Read the remaining bytes in the stream.
    fn read_bytes_full(&mut self) -> Result<Vec<u8>> {
        // TODO: benchmark against:
        // let mut buffer = vec![0; 0];
        let mut buffer = Vec::with_capacity(self.size()? as usize);

        match self.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e.into()),
        }
    }

    /// Read bytes up to a terminator.
    ///
    /// The Include flag determines whether the terminator is included in the return value. If the
    /// Consumed flag is set, the stream points to the character after the terminator, otherwise
    /// it points to the terminator.
    fn read_bytes_term(&mut self, term: char, flags: &[TerminatorFlags]) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();

        loop {
            let mut temp_buffer = [0u8; 1];
            let bytes_read = self.read(&mut temp_buffer)?;

            if bytes_read == 0 {
                return Err(KaitaiError::EofBeforeTerminator(term));
            }

            if temp_buffer[0] as char == term {
                if flags.contains(&TerminatorFlags::Include) {
                    // buffer.extend_from_slice(&temp_buffer);
                    // NIGHTLY FEATURE
                    buffer.extend_one(temp_buffer[0]);
                } else if !flags.contains(&TerminatorFlags::Consume) {
                    self.seek(SeekFrom::Current(-1))?;
                }
                return Ok(buffer);
            }

            // buffer.extend_from_slice(&temp_buffer);
            // NIGHTLY FEATURE
            buffer.extend_one(temp_buffer[0]);
        }
    }

    fn ensure_fixed_contents(&mut self, expected: Vec<u8>) -> Result<()> {
        let mut buf = vec![0; expected.len()];
        match self.read_exact(&mut buf) {
            Ok(_) => {
                if buf == expected {
                    Ok(())
                } else {
                    Err(KaitaiError::UnexpectedFileContents {
                        actual: buf,
                        expected,
                    })
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    // generate_read_functions can't generate u1 => u8 and s1 => i8 as they don't have an endian
    // generic. Guess this works as additional documentation for how the macro works :)

    /// Read in a u8 (KS: u1)
    fn read_u1(&mut self) -> Result<u8> {
        self.read_u8().map_err(|e| e.into())
    }

    /// Read in an i8 (KS: s1)
    fn read_s1(&mut self) -> Result<i8> {
        self.read_i8().map_err(|e| e.into())
    }

    generate_read_functions!(u; [2, 4, 8] => [u16, u32, u64]);
    generate_read_functions!(s; [2, 4, 8] => [i16, i32, i64]);
    generate_read_functions!(f; [4, 8] => [f32, f64]);
}

impl<T: Read + Seek> KaitaiStream for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn new_buf() -> Cursor<Vec<u8>> {
        Cursor::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn is_eof() {
        let mut buf = new_buf();

        buf.seek(SeekFrom::End(0)).unwrap();
        assert!(buf.is_eof().unwrap());

        buf.seek(SeekFrom::Current(-3)).unwrap();
        assert!(!buf.is_eof().unwrap());
    }

    #[test]
    fn pos() {
        let mut buf = new_buf();

        assert_eq!(buf.pos().unwrap(), 0);

        buf.seek(SeekFrom::Current(4)).unwrap();
        assert_eq!(buf.pos().unwrap(), 4);

        buf.seek(SeekFrom::End(0)).unwrap();
        assert_eq!(buf.pos().unwrap(), 10);
    }

    #[test]
    fn size() {
        let mut buf = new_buf();

        assert_eq!(buf.size().unwrap(), 10)
    }

    #[test]
    fn read_bytes() {
        let mut buf = new_buf();

        assert_eq!(vec![0, 1], buf.read_bytes(2).unwrap());
        assert_eq!(vec![2, 3, 4], buf.read_bytes(3).unwrap());
    }

    #[test]
    fn read_bytes_full() {
        let mut buf = new_buf();

        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            buf.read_bytes_full().unwrap()
        );
    }

    #[test]
    fn read_bytes_term() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(
            vec![0, 1, 2],
            buf.read_bytes_term('\u{3}', &[TerminatorFlags::Consume])
                .unwrap()
        );
        assert_eq!(vec![4, 5], buf.read_bytes_term('\u{6}', &[]).unwrap());
        assert_eq!(
            vec![6, 7],
            buf.read_bytes_term(
                '\u{7}',
                &[TerminatorFlags::Include, TerminatorFlags::Consume]
            )
            .unwrap()
        );
        assert_eq!(
            vec![8],
            buf.read_bytes_term('\u{8}', &[TerminatorFlags::Include])
                .unwrap()
        );
        assert!(buf.read_bytes_term('\u{15}', &[]).is_err());
    }

    #[test]
    fn ensure_fixed_contents() {
        let mut buf = new_buf();

        assert!(buf.ensure_fixed_contents(vec![0, 1, 2]).is_ok());
        assert!(buf.ensure_fixed_contents(vec![3, 4]).is_ok());
        buf.seek(SeekFrom::Current(1));
        assert!(buf.ensure_fixed_contents(vec![6, 7, 8]).is_ok());
        assert!(buf.ensure_fixed_contents(vec![8, 9, 10]).is_err());
    }

    macro_rules! test_read_integer {
        ($name:ident, $value:expr) => {
            #[test]
            fn $name() {
                let mut buf = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
                assert_eq!(buf.$name().unwrap(), $value);
            }
        };
    }

    test_read_integer!(read_u1, 1);
    test_read_integer!(read_s1, 1);

    test_read_integer!(read_s2le, 513);
    test_read_integer!(read_s2be, 258);
    test_read_integer!(read_u2le, 513);
    test_read_integer!(read_u2be, 258);

    test_read_integer!(read_s4le, 67305985);
    test_read_integer!(read_s4be, 16909060);
    test_read_integer!(read_u4le, 67305985);
    test_read_integer!(read_u4be, 16909060);

    test_read_integer!(read_s8le, 578437695752307201);
    test_read_integer!(read_s8be, 72623859790382856);
    test_read_integer!(read_u8le, 578437695752307201);
    test_read_integer!(read_u8be, 72623859790382856);

    #[test]
    fn read_f4le() {
        let mut buf = Cursor::new(vec![0, 0, 128, 62]);
        assert_eq!(buf.read_f4le().unwrap(), 0.25);
    }

    #[test]
    fn read_f4be() {
        let mut buf = Cursor::new(vec![62, 128, 0, 0]);
        assert_eq!(buf.read_f4be().unwrap(), 0.25);
    }

    #[test]
    fn read_f8le() {
        let mut buf = Cursor::new(vec![0, 0, 0, 0, 0, 0, 208, 63]);
        assert_eq!(buf.read_f8le().unwrap(), 0.25);
    }

    #[test]
    fn read_f8be() {
        let mut buf = Cursor::new(vec![63, 208, 0, 0, 0, 0, 0, 0]);
        assert_eq!(buf.read_f8be().unwrap(), 0.25);
    }
}
