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

pub use default_store::{DefaultStore, DefaultStore2};
pub use h256::H256;
pub use merkle_proof::{CompiledMerkleProof, MerkleProof};
pub use traits::*;
pub use tree::{SparseMerkleTree, SparseMerkleTree2};

/// Expected path size: log2(256) * 2, used for hint vector capacity
pub const EXPECTED_PATH_SIZE: usize = 16;

// Max stack size can be used when verify compiled proof
pub(crate) const MAX_STACK_SIZE: usize = 257;

/// An out-of-the-box implementation.
pub type VsSmt<V> =
    SparseMerkleTree<blake3_hasher::Blake3Hasher, V, default_store::DefaultStore<V>>;

/// An out-of-the-box implementation for double-key scene.
pub type VsSmt2<X, V> = SparseMerkleTree2<
    X,
    blake3_hasher::Blake3Hasher,
    V,
    DefaultStore<H256>,
    DefaultStore2<X, V>,
>;

#[macro_export(crate)]
macro_rules! chg_store {
    ($op: expr) => {
        if let Err(e) = $op.c(d!()) {
            return Err(Error::Store(e.to_string()));
        }
    };
}
