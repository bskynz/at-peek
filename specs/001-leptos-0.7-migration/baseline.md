# Migration Baseline Measurements

**Date:** 2025-10-07  
**Branch:** 001-leptos-0.7-migration  
**Leptos Version (Before):** 0.6.x

---

## WASM Bundle Size (Before Migration)

**File:** `at-peek-web-3ef72cc91cced833_bg.wasm`  
**Size:** 6.5 MB (uncompressed)

**Build Command:**
```bash
cd crates/at-peek-web
trunk build --release
```

**Expected Improvement:** 15-20% size reduction (~5.2-5.5 MB target)

---

## Build Configuration

### Cargo.toml (Workspace)
```toml
[workspace.dependencies]
leptos = { version = "0.6", features = ["csr"] }
leptos_meta = { version = "0.6", features = ["csr"] }
leptos_router = { version = "0.6", features = ["csr"] }
```

### Release Profile
```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = 'abort'
strip = true
```

---

## Compile Times

(To be measured during migration)

---

## Functional Baseline

All features working:
- ✅ Single label check
- ✅ Bulk analysis
- ✅ Authentication
- ✅ Local storage persistence
- ✅ Dark mode
- ✅ Error handling

---

## Post-Migration Targets

- **WASM Size:** < 5.5 MB (15% reduction minimum)
- **Compile Time:** Comparable or better
- **Functionality:** 100% preserved
- **No Regressions:** All features work identically

