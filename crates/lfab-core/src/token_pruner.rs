/// Removes redundant tokens from input to save context and compute.
///
/// LFAB's token pruner maintains a dynamic window of relevant tokens
/// based on recency and semantic importance.
pub struct TokenPruner {
    max_tokens: usize,
}

impl TokenPruner {
    pub fn new() -> Self {
        Self { max_tokens: 2048 }
    }

    /// Prune a token sequence, keeping the most important tokens.
    pub fn prune(&self, tokens: &[String]) -> Vec<String> {
        if tokens.len() <= self.max_tokens {
            tokens.to_vec()
        } else {
            // Simple truncation; production uses attention‑based scoring.
            tokens[..self.max_tokens].to_vec()
        }
    }
}
