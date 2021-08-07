use kaitai::{kaitai_source, runtime::KaitaiStruct};

#[kaitai_source("formats/basic_be.ksy")]
struct BasicBigEndian;

#[kaitai_source("formats/basic_le.ksy")]
#[derive(Debug)]
struct BasicLittleEndian;

#[test]
fn basic_big_endian() {
    let file = BasicBigEndian::from_file("tests/files/example.basic").unwrap();
    assert_eq!(file.header, 0x50_4b);
    assert_eq!(file.body, 0x03_04_14_00_02_00_00_00);
    assert_eq!(file.tail, 0x02_5d_5e_49);
}

#[test]
fn basic_little_endian() {
    let file = BasicLittleEndian::from_file("tests/files/example.basic").unwrap();
    assert_eq!(file.header, 0x4b_50);
    assert_eq!(file.body, 0x02_00_14_04_03);
    assert_eq!(file.tail, 0x49_5e_5d_02);
}

/// If `kaitai_source` properly passes through other attributes, then `BasicLittleEndian` should have
/// `Debug` derived.
#[test]
fn other_attributes_untouched() {
    let file = BasicLittleEndian::from_file("tests/files/example.basic").unwrap();
    println!("{:#?}", file);
}
