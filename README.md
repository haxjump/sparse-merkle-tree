![GitHub top language](https://img.shields.io/github/languages/top/ccmlm/xSMT)
[![Latest Version](https://img.shields.io/crates/v/xSMT.svg)](https://crates.io/crates/xSMT)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/xSMT)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.60+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)

# xSMT

An implementation of 'Sparse Merkle Tree' with versioned features.

## Examples

```rust
type Smt = xsmt::VsSmt<Vec<u8>>;
let smt = Smt::default();
...

type Smt = xsmt::VsSmt2<Vec<u8>, Vec<u8>>;
let smt = Smt::default();
...
```

SEE ALSO:
- [**VSDB**](https://crates.io/crates/vsdb), a database with powerful version management capabilities
- [**sparse_merkle_tree**](https://crates.io/crates/sparse_merkle_tree), the original upstream crate
