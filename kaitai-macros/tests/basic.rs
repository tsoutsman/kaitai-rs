use kaitai_macros::include_kaitai;

#[test]
fn basic_syntax() {
    include_kaitai!("kaitai-macros/tests/ksy/gltf_binary.ksy");
}
