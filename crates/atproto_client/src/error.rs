// SPDX-License-Identifier: MIT OR Apache-2.0
//! Error types for ATproto operations

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to resolve handle: {0}")]
    HandleResolution(String),

    #[error("Invalid DID format: {0}")]
    InvalidDid(String),

    #[error("Invalid AT-URI format: {0}")]
    InvalidAtUri(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Labeler service unavailable: {0}")]
    LabelerUnavailable(String),

    #[error("Invalid label schema: {0}")]
    LabelValidation(String),

    #[error("Rate limited: retry after {0:?} seconds")]
    RateLimited(Option<u64>),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
