use ed25519_dalek::Signer;
use serde_json::json;

/// Builds signed compliance receipts.
pub struct SignedReceiptBuilder {
    signing_key: ed25519_dalek::SigningKey,
}

impl SignedReceiptBuilder {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let key = ed25519_dalek::SigningKey::generate(&mut rng);
        Self { signing_key: key }
    }

    pub fn sign(&self, aat_record: &super::aat_formatter::AATRecord) -> String {
        let payload = serde_json::to_vec(aat_record).unwrap();
        let sig = self.signing_key.sign(&payload);
        format!("sig:{}", hex::encode(sig.to_bytes()))
    }
}
