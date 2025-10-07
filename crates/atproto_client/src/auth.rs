// SPDX-License-Identifier: MIT OR Apache-2.0
//! Authentication with ATproto services

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,
    
    pub did: String,
    pub handle: String,
}

/// Authenticate with ATproto service and get access token
pub async fn create_session(identifier: &str, password: &str) -> Result<SessionResponse> {
    let url = "https://bsky.social/xrpc/com.atproto.server.createSession";
    
    let request = CreateSessionRequest {
        identifier: identifier.to_string(),
        password: password.to_string(),
    };
    
    log::debug!("Creating session for {}", identifier);
    
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(|e| Error::Network(e))?;
    
    let status = response.status();
    
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(Error::HandleResolution(format!(
            "Authentication failed (HTTP {}): {}",
            status, error_text
        )));
    }
    
    let session: SessionResponse = response.json().await
        .map_err(|e| Error::Parse(format!("Failed to parse session response: {}", e)))?;
    
    log::info!("Successfully authenticated as {}", session.handle);
    
    Ok(session)
}

