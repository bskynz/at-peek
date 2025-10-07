# Plan: MVP Phase 1 - Content Moderation Label Inspector

**Created:** 2025-10-07  
**Status:** Active  
**Owner(s):** Project Maintainers

---

## Overview

Phase 1 delivers the minimum viable product for at-peek: a functional web UI that can accept a Bluesky handle, DID, or post AT-URI, fetch associated content moderation labels from labeler services, and display them in an intuitive format. This establishes the foundation for moderation transparency while proving the core value proposition of label visibility in the decentralized ATproto ecosystem.

---

## Constitution Check

- [x] **Rust Safety & Performance**: Core logic in safe Rust; web UI via WASM or lightweight web framework. No unsafe code planned.
- [x] **User Privacy by Design**: All processing local; no backend server required. Request logging visible in UI.
- [x] **Protocol Fidelity & Data Accuracy**: Will target atproto v0.3.x specification; raw JSON views included.
- [x] **Clarity & Discoverability in UI**: Collapsible tree view for records; tooltips for ATproto terms.
- [x] **Open Source Transparency & Community**: Development in public repo; MIT/Apache-2.0 dual license.

---

## Goals

1. Enable users to input a Bluesky handle, DID, or post AT-URI to query moderation labels
2. Fetch labels from Bluesky's moderation service (and optionally third-party labelers)
3. Display labels with clear categorization (severity, source, timestamp, values)
4. Show both user-level labels (applied to DID) and post-level labels (applied to AT-URI)
5. Display all network requests transparently (request log panel)
6. Implement local storage for session state with clear data option
7. Achieve basic UI responsiveness and dark mode support

---

## Non-Goals

- Label history timeline (when labels were applied/removed) - deferred to Phase 2
- Label export functionality (JSON/CSV) - deferred to Phase 2
- Offline/caching capabilities
- Multi-user label comparison views
- Custom labeler service configuration (only Bluesky's official labeler in MVP)
- Label appeal workflow integration

---

## Technical Approach

### Core Architecture

```
┌─────────────────┐
│   Web UI Layer  │  ← Leptos/Yew (Rust WASM) or Axum + HTMX
├─────────────────┤
│  App Logic      │  ← Rust core: DID resolution, record fetching
├─────────────────┤
│  ATproto Client │  ← HTTP client to PDS/AppView endpoints
└─────────────────┘
```

### Technology Stack

- **Backend/Core**: Rust (stable channel)
- **Web Framework**: Leptos (WASM) or Axum + HTMX (server-rendered)
- **ATproto Client**: `reqwest` with custom lexicon parsing
- **UI Styling**: TailwindCSS or similar utility-first framework
- **Local Storage**: Browser localStorage API (Web) or sled/rocksdb (native)

### Key Components

1. **DID Resolver**: Resolve Bluesky handles to DIDs via DNS/HTTP
2. **Label Fetcher**: Query `com.atproto.label.queryLabels` endpoint from labeler services
3. **Label Display Component**: Visual badges/cards showing label categories, severity, and metadata
4. **Request Logger**: In-memory log of all outbound requests with timestamps
5. **Local State Manager**: Save/load user preferences and session data
6. **Label Explainer**: Tooltips/help text explaining what each label type means

---

## Milestones

| Milestone | Target Date | Status |
|-----------|-------------|--------|
| M1: Project scaffold + DID resolution | 2025-10-21 | [ ] |
| M2: ATproto record fetching (happy path) | 2025-11-04 | [ ] |
| M3: Basic Web UI with JSON tree view | 2025-11-18 | [ ] |
| M4: Request logging + local storage | 2025-12-02 | [ ] |
| M5: Error handling, dark mode, polish | 2025-12-16 | [ ] |
| M6: MVP Release (v0.1.0) | 2025-12-20 | [ ] |

---

## Dependencies

### External

- **ATproto Specification**: Must track atproto v0.3.x (or latest stable)
- **Rust Toolchain**: Rust 1.75+ (or stable at time of development)
- **Public PDS Endpoints**: Rely on Bluesky's public AppView for MVP

### Internal

- None (greenfield project)

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| ATproto spec changes mid-development | Medium | High | Pin to specific spec version; document assumptions |
| WASM bundle size too large | Medium | Medium | Use code splitting; consider server-rendered alternative |
| DID resolution edge cases (e.g., custom domains) | High | Medium | Start with official Bluesky handles; add custom domains in Phase 2 |
| CORS issues fetching from PDS | Low | High | Use CORS proxy for dev; document self-hosting for production |
| UI complexity for nested records | Medium | Medium | Limit nesting depth in MVP; use lazy loading |

---

## Success Criteria

- [ ] User can paste a Bluesky handle (e.g., `alice.bsky.social`) and see moderation labels applied to that user
- [ ] User can paste a post AT-URI and see moderation labels applied to that specific post
- [ ] Labels displayed with clear categorization (content warning, hide, blur, etc.)
- [ ] Each label shows: label value, source labeler, timestamp (if available)
- [ ] Empty state shown when no labels exist ("No moderation labels found")
- [ ] Request log shows all labeler service queries with timestamps
- [ ] "Clear All Data" button wipes local storage
- [ ] Dark mode toggle functional
- [ ] No `cargo clippy` warnings or linter errors
- [ ] README includes build instructions for Linux, macOS, Windows
- [ ] Published as v0.1.0 release with binary artifacts

---

## Notes

- **Web Framework Decision**: Lean toward Leptos (full Rust stack, WASM) for initial MVP. If bundle size is prohibitive, pivot to Axum + HTMX.
- **Labeler Services**: MVP queries Bluesky's official moderation service (`mod.bsky.app`). Third-party labelers in Phase 2.
- **Label Endpoints**: Primary endpoint is `com.atproto.label.queryLabels` with `uriPatterns` parameter.
- **Testing Strategy**: Unit tests for DID resolution and label parsing. Manual QA for UI flows (automated UI tests deferred to Phase 2).
- **Performance Target**: < 2s load time for label queries. Labeler services are typically fast (< 500ms response).

