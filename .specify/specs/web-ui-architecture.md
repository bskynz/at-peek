# Specification: Web UI Architecture

**Version:** 0.1.0  
**Author(s):** Project Maintainers  
**Created:** 2025-10-07  
**Status:** Draft

---

## Summary

The Web UI provides an intuitive, privacy-preserving interface for inspecting content moderation labels on ATproto users and posts. Built with Rust (WASM via Leptos framework), it features visual label badges, categorized label display, request logging panel, dark mode support, and accessibility compliance. All processing happens in the browser; no backend server is required.

---

## Constitution Alignment

| Principle | Compliance Notes |
|-----------|------------------|
| Rust Safety & Performance | Full Rust WASM stack (Leptos); minimal JavaScript; optimized bundle size |
| User Privacy by Design | No analytics, no cookies; all data in localStorage; "Clear All Data" button |
| Protocol Fidelity & Data Accuracy | Raw JSON view alongside parsed records; validation errors displayed inline |
| Clarity & Discoverability in UI | Collapsible trees, tooltips for ATproto terms, loading states, helpful errors |
| Open Source Transparency & Community | Clean component structure; documented state management; accessibility tested |

---

## Requirements

### Functional Requirements

1. **FR-1**: Input field for Bluesky handle, DID, or post AT-URI with validation feedback
2. **FR-2**: "Check Labels" button that triggers label queries
3. **FR-3**: Visual label badges with color-coding by severity (content warning, moderation action, etc.)
4. **FR-4**: Label grouping by category (adult content, violence, spam, hate, etc.)
5. **FR-5**: Display label metadata: source labeler DID, timestamp, expiration
6. **FR-6**: Empty state when no labels found ("No moderation labels applied")
7. **FR-7**: Tabs or toggle between "Labels" and "Raw JSON"
8. **FR-8**: Request log panel showing all labeler service queries (timestamp, URL, status)
9. **FR-9**: Dark mode toggle with system preference detection
10. **FR-10**: "Clear All Data" button to wipe localStorage
11. **FR-11**: Inline error messages for failed queries or validation errors
12. **FR-12**: Loading spinners during async operations
13. **FR-13**: Tooltips explaining label types and what they mean

### Non-Functional Requirements

1. **NFR-1**: WCAG 2.1 AA accessibility compliance (keyboard nav, screen reader support)
2. **NFR-2**: < 500 KB WASM bundle size (optimized release build)
3. **NFR-3**: < 100 ms UI response time for interactions (tree expand/collapse)
4. **NFR-4**: Responsive design (mobile, tablet, desktop)
5. **NFR-5**: Progressive enhancement (basic functionality without JavaScript if feasible)

---

## Design

### Architecture

```
┌─────────────────────────────────────────────────┐
│                Browser (WASM)                   │
│  ┌───────────────────────────────────────────┐  │
│  │          Leptos Components                │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  AppShell                           │  │  │
│  │  │  - Header (logo, dark mode toggle) │  │  │
│  │  │  - Main content area               │  │  │
│  │  │  - Footer (links)                  │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  InputPanel                         │  │  │
│  │  │  - Handle/DID/AT-URI input         │  │  │
│  │  │  - Check Labels button             │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  LabelViewer                        │  │  │
│  │  │  - Tabs: Labels | Raw JSON         │  │  │
│  │  │  - LabelBadge (per label)          │  │  │
│  │  │  - LabelCategory groups            │  │  │
│  │  │  - EmptyState (no labels)          │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │  RequestLogPanel                    │  │  │
│  │  │  - List of HTTP requests           │  │  │
│  │  │  - Collapsible/expandable          │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────┐  │
│  │          State Management                 │  │
│  │  - AppState (signals/resources)           │  │
│  │  - LocalStorage bindings                  │  │
│  └───────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────┐  │
│  │      AtProtoClient (WASM-compiled)        │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### Component Hierarchy

```
App
├── AppShell
│   ├── Header
│   │   ├── Logo
│   │   ├── DarkModeToggle
│   │   └── HelpTooltip
│   ├── MainContent
│   │   ├── InputPanel
│   │   │   ├── SubjectInput (handle/DID/AT-URI)
│   │   │   └── CheckLabelsButton
│   │   ├── LabelViewer
│   │   │   ├── TabBar (Labels | Raw JSON)
│   │   │   ├── LabelCategories
│   │   │   │   ├── AdultContentGroup
│   │   │   │   ├── ViolenceGroup
│   │   │   │   ├── SpamGroup
│   │   │   │   └── OtherGroup
│   │   │   ├── LabelBadge (per label)
│   │   │   └── EmptyState
│   │   └── ErrorBanner
│   └── Footer
└── RequestLogPanel (slide-out drawer)
```

### Data Models

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

/// Global application state
#[derive(Clone, Debug)]
pub struct AppState {
    /// Current input (handle, DID, or AT-URI)
    pub subject_input: RwSignal<String>,
    
    /// Fetched labels
    pub labels: RwSignal<Option<LabelCollection>>,
    
    /// Loading state
    pub is_loading: RwSignal<bool>,
    
    /// Error message (if any)
    pub error: RwSignal<Option<String>>,
    
    /// Request log entries
    pub request_log: RwSignal<Vec<RequestLogEntry>>,
    
    /// Dark mode enabled
    pub dark_mode: RwSignal<bool>,
    
    /// Active view: Labels or Raw JSON
    pub view_mode: RwSignal<ViewMode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Parsed,
    RawJson,
}

/// Log entry for a single HTTP request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestLogEntry {
    pub timestamp: String, // ISO 8601
    pub method: String,
    pub url: String,
    pub status: Option<u16>,
    pub duration_ms: Option<u64>,
}

/// Props for RecordTreeNode component
#[derive(Clone, Debug)]
pub struct TreeNodeProps {
    pub key: String,
    pub value: serde_json::Value,
    pub depth: usize,
}
```

