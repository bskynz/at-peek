# Specification: Privacy & Local Storage

**Version:** 0.1.0  
**Author(s):** Project Maintainers  
**Created:** 2025-10-07  
**Status:** Draft

---

## Summary

This specification defines how at-peek handles user data, session state, and network requests to ensure maximum privacy. All data processing occurs locally in the user's browser (via WASM), with no server-side storage or third-party transmission. Users can clear all data on demand via a prominent UI control.

---

## Constitution Alignment

| Principle | Compliance Notes |
|-----------|------------------|
| Rust Safety & Performance | Safe localStorage API bindings; no unsafe code |
| User Privacy by Design | Core principle: no tracking, no telemetry, no data transmission to third parties |
| Protocol Fidelity & Data Accuracy | N/A (privacy layer; does not affect protocol parsing) |
| Clarity & Discoverability in UI | "Clear All Data" button prominent; request log shows all outbound requests |
| Open Source Transparency & Community | Privacy policy documented in README; code auditable |

---

## Requirements

### Functional Requirements

1. **FR-1**: Store user preferences (dark mode, last used handle) in browser localStorage
2. **FR-2**: Store request log entries (timestamp, URL, status) in sessionStorage
3. **FR-3**: Provide "Clear All Data" button that wipes all localStorage and sessionStorage
4. **FR-4**: Display all outbound HTTP requests in real-time (request log panel)
5. **FR-5**: Show privacy notice on first app load (stored in localStorage to avoid repeat)
6. **FR-6**: No cookies, no third-party scripts, no external analytics

### Non-Functional Requirements

1. **NFR-1**: localStorage quota: < 5 MB total (typical limit is 10 MB per origin)
2. **NFR-2**: Data cleared when user clicks "Clear All Data" or manually clears browser storage
3. **NFR-3**: No Personally Identifiable Information (PII) stored except user-provided handles/DIDs
4. **NFR-4**: All network requests transparently logged (no hidden queries)

---

## Design

### Architecture

```
┌─────────────────────────────────────────────────┐
│               Browser Environment               │
│  ┌───────────────────────────────────────────┐  │
│  │          at-peek WASM App                 │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  StorageManager                     │  │  │
│  │  │  - read/write localStorage          │  │  │
│  │  │  - read/write sessionStorage        │  │  │
│  │  │  - clear_all()                      │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  RequestLogger (impl trait)         │  │  │
│  │  │  - logs to sessionStorage           │  │  │
│  │  │  - displayed in UI                  │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────┘  │
│                                                  │
│  Browser APIs (via web-sys):                    │
│  - window.localStorage                          │
│  - window.sessionStorage                        │
│  - No cookies, no IndexedDB (overkill for MVP)  │
└─────────────────────────────────────────────────┘
```

### Data Models

```rust
use serde::{Deserialize, Serialize};

/// User preferences stored in localStorage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Dark mode enabled
    pub dark_mode: bool,
    
    /// Last used handle or DID (for convenience)
    pub last_input: Option<String>,
    
    /// Whether user has acknowledged privacy notice
    pub privacy_notice_acknowledged: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            dark_mode: false, // Or detect system preference
            last_input: None,
            privacy_notice_acknowledged: false,
        }
    }
}

/// Request log entry (stored in sessionStorage)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLogEntry {
    pub id: String, // UUID
    pub timestamp: String, // ISO 8601
    pub method: String, // GET, POST, etc.
    pub url: String,
    pub status: Option<u16>,
    pub duration_ms: Option<u64>,
    pub error: Option<String>,
}
```

### API / Interfaces

