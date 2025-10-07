# Plan: Leptos 0.7 Migration

**Created:** 2025-10-07  
**Status:** Active  
**Owner(s):** at-peek maintainers

---

## Overview

This plan outlines the migration of at-peek from Leptos 0.6 to Leptos 0.7 while maintaining the existing Client-Side Rendering (CSR) architecture. The migration aims to capture the performance improvements (~20% smaller WASM binaries), enhanced reactivity, and better developer experience offered by Leptos 0.7 without changing the fundamental privacy-first, client-side-only design.

This migration is strategic for the project's future as it provides immediate performance benefits while positioning the codebase on the latest framework version for ongoing support and future enhancements.

---

## Constitution Check

- [x] **Rust Safety & Performance**: Migration enhances performance with ~20% WASM reduction and faster rendering. New Xilem-inspired architecture maintains memory safety. No unsafe code introduced. Aligns with Principle 1.

- [x] **User Privacy by Design**: No changes to data flow. All processing remains client-side. No new data collection or telemetry. Privacy model completely unchanged. Aligns with Principle 2.

- [x] **Protocol Fidelity & Data Accuracy**: No changes to ATproto parsing, validation, or API interaction. This is purely a framework upgrade. Aligns with Principle 3.

- [x] **Clarity & Discoverability in UI**: Migration should be transparent to users. UI behavior and appearance remain identical. Internal reactivity improvements may enhance perceived responsiveness. Aligns with Principle 4.

- [x] **Open Source Transparency & Community**: Migration fully documented in spec and changelog. Breaking changes documented for contributors. Public visibility maintained. Aligns with Principle 5.

---

## Goals

1. **Upgrade to Leptos 0.7** - Update all Leptos dependencies from 0.6 to 0.7
2. **Maintain CSR Architecture** - Continue with client-side rendering, no SSR adoption
3. **Achieve WASM Size Reduction** - Realize the ~15-20% binary size improvement
4. **Preserve Functionality** - Ensure all existing features work identically
5. **Improve Developer Experience** - Benefit from cleaner APIs and better compile times
6. **Zero User Impact** - Complete migration without any user-facing changes

---

## Non-Goals

- **SSR or Islands Architecture** - Not adopting server-side rendering (incompatible with privacy-first design)
- **Feature Additions** - No new features in this migration
- **UI Redesign** - No visual or UX changes
- **API Changes** - No changes to ATproto client or data handling
- **Build Tool Changes** - Continue using Trunk (not switching to cargo-leptos)

---

## Technical Approach

### High-Level Strategy

1. **Baseline Documentation** - Record current WASM sizes and compile times
2. **Dependency Update** - Update Cargo.toml with Leptos 0.7 versions
3. **Incremental Migration** - Fix compilation errors systematically by component
4. **Testing** - Verify functionality after each component update
5. **Measurement** - Confirm WASM size reduction and performance gains
6. **Documentation** - Update changelog and build docs if needed

### Component-by-Component Approach

The migration will proceed through these components in order:
1. `lib.rs` - Update main entry point and imports
2. `state.rs` - Update signal and state management
3. `utils.rs` - Update any reactive utilities
4. `components/mod.rs` - Update module structure
5. `components/app.rs` - Update root component
6. `components/header.rs` - Stateless component (low risk)
7. `components/empty_state.rs` - Stateless component (low risk)
8. `components/label_badge.rs` - Stateless component (low risk)
9. `components/input_panel.rs` - Stateful component (moderate complexity)
10. `components/label_viewer.rs` - Stateful with resources (high complexity)
11. `components/bulk_analysis.rs` - Stateful with async (high complexity)
12. `components/auth_panel.rs` - Stateful with storage (moderate complexity)

### Key API Migrations

Based on research, expected changes include:

**Imports:**
```rust
// Old (0.6)
use leptos::*;

// New (0.7)
use leptos::prelude::*;
```

**Signals:**
```rust
// Old (0.6)
use leptos::{SignalGet, SignalSet, SignalUpdate};

// New (0.7)
use leptos::prelude::{Get, Set, Update};
```

**Mount:**
```rust
// Old (0.6)
leptos::mount_to_body(App);

// New (0.7)
leptos::mount::mount_to_body(App);
```

**Resources (Enhanced):**
```rust
// New in 0.7: Direct .await support
let data = create_resource(|| fetch());
view! {
    <Suspense fallback=|| "Loading...">
        {move || async move {
            let value = data.await;
            view! { <div>{value}</div> }
        }}
    </Suspense>
}
```

