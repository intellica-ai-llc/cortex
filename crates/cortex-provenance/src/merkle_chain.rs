use sha2::{Sha256, Digest};
use std::collections::LinkedList;

/// Hash‑chain integrity builder.
pub struct MerkleChainBuilder {
    leaves: LinkedList<String>,
}

impl MerkleChainBuilder {
    pub fn new() -> Self {
        Self { leaves: LinkedList::new() }
    }

    pub fn append(&mut self, data: &[u8]) {
        let hash = Sha256::digest(data);
        self.leaves.push_back(hex::encode(hash));
    }

    pub fn root(&self) -> Option<String> {
        if self.leaves.is_empty() {
            return None;
        }
        let concatenated: String = self.leaves.iter().fold(String::new(), |acc, h| acc + h);
        Some(hex::encode(Sha256::digest(concatenated.as_bytes())))
    }
}
