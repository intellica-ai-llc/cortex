use ed25519_dalek::{SigningKey, Signature, Signer as DalekSigner};

/// Ed25519 signing context.
pub struct Signer {
    key: SigningKey,
}

impl Signer {
    pub fn new(key_bytes: [u8; 32]) -> Self {
        Self { key: SigningKey::from_bytes(&key_bytes) }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.key.sign(message)
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }
}
