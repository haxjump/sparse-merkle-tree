use crate::{
    error::Error,
    tree::{BranchKey, BranchNode},
    H256,
};
use std::result::Result as StdResult;
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

impl<H> Value<H> for pt11::H256 {
    fn to_h256(&self) -> H256 {
        <H256 as Value<H>>::to_h256(&H256::from(self))
    }
}

impl<H> Value<H> for pt11::H160 {
    fn to_h256(&self) -> H256 {
        <H256 as Value<H>>::to_h256(&H256::from(self))
    }
}

impl<H> Value<H> for pt10::H256 {
    fn to_h256(&self) -> H256 {
        <H256 as Value<H>>::to_h256(&H256::from(self))
    }
}

impl<H> Value<H> for pt10::H160 {
    fn to_h256(&self) -> H256 {
        <H256 as Value<H>>::to_h256(&H256::from(self))
    }
}

// impl<T: ValueEn, H: Hasher> Value<H> for T {
//     fn to_h256(&self) -> H256 {
//         H::hash(&self.encode_value()[..])
//     }
// }

// impl<T: AsRef<[u8]>, H: Hasher> Value<H> for T {
//     fn to_h256(&self) -> H256 {
//         H::hash(self.as_ref())
//     }
// }

/// Trait for customize backend storage
pub trait Store<V>: VsMgmt {
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> StdResult<(), Error>;
    fn remove_branch(&mut self, node_key: &BranchKey) -> StdResult<(), Error>;
    fn get_branch(&self, branch_key: &BranchKey) -> StdResult<Option<BranchNode>, Error>;

    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> StdResult<(), Error>;
    fn remove_leaf(&mut self, leaf_key: &H256) -> StdResult<(), Error>;
    fn get_leaf(&self, leaf_key: &H256) -> StdResult<Option<V>, Error>;

    fn update_root(&mut self, new_root: H256) -> StdResult<(), Error>;
    fn get_root(&self) -> StdResult<H256, Error>;
}

/// Trait for customize backend storage,
/// useful in some double-key scenes.
pub trait Store2<X, V>: VsMgmt {
    fn insert_branch(
        &mut self,
        xid: &X,
        node_key: BranchKey,
        branch: BranchNode,
    ) -> StdResult<(), Error>;
    fn remove_branch(&mut self, xid: &X, node_key: &BranchKey) -> StdResult<(), Error>;
    fn get_branch(&self, xid: &X, branch_key: &BranchKey) -> StdResult<Option<BranchNode>, Error>;

    fn insert_leaf(&mut self, xid: &X, leaf_key: H256, leaf: V) -> StdResult<(), Error>;
    fn remove_leaf(&mut self, xid: &X, leaf_key: &H256) -> StdResult<(), Error>;
    fn get_leaf(&self, xid: &X, leaf_key: &H256) -> StdResult<Option<V>, Error>;

    // Remove all data under the xid(top-level key).
    fn remove_x(&mut self, xid: &X) -> StdResult<(), Error>;

    fn update_root(&mut self, xid: &X, new_root: H256) -> StdResult<(), Error>;
    fn get_root(&self, xid: &X) -> StdResult<H256, Error>;
}
