# at-peek 🔍

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Constitution](https://img.shields.io/badge/constitution-v1.0.0-green.svg)](.specify/memory/constitution.md)

> A Rust-powered web UI tool for investigating and visualizing ATproto records

**at-peek** empowers moderators, researchers, and users to investigate content moderation labels applied to Bluesky users and posts. See which labels are applied, by which labeler services, and understand moderation decisions in the decentralized ATproto ecosystem.

---

## Features

- 🔍 **Moderation Label Visibility** – See all labels applied to users and posts
- 🏷️ **Multi-Labeler Support** – Query labels from Bluesky and third-party labeler services
- 🦀 **Built with Rust** – Memory-safe, blazingly fast core
- 🌐 **Web-First UI** – Intuitive interface for understanding moderation decisions
- 🔒 **Privacy-Preserving** – All processing happens locally; no data leaves your device
- 📊 **Protocol-Accurate** – Strict adherence to ATproto label specifications
- 🎯 **Moderator-Friendly** – Clear label explanations, timestamps, and source attribution

---

## Quick Start

> **Note:** This project is currently in early development. While the Rust/web UI is under construction, you can use the CLI scripts to check labels right now.

### Using the CLI Scripts

Two scripts are available for immediate use:

**Bash Script** (requires `jq` and `curl`):
```bash
./check-labels.sh "https://bsky.app/profile/handle.bsky.social/post/POSTID"
```

**Python Script** (requires Python 3 and `requests`):
```bash
python check-labels.py "https://bsky.app/profile/handle.bsky.social/post/POSTID"
```

Both scripts support:
- AT-URIs: `at://did:plc:abc123/app.bsky.feed.post/rkey`
- Bluesky URLs: `https://bsky.app/profile/handle.bsky.social/post/rkey`
- API URLs: `https://bsky.social/xrpc/com.atproto.repo.getRecord?repo=...`

**Authentication** (optional, for protected posts):
```bash
# Set environment variables
export BLUESKY_HANDLE="your-handle.bsky.social"
export BLUESKY_APP_PASSWORD="your-app-password"

# Or let the script prompt you
./check-labels.sh "post-url"
```

### Building the Web UI

**Prerequisites:**
- [Rust](https://rustup.rs/) (1.75+ recommended)
- [Trunk](https://trunkrs.dev/) - WASM build tool

**Installation:**

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk for WASM builds
cargo install trunk

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Clone the repository
git clone https://github.com/bskynz/at-peek.git
cd at-peek

# Run development server with hot reload
cd crates/at-peek-web
trunk serve

# Open http://localhost:8080 in your browser
```

**Production build:**

```bash
cd crates/at-peek-web
trunk build --release

# Optimized build will be in dist/
# Deploy dist/ folder to any static hosting (Netlify, Vercel, GitHub Pages, etc.)
```

---

## Use Cases

- **Content Moderators**: Investigate which labels are applied to reported users/posts
- **Trust & Safety Teams**: Audit moderation decisions and label sources
- **Content Creators**: Check if your posts have been labeled and why
- **Researchers**: Analyze moderation patterns across different labeler services
- **Appeal Handlers**: Understand existing labels when reviewing appeals

---

## Architecture

at-peek follows a **privacy-first, local-first** architecture:

1. **User Input**: Enter a DID, Bluesky handle, or post AT-URI
2. **Label Fetching**: Query ATproto label endpoints and labeler services (requests logged in UI)
3. **Local Processing**: Parse, validate, and visualize moderation labels in your browser
4. **Multi-Source Labels**: Fetch from Bluesky's moderation service and third-party labelers
5. **No Server Storage**: All state kept in browser local storage; clearable on demand

---

## Project Principles

This project is governed by a [constitution](.specify/memory/constitution.md) that establishes five core principles:

1. **Rust Safety & Performance** – Memory-safe, idiomatic code with zero-cost abstractions
2. **User Privacy by Design** – No tracking, no third-party data transmission
3. **Protocol Fidelity & Data Accuracy** – Strict ATproto spec compliance
4. **Clarity & Discoverability in UI** – Intuitive interfaces with inline help
5. **Open Source Transparency & Community** – Public development, welcoming contributions

All contributions must align with these principles. See [CONTRIBUTING.md](CONTRIBUTING.md) (coming soon) for details.

---

## Contributing

We welcome contributions! Please:

1. Read the [constitution](.specify/memory/constitution.md) to understand project values
2. Check open [issues](https://github.com/bskynz/at-peek/issues) or propose new ones
3. Submit pull requests with clear descriptions and tests
4. Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

---

## Roadmap

- [ ] Core ATproto label fetching from moderation services
- [ ] Web UI prototype with DID/handle resolution
- [ ] Label visualization (badges, categories, severity)
- [ ] Multi-labeler service support (Bluesky + third-party)
- [ ] Post label inspection (by AT-URI)
- [ ] Label history timeline (when labels were applied/removed)
- [ ] Dark mode and accessibility improvements
- [ ] Export labels to JSON/CSV for analysis

See [open issues](https://github.com/bskynz/at-peek/issues) for detailed status.

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## Acknowledgments

- [AT Protocol](https://atproto.com/) – The protocol this tool investigates
- [Bluesky](https://blueskyweb.xyz/) – Reference implementation and community

---

**Project Status:** 🚧 Early Development  
**ATproto Compatibility:** TBD (will track official spec versions)

