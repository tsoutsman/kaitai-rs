use kaitai::{kaitai_source, runtime::KaitaiStruct};

#[kaitai_source("formats/user_type_be.ksy")]
struct UserTypeBigEndian;

#[test]
fn user_type() {
    let file = UserTypeBigEndian::from_file("tests/files/example.png").unwrap();

    assert_eq!(file.header.seq_1, 0x89504E47);
    assert_eq!(file.header.seq_2, 0xd0a1a0a0000000d);

    assert_eq!(file.body.seq_1, 0x494844520000012c);
    assert_eq!(file.body.seq_2, 0x0);

    assert_eq!(file.tail.seq_1, 0x12c0802);
    assert_eq!(file.tail.seq_2, 0xf61f192200);
}
