# Task: M4 - Request Logging and Local Storage

**Created:** 2025-10-07  
**Assignee:** TBD  
**Priority:** High  
**Status:** Backlog

---

## Description

Implement transparent request logging and local storage for user preferences. All HTTP requests to ATproto endpoints must be logged to sessionStorage and displayed in a slide-out panel. User preferences (dark mode, last input) saved to localStorage with "Clear All Data" button.

---

## Category

- [ ] **Safety & Performance**
- [x] **Privacy & Security** (Transparent logging, data clearability)
- [ ] **Protocol Compliance**
- [x] **UI/UX** (Request log panel, clear data button)
- [ ] **Community & Docs**
- [ ] **Infrastructure**

---

## Acceptance Criteria

- [ ] `StorageManager` module created with localStorage/sessionStorage bindings
- [ ] User preferences (dark mode, last input) saved to localStorage
- [ ] Request log entries saved to sessionStorage (max 100 entries)
- [ ] Request log panel displays all HTTP requests (timestamp, URL, status, duration)
- [ ] "Clear All Data" button wipes both localStorage and sessionStorage
- [ ] Privacy notice modal shown on first app load
- [ ] `RequestLogger` trait implemented by Web UI
- [ ] No cookies present (verified in browser DevTools)
- [ ] Unit tests for storage operations

---

## Constitution Check

Does this task impact any constitutional principles? If yes, document:

- **Principle(s) affected**: 
  - User Privacy by Design (core privacy implementation)
  - Clarity & Discoverability in UI (transparent request logging)
- **Compliance notes**: 
  - All network requests logged (no hidden queries)
  - User can clear all data on demand
  - No PII stored except user-provided handles/DIDs
  - Privacy notice explains data storage model

---

## Technical Details

### Files to create/modify

```
crates/at-peek-web/src/
├── storage.rs (new: StorageManager)
├── logger.rs (new: RequestLogger impl)
├── components/
│   ├── request_log_panel.rs (new: slide-out panel)
│   ├── privacy_notice.rs (new: first-load modal)
│   └── clear_data_button.rs (new: prominent button)
└── lib.rs (integrate storage, logger)
```

### StorageManager API

```rust
use web_sys::Storage;

pub struct StorageManager {
    local: Storage,
    session: Storage,
}

impl StorageManager {
    pub fn new() -> Result<Self, JsValue>;
    pub fn load_preferences(&self) -> Result<UserPreferences>;
    pub fn save_preferences(&self, prefs: &UserPreferences) -> Result<()>;
    pub fn log_request(&self, entry: &RequestLogEntry) -> Result<()>;
    pub fn load_request_log(&self) -> Result<Vec<RequestLogEntry>>;
    pub fn clear_all(&self) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct UserPreferences {
    pub dark_mode: bool,
    pub last_input: Option<String>,
    pub privacy_notice_acknowledged: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestLogEntry {
    pub id: String, // UUID
    pub timestamp: String, // ISO 8601
    pub method: String,
    pub url: String,
    pub status: Option<u16>,
    pub duration_ms: Option<u64>,
}
```

### Request Logger Integration

```rust
// Inject logger into AtProtoClient
impl RequestLogger for StorageManager {
    fn log_request(&self, method: &str, url: &str, timestamp: DateTime<Utc>) {
        let entry = RequestLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: timestamp.to_rfc3339(),
            method: method.to_string(),
            url: url.to_string(),
            status: None,
            duration_ms: None,
        };
        self.storage.log_request(&entry).ok();
    }
    
    fn log_response(&self, status: u16, duration_ms: u64) {
        // Update last entry with status and duration
    }
}
```

### UI Components

#### Request Log Panel (Slide-Out)

```rust
#[component]
pub fn RequestLogPanel(state: AppState) -> impl IntoView {
    let log = create_resource(|| (), |_| async {
        state.storage.load_request_log().unwrap_or_default()
    });
    
    view! {
        <div class="request-log-panel">
            <h3>"Request Log"</h3>
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || log.get().map(|entries| view! {
                    <ul>
                        {entries.iter().map(|entry| view! {
                            <li>
                                <span class="timestamp">{&entry.timestamp}</span>
                                <span class="method">{&entry.method}</span>
                                <span class="url">{&entry.url}</span>
                                <span class="status">{entry.status.unwrap_or(0)}</span>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                })}
            </Suspense>
        </div>
    }
}
```

#### Clear Data Button

```rust
#[component]
pub fn ClearDataButton(state: AppState) -> impl IntoView {
    let on_click = move |_| {
        if window().confirm_with_message("Clear all data? This cannot be undone.").unwrap() {
            state.storage.clear_all().ok();
            window().location().reload().ok();
        }
    };
    
    view! {
        <button on:click=on_click class="clear-data-btn">"Clear All Data"</button>
    }
}
```

#### Privacy Notice Modal

```rust
#[component]
pub fn PrivacyNotice(state: AppState) -> impl IntoView {
    let show = create_rw_signal(!state.preferences.privacy_notice_acknowledged);
    
    let on_acknowledge = move |_| {
        let mut prefs = state.preferences.clone();
        prefs.privacy_notice_acknowledged = true;
        state.storage.save_preferences(&prefs).ok();
        show.set(false);
    };
    
    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay">
                <div class="modal">
                    <h2>"🔒 Privacy Notice"</h2>
                    <p>"at-peek is a local-first tool. All data processing happens in your browser."</p>
                    <ul>
                        <li>"No data sent to our servers (we don't have any!)"</li>
                        <li>"No cookies or tracking"</li>
                        <li>"All requests go to ATproto endpoints only"</li>
                        <li>"You can clear all data anytime"</li>
                    </ul>
                    <button on:click=on_acknowledge>"I Understand"</button>
                </div>
            </div>
        </Show>
    }
}
```

### Testing approach

- Unit tests for `StorageManager` with mocked `Storage` API
- Integration test: save prefs, reload page, verify persistence
- Manual test: open DevTools → Application → Local Storage, verify no cookies
- Manual test: click "Clear All Data", verify localStorage empty

---

## Estimates

- **Effort**: Medium (2-8h)
- **Risk**: Low (web-sys Storage API is stable)

---

## Notes

- **localStorage Quota**: Typically 10 MB per origin. Monitor usage; log warning if approaching limit.
- **sessionStorage**: Cleared when tab closes. Perfect for request log (transient data).
- **UUID Generation**: Use `uuid` crate with `wasm-bindgen` feature.
- **Timestamp Format**: ISO 8601 (RFC 3339) for consistency.
- **Privacy Notice**: Only show once per browser; stored in localStorage flag.


