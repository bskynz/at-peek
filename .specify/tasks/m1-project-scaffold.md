# Task: M1 - Project Scaffold and DID Resolution

**Created:** 2025-10-07  
**Assignee:** TBD  
**Priority:** Critical  
**Status:** Backlog

---

## Description

Set up the initial at-peek project structure with Rust workspace, ATproto client crate, and basic DID resolution functionality. This is the foundational task for MVP Phase 1.

---

## Category

- [x] **Safety & Performance** (Rust project setup)
- [ ] **Privacy & Security**
- [x] **Protocol Compliance** (DID resolution)
- [ ] **UI/UX**
- [x] **Community & Docs** (README, CONTRIBUTING)
- [x] **Infrastructure** (Build system, CI)

---

## Acceptance Criteria

- [ ] Cargo workspace created with at least two crates: `atproto_client` and `at-peek-web`
- [ ] `atproto_client` implements `resolve_handle()` to convert Bluesky handle → DID
- [ ] `atproto_client` implements `resolve_did()` to fetch DID document and extract PDS endpoint
- [ ] Unit tests for handle/DID resolution with mocked HTTP responses
- [ ] Integration test successfully resolves `alice.bsky.social` (marked `#[ignore]` for CI)
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` applied to all code
- [ ] README updated with build instructions
- [ ] GitHub Actions CI configured (build, test, clippy, fmt)
- [ ] LICENSE files (MIT + Apache-2.0) added

---

## Constitution Check

Does this task impact any constitutional principles? If yes, document:

- **Principle(s) affected**: 
  - Rust Safety & Performance (establishes safe Rust foundation)
  - Protocol Fidelity & Data Accuracy (DID resolution per ATproto spec)
  - Open Source Transparency & Community (CI, licenses)
- **Compliance notes**: 
  - Use `#![forbid(unsafe_code)]` in all crates
  - Document ATproto spec version compatibility (v0.3.x)
  - Dual license (MIT OR Apache-2.0)

---

## Technical Details

### Files to create

```
at-peek/
├── Cargo.toml (workspace)
├── LICENSE-MIT
├── LICENSE-APACHE
├── .github/
│   └── workflows/
│       └── ci.yml
├── crates/
│   ├── atproto_client/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── did.rs (DID/Handle types)
│   │   │   ├── resolver.rs (DID resolution logic)
│   │   │   └── error.rs (AtProtoError enum)
│   │   └── tests/
│   │       └── integration_tests.rs
│   └── at-peek-web/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs (placeholder)
└── README.md (updated with build instructions)
```

### Dependencies (for `atproto_client`)

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
mockito = "1.0" # or wiremock
tokio-test = "0.4"
```

### DID Resolution Algorithm

1. **Handle → DID**:
   - Try DNS TXT lookup: `_atproto.{handle}` → `did=did:plc:xyz`
   - Fallback: HTTPS `GET https://{handle}/.well-known/atproto-did`
   - Parse and validate DID format
   
2. **DID → PDS**:
   - Fetch DID document: `GET https://plc.directory/{did}` (for `did:plc`)
   - Parse `service` array, find entry with `type: "AtprotoPersonalDataServer"`
   - Extract `serviceEndpoint` URL

### Testing approach

- Unit tests with `mockito` for HTTP mocking
- Integration test hits real Bluesky PDS (requires network)
- CI runs unit tests only; integration tests manual

---

## Estimates

- **Effort**: Large (1-3d)
- **Risk**: Low (well-defined scope, clear ATproto docs)

---

## Notes

- **DID Methods**: MVP supports `did:plc` only (Bluesky's default). `did:web` deferred to Phase 2.
- **Error Handling**: Return structured errors (network, parse, not found). Don't panic!
- **CI**: Use GitHub Actions with Rust stable (Linux, macOS, Windows matrix).
- **License Headers**: Add SPDX headers to all source files: `// SPDX-License-Identifier: MIT OR Apache-2.0`


