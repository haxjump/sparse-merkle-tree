use crate::{traits::Hasher, H256};

#[derive(Default)]
pub struct Blake3Hasher(blake3::Hasher);

impl Hasher for Blake3Hasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }

    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }

    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(self.0.finalize().as_bytes());
        hash.into()
    }
}
