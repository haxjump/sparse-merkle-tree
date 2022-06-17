use crate::{
    chg_store,
    error::Error,
    traits::{Store, Store2},
    tree::{BranchKey, BranchNode},
    H256,
};
use ruc::*;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
#[cfg(test)]
use vsdb::VsMgmt;
use vsdb::{
    BranchName, KeyEnDe, MapxDkVs, MapxVs, OrphanVs, ValueEnDe, VersionName, Vs,
};

#[derive(Vs, Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct DefaultStore<V: ValueEnDe> {
    root: OrphanVs<H256>,
    branches_map: MapxVs<BranchKey, BranchNode>,
    leaves_map: MapxVs<H256, V>,
}

impl<V: ValueEnDe> Default for DefaultStore<V> {
    #[cfg(not(test))]
    #[inline(always)]
    fn default() -> Self {
        Self {
            root: OrphanVs::new(),
            branches_map: MapxVs::new(),
            leaves_map: MapxVs::new(),
        }
    }

    #[cfg(test)]
    #[inline(always)]
    fn default() -> Self {
        let mut ds = Self {
            root: OrphanVs::new(),
            branches_map: MapxVs::new(),
            leaves_map: MapxVs::new(),
        };

        pnk!(ds.version_create((&[0u8; 0][..]).into()));

        ds
    }
}

impl<V: ValueEnDe> Store<V> for DefaultStore<V> {
    #[inline(always)]
    fn insert_branch(
        &mut self,
        branch_key: BranchKey,
        branch: BranchNode,
    ) -> StdResult<(), Error> {
        chg_store!(self.branches_map.insert(&branch_key, &branch));
        Ok(())
    }

    #[inline(always)]
    fn remove_branch(&mut self, branch_key: &BranchKey) -> StdResult<(), Error> {
        chg_store!(self.branches_map.remove(branch_key));
        Ok(())
    }

    #[inline(always)]
    fn get_branch(
        &self,
        branch_key: &BranchKey,
    ) -> StdResult<Option<BranchNode>, Error> {
        Ok(self.branches_map.get(branch_key))
    }

    #[inline(always)]
    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> StdResult<(), Error> {
        chg_store!(self.leaves_map.insert(&leaf_key, &leaf));
        Ok(())
    }

    #[inline(always)]
    fn remove_leaf(&mut self, leaf_key: &H256) -> StdResult<(), Error> {
        chg_store!(self.leaves_map.remove(leaf_key));
        Ok(())
    }

    #[inline(always)]
    fn get_leaf(&self, leaf_key: &H256) -> StdResult<Option<V>, Error> {
        Ok(self.leaves_map.get(leaf_key))
    }

    #[inline(always)]
    fn get_leaf_by_branch(
        &self,
        leaf_key: &H256,
        br: BranchName,
    ) -> StdResult<Option<V>, Error> {
        Ok(self.leaves_map.get_by_branch(leaf_key, br))
    }

    #[inline(always)]
    fn get_leaf_by_branch_version(
        &self,
        leaf_key: &H256,
        br: BranchName,
        ver: VersionName,
    ) -> StdResult<Option<V>, Error> {
        Ok(self.leaves_map.get_by_branch_version(leaf_key, br, ver))
    }

    #[inline(always)]
    fn update_root(&mut self, new_root: H256) -> StdResult<(), Error> {
        chg_store!(self.root.set_value(&new_root));
        Ok(())
    }

    #[inline(always)]
    fn get_root(&self) -> StdResult<H256, Error> {
        Ok(self.root.get_value().unwrap_or_else(H256::zero))
    }
}

#[derive(Vs, Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct DefaultStore2<X: KeyEnDe, V: ValueEnDe> {
    root: MapxVs<X, H256>,
    branches_map: MapxDkVs<X, BranchKey, BranchNode>,
    leaves_map: MapxDkVs<X, H256, V>,
}

