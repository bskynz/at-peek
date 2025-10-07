// SPDX-License-Identifier: MIT OR Apache-2.0
//! Core types for ATproto client

use serde::{Deserialize, Serialize};

/// Represents an ATproto DID (Decentralized Identifier)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Did(pub String);

impl Did {
    pub fn new(did: String) -> Self {
        Self(did)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Validate DID format
    pub fn validate(&self) -> bool {
        self.0.starts_with("did:") && self.0.len() > 4
    }
}

impl std::fmt::Display for Did {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a handle (e.g., alice.bsky.social)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handle(pub String);

impl Handle {
    pub fn new(handle: String) -> Self {
        Self(handle)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Validate handle format
    pub fn validate(&self) -> bool {
        self.0.contains('.') && !self.0.starts_with('.') && !self.0.ends_with('.')
    }
}

impl std::fmt::Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A content moderation label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    /// The label value (e.g., "porn", "sexual", "graphic-media", "spam")
    pub val: String,
    
    /// Subject of the label (DID or AT-URI)
    pub uri: String,
    
    /// Optional CID for content-addressed labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    
    /// DID of the labeler that created this label
    pub src: String,
    
    /// Timestamp when label was created (ISO 8601)
    pub cts: String,
    
    /// Optional expiration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<String>,
    
    /// Negation flag (if true, removes a previously applied label)
    #[serde(default)]
    pub neg: bool,
}

impl Label {
    /// Get the category of this label
    pub fn category(&self) -> LabelCategory {
        LabelCategory::from_value(&self.val)
    }
    
    /// Get a human-readable description of this label
    pub fn description(&self) -> &'static str {
        match self.val.as_str() {
            // Adult Content Labels
            "porn" => "Pornographic content (18+, requires user opt-in)",
            "sexual" => "Sexually suggestive content (18+, requires user opt-in)",
            "nudity" => "Nudity (artistic or otherwise, can be disabled)",
            
            // Violence Labels
            "graphic-media" => "Graphic violence or disturbing imagery (18+, requires user opt-in)",
            "gore" => "Extreme violence or gore",
            
            // Spam & Quality Labels
            "spam" => "Spam or low-quality content",
            
            // Hate & Harassment Labels
            "hate" => "Hateful or discriminatory content",
            
            // Moderation Action Labels (admin/moderator applied)
            "!hide" => "Hidden from feeds (cannot be clicked through)",
            "!warn" => "Warning required before viewing (can be clicked through)",
            "!no-unauthenticated" => "Inaccessible to logged-out users",
            "!takedown" => "Content taken down by moderators",
            "!blur" => "Blurred in feeds",
            "!no-promote" => "Excluded from algorithmic promotion",
            "!filter" => "Filtered from search results",
            
            // Catch-all for custom labels
            _ if self.val.starts_with('!') => "Custom moderation action",
            _ => "Custom content label",
        }
    }
}

/// Response from queryLabels endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelsResponse {
    pub labels: Vec<Label>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// An ATproto record (e.g., a post)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtRecord {
    pub uri: String,
    pub cid: String,
    pub value: serde_json::Value,
}

/// Response from listRecords endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRecordsResponse {
    pub records: Vec<AtRecord>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Collection of labels with metadata
#[derive(Debug, Clone)]
pub struct LabelCollection {
    pub labels: Vec<Label>,
    pub labeler_did: String,
    pub query_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Label categories for grouping
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelCategory {
    AdultContent,
    Violence,
    Spam,
    Hate,
    ModerationAction,
    Other,
}

impl LabelCategory {
    pub fn from_value(val: &str) -> Self {
        match val {
            "porn" | "sexual" | "nudity" => Self::AdultContent,
            "graphic-media" | "gore" => Self::Violence,
            "spam" => Self::Spam,
            "hate" => Self::Hate,
            s if s.starts_with('!') => Self::ModerationAction,
            _ => Self::Other,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::AdultContent => "Adult Content",
            Self::Violence => "Violence & Gore",
            Self::Spam => "Spam",
            Self::Hate => "Hate & Harassment",
            Self::ModerationAction => "Moderation Actions",
            Self::Other => "Other Labels",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::AdultContent => "🔞",
            Self::Violence => "⚠️",
            Self::Spam => "🚫",
            Self::Hate => "🛑",
            Self::ModerationAction => "👁️",
            Self::Other => "🏷️",
        }
    }
}