```rust
use web_sys::Storage;

/// Manages localStorage and sessionStorage interactions
pub struct StorageManager {
    local: Storage,   // localStorage
    session: Storage, // sessionStorage
}

impl StorageManager {
    /// Initialize from browser environment
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No window")?;
        let local = window.local_storage()?.ok_or("No localStorage")?;
        let session = window.session_storage()?.ok_or("No sessionStorage")?;
        Ok(Self { local, session })
    }
    
    /// Load user preferences from localStorage
    pub fn load_preferences(&self) -> Result<UserPreferences, StorageError> {
        match self.local.get_item("user_preferences")? {
            Some(json) => serde_json::from_str(&json).map_err(Into::into),
            None => Ok(UserPreferences::default()),
        }
    }
    
    /// Save user preferences to localStorage
    pub fn save_preferences(&self, prefs: &UserPreferences) -> Result<(), StorageError> {
        let json = serde_json::to_string(prefs)?;
        self.local.set_item("user_preferences", &json)?;
        Ok(())
    }
    
    /// Append a request log entry to sessionStorage
    pub fn log_request(&self, entry: &RequestLogEntry) -> Result<(), StorageError> {
        let mut log = self.load_request_log()?;
        log.push(entry.clone());
        
        // Limit to last 100 entries to avoid quota issues
        if log.len() > 100 {
            log.drain(0..(log.len() - 100));
        }
        
        let json = serde_json::to_string(&log)?;
        self.session.set_item("request_log", &json)?;
        Ok(())
    }
    
    /// Load request log from sessionStorage
    pub fn load_request_log(&self) -> Result<Vec<RequestLogEntry>, StorageError> {
        match self.session.get_item("request_log")? {
            Some(json) => serde_json::from_str(&json).map_err(Into::into),
            None => Ok(Vec::new()),
        }
    }
    
    /// Clear all localStorage and sessionStorage
    pub fn clear_all(&self) -> Result<(), StorageError> {
        self.local.clear()?;
        self.session.clear()?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Browser storage API error: {0}")]
    JsError(String),
    
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),
}
```

---

## Implementation Notes

- **Dependencies**:
  - `web-sys` (with `Storage` feature enabled)
  - `serde` and `serde_json` for serialization
  - `thiserror` for error handling
- **Testing Strategy**:
  - Mock `Storage` API in unit tests (use `wasm-bindgen-test`)
  - Integration test: write, read, clear in real browser environment (headless Chrome)
  - Manual test: verify data persists across page reloads
- **Quota Management**:
  - Monitor localStorage usage; warn user if approaching 5 MB
  - Automatically trim request log to last 100 entries

---

## ATproto Compatibility

- N/A (storage layer; does not interact with ATproto)

---

## Security & Privacy Considerations

### Privacy Guarantees

1. **No Server-Side Storage**: All data stored in user's browser; never transmitted to at-peek servers (there are no servers!)
2. **No Telemetry**: Zero analytics, crash reporting, or usage tracking
3. **No Cookies**: All state in localStorage/sessionStorage
4. **No Third-Party Scripts**: No CDN dependencies; all assets self-hosted or inlined
5. **Transparent Requests**: Every HTTP request logged and displayed to user

### Privacy Notice

Display on first app load:

```
🔒 Privacy Notice

at-peek is a local-first tool. All data processing happens in your browser.

- No data is sent to our servers (we don't have servers!)
- No cookies or tracking scripts
- All network requests are to ATproto endpoints only
- You can clear all stored data anytime via "Clear All Data" button

By using at-peek, you acknowledge that you understand this privacy model.
```

### Data Stored

| Key | Storage | Contents | Retention |
|-----|---------|----------|-----------|
| `user_preferences` | localStorage | Dark mode, last input, privacy notice flag | Until user clears |
| `request_log` | sessionStorage | HTTP request history | Until browser tab closed |

### GDPR / Privacy Compliance

- **Right to Access**: User can inspect localStorage via browser DevTools
- **Right to Erasure**: "Clear All Data" button implements this
- **Data Minimization**: Only essential data stored (preferences, request log)
- **No Consent Required**: No PII processed; all data is public ATproto records or user-provided inputs

---

## Testing Plan

- [ ] Unit test: save/load preferences
- [ ] Unit test: append to request log (verify 100-entry limit)
- [ ] Unit test: clear_all() wipes all keys
- [ ] Integration test: persist dark mode across page reload
- [ ] Manual test: verify no cookies in DevTools → Application → Cookies
- [ ] Manual test: verify no external requests in DevTools → Network

---

## Rollout Plan

1. **Phase 1**: Implement `StorageManager` with preferences
2. **Phase 2**: Add request logging to sessionStorage
3. **Phase 3**: Add "Clear All Data" button to UI
4. **Phase 4**: Display privacy notice modal on first load

---

## Open Questions

- Should we offer export/import of preferences (e.g., JSON file download)?
  - **Decision**: Deferred to Phase 2; not essential for MVP
- What if user disables localStorage?
  - **Decision**: Show warning banner; app will work but lose state on reload
- Should we use IndexedDB for larger datasets (e.g., caching records)?
  - **Decision**: Not for MVP; localStorage sufficient for now

---

## References

- [Web Storage API (MDN)](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)
- [GDPR Compliance Guide](https://gdpr.eu/)
- [Privacy by Design Principles](https://www.ipc.on.ca/wp-content/uploads/resources/7foundationalprinciples.pdf)


