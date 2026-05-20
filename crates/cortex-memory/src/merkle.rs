use sha2::{Sha256, Digest};

/// Merkle integrity verification for memory layers.
pub struct MerkleIntegrity;

impl MerkleIntegrity {
    pub fn new() -> Self { Self }

    /// Compute a Merkle root from a set of leaf hashes.
    pub fn compute_root(&self, leaves: &[String]) -> String {
        if leaves.is_empty() {
            return String::new();
        }
        let mut hashes: Vec<String> = leaves.iter().map(|l| {
            hex::encode(Sha256::digest(l.as_bytes()))
        }).collect();

        while hashes.len() > 1 {
            let mut next = Vec::with_capacity((hashes.len() + 1) / 2);
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    chunk[0].clone()
                };
                next.push(hex::encode(Sha256::digest(combined.as_bytes())));
            }
            hashes = next;
        }
        hashes[0].clone()
    }
}
