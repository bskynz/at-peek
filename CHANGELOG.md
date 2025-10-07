# Changelog

All notable changes to at-peek will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with Cargo workspace
- `atproto_client` crate for ATproto API interactions
  - Handle to DID resolution via `.well-known/atproto-did`
  - AT-URI parsing and validation
  - Label querying from labeler services
  - Type-safe `Label`, `Did`, `Handle` types
  - Label categorization (Adult Content, Violence, Spam, Hate, Moderation Actions)
- `at-peek-web` crate for web UI
  - Leptos-based reactive WASM application
  - Input panel for handles, DIDs, and AT-URIs
  - Label viewer with categorized display
  - Color-coded label badges with metadata
  - Empty state handling
  - Error handling and display
- Project documentation
  - Constitution v1.0.0 defining project principles
  - MVP Phase 1 implementation plan
  - Technical specifications for all components
  - Build instructions and development guide
  - Contributing guidelines
  - MIT/Apache-2.0 dual licensing
- GitHub Actions CI workflow
  - Format checking
  - Clippy linting
  - Test execution
  - WASM build verification
- Development tooling
  - rustfmt configuration
  - Trunk configuration for WASM builds
  - .gitignore for Rust/WASM projects

## [0.1.0] - TBD (MVP Release)

### Target Features
- [x] Basic label querying for handles and DIDs
- [x] Visual label display with categories
- [ ] Request logging panel
- [ ] Local storage for preferences
- [ ] Dark mode toggle
- [ ] Accessibility compliance (WCAG 2.1 AA)
- [ ] Production-ready WASM bundle (< 500 KB)

### Breaking Changes
None (initial release)

### Security
- All code uses safe Rust (`#![forbid(unsafe_code)]`)
- HTTPS-only connections to labeler services
- No data storage or third-party transmission
- No telemetry or tracking

---

## Version History

- **Unreleased** - Initial development
- **0.1.0** - Planned MVP release

## Versioning Policy

- **Major** (x.0.0): Breaking API changes, major feature additions
- **Minor** (0.x.0): New features, backwards-compatible
- **Patch** (0.0.x): Bug fixes, documentation updates

## Constitution Changelog

See `.specify/memory/constitution.md` for constitutional amendments.

Current version: 1.0.0 (ratified 2025-10-07)

