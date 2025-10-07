// SPDX-License-Identifier: MIT OR Apache-2.0
//! Post fetching from ATproto PDS

use crate::{Did, AtRecord, ListRecordsResponse, Error, Result, resolver};

/// Client for fetching posts from a PDS
#[derive(Clone)]
pub struct PostClient {
    client: reqwest::Client,
}

impl PostClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .build()
                .unwrap_or_default(),
        }
    }
    
    /// Fetch posts directly from PDS (includes ALL posts, even taken-down ones)
    pub async fn list_records(
        &self,
        did: &Did,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<ListRecordsResponse> {
        // Resolve DID to PDS endpoint
        let pds_url = resolver::resolve_did(did).await?;
        
        let mut url = format!(
            "{}/xrpc/com.atproto.repo.listRecords?repo={}&collection=app.bsky.feed.post",
            pds_url,
            urlencoding::encode(did.as_str())
        );
        
        if let Some(lim) = limit {
            url.push_str(&format!("&limit={}", lim));
        }
        
        if let Some(cur) = cursor {
            url.push_str(&format!("&cursor={}", urlencoding::encode(&cur)));
        }
        
        log::debug!("Fetching posts from PDS: {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(e))?;
        
        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::LabelerUnavailable(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        
        let records_response: ListRecordsResponse = response.json().await
            .map_err(|e| Error::Parse(format!("Failed to parse records response: {}", e)))?;
        
        log::info!("Fetched {} posts from PDS", records_response.records.len());
        
        Ok(records_response)
    }
    
    /// Fetch up to N posts for a given DID directly from their PDS
    pub async fn fetch_posts(&self, did: &Did, max_posts: usize) -> Result<Vec<AtRecord>> {
        let mut all_posts = Vec::new();
        let mut cursor: Option<String> = None;
        
        while all_posts.len() < max_posts {
            let remaining = max_posts - all_posts.len();
            let limit = remaining.min(100); // PDS limit is usually 100
            
            let response = self.list_records(did, Some(limit as u32), cursor).await?;
            
            if response.records.is_empty() {
                break;
            }
            
            all_posts.extend(response.records);
            
            match response.cursor {
                Some(c) if !c.is_empty() => cursor = Some(c),
                _ => break,
            }
        }
        
        Ok(all_posts)
    }
}

impl Default for PostClient {
    fn default() -> Self {
        Self::new()
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}

