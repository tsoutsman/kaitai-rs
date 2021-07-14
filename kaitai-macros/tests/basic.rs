use kaitai_macros::include_kaitai;

#[test]
fn basic_syntax() {
    include_kaitai!("ksy/gltf_binary.ksy");
}
