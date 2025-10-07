# Specification: Leptos 0.7 Migration

**Version:** 1.0.0  
**Author(s):** at-peek maintainers  
**Created:** 2025-10-07  
**Status:** Draft

---

## Summary

This specification defines the migration of the at-peek web UI from Leptos 0.6 to Leptos 0.7 while maintaining the existing Client-Side Rendering (CSR) architecture. The migration aims to leverage Leptos 0.7's improved performance (~20% smaller WASM binaries), enhanced reactivity system, better async handling, and improved developer experience without changing the fundamental privacy-first, client-side-only architecture.

---

## Constitution Alignment

| Principle | Compliance Notes |
|-----------|------------------|
| Rust Safety & Performance | Migration enhances performance with ~20% WASM size reduction and faster rendering. No new unsafe code introduced. New rendering architecture (Xilem-inspired) maintains memory safety while improving efficiency. |
| User Privacy by Design | No impact on privacy model. All processing remains client-side. No new data collection or transmission. The migration is purely an internal framework upgrade. |
| Protocol Fidelity & Data Accuracy | No changes to ATproto parsing or validation logic. API client remains unchanged. All existing data handling continues as-is. |
| Clarity & Discoverability in UI | Migration should be transparent to users. UI behavior remains identical. Internal improvements to reactivity may improve perceived responsiveness. |
| Open Source Transparency & Community | Migration process fully documented. Version bump clearly communicated in changelog. Breaking changes in API usage documented for potential contributors. |

---

## Requirements

### Functional Requirements

1. **FR-1**: All existing features must continue to work identically after migration
   - Single label checking functionality
   - Bulk analysis functionality
   - Authentication flow
   - Local storage persistence
   - Input validation and error handling

2. **FR-2**: Build system must successfully compile with Leptos 0.7
   - Cargo.toml dependencies updated
   - All feature flags migrated to 0.7 conventions
   - Trunk build continues to work

3. **FR-3**: No user-facing behavior changes
   - UI must render identically
   - All interactions must work the same way
   - Dark mode and styling preserved

### Non-Functional Requirements

1. **NFR-1**: WASM binary size should decrease by approximately 15-20%
   - Baseline measurement taken before migration
   - Post-migration measurement confirms reduction
   - Size tracked in build artifacts

2. **NFR-2**: Compile times should improve or remain comparable
   - Development build times tracked
   - Release build times tracked

3. **NFR-3**: All existing tests must pass
   - No new test failures introduced
   - Code coverage maintained or improved

4. **NFR-4**: Migration should be completable in a single PR
   - All changes atomic and reviewable
   - No intermediate broken states

---

## Design

### Architecture

The migration maintains the existing architecture:
- **CSR-only**: All code runs in the browser as WASM
- **No SSR**: No server-side rendering or islands (incompatible with current architecture)
- **Local-first**: All state management remains in browser storage
- **Privacy-preserving**: No changes to data flow

Key changes are internal to Leptos framework:
- New reactivity system with improved signal tracking
- New rendering engine (Xilem-inspired)
- Improved async/await support in components
- Updated component APIs and macros

### Migration Strategy

**Phase 1: Dependency Updates**
1. Update `Cargo.toml` workspace dependencies to Leptos 0.7
2. Remove deprecated feature propagations (`leptos_meta/csr`, `leptos_router/csr`)
3. Update to new feature flag conventions

**Phase 2: API Updates**
1. Update imports (e.g., `leptos::SignalGet` → `leptos::prelude::Get`)
2. Update component macro syntax if changed
3. Update router configuration
4. Update signal and reactive primitive usage

**Phase 3: Component Refactoring**
1. Update `App` component
2. Update `InputPanel` component
3. Update `LabelViewer` component
4. Update `BulkAnalysis` component
5. Update `Header`, `EmptyState`, `LabelBadge`, `AuthPanel` components
6. Update `AppState` and utilities

**Phase 4: Build System Updates**
1. Update `Trunk.toml` if needed
2. Verify build scripts
3. Update CI/CD configuration

### Data Models

No changes to data models. All existing structures remain:
```rust
// Existing structures unchanged:
// - AppState
// - LabelData
// - UserProfile
// - PostData
// - etc.
```

### API / Interfaces

**Breaking Changes in Leptos 0.7:**
- Signal traits moved to `leptos::prelude::*`
- Mount functions moved to `leptos::mount::*`
- Router API changes (exact details TBD based on 0.7 docs)
- Resource handling with new `.await` syntax support
- View macro may have syntax updates

**Public API (No Changes):**
Our component public interfaces remain stable - this is purely internal.

---

## Implementation Notes

### Rust Crate Structure
- No changes to crate organization
- `at-peek-web` remains a cdylib
- `atproto_client` remains unchanged

### Key Migration Areas

