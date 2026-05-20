/// Immutable append‑only audit ledger.
pub struct AuditLog {
    entries: Vec<String>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn append(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }
}
