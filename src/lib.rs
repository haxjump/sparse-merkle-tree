//!
//! Constructs a new `SparseMerkleTree<H, V, S>`.
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub mod blake3_hasher;
pub mod default_store;
pub mod error;
pub mod h256;
pub mod merge;
pub mod merkle_proof;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod tree;

pub use h256::H256;
pub use merkle_proof::{CompiledMerkleProof, MerkleProof};
pub use tree::SparseMerkleTree;

/// Expected path size: log2(256) * 2, used for hint vector capacity
pub const EXPECTED_PATH_SIZE: usize = 16;
// Max stack size can be used when verify compiled proof
pub(crate) const MAX_STACK_SIZE: usize = 257;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::collections;
        use std::vec;
        use std::string;
    } else {
        extern crate alloc;
        use alloc::collections;
        use alloc::vec;
        use alloc::string;
    }
}
