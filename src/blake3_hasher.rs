use crate::{traits::Hasher, H256};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Blake3Hasher {
    #[serde(skip)]
    hasher: blake3::Hasher,
}

impl Hasher for Blake3Hasher {
    #[inline(always)]
    fn write_h256(&mut self, h: &H256) {
        self.hasher.update(h.as_slice());
    }

    #[inline(always)]
    fn write_byte(&mut self, b: u8) {
        self.hasher.update(&[b][..]);
    }

    #[inline(always)]
    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(self.hasher.finalize().as_bytes());
        hash.into()
    }

    #[inline(always)]
    fn hash(bytes: &[u8]) -> H256 {
        let mut bh = Self::default();
        bh.hasher.update(bytes);

        let mut hash = [0u8; 32];
        hash.copy_from_slice(bh.hasher.finalize().as_bytes());
        hash.into()
    }
}

impl fmt::Debug for Blake3Hasher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Blake3Hasher").finish()
    }
}
