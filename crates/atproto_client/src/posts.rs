// SPDX-License-Identifier: MIT OR Apache-2.0
//! Post fetching from ATproto PDS

use crate::{resolver, AtRecord, Did, Error, ListRecordsResponse, Result};

/// Client for fetching posts from a PDS
#[derive(Clone)]
pub struct PostClient {
    client: reqwest::Client,
}

impl PostClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder().build().unwrap_or_default(),
        }
    }

    /// Fetch posts directly from PDS
    /// Note: Individual posts with moderation labels may be included, but banned/suspended
    /// accounts may have their repositories completely inaccessible via PDS
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

        let response = self.client.get(&url).send().await.map_err(Error::Network)?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            
            // Provide user-friendly messages for common PDS errors
            let error_message = match status.as_u16() {
                400 => format!(
                    "Bad request to PDS (HTTP 400). The repository may not exist or be inaccessible. {}",
                    if !error_text.is_empty() { 
                        format!("Details: {}", error_text) 
                    } else { 
                        String::new() 
                    }
                ),
                403 => format!(
                    "Access forbidden (HTTP 403). This account may be suspended, banned, or have restricted access. \
                    The account's posts cannot be retrieved. {}",
                    if !error_text.is_empty() { 
                        format!("Details: {}", error_text) 
                    } else { 
                        String::new() 
                    }
                ),
                404 => format!(
                    "Repository not found (HTTP 404). This account may have been deleted, deactivated, \
                    or the PDS endpoint may be incorrect. {}",
                    if !error_text.is_empty() { 
                        format!("Details: {}", error_text) 
                    } else { 
                        String::new() 
                    }
                ),
                410 => "Account has been permanently deleted (HTTP 410)".to_string(),
                500..=599 => format!(
                    "PDS server error (HTTP {}). The Personal Data Server is experiencing issues. \
                    Try again later.",
                    status
                ),
                _ => format!(
                    "Failed to fetch posts from PDS (HTTP {}). {}",
                    status,
                    if !error_text.is_empty() {
                        format!("Details: {}", error_text)
                    } else {
                        "The repository may be unavailable.".to_string()
                    }
                ),
            };
            
            return Err(Error::LabelerUnavailable(error_message));
        }

        let records_response: ListRecordsResponse = response
            .json()
            .await
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
