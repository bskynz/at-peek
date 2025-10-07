# Task: M3 - Basic Web UI with JSON Tree View

**Created:** 2025-10-07  
**Assignee:** TBD  
**Priority:** High  
**Status:** Backlog

---

## Description

Build the foundational web UI using Leptos (Rust WASM framework). Implement input field for handle/DID, "Fetch Records" button, and collapsible JSON tree view for displaying ATproto records. Integrate with `atproto_client` crate.

---

## Category

- [ ] **Safety & Performance**
- [ ] **Privacy & Security**
- [ ] **Protocol Compliance**
- [x] **UI/UX** (Core user interface)
- [ ] **Community & Docs**
- [x] **Infrastructure** (WASM build setup)

---

## Acceptance Criteria

- [ ] Leptos app scaffolded in `crates/at-peek-web`
- [ ] Input field accepts Bluesky handle or DID
- [ ] "Fetch Records" button triggers `atproto_client` to fetch records
- [ ] Loading spinner displayed during async fetch
- [ ] Collapsible JSON tree view renders fetched records
- [ ] Error banner displays network/validation errors
- [ ] Raw JSON view toggle (show parsed tree OR raw JSON)
- [ ] Responsive layout (mobile, tablet, desktop)
- [ ] WASM bundle builds with `trunk build --release`
- [ ] Bundle size < 500 KB (optimized with `wasm-opt`)

---

## Constitution Check

Does this task impact any constitutional principles? If yes, document:

- **Principle(s) affected**: 
  - Clarity & Discoverability in UI (core UI implementation)
  - User Privacy by Design (no analytics, local-only processing)
- **Compliance notes**: 
  - No external scripts or tracking
  - All ATproto queries initiated by user action (no auto-fetch)
  - Error messages suggest remediation (e.g., "Handle not found. Check spelling.")

---

## Technical Details

### Files to create

```
crates/at-peek-web/
├── Cargo.toml
├── Trunk.toml (build config)
├── index.html
├── src/
│   ├── lib.rs (Leptos app entry)
│   ├── components/
│   │   ├── mod.rs
│   │   ├── app_shell.rs
│   │   ├── input_panel.rs
│   │   ├── record_viewer.rs
│   │   └── tree_node.rs (recursive component)
│   └── state.rs (AppState signals)
└── styles/
    └── tailwind.css
```

### Dependencies (for `at-peek-web`)

```toml
[dependencies]
leptos = { version = "0.6", features = ["csr"] }
atproto_client = { path = "../atproto_client" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"

[build-dependencies]
trunk = "0.18"
```

### Component Structure

```rust
// src/lib.rs
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    
    view! {
        <AppShell state=state.clone()>
            <InputPanel state=state.clone() />
            <RecordViewer state=state.clone() />
        </AppShell>
    }
}

// src/components/tree_node.rs (recursive)
#[component]
pub fn TreeNode(key: String, value: serde_json::Value, depth: usize) -> impl IntoView {
    let expanded = create_rw_signal(false);
    
    match value {
        serde_json::Value::Object(map) => view! {
            <div class="tree-node-object">
                <span on:click=move |_| expanded.update(|e| *e = !*e)>
                    {if expanded() { "▼" } else { "▶" }} {key}
                </span>
                <Show when=move || expanded()>
                    <For
                        each=move || map.iter()
                        key=|(k, _)| k.clone()
                        let:child
                    >
                        <TreeNode key=child.0.clone() value=child.1.clone() depth=depth+1 />
                    </For>
                </Show>
            </div>
        },
        _ => view! {
            <div class="tree-node-leaf">
                <span class="key">{key}</span>
                <span class="value">{value.to_string()}</span>
            </div>
        }
    }
}
```

### Build Setup

```bash
# Install trunk
cargo install trunk

# Serve dev build (hot reload)
trunk serve

# Build optimized release
trunk build --release

# Optimize WASM
wasm-opt -Oz -o dist/optimized.wasm dist/at_peek_web_bg.wasm
```

### Styling

- Use TailwindCSS via CDN (or PostCSS build step)
- Dark mode: light background in default, dark background with `.dark` class
- Tree node indent: 20px per depth level
- Monospace font for JSON keys/values

### Testing approach

- Manual QA: paste handle, verify records render
- Visual regression tests (Percy or similar; optional for MVP)
- Lighthouse performance audit (target score > 90)

---

## Estimates

- **Effort**: Large (1-3d)
- **Risk**: Medium (Leptos learning curve, WASM bundle size)

---

## Notes

- **WASM Debugging**: Use `console_error_panic_hook` for better error messages in browser console.
- **Hot Reload**: `trunk serve` watches for file changes; very fast iteration.
- **Alternative**: If Leptos is too heavy, pivot to Axum + HTMX (server-rendered). Decision point: if WASM bundle > 500 KB after optimization.
- **Accessibility**: Add `aria-expanded` attributes to tree nodes for screen readers.


