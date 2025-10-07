// SPDX-License-Identifier: MIT OR Apache-2.0
//! Handle to DID resolution

use crate::{Did, Handle, Error, Result};

/// Resolve a Bluesky handle to a DID via DNS or .well-known endpoint
pub async fn resolve_handle(handle: &Handle) -> Result<Did> {
    if !handle.validate() {
        return Err(Error::HandleResolution(format!("Invalid handle format: {}", handle)));
    }
    
    // Try DNS TXT record lookup first (more reliable for custom domains)
    // For WASM, we can't do DNS lookups directly, so we use a DNS-over-HTTPS service
    let dns_url = format!(
        "https://dns.google/resolve?name=_atproto.{}&type=TXT",
        urlencoding::encode(handle.as_str())
    );
    
    log::debug!("Resolving handle {} via DNS", handle);
    
    match reqwest::get(&dns_url).await {
        Ok(response) if response.status().is_success() => {
            if let Ok(dns_response) = response.json::<serde_json::Value>().await {
                // Extract DID from TXT record
                if let Some(answers) = dns_response.get("Answer").and_then(|a| a.as_array()) {
                    for answer in answers {
                        if let Some(data) = answer.get("data").and_then(|d| d.as_str()) {
                            // Remove quotes and look for did= prefix
                            let data_clean = data.trim_matches('"');
                            if let Some(did_str) = data_clean.strip_prefix("did=") {
                                let did = Did::new(did_str.to_string());
                                if did.validate() {
                                    log::info!("Resolved {} to {} via DNS", handle, did);
                                    return Ok(did);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(response) => {
            log::debug!("DNS resolution failed with status {}, trying HTTPS", response.status());
        }
        Err(e) => {
            log::debug!("DNS resolution failed: {}, trying HTTPS", e);
        }
    }
    
    // Fallback to HTTPS .well-known endpoint
    let url = format!("https://{}/.well-known/atproto-did", handle.as_str());
    
    log::debug!("Trying HTTPS resolution for {}", handle);
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| Error::HandleResolution(format!("Failed to fetch DID document: {}", e)))?;
    
    if !response.status().is_success() {
        return Err(Error::HandleResolution(format!(
            "Could not resolve handle {} via DNS or HTTPS (HTTP {})",
            handle,
            response.status()
        )));
    }
    
    let did_str = response.text().await
        .map_err(|e| Error::HandleResolution(format!("Failed to read response: {}", e)))?;
    
    let did = Did::new(did_str.trim().to_string());
    
    if !did.validate() {
        return Err(Error::InvalidDid(format!("Invalid DID returned: {}", did)));
    }
    
    log::info!("Resolved {} to {} via HTTPS", handle, did);
    
    Ok(did)
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}

/// Resolve a DID to its PDS endpoint
pub async fn resolve_did(did: &Did) -> Result<String> {
    // For did:plc, use plc.directory
    if did.as_str().starts_with("did:plc:") {
        let url = format!("https://plc.directory/{}", did.as_str());
        
        log::debug!("Resolving DID {} via {}", did, url);
        
        let response = reqwest::get(&url).await
            .map_err(|e| Error::HandleResolution(format!("Failed to fetch DID document: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::HandleResolution(format!(
                "HTTP {} from PLC directory",
                response.status()
            )));
        }
        
        let did_doc: serde_json::Value = response.json().await
            .map_err(|e| Error::HandleResolution(format!("Failed to parse DID document: {}", e)))?;
        
        // Extract PDS endpoint from service array
        if let Some(services) = did_doc.get("service").and_then(|s| s.as_array()) {
            for service in services {
                if let Some(service_type) = service.get("type").and_then(|t| t.as_str()) {
                    if service_type == "AtprotoPersonalDataServer" {
                        if let Some(endpoint) = service.get("serviceEndpoint").and_then(|e| e.as_str()) {
                            log::info!("Resolved {} to PDS: {}", did, endpoint);
                            return Ok(endpoint.to_string());
                        }
                    }
                }
            }
        }
        
        Err(Error::HandleResolution("No PDS endpoint found in DID document".to_string()))
    } else {
        Err(Error::HandleResolution(format!("Unsupported DID method: {}", did)))
    }
}