impl<X: KeyEnDe, V: ValueEnDe> Default for DefaultStore2<X, V> {
    #[cfg(not(test))]
    #[inline(always)]
    fn default() -> Self {
        Self {
            root: MapxVs::new(),
            branches_map: MapxDkVs::new(),
            leaves_map: MapxDkVs::new(),
        }
    }

    #[cfg(test)]
    #[inline(always)]
    fn default() -> Self {
        let mut ds = Self {
            root: MapxVs::new(),
            branches_map: MapxDkVs::new(),
            leaves_map: MapxDkVs::new(),
        };

        pnk!(ds.version_create((&[0u8; 0][..]).into()));

        ds
    }
}

impl<X: KeyEnDe, V: ValueEnDe> Store2<X, V> for DefaultStore2<X, V> {
    #[inline(always)]
    fn insert_branch(
        &mut self,
        xid: &X,
        node_key: BranchKey,
        branch: BranchNode,
    ) -> StdResult<(), Error> {
        chg_store!(self.branches_map.insert(&(xid, &node_key), &branch));
        Ok(())
    }

    #[inline(always)]
    fn remove_branch(&mut self, xid: &X, node_key: &BranchKey) -> StdResult<(), Error> {
        chg_store!(self.branches_map.remove(&(xid, Some(node_key))));
        Ok(())
    }

    #[inline(always)]
    fn get_branch(
        &self,
        xid: &X,
        branch_key: &BranchKey,
    ) -> StdResult<Option<BranchNode>, Error> {
        Ok(self.branches_map.get(&(xid, branch_key)))
    }

    #[inline(always)]
    fn insert_leaf(&mut self, xid: &X, leaf_key: H256, leaf: V) -> StdResult<(), Error> {
        chg_store!(self.leaves_map.insert(&(xid, &leaf_key), &leaf));
        Ok(())
    }

    #[inline(always)]
    fn remove_leaf(&mut self, xid: &X, leaf_key: &H256) -> StdResult<(), Error> {
        chg_store!(self.leaves_map.remove(&(xid, Some(leaf_key))));
        Ok(())
    }

    #[inline(always)]
    fn get_leaf(&self, xid: &X, leaf_key: &H256) -> StdResult<Option<V>, Error> {
        Ok(self.leaves_map.get(&(xid, leaf_key)))
    }

    #[inline(always)]
    fn get_leaf_by_branch(
        &self,
        xid: &X,
        leaf_key: &H256,
        br: BranchName,
    ) -> StdResult<Option<V>, Error> {
        Ok(self.leaves_map.get_by_branch(&(xid, leaf_key), br))
    }

    #[inline(always)]
    fn get_leaf_by_branch_version(
        &self,
        xid: &X,
        leaf_key: &H256,
        br: BranchName,
        ver: VersionName,
    ) -> StdResult<Option<V>, Error> {
        Ok(self
            .leaves_map
            .get_by_branch_version(&(xid, leaf_key), br, ver))
    }

    // Remove all data under the xid(top-level key).
    #[inline(always)]
    fn remove_x(&mut self, xid: &X) -> StdResult<(), Error> {
        chg_store!(self.root.remove(xid));
        chg_store!(self.branches_map.remove(&(xid, None)));
        chg_store!(self.leaves_map.remove(&(xid, None)));
        Ok(())
    }

    #[inline(always)]
    fn update_root(&mut self, xid: &X, new_root: H256) -> StdResult<(), Error> {
        chg_store!(self.root.insert(xid, &new_root));
        Ok(())
    }

    #[inline(always)]
    fn get_root(&self, xid: &X) -> StdResult<H256, Error> {
        Ok(self.root.get(xid).unwrap_or_else(H256::zero))
    }
}

///////////////////////////
//////// Test only ////////
///////////////////////////

impl<V: ValueEnDe> DefaultStore<V> {
    #[cfg(test)]
    #[inline(always)]
    pub(crate) fn branches_map(&self) -> &MapxVs<BranchKey, BranchNode> {
        &self.branches_map
    }

    #[cfg(test)]
    #[inline(always)]
    pub(crate) fn leaves_map(&self) -> &MapxVs<H256, V> {
        &self.leaves_map
    }
}
