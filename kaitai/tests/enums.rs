use kaitai::{kaitai_source, KaitaiStruct};

#[kaitai_source("formats/enums.ksy")]
struct EnumsStruct;

#[test]
fn main() {
    let e = EnumsStruct::from_bytes(&[1, 6, 17]).unwrap();
    assert_eq!(e.protocol, IpProtocol::Icmp);
    assert_eq!(e.protocol2, IpProtocol::Tcp);
    assert_eq!(e.protocol3, IpProtocol::Udp);
}