1. **Component Imports**
   ```rust
   // Old (0.6)
   use leptos::*;
   
   // New (0.7)
   use leptos::prelude::*;
   ```

2. **Signal Usage**
   ```rust
   // Old (0.6)
   let (value, set_value) = create_signal(initial);
   
   // New (0.7) - syntax may be similar but internal implementation improved
   let (value, set_value) = signal(initial);
   ```

3. **Resource Handling**
   ```rust
   // New in 0.7: can await resources
   let data = create_resource(|| fetch_data());
   
   view! {
       <Suspense fallback=|| view! { "Loading..." }>
           {move || async move {
               let result = data.await;
               view! { <div>{result}</div> }
           }}
       </Suspense>
   }
   ```

4. **Router Updates**
   - Review router API changes in 0.7
   - Update route definitions if syntax changed
   - Verify routing behavior unchanged

### Testing Strategy

1. **Pre-migration**: Document current WASM size and compile times
2. **During migration**: Incremental testing of each component
3. **Post-migration**: Full regression test suite
4. **Manual testing**: Test all user flows in browser

---

## ATproto Compatibility

- **Spec Version**: No changes, continues to support current ATproto version
- **Lexicons Used**: No changes to ATproto integration
- **Known Limitations**: None related to this migration

The migration is purely framework-internal and does not affect ATproto interaction.

---

## Security & Privacy Considerations

- No new security concerns introduced
- Privacy model unchanged (all client-side processing)
- No new network requests or data transmission
- Local storage handling unchanged
- Authentication flow unchanged

---

## Testing Plan

- [x] Baseline measurements (WASM size, compile times)
- [ ] Update dependencies in Cargo.toml
- [ ] Fix compilation errors from API changes
- [ ] Run existing test suite (if present)
- [ ] Manual testing: Single label check
- [ ] Manual testing: Bulk analysis
- [ ] Manual testing: Authentication
- [ ] Manual testing: Local storage persistence
- [ ] Manual testing: Error states
- [ ] Manual testing: Dark mode
- [ ] Verify WASM size reduction
- [ ] Verify compile time improvement
- [ ] Browser compatibility check (Chrome, Firefox, Safari)

---

## Rollout Plan

1. **Phase 1: Migration** (This PR)
   - Complete all code changes
   - Verify functionality
   - Measure improvements

2. **Phase 2: Testing** (Pre-merge)
   - Thorough manual testing
   - Deploy to staging environment (if available)
   - Community testing if applicable

3. **Phase 3: Deployment** (Post-merge)
   - Merge to main
   - Update CHANGELOG.md
   - Deploy to production (Cloudflare Pages)
   - Monitor for issues

---

## Open Questions

1. **Leptos 0.7 Stability**: Is 0.7 fully stable or still in beta/RC?
   - Need to verify on crates.io before proceeding
   - May need to use specific version (0.7.0, 0.7.1, etc.)

2. **Breaking Changes**: What are ALL the breaking changes between 0.6 and 0.7?
   - Need comprehensive migration guide or changelog
   - May discover issues during implementation

3. **Trunk Compatibility**: Does Trunk fully support Leptos 0.7?
   - May need Trunk version update
   - May need `wasm-bindgen` version update

4. **Dependencies**: Are all our other dependencies compatible with Leptos 0.7?
   - `reqwest` version may need update
   - `serde` compatibility
   - Other WASM-related crates

---

## References

- [Leptos 0.7 Release Notes](https://github.com/leptos-rs/leptos/releases)
- [Leptos Book - Getting Started](https://book.leptos.dev/)
- [Leptos Migration Guide](https://gist.github.com/azriel91/c7ee2d0275dcec48586d193927414e06)
- [at-peek Constitution](.specify/memory/constitution.md)
- [at-peek README](README.md)

---

## Clarifications

### Session 1: 2025-10-07 - Initial Planning

**Question**: Would there be anything to gain from moving to leptos 0.7 and still using CSR?

**Answer**: Yes, significant benefits for CSR users:
1. ~20% smaller WASM binaries (critical for client-side apps)
2. Faster rendering performance with new Xilem-inspired architecture
3. Better async/await handling in components
4. Improved developer experience with cleaner APIs
5. Enhanced reactivity system with fine-grained updates
6. WebSocket support for potential future features
7. Better compile times

The islands architecture is SSR-specific and not applicable, but the underlying framework improvements benefit all rendering modes including CSR.

**Question**: Is migration feasible for this project?

**Answer**: Yes, highly feasible:
1. Codebase is relatively small (~10 components)
2. Project is in early development stage
3. No SSR dependencies to migrate
4. Clear migration path available
5. Benefits align with project goals (performance, privacy)

**Decision**: Proceed with migration to Leptos 0.7 while maintaining CSR architecture.

