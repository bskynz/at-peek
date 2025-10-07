# Command: Project Setup

**Purpose:** Set up development environment from scratch

**Category:** Setup

---

## Prerequisites

### Install Rust

```bash
# Install Rust via rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust to latest stable
rustup update stable

# Verify installation
rustc --version
cargo --version
```

### Install Additional Tools

```bash
# Trunk (for WASM builds)
cargo install trunk

# wasm-opt (for WASM optimization)
cargo install wasm-opt

# cargo-watch (for auto-reload during development)
cargo install cargo-watch

# cargo-audit (security vulnerability scanning)
cargo install cargo-audit

# cargo-tarpaulin (code coverage, Linux only)
cargo install cargo-tarpaulin
```

---

## Clone and Build

```bash
# Clone repository
git clone https://github.com/yourusername/at-peek.git
cd at-peek

# Build workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Check code quality
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

---

## IDE Setup

### VS Code

Install extensions:
- `rust-analyzer` (Rust language server)
- `CodeLLDB` (Rust debugger)
- `crates` (dependency version management)
- `Even Better TOML` (Cargo.toml syntax)

Workspace settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Zed

Zed has built-in Rust support. Just open the workspace:
```bash
zed .
```

---

## Environment Configuration

### Optional: `.env` file

```bash
# Create .env file (not tracked in git)
cat > .env <<EOF
RUST_LOG=debug
ATPROTO_LABELER_URL=https://mod.bsky.app
EOF
```

### Git Hooks (Optional)

```bash
# Set up pre-commit hook
cat > .git/hooks/pre-commit <<'EOF'
#!/bin/bash
set -e
echo "🔍 Running pre-commit checks..."
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
echo "✅ Pre-commit checks passed!"
EOF

chmod +x .git/hooks/pre-commit
```

---

## Verify Setup

```bash
# Run full verification
./scripts/verify-setup.sh

# Or manually:
echo "✅ Rust toolchain:"
rustc --version

echo "✅ Cargo:"
cargo --version

echo "✅ Trunk:"
trunk --version

echo "✅ wasm-opt:"
wasm-opt --version

echo "✅ Project builds:"
cargo build --workspace

echo "✅ Tests pass:"
cargo test --workspace

echo "✅ Lints pass:"
cargo clippy --workspace -- -D warnings

echo "✅ Format check:"
cargo fmt --all -- --check

echo ""
echo "🎉 Setup complete! You're ready to develop."
```

---

## Platform-Specific Notes

### macOS

```bash
# Install Xcode Command Line Tools (if not already installed)
xcode-select --install

# Homebrew dependencies (optional)
brew install jq curl
```

### Linux

```bash
# Ubuntu/Debian dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev curl jq

# Fedora/RHEL dependencies
sudo dnf install -y gcc pkg-config openssl-devel curl jq
```

### Windows

```bash
# Install Visual Studio C++ Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Use WSL2 for better experience (recommended)
wsl --install
# Then follow Linux instructions inside WSL
```

---

## Troubleshooting

### "linker not found" error
- **macOS**: Install Xcode Command Line Tools
- **Linux**: Install `build-essential` or equivalent
- **Windows**: Install VS C++ Build Tools

### "OpenSSL not found" error
- **macOS**: `brew install openssl@3`
- **Linux**: `sudo apt-get install libssl-dev`
- **Windows**: Use `rustls` instead (already configured)

### WASM build fails
```bash
# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Reinstall trunk
cargo install trunk --force
```

---

## Constitution Check

- All dependencies audited (`cargo audit`)
- No unsafe dependencies by default
- Open source toolchain (Rust, trunk, etc.)
- Privacy-preserving (no analytics in dev tools)


