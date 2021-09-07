use kaitai::{kaitai_source, KaitaiStruct};

#[kaitai_source("formats/fixed.ksy")]
pub struct Fixed;

#[test]
fn fixed_ok() {
    let mut input: Vec<u8> = Vec::new();
    input.extend(b"test string");
    input.extend(&[0xab, 0xad, 0xba, 0xbe]);
    input.extend(&[0x12, 0x34]);

    let result = Fixed::from_bytes(&input).unwrap();

    assert_eq!(result.variable, 0x1234);
}

#[test]
fn fixed_err() {
    let mut input: Vec<u8> = Vec::new();
    input.extend(b"test_string");
    input.extend(&[0xab, 0xad, 0xba, 0xbe]);

    assert!(Fixed::from_bytes(&input).is_err());

    let mut input: Vec<u8> = Vec::new();
    input.extend(b"test string");
    input.extend(&[0xbb, 0xad, 0xba, 0xbe]);

    assert!(Fixed::from_bytes(&input).is_err());
}
