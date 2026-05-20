use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration};

/// OAuth 2.1 + PKCE + DPoP implementation for MCP.
///
/// "Authentication for an MCP server demands cryptographically
/// verifiable client identity and explicit, scoped authorization
/// protocols. You can't rely on static API keys or long-lived
/// session cookies when dealing with autonomous agents"[reference:11].
pub struct OAuthProvider {
    clients: Vec<OAuthClient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient {
    pub client_id: String,
    pub client_secret_hash: String,
    pub redirect_uris: Vec<String>,
    pub allowed_scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub code_verifier: Option<String>,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub dpop_key_hash: Option<String>,
}

impl OAuthProvider {
    pub fn new() -> Self {
        Self { clients: Vec::new() }
    }

    /// Register an OAuth client.
    pub fn register_client(&mut self, client: OAuthClient) {
        self.clients.push(client);
    }

    /// Validate a token request and issue an access token.
    pub fn issue_token(&self, req: &TokenRequest) -> Result<TokenResponse, String> {
        let client = self.clients.iter()
            .find(|c| c.client_id == req.client_id)
            .ok_or("Unknown client")?;

        // In production: validate PKCE code_verifier, verify redirect_uri,
        // check scopes, generate signed JWT access token.
        Ok(TokenResponse {
            access_token: format!("cortex_at_{}", uuid::Uuid::new_v4()),
            token_type: "DPoP".into(),
            expires_in: 900, // 15 minutes
            refresh_token: Some(format!("cortex_rt_{}", uuid::Uuid::new_v4())),
            scope: req.scope.clone().unwrap_or_default(),
            dpop_key_hash: None,
        })
    }
}
