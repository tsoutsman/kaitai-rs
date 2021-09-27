# kaitai

[![Crates.io](https://img.shields.io/crates/v/kaitai.svg)](https://crates.io/crates/kaitai)
[![Documentation](https://docs.rs/kaitai/badge.svg)](https://docs.rs/kaitai/)
[![Workflow Status](https://github.com/TypicalFork/kaitai-rs/workflows/CI/badge.svg)](https://github.com/TypicalFork/kaitai-rs/actions?query=workflow%3A%22CI%22)

This crate is still very much a work in progress; it has a **very** limited feature set.

A macro for compiling Kaitai Struct into Rust.

## Example
Given the following file `basic_be.ksy`:
```yaml
meta:
  id: basic
  endian: be
seq:
  - id: header
    type: u2
  - id: body
    type: s8
  - id: tail
    type: u4
```
The rust code to read a `basic_be` buffer would look something like this:
```rust
#[kaitai_source("../tests/formats/basic_be.ksy")]
struct BasicBigEndian;

fn main() -> Result<()> {
    let file = BasicBigEndian::from_file("tests/files/example.basic")?;

    println!("header: {}", file.header);
    println!("body: {}", file.body);
    println!("tail: {}", file.tail);
}
```
## Semantics
The filepath provided to `kaitai_source` is taken relative to the current file, similarly to how
modules are found. However, the filepath provided to `from_file` is taken relative to the root
of the project, like `std::fs::File::open`.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
