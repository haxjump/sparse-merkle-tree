use crate::{traits::Hasher, H256};

#[derive(Default)]
pub struct Blake3Hasher(blake3::Hasher);

impl Hasher for Blake3Hasher {
    #[inline(always)]
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }

    #[inline(always)]
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }

    #[inline(always)]
    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(self.0.finalize().as_bytes());
        hash.into()
    }

    #[inline(always)]
    fn hash(bytes: &[u8]) -> H256 {
        let mut hasher = Self::default();
        hasher.0.update(bytes);

        let mut hash = [0u8; 32];
        hash.copy_from_slice(hasher.0.finalize().as_bytes());
        hash.into()
    }
}
