# at-peek: Project Summary

**Status:** 🚧 MVP Implementation Complete - Ready for Development

## What We've Built

A complete Rust web application for inspecting content moderation labels on ATproto users and posts.

## Core Features

✅ **ATproto Client Library** (`atproto_client`)
- Handle-to-DID resolution
- AT-URI parsing
- Label fetching from labeler services (Bluesky's mod.bsky.app)
- Type-safe label representations
- Label categorization (Adult Content, Violence, Spam, Hate, etc.)

✅ **Web UI** (`at-peek-web`)
- Leptos-based reactive UI compiled to WASM
- **Two operation modes:**
  - **Single Check**: Query labels for individual handles, DIDs, or AT-URIs
  - **Bulk Analysis**: Analyze last 1000 posts from a user with statistics
- Authentication support for viewing restricted content and admin labels
- Visual label badges with color-coding
- Categorized label display with expand/collapse
- Post details modal with media display (images/videos)
- Like/repost counts and user lists
- Empty state handling
- Error handling with user-friendly messages
- Dark mode ready (styling pending)

## Architecture

```
User Input → Web UI (WASM) → atproto_client → Labeler Service
                                                    ↓
                                              com.atproto.label.queryLabels
                                                    ↓
                                              Labels Response
                                                    ↓
                                  UI displays categorized labels
```

## Tech Stack

- **Language**: Rust (100% safe, no unsafe code)
- **Frontend**: Leptos 0.6 (Reactive WASM framework)
- **HTTP Client**: reqwest (with rustls for HTTPS)
- **Build Tool**: Trunk (WASM bundler with hot reload)
- **Styling**: TailwindCSS (CDN in dev, can optimize for production)
- **Target**: wasm32-unknown-unknown

## Privacy & Security

✅ All processing happens locally in browser  
✅ No backend server required  
✅ No data storage (except localStorage for preferences - not yet implemented)  
✅ No telemetry or tracking  
✅ All network requests visible to user  
✅ HTTPS-only labeler queries  

## Constitutional Compliance

| Principle | Status |
|-----------|--------|
| Rust Safety & Performance | ✅ All code safe, optimized for WASM |
| User Privacy by Design | ✅ Local-first, no tracking |
| Protocol Fidelity & Data Accuracy | ✅ Strict ATproto compliance |
| Clarity & Discoverability in UI | ✅ Clear labels, tooltips, categories |
| Open Source Transparency | ✅ MIT/Apache-2.0, documented |

## File Structure

```
at-peek/
├── crates/
│   ├── atproto_client/          # Core ATproto library
│   │   ├── src/
│   │   │   ├── lib.rs           # Public API
│   │   │   ├── types.rs         # Did, Handle, Label types
│   │   │   ├── error.rs         # Error handling
│   │   │   ├── resolver.rs      # Handle resolution
│   │   │   └── labeler.rs       # Label client
│   │   └── Cargo.toml
│   │
│   └── at-peek-web/             # Web UI
│       ├── src/
│       │   ├── lib.rs           # WASM entry
│       │   ├── components/      # UI components
│       │   │   ├── app.rs       # Root app with tab navigation
│       │   │   ├── header.rs    # Header with branding
│       │   │   ├── auth_panel.rs # Authentication UI
│       │   │   ├── input_panel.rs # Single check input
│       │   │   ├── label_viewer.rs # Label display
│       │   │   ├── label_badge.rs # Label badges
│       │   │   ├── bulk_analysis.rs # Bulk analysis mode
│       │   │   └── empty_state.rs # Empty states
│       │   ├── state.rs         # App state
│       │   └── utils.rs         # Helpers
│       ├── index.html
│       ├── Trunk.toml
│       └── Cargo.toml
│
├── .specify/                     # Spec kit documentation
│   ├── memory/
│   │   └── constitution.md      # Project constitution v1.0.0
│   ├── plans/
│   │   └── mvp-phase1.md        # MVP implementation plan
│   ├── specs/
│   │   ├── atproto-client.md    # Client spec
│   │   ├── web-ui-architecture.md
│   │   ├── privacy-storage.md
│   │   └── label-display-components.md
│   ├── tasks/
│   │   ├── m1-project-scaffold.md
│   │   ├── m2-record-fetching.md
│   │   ├── m3-web-ui-prototype.md
│   │   ├── m4-logging-storage.md
│   │   └── m5-polish-dark-mode.md
│   └── templates/
│       ├── commands/            # Common operations
│       │   ├── setup-project.md
│       │   ├── build-and-test.md
│       │   ├── code-quality.md
│       │   ├── dev-server.md
│       │   ├── release-build.md
│       │   └── query-labels-manual.md
│       └── ... (spec templates)
│
├── .github/workflows/
│   └── ci.yml                   # CI/CD pipeline
│
├── Cargo.toml                   # Workspace manifest
├── BUILD.md                     # Detailed build guide
├── CONTRIBUTING.md              # Contribution guide
├── README.md                    # Main readme
├── LICENSE-MIT
├── LICENSE-APACHE
├── .gitignore
└── rustfmt.toml
```

## What Works Now

✅ **Single Check Mode**
- Handle resolution (alice.bsky.social → DID)  
- DID input (did:plc:...)  
- AT-URI input (at://...)  
- Label querying from mod.bsky.app and user PDS
- Label display with categories  
- Color-coded badges  

✅ **Bulk Analysis Mode**
- Analyze last 1000 posts from any user
- Label statistics by category
- Account-level moderation detection
- Post details with media preview
- Like/repost counts and user attribution
- Timeline of when labels were applied

✅ **Authentication**
- Bluesky login with app passwords
- Access to restricted content and admin labels (!ban, !takedown, etc.)
- Session-only token storage (browser memory)

✅ **UI/UX**
- Empty state handling  
- Error messages with helpful guidance
- Responsive layout (basic)
- Loading states and progress indicators  

## What's Next (MVP Phase 1 Completion)

### Immediate (M4 - M5)
- [ ] Request logging panel
- [ ] localStorage for preferences
- [ ] "Clear All Data" button
- [ ] Dark mode toggle
- [ ] Better loading states
- [ ] Accessibility improvements (ARIA labels, keyboard nav)
- [ ] Tooltip explanations for each label type

### Soon (Phase 2)
- [ ] Multiple labeler service support
- [ ] Label history timeline
- [ ] Export labels to JSON/CSV
- [ ] Post AT-URI validation before querying
- [ ] Better error messages with remediation steps
- [ ] Performance optimizations
- [ ] Bundle size reduction

### Future
- [ ] Custom labeler configuration
- [ ] Label comparison (multiple users/posts)
- [ ] Analytics dashboard (local-only, privacy-preserving)
- [ ] Export bulk analysis results to JSON/CSV

## How to Get Started

### Development

```bash
# Install prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install trunk
rustup target add wasm32-unknown-unknown

# Clone and run
git clone <repo-url>
cd at-peek/crates/at-peek-web
trunk serve

# Open http://localhost:8080
```

### Testing

```bash
# Try these examples in Single Check mode:
1. Enter: alice.bsky.social (or any handle)
2. Enter: did:plc:z72i7hdynmk6r22z27h6tvur (Bluesky official account)
3. Enter: at://did:plc:example/app.bsky.feed.post/abc123 (any post AT-URI)

# Try these in Bulk Analysis mode:
1. Enter a handle to analyze their last 1000 posts
2. View label statistics and categorization
3. Click on labeled posts to see details
4. Check account-level moderation labels
```

## Documentation

- **Constitution**: `.specify/memory/constitution.md` - Project principles
- **Build Guide**: `BUILD.md` - Detailed build instructions
- **Contributing**: `CONTRIBUTING.md` - How to contribute
- **Specs**: `.specify/specs/` - Technical specifications
- **Tasks**: `.specify/tasks/` - Implementation tasks

## Performance Metrics

Current targets:
- WASM bundle: < 500 KB (not yet optimized)
- Label query: < 1 second
- UI response: < 100 ms

## Security Considerations

✅ No unsafe Rust code (`#![forbid(unsafe_code)]`)  
✅ HTTPS-only connections  
✅ Authentication tokens stored in browser memory only (not localStorage/cookies)  
✅ App passwords recommended (not main account password)  
✅ No credential persistence between sessions  
✅ No third-party tracking  
✅ No cookies  
✅ Local-only processing  

## Known Limitations

- Only supports Bluesky's official labeler (mod.bsky.app) and user PDS servers
- No label history/timeline for single check mode (available in bulk analysis)
- No self-labels support yet
- Basic styling (TailwindCSS via CDN)
- No offline support
- Authentication token stored in browser memory only (cleared on page refresh)

## License

Dual-licensed under MIT OR Apache-2.0 at your option.

## Questions?

- Read the constitution: `.specify/memory/constitution.md`
- Check build guide: `BUILD.md`
- Review specs: `.specify/specs/`
- Open an issue on GitHub

---

**Next Step**: Run `cd crates/at-peek-web && trunk serve` to start developing! 🚀

