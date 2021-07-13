use kaitai::include_kaitai;

#[test]
fn basic_syntax() {
    include_kaitai!("tests/ksy/gltf_binary.ksy");
}
