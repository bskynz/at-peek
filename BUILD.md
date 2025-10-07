# Build Instructions

Complete guide to building and running at-peek.

## Prerequisites

### Required

- **Rust** 1.75 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Trunk** - WASM web application bundler
  ```bash
  cargo install trunk
  ```

- **wasm32 target**
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Platform-Specific Dependencies

#### macOS
```bash
# Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf install -y gcc pkg-config openssl-devel
```

#### Windows
- Install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/downloads/)
- Or use [WSL2](https://docs.microsoft.com/en-us/windows/wsl/install) and follow Linux instructions

## Development

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/at-peek.git
cd at-peek
```

### 2. Verify Installation

```bash
# Check Rust
rustc --version
cargo --version

# Check Trunk
trunk --version

# Check wasm target
rustup target list --installed | grep wasm32
```

### 3. Run Development Server

```bash
cd crates/at-peek-web
trunk serve
```

This will:
- Compile Rust to WASM
- Start a development server at `http://localhost:8080`
- Enable hot reload (automatically rebuild on file changes)

### 4. Open in Browser

Navigate to http://localhost:8080 and start testing!

## Production Build

### Build for Release

```bash
cd crates/at-peek-web
trunk build --release
```

This creates an optimized build in `dist/` with:
- Minified WASM
- Compressed assets
- Optimized for size (target: < 500 KB)

### Optimize Further

```bash
# Install wasm-opt (optional but recommended)
cargo install wasm-opt

# After trunk build
cd dist
wasm-opt -Oz -o optimized.wasm *.wasm
```

### Deploy

The `dist/` folder contains static files that can be deployed to:

- **Netlify**: Drag and drop `dist/` folder
- **Vercel**: `vercel deploy dist/`
- **GitHub Pages**: 
  ```bash
  cd dist
  git init
  git add .
  git commit -m "Deploy"
  git push -f git@github.com:user/repo.git main:gh-pages
  ```
- **Any static host**: Upload contents of `dist/`

## Code Quality

### Format Code

```bash
cargo fmt --all
```

### Lint Code

```bash
cargo clippy --workspace -- -D warnings
```

### Run Tests

```bash
# All tests (currently lib only)
cargo test --workspace

# With output
cargo test --workspace -- --nocapture
```

### Full CI Check

```bash
# Run all CI checks locally
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

## Troubleshooting

### "linker not found" error

- **macOS**: Install Xcode Command Line Tools
- **Linux**: Install `build-essential` or equivalent
- **Windows**: Install VS C++ Build Tools

### "OpenSSL not found" error

The project uses `rustls` by default, so this shouldn't occur. If it does:
- **macOS**: `brew install openssl@3`
- **Linux**: `sudo apt-get install libssl-dev`

### WASM build fails

```bash
# Reinstall wasm32 target
rustup target remove wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

# Reinstall trunk
cargo install trunk --force
```

### Hot reload not working

```bash
# Clear trunk cache
rm -rf .trunk

# Restart trunk serve
trunk serve
```

### Bundle too large

```bash
# Check current size
cd crates/at-peek-web/dist
ls -lh *.wasm

# Optimize with wasm-opt
wasm-opt -Oz -o optimized.wasm *.wasm

# Check size again
ls -lh optimized.wasm
```

## Project Structure

```
at-peek/
├── crates/
│   ├── atproto_client/      # ATproto API client (Rust lib)
│   │   ├── src/
│   │   │   ├── lib.rs       # Public API
│   │   │   ├── types.rs     # DID, Handle, Label types
│   │   │   ├── error.rs     # Error types
│   │   │   ├── resolver.rs  # Handle → DID resolution
│   │   │   └── labeler.rs   # Label query client
│   │   └── Cargo.toml
│   │
│   └── at-peek-web/         # Web UI (Leptos WASM)
│       ├── src/
│       │   ├── lib.rs       # Entry point
│       │   ├── components/  # UI components
│       │   ├── state.rs     # App state management
│       │   └── utils.rs     # Helper functions
│       ├── index.html       # HTML template
│       ├── Trunk.toml       # Trunk configuration
│       └── Cargo.toml
│
├── .specify/                # Spec kit documentation
├── Cargo.toml               # Workspace manifest
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md
```

## Performance Targets

- **WASM bundle**: < 500 KB (gzipped)
- **First load**: < 2 seconds
- **Label query**: < 1 second
- **Lighthouse score**: > 90

## Next Steps

- Read [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines
- Review [Constitution](.specify/memory/constitution.md) for project principles
- Check [specs](.specify/specs/) for technical details
- Join discussions on GitHub

## Support

- **Issues**: https://github.com/yourusername/at-peek/issues
- **Discussions**: https://github.com/yourusername/at-peek/discussions
- **Bluesky Moderation Docs**: https://docs.bsky.app/docs/advanced-guides/moderation