---

## Milestones

| Milestone | Target Date | Status |
|-----------|-------------|--------|
| Research and Planning Complete | 2025-10-07 | [x] |
| Dependencies Updated, Build Compiling | 2025-10-07 | [ ] |
| All Components Migrated | 2025-10-07 | [ ] |
| Testing Complete | 2025-10-07 | [ ] |
| PR Ready for Review | 2025-10-07 | [ ] |

---

## Dependencies

### External Dependencies
- **Leptos 0.7** - Must verify stable version on crates.io
- **Trunk** - Must support Leptos 0.7 (likely requires latest version)
- **wasm-bindgen** - May need version update for compatibility
- **leptos_meta** - Version must match leptos core
- **leptos_router** - Version must match leptos core

### Internal Dependencies
- **atproto_client crate** - Should be unaffected (no Leptos dependency)
- **Build system** - Trunk.toml and build scripts
- **CI/CD** - GitHub Actions workflow may need updates

### Potential Blockers
- If Leptos 0.7 is not yet stable, may need to use beta/RC version
- If Trunk doesn't support 0.7, may need workaround or wait for update
- If breaking changes are more extensive than expected, migration complexity increases

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Leptos 0.7 not fully stable | Medium | Medium | Use latest RC/beta if needed; monitor for issues |
| More breaking changes than expected | Medium | High | Allocate extra time; reference migration guide |
| WASM size doesn't decrease as expected | Low | Low | Profile and optimize; document actual results |
| Regression in functionality | Low | High | Comprehensive manual testing; rollback plan ready |
| Trunk incompatibility | Low | Medium | Update Trunk; consider temporary build workarounds |
| CI/CD pipeline breaks | Medium | Medium | Test locally first; update GHA workflow if needed |
| Dependency conflicts | Low | Medium | Use `cargo tree` to debug; update transitive deps |
| Longer compile times | Low | Low | Use `--cfg=erase_components` flag if needed |

---

## Success Criteria

- [x] Feature specification created and approved
- [ ] All components compile without errors with Leptos 0.7
- [ ] All existing functionality works identically to 0.6 version
- [ ] WASM binary size reduced by at least 10% (target: 15-20%)
- [ ] No new console errors or warnings in browser
- [ ] Manual testing passes for all user flows
- [ ] Build times remain comparable or improve
- [ ] CI/CD pipeline passes
- [ ] Documentation updated (CHANGELOG, README if needed)
- [ ] No regressions in dark mode or styling
- [ ] Local storage persistence works correctly
- [ ] Authentication flow works correctly

---

## Implementation Steps

### Phase 0: Preparation
1. [x] Create feature branch `001-leptos-0.7-migration`
2. [x] Create specification document
3. [x] Create implementation plan (this document)
4. [ ] Verify Leptos 0.7 version availability
5. [ ] Document baseline WASM size
6. [ ] Document baseline compile times

### Phase 1: Dependency Updates
1. [ ] Update `Cargo.toml` workspace dependencies
   - Update `leptos` to 0.7.x
   - Update `leptos_meta` to 0.7.x
   - Update `leptos_router` to 0.7.x
   - Remove deprecated feature flags
   - Update `wasm-bindgen` if needed
2. [ ] Update `crates/at-peek-web/Cargo.toml`
   - Verify all dependencies compatible
3. [ ] Run `cargo update` to refresh lock file
4. [ ] Attempt initial build to identify errors

### Phase 2: Core Migrations
1. [ ] Update `lib.rs`
   - Fix imports
   - Update mount function
   - Verify panic hook and logging
2. [ ] Update `state.rs`
   - Fix signal imports
   - Update AppState implementation
   - Verify reactivity works
3. [ ] Update `utils.rs`
   - Fix any Leptos utility usage

### Phase 3: Component Migrations
1. [ ] Update `components/mod.rs`
2. [ ] Update `components/header.rs`
3. [ ] Update `components/empty_state.rs`
4. [ ] Update `components/label_badge.rs`
5. [ ] Update `components/input_panel.rs`
6. [ ] Update `components/label_viewer.rs`
7. [ ] Update `components/bulk_analysis.rs`
8. [ ] Update `components/auth_panel.rs`
9. [ ] Update `components/app.rs` (root component)

