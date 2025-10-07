# Specification: ATproto Client Module

**Version:** 0.1.0  
**Author(s):** Project Maintainers  
**Created:** 2025-10-07  
**Status:** Draft

---

## Summary

The ATproto Client module provides a Rust library for fetching and parsing content moderation labels from ATproto labeler services. It handles DID resolution, HTTP communication with labeler endpoints (primarily `com.atproto.label.queryLabels`), label validation, and error handling. This module is the core data layer for at-peek's moderation label inspection functionality.

---

## Constitution Alignment

| Principle | Compliance Notes |
|-----------|------------------|
| Rust Safety & Performance | Pure safe Rust; async for performance; zero-copy parsing where possible |
| User Privacy by Design | No telemetry; all requests logged to caller; no data persistence in this module |
| Protocol Fidelity & Data Accuracy | Strict adherence to atproto v0.3.x; validation errors surfaced to UI |
| Clarity & Discoverability in UI | N/A (backend module), but errors include user-friendly messages |
| Open Source Transparency & Community | Public API documented; examples in doc comments; integration tests |

---

## Requirements

### Functional Requirements

1. **FR-1**: Resolve a Bluesky handle (e.g., `alice.bsky.social`) to a DID via DNS TXT lookup or `.well-known/atproto-did`
2. **FR-2**: Query moderation labels for a given DID (user-level labels) using `com.atproto.label.queryLabels`
3. **FR-3**: Query moderation labels for a given AT-URI (post-level labels)
4. **FR-4**: Support multiple labeler services (start with Bluesky's `mod.bsky.app`, allow others)
5. **FR-5**: Parse label responses into structured `Label` objects with: value, source, timestamp, metadata
6. **FR-6**: Distinguish between different label categories (content warning, hide, blur, etc.)
7. **FR-7**: Return raw JSON alongside parsed labels for debugging
8. **FR-8**: Log all HTTP requests with timestamps for transparency

### Non-Functional Requirements

1. **NFR-1**: Support concurrent requests (async/await with Tokio runtime)
2. **NFR-2**: Handle rate limiting gracefully (exponential backoff, respects `Retry-After` headers)
3. **NFR-3**: Timeout requests after 30 seconds (configurable)
4. **NFR-4**: Minimize dependencies (avoid large crates; audit for security)
5. **NFR-5**: Comprehensive error types (distinguish network, parsing, protocol errors)

---

## Design

### Architecture

```
┌───────────────────────────────────────┐
│          AtProtoClient                │
│  ┌─────────────────────────────────┐  │
│  │  DID Resolver                   │  │
│  │  - handle → DID                 │  │
│  │  - DID → PDS endpoint           │  │
│  └─────────────────────────────────┘  │
│  ┌─────────────────────────────────┐  │
│  │  Record Fetcher                 │  │
│  │  - listRecords                  │  │
│  │  - getRecord                    │  │
│  └─────────────────────────────────┘  │
│  ┌─────────────────────────────────┐  │
│  │  Lexicon Parser                 │  │
│  │  - Validate schemas             │  │
│  │  - Deserialize to Rust types    │  │
│  └─────────────────────────────────┘  │
│  ┌─────────────────────────────────┐  │
│  │  Request Logger (trait)         │  │
│  └─────────────────────────────────┘  │
└───────────────────────────────────────┘
```

### Data Models

```rust
/// Represents an ATproto DID (Decentralized Identifier)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Did(String);

/// Represents a handle (e.g., alice.bsky.social)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle(String);

/// A content moderation label
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

/// Response from queryLabels endpoint
#[derive(Debug, Clone, serde::Deserialize)]
pub struct LabelsResponse {
    pub labels: Vec<Label>,
    
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

/// Error types for ATproto operations
#[derive(Debug, thiserror::Error)]
pub enum AtProtoError {
    #[error("Failed to resolve handle: {0}")]
    HandleResolutionError(String),
    
    #[error("Invalid DID format: {0}")]
    InvalidDid(String),
    
    #[error("Invalid AT-URI format: {0}")]
    InvalidAtUri(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Labeler service unavailable: {0}")]
    LabelerUnavailable(String),
    
    #[error("Invalid label schema: {0}")]
    LabelValidationError(String),
    
    #[error("Rate limited: retry after {retry_after:?}")]
    RateLimited { retry_after: Option<u64> },
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, AtProtoError>;
```

### API / Interfaces

```rust
/// Main client for interacting with ATproto labeler services
pub struct AtProtoClient {
    http_client: reqwest::Client,
    logger: Box<dyn RequestLogger>,
    /// Default labeler service (e.g., "https://mod.bsky.app")
    default_labeler: String,
}

impl AtProtoClient {
    /// Create a new client with Bluesky's default labeler
    pub fn new() -> Self;
    
    /// Create a client with custom logger and labeler service
    pub fn with_config(logger: Box<dyn RequestLogger>, labeler_url: String) -> Self;
    
    /// Resolve a handle to a DID
    pub async fn resolve_handle(&self, handle: &Handle) -> Result<Did>;
    
    /// Query labels for a given DID (user-level labels)
    pub async fn query_labels_for_did(&self, did: &Did) -> Result<LabelCollection>;
    
    /// Query labels for a given AT-URI (post-level labels)
    pub async fn query_labels_for_uri(&self, uri: &str) -> Result<LabelCollection>;
    
    /// Query labels from a specific labeler service
    pub async fn query_labels_from_labeler(
        &self,
        labeler_url: &str,
        subjects: &[String], // DIDs or AT-URIs
    ) -> Result<LabelCollection>;
    
    /// Validate an AT-URI format
    pub fn validate_at_uri(uri: &str) -> Result<()>;
}

/// Trait for logging HTTP requests (injected by caller)
pub trait RequestLogger: Send + Sync {
    fn log_request(&self, method: &str, url: &str, timestamp: chrono::DateTime<chrono::Utc>);
    fn log_response(&self, status: u16, duration_ms: u64);
}
```

---

## Implementation Notes

- **Crate Structure**: Create a `atproto_client` crate in `crates/atproto_client/`
- **Dependencies**:
  - `reqwest` (with `rustls` for HTTPS)
  - `serde` and `serde_json` for JSON parsing
  - `tokio` for async runtime
  - `thiserror` for error handling
  - `chrono` for timestamps
  - `url` for AT-URI parsing
- **Testing Strategy**:
  - Unit tests with mocked HTTP responses (`mockito` or `wiremock`)
  - Integration tests against live Bluesky labeler (marked `#[ignore]` for CI)
  - Property tests for DID/handle/AT-URI parsing (`proptest`)
  - Test with various label types (content warnings, moderation actions, etc.)

---

## ATproto Compatibility

- **Spec Version**: atproto v0.3.x (as of October 2024)
- **Lexicons Used**:
  - `com.atproto.label.queryLabels` (primary endpoint)
  - `com.atproto.label.defs` (label value definitions)
- **Label Values** (standard set):
  - `porn`, `sexual`, `nudity` - Adult content
  - `graphic-media`, `gore` - Graphic violence
  - `spam` - Spam content
  - `hate` - Hateful content
  - Custom labels from third-party labelers
- **Known Limitations**:
  - MVP only supports public label queries (no authentication)
  - Does not handle label negation history (only current state)
  - Self-labels not yet supported (only labeler-applied labels)

---

## Security & Privacy Considerations

- **No Credentials Storage**: MVP fetches public labels only; no auth tokens
- **Request Logging**: All requests logged via `RequestLogger` trait; caller controls where logs go
- **HTTPS Only**: Reject non-HTTPS labeler endpoints
- **Timeout Protection**: 30-second timeout prevents hanging on slow labeler services
- **No Caching**: This module does not cache labels; caller decides caching strategy (labels can change over time)
- **Labeler Trust**: Users should be aware that third-party labelers may apply labels arbitrarily; source DID always displayed

---

## Testing Plan

- [ ] Unit tests for `Did`, `Handle`, and AT-URI parsing
- [ ] Unit tests for `Label` deserialization
- [ ] Integration test: resolve `alice.bsky.social` to DID
- [ ] Integration test: query labels for known DID with labels
- [ ] Integration test: query labels for known post AT-URI
- [ ] Error handling test: invalid handle, network failure, 404 responses
- [ ] Rate limiting test: simulate 429 response with `Retry-After`
- [ ] Test empty label responses (subjects with no labels)

---

## Rollout Plan

1. **Phase 1**: Implement DID resolution (handle → DID)
2. **Phase 2**: Implement `queryLabels` for DIDs (user-level labels)
3. **Phase 3**: Add support for AT-URI label queries (post-level labels)
4. **Phase 4**: Add multi-labeler support (third-party labelers)
5. **Phase 5**: Optimize with connection pooling and retry logic

---

## Open Questions

- Should we bundle known label definitions, or fetch them dynamically from labeler services?
  - **Decision**: Bundle common Bluesky labels for MVP; dynamic discovery in Phase 2
- How to handle label negation (labels that were removed)?
  - **Decision**: MVP only shows current labels (neg=false); label history in Phase 2
- Support for self-labels (user-applied labels)?
  - **Decision**: Deferred to Phase 2; MVP focuses on labeler-applied labels
- How to validate labeler service endpoints?
  - **Decision**: Must be HTTPS; verify label response schema matches expected format

---

## References

- [AT Protocol Specifications](https://atproto.com/specs/atp)
- [ATproto Label Lexicon](https://atproto.com/specs/label)
- [Bluesky Moderation Docs](https://docs.bsky.app/docs/advanced-guides/moderation)
- [DID Core Specification](https://www.w3.org/TR/did-core/)
- [AT URI Scheme](https://atproto.com/specs/at-uri-scheme)
- [Bluesky Labeler Service](https://github.com/bluesky-social/atproto/tree/main/packages/api/src/client/types/com/atproto/label)

