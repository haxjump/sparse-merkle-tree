//!
//! Constructs a new `SparseMerkleTree<H, V, S>`.
//!

pub mod blake3_hasher;
pub mod default_store;
pub mod error;
pub mod h256;
pub mod merge;
pub mod merkle_proof;
pub mod traits;
pub mod tree;

#[cfg(test)]
mod tests;

pub use h256::H256;
pub use merkle_proof::{CompiledMerkleProof, MerkleProof};
pub use tree::SparseMerkleTree;

/// Expected path size: log2(256) * 2, used for hint vector capacity
pub const EXPECTED_PATH_SIZE: usize = 16;

// Max stack size can be used when verify compiled proof
pub(crate) const MAX_STACK_SIZE: usize = 257;

/// An out-of-the-box implementation.
pub type VsSmt<V> =
    SparseMerkleTree<blake3_hasher::Blake3Hasher, V, default_store::DefaultStore<V>>;
