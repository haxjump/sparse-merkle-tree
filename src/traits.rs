use crate::{
    error::Error,
    tree::{BranchKey, BranchNode},
    H256,
};
use vsdb::VsMgmt;

/// Trait for customize hash function
pub trait Hasher: Default {
    fn write_h256(&mut self, h: &H256);
    fn write_byte(&mut self, b: u8);
    fn finish(self) -> H256;
    fn hash(bytes: &[u8]) -> H256;
}

/// Trait for define value structures
pub trait Value<H> {
    fn to_h256(&self) -> H256;
}

impl<H> Value<H> for H256 {
    fn to_h256(&self) -> H256 {
        *self
    }
}

impl<T: AsRef<[u8]>, H: Hasher> Value<H> for T {
    fn to_h256(&self) -> H256 {
        H::hash(self.as_ref())
    }
}

/// Trait for customize backend storage
pub trait Store<V>: VsMgmt {
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error>;
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error>;
    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error>;

    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> Result<(), Error>;
    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error>;
    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error>;

    fn update_root(&mut self, new_root: H256) -> Result<(), Error>;
    fn get_root(&self) -> Result<H256, Error>;
}