### Phase 4: Build and Testing
1. [ ] Verify successful compilation
2. [ ] Build release WASM
3. [ ] Measure WASM size
4. [ ] Run `trunk serve` and test locally
5. [ ] Manual testing checklist (see Testing Plan)
6. [ ] Browser compatibility testing

### Phase 5: Documentation and Cleanup
1. [ ] Update CHANGELOG.md
2. [ ] Update BUILD.md if needed
3. [ ] Update README.md if needed
4. [ ] Add migration notes to SUMMARY.md
5. [ ] Clean up any temporary code or comments

### Phase 6: Review and Merge
1. [ ] Self-review all changes
2. [ ] Create PR with detailed description
3. [ ] Address review feedback
4. [ ] Merge to main
5. [ ] Deploy to Cloudflare Pages
6. [ ] Monitor for issues

---

## Testing Plan

### Automated Testing
- [ ] `cargo check` passes
- [ ] `cargo clippy` passes with no new warnings
- [ ] `cargo build --release` succeeds
- [ ] `trunk build --release` succeeds
- [ ] CI/CD pipeline passes (if present)

### Manual Testing Checklist

**Single Label Check:**
- [ ] Enter a valid handle (e.g., alice.bsky.social)
- [ ] Verify labels load
- [ ] Check label display is correct
- [ ] Try invalid handle, verify error message

**Bulk Analysis:**
- [ ] Switch to bulk analysis tab
- [ ] Enter multiple handles
- [ ] Verify batch processing works
- [ ] Check results display correctly

**Authentication:**
- [ ] Open auth panel
- [ ] Enter credentials
- [ ] Verify authentication succeeds
- [ ] Check auth state persists
- [ ] Log out, verify state clears

**Local Storage:**
- [ ] Make changes that persist (auth, preferences)
- [ ] Refresh page
- [ ] Verify data restored
- [ ] Click "Clear All Data"
- [ ] Verify all data cleared

**Error Handling:**
- [ ] Test with invalid inputs
- [ ] Test with network offline
- [ ] Verify error messages display correctly
- [ ] Verify app doesn't crash

**UI/UX:**
- [ ] Verify dark mode works
- [ ] Check all buttons clickable
- [ ] Verify loading states display
- [ ] Check responsive layout
- [ ] Verify no visual regressions

**Browser Compatibility:**
- [ ] Test in Chrome
- [ ] Test in Firefox
- [ ] Test in Safari
- [ ] Check console for errors

---

## Rollback Plan

If critical issues are discovered:

1. **Pre-merge**: Simply don't merge the PR, iterate on fixes
2. **Post-merge**: 
   - Create revert PR immediately
   - Revert to previous commit
   - Redeploy previous version
   - Document issues in GitHub issue
   - Plan fixes on a new branch

---

## Notes

### Research Findings

**Leptos 0.7 Benefits for CSR:**
- 20% smaller WASM binaries (Xilem-inspired architecture)
- Improved compile times
- Better async/await support in components
- Enhanced reactivity with fine-grained updates
- WebSocket support (for future use)
- Cleaner, more idiomatic Rust APIs

**Migration Complexity:**
- Expected to be moderate (small codebase, clear migration path)
- Main challenges: API imports and signal trait changes
- Router API changes need careful attention
- Resource handling improvements may require refactoring

**Compatibility Considerations:**
- atproto_client crate unaffected (no Leptos dependency)
- Trunk should support 0.7 (verify version)
- No SSR means we avoid complex migration issues
- CSR-only simplifies the migration significantly

### Open Questions (To Be Resolved)

1. **Exact Leptos version**: What's the latest stable 0.7.x version?
2. **Trunk version**: Do we need to update Trunk?
3. **wasm-bindgen version**: Compatibility requirements?
4. **Migration guide completeness**: Are there undocumented breaking changes?
5. **Development workflow**: Any new flags or commands to use?

### Related Issues/PRs

- TBD: Will be added as issues are discovered/created

---

## Progress Tracking

**Overall Status**: 🟡 In Progress

- [x] Phase 0: Preparation (Spec and Plan)
- [ ] Phase 1: Dependency Updates
- [ ] Phase 2: Core Migrations
- [ ] Phase 3: Component Migrations
- [ ] Phase 4: Build and Testing
- [ ] Phase 5: Documentation
- [ ] Phase 6: Review and Merge

**Last Updated**: 2025-10-07
