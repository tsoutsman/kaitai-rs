use kaitai::kaitai_source;

#[kaitai_source("formats/basic_be.ksy")]
struct BasicBigEndian;

#[kaitai_source("formats/basic_le.ksy")]
struct BasicLittleEndian;

fn main() {
    // let file = BasicBigEndian::from_file("files/example.basic");
}
