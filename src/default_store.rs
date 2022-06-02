use crate::{
    error::Error,
    traits::Store,
    tree::{BranchKey, BranchNode},
    H256,
};
use ruc::*;
use std::result::Result as StdResult;
use vsdb::{MapxVs, OrphanVs, ValueEnDe, Vs, VsMgmt};

#[derive(Vs, Debug, Clone)]
pub struct DefaultStore<V: ValueEnDe> {
    root: OrphanVs<H256>,
    branches_map: MapxVs<BranchKey, BranchNode>,
    leaves_map: MapxVs<H256, V>,
}

impl<V: ValueEnDe> Default for DefaultStore<V> {
    fn default() -> Self {
        let ds = Self {
            root: OrphanVs::new(),
            branches_map: MapxVs::new(),
            leaves_map: MapxVs::new(),
        };
        pnk!(ds.version_create((&[0u8; 0][..]).into()));
        pnk!(ds.root.set_value(H256::zero()));
        ds
    }
}

impl<V: ValueEnDe> DefaultStore<V> {
    #[inline(always)]
    pub fn branches_map(&self) -> &MapxVs<BranchKey, BranchNode> {
        &self.branches_map
    }

    #[inline(always)]
    pub fn leaves_map(&self) -> &MapxVs<H256, V> {
        &self.leaves_map
    }
}

macro_rules! chg_store {
    ($op: expr) => {
        if let Err(e) = $op.c(d!()) {
            return Err(Error::Store(e.to_string()));
        }
    };
}

impl<V: ValueEnDe> Store<V> for DefaultStore<V> {
    fn get_branch(&self, branch_key: &BranchKey) -> StdResult<Option<BranchNode>, Error> {
        Ok(self.branches_map.get(branch_key))
    }

    fn get_leaf(&self, leaf_key: &H256) -> StdResult<Option<V>, Error> {
        Ok(self.leaves_map.get(leaf_key))
    }

    fn insert_branch(&mut self, branch_key: BranchKey, branch: BranchNode) -> StdResult<(), Error> {
        chg_store!(self.branches_map.insert(branch_key, branch));
        Ok(())
    }

    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> StdResult<(), Error> {
        chg_store!(self.leaves_map.insert(leaf_key, leaf));
        Ok(())
    }

    fn remove_branch(&mut self, branch_key: &BranchKey) -> StdResult<(), Error> {
        chg_store!(self.branches_map.remove(branch_key));
        Ok(())
    }

    fn remove_leaf(&mut self, leaf_key: &H256) -> StdResult<(), Error> {
        chg_store!(self.leaves_map.remove(leaf_key));
        Ok(())
    }

    fn update_root(&mut self, new_root: H256) -> StdResult<(), Error> {
        chg_store!(self.root.set_value(new_root));
        Ok(())
    }

    fn get_root(&self) -> StdResult<H256, Error> {
        match self.root.get_value().c(d!()) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::Store(e.to_string())),
        }
    }
}
