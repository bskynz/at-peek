# Task: M2 - ATproto Record Fetching

**Created:** 2025-10-07  
**Assignee:** TBD  
**Priority:** Critical  
**Status:** Backlog

---

## Description

Implement ATproto record fetching in the `atproto_client` crate. Support listing all records for a given DID and collection (e.g., `app.bsky.feed.post`), with pagination via cursors. Return records with URI, CID, and raw JSON value.

---

## Category

- [x] **Safety & Performance** (Async I/O, efficient parsing)
- [ ] **Privacy & Security**
- [x] **Protocol Compliance** (ATproto listRecords/getRecord endpoints)
- [ ] **UI/UX**
- [ ] **Community & Docs**
- [ ] **Infrastructure**

---

## Acceptance Criteria

- [ ] `atproto_client::list_records()` fetches records from PDS
- [ ] Pagination supported via `cursor` parameter
- [ ] Rate limiting handled gracefully (exponential backoff on 429 responses)
- [ ] `atproto_client::get_record()` fetches single record by AT-URI
- [ ] Records parsed into `AtRecord` struct with URI, CID, and value
- [ ] Timeout set to 30 seconds (configurable)
- [ ] Unit tests with mocked PDS responses
- [ ] Integration test fetches real records from Bluesky (marked `#[ignore]`)
- [ ] Error handling for 404, 500, network failures
- [ ] Documentation for all public APIs

---

## Constitution Check

Does this task impact any constitutional principles? If yes, document:

- **Principle(s) affected**: 
  - Rust Safety & Performance (async/await, connection pooling)
  - Protocol Fidelity & Data Accuracy (strict ATproto endpoint usage)
- **Compliance notes**: 
  - Use official ATproto endpoint paths (no undocumented APIs)
  - Document atproto spec version (v0.3.x)
  - Surface all protocol errors to caller (no silent failures)

---

## Technical Details

### Files to modify

- `crates/atproto_client/src/lib.rs` (add `list_records`, `get_record`)
- `crates/atproto_client/src/records.rs` (new module for record types)
- `crates/atproto_client/tests/integration_tests.rs` (add record fetch tests)

### API Design

```rust
impl AtProtoClient {
    /// List records for a given repo and collection
    pub async fn list_records(
        &self,
        did: &Did,
        collection: &str,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<RecordCollection>;
    
    /// Fetch a single record by AT-URI
    pub async fn get_record(&self, uri: &str) -> Result<AtRecord>;
}

pub struct RecordCollection {
    pub records: Vec<AtRecord>,
    pub cursor: Option<String>,
}

pub struct AtRecord {
    pub uri: String,
    pub cid: String,
    pub value: serde_json::Value,
}
```

### ATproto Endpoints

- **List Records**: `GET {pds}/xrpc/com.atproto.repo.listRecords?repo={did}&collection={nsid}&limit={n}&cursor={c}`
- **Get Record**: `GET {pds}/xrpc/com.atproto.repo.getRecord?repo={did}&collection={nsid}&rkey={key}`

### Rate Limiting Strategy

```rust
async fn fetch_with_retry(url: &str) -> Result<Response> {
    let mut backoff = Duration::from_secs(1);
    loop {
        match reqwest::get(url).await {
            Ok(resp) if resp.status() == 429 => {
                let retry_after = resp.headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(backoff.as_secs());
                tokio::time::sleep(Duration::from_secs(retry_after)).await;
                backoff *= 2; // Exponential backoff
            }
            Ok(resp) => return Ok(resp),
            Err(e) => return Err(e.into()),
        }
    }
}
```

### Testing approach

- Mock PDS responses with `mockito`
- Test pagination (fetch 10 records, cursor, fetch next 10)
- Test rate limiting (simulate 429, verify retry)
- Integration test: fetch `app.bsky.feed.post` for known Bluesky user

---

## Estimates

- **Effort**: Medium (2-8h)
- **Risk**: Low (well-documented ATproto endpoints)

---

## Notes

- **Collections**: Start with `app.bsky.feed.post` and `app.bsky.actor.profile`. Add more as needed.
- **Pagination**: Default limit is 50; max is 100 per ATproto spec.
- **Authentication**: MVP fetches public records only (no auth tokens). Private repos deferred to Phase 2.
- **CID Validation**: Don't validate CID format in MVP; just pass through as string.


