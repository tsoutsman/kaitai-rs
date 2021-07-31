use kaitai_macros::*;

#[derive(KaitaiStruct)]
#[kaitai_source("ksy/gltf_binary.ksy")]
struct Temp;

#[test]
fn ok() {
    println!("Hello");
}
