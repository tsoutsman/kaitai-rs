# kaitai

[![Crates.io](https://img.shields.io/crates/v/kaitai.svg)](https://crates.io/crates/kaitai)
[![Documentation](https://docs.rs/kaitai/badge.svg)](https://docs.rs/kaitai/)
[![Workflow Status](https://github.com/TypicalFork/kaitai-rs/workflows/CI/badge.svg)](https://github.com/TypicalFork/kaitai-rs/actions?query=workflow%3A%22CI%22)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)
[![dependency status](https://deps.rs/crate/kaitai/0.1.0/status.svg)](https://deps.rs/crate/kaitai/0.1.0)

This crate is still very much a work in progress; it does not work.

A macro for compiling Kaitai Struct into Rust.

## Syntax
```dont_run
# use kaitai::include_kaitai;
include_kaitai!("filename");
```
## Semantics
The filepath is taken relative to the project's root directory.

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