### Key Components

#### InputPanel

```rust
#[component]
pub fn InputPanel(state: AppState) -> impl IntoView {
    let on_submit = move |_| {
        // Validate input
        // Trigger record fetch
        // Update loading state
    };
    
    view! {
        <div class="input-panel">
            <input 
                type="text"
                placeholder="Enter Bluesky handle or DID"
                prop:value=state.input
                on:input=move |ev| state.input.set(event_target_value(&ev))
            />
            <button on:click=on_submit disabled=state.is_loading>
                {move || if state.is_loading.get() { "Loading..." } else { "Fetch Records" }}
            </button>
        </div>
    }
}
```

#### RecordTreeNode (Recursive)

```rust
#[component]
pub fn RecordTreeNode(props: TreeNodeProps) -> impl IntoView {
    let expanded = create_rw_signal(false);
    
    view! {
        <div class="tree-node" style=format!("padding-left: {}px", props.depth * 20)>
            {move || match &props.value {
                serde_json::Value::Object(map) => view! {
                    <div on:click=move |_| expanded.update(|e| *e = !*e)>
                        <span class="toggle-icon">{if expanded.get() { "▼" } else { "▶" }}</span>
                        <span class="key">{&props.key}</span>
                    </div>
                    <Show when=move || expanded.get()>
                        <For
                            each=move || map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()
                            key=|(k, _)| k.clone()
                            let:child
                        >
                            <RecordTreeNode 
                                key=child.0.clone()
                                value=child.1.clone()
                                depth=props.depth + 1
                            />
                        </For>
                    </Show>
                },
                _ => view! {
                    <div>
                        <span class="key">{&props.key}</span>
                        <span class="value">{props.value.to_string()}</span>
                    </div>
                }
            }}
        </div>
    }
}
```

---

## Implementation Notes

- **Framework**: Leptos 0.6+ (reactive UI with WASM)
- **Styling**: TailwindCSS via `trunk` build tool
- **Icons**: Inline SVG or lightweight icon font (avoid large libraries)
- **Build Tool**: `trunk serve` for dev, `trunk build --release` for production
- **Bundle Optimization**:
  - `wasm-opt` for size reduction
  - Code splitting if bundle exceeds 500 KB
  - Lazy load request log panel

---

## ATproto Compatibility

- N/A (UI layer; relies on `atproto_client` for protocol interactions)

---

## Security & Privacy Considerations

- **No Cookies**: All state in localStorage (can be cleared)
- **No Analytics**: No Google Analytics, Mixpanel, or similar tracking
- **CSP Headers**: Content Security Policy prohibits external scripts (self-host or inline all assets)
- **HTTPS Only**: App must be served over HTTPS in production
- **Local Storage Encryption**: Not implemented in MVP (all data is public ATproto records anyway)

---

## Testing Plan

- [ ] Component unit tests (Leptos testing utils)
- [ ] Accessibility audit with axe-core
- [ ] Manual keyboard navigation testing
- [ ] Screen reader testing (NVDA, JAWS, VoiceOver)
- [ ] Cross-browser testing (Chrome, Firefox, Safari, Edge)
- [ ] Mobile responsiveness testing (iOS Safari, Android Chrome)
- [ ] Performance profiling with Lighthouse (target score: > 90)

---

## Rollout Plan

1. **Phase 1**: Static mockups and component skeleton
2. **Phase 2**: Integrate with `atproto_client` (fetch real data)
3. **Phase 3**: Add dark mode, tooltips, and accessibility features
4. **Phase 4**: Polish animations, error states, and edge cases
5. **Phase 5**: Performance optimization and bundle size reduction

---

## Open Questions

- **Progressive Enhancement**: Can we offer a no-JS fallback (server-rendered HTML)?
  - **Decision**: Deferred to Phase 2; WASM-only for MVP
- **Mobile App**: Should we build native iOS/Android apps using Tauri?
  - **Decision**: Deferred; web-first for MVP
- **Internationalization**: Multi-language support?
  - **Decision**: English-only for MVP; i18n in Phase 2

---

## References

- [Leptos Book](https://leptos-rs.github.io/leptos/)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [TailwindCSS Docs](https://tailwindcss.com/docs)
- [Web Accessibility Initiative](https://www.w3.org/WAI/)

