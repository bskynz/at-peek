// SPDX-License-Identifier: MIT OR Apache-2.0
//! Labeler service client for querying moderation labels

use crate::{Did, Label, LabelCollection, LabelsResponse, Error, Result};

/// Client for querying labels from a labeler service
#[derive(Clone)]
pub struct LabelerClient {
    client: reqwest::Client,
    labeler_url: String,
    auth_token: Option<String>,
}

impl LabelerClient {
    /// Create a new labeler client with Bluesky's default labeler
    pub fn new() -> Self {
        Self::with_url("https://mod.bsky.app".to_string())
    }
    
    /// Create a new labeler client with authentication
    pub fn new_authenticated(auth_token: String) -> Self {
        Self {
            client: reqwest::Client::builder().build().unwrap_or_default(),
            labeler_url: "https://mod.bsky.app".to_string(),
            auth_token: Some(auth_token),
        }
    }
    
    /// Create a new labeler client with a custom labeler URL
    pub fn with_url(labeler_url: String) -> Self {
        let client = reqwest::Client::builder()
            .build()
            .unwrap_or_default();
        
        Self {
            client,
            labeler_url,
            auth_token: None,
        }
    }
    
    /// Set authentication token
    pub fn with_auth(mut self, auth_token: String) -> Self {
        self.auth_token = Some(auth_token);
        self
    }
    
    /// Query labels for a given DID (user-level labels)
    pub async fn query_labels_for_did(&self, did: &Did) -> Result<LabelCollection> {
        self.query_labels(&[did.as_str().to_string()]).await
    }
    
    /// Query labels for a given AT-URI (post-level labels)
    pub async fn query_labels_for_uri(&self, uri: &str) -> Result<LabelCollection> {
        if !uri.starts_with("at://") {
            return Err(Error::InvalidAtUri("URI must start with at://".to_string()));
        }
        
        self.query_labels(&[uri.to_string()]).await
    }
    
    /// Query labels for multiple subjects (DIDs or AT-URIs)
    pub async fn query_labels(&self, subjects: &[String]) -> Result<LabelCollection> {
        if subjects.is_empty() {
            return Ok(LabelCollection {
                labels: Vec::new(),
                labeler_did: self.labeler_url.clone(),
                query_timestamp: chrono::Utc::now(),
            });
        }
        
        // Build URL with multiple uriPatterns query parameters
        // Note: Each URI must be a separate query parameter, not comma-separated!
        let encoded_patterns: Vec<String> = subjects
            .iter()
            .map(|s| format!("uriPatterns={}", urlencoding::encode(s)))
            .collect();
        let query_string = encoded_patterns.join("&");
        let url = format!(
            "{}/xrpc/com.atproto.label.queryLabels?{}",
            self.labeler_url, query_string
        );
        
        log::debug!("Querying labels from: {}", url);
        
        let mut request = self.client.get(&url);
        
        // Add auth header if token is present
        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| Error::Network(e))?;
        
        let status = response.status();
        
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            
            return Err(Error::RateLimited(retry_after));
        }
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::LabelerUnavailable(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        
        let response_text = response.text().await
            .map_err(|e| Error::Parse(format!("Failed to read response: {}", e)))?;
        
        log::debug!("Raw API response: {}", &response_text[..response_text.len().min(500)]);
        
        let labels_response: LabelsResponse = serde_json::from_str(&response_text)
            .map_err(|e| Error::Parse(format!("Failed to parse label response: {}. Response: {}", e, &response_text[..response_text.len().min(200)])))?;
        
        log::info!("API returned {} total labels (before filtering)", labels_response.labels.len());
        
        // Log all labels before filtering
        for label in &labels_response.labels {
            log::info!("  Raw label: val={}, neg={}, uri={}", label.val, label.neg, label.uri);
        }
        
        // Filter out negated labels
        let active_labels: Vec<Label> = labels_response.labels
            .into_iter()
            .filter(|label| !label.neg)
            .collect();
        
        log::info!("Found {} active labels after filtering", active_labels.len());
        
        Ok(LabelCollection {
            labels: active_labels,
            labeler_did: self.labeler_url.clone(),
            query_timestamp: chrono::Utc::now(),
        })
    }
}

impl Default for LabelerClient {
    fn default() -> Self {
        Self::new()
    }
}

// Add urlencoding functionality
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}


