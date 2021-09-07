use kaitai::{kaitai_source, KaitaiStruct};

#[kaitai_source("formats/repeat.ksy")]
#[derive(Debug, PartialEq, Eq)]
pub struct Repeat;

#[test]
fn repeat_eos() {
    let mut input = vec![0xde, 0xad];
    for i in 0..24 {
        input.push(i);
    }

    let expected = vec![
        Temp {
            header: 0x100,
            body: 0x5040302,
            tail: 0x706,
        },
        Temp {
            header: 0x908,
            body: 0xd0c0b0a,
            tail: 0xf0e,
        },
        Temp {
            header: 0x1110,
            body: 0x15141312,
            tail: 0x1716,
        },
    ];

    let result = Repeat::from_bytes(&input).unwrap();

    assert_eq!(result.pre_repeat, 0xadde);
    assert_eq!(result.main, expected);
}
