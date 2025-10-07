# Command: Build and Test

**Purpose:** Build the project and run all tests

**Category:** Development

---

## Quick Commands

```bash
# Full build with all crates
cargo build --workspace

# Release build (optimized)
cargo build --workspace --release

# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name --package crate_name
```

---

## Testing Strategy

### Unit Tests

```bash
# Test specific crate
cargo test --package atproto_client

# Test with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html
```

### Integration Tests

```bash
# Run integration tests (network required)
cargo test --test integration_tests -- --ignored

# Run only fast tests (skip network tests)
cargo test --workspace
```

### Web UI Tests

```bash
# Build WASM
cd crates/at-peek-web
trunk build

# Run WASM tests in browser
wasm-pack test --headless --firefox
```

---

## CI/CD Equivalent

```bash
# Reproduce CI locally
cargo fmt -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

---

## Constitution Check

- Ensures Rust Safety & Performance (build passes)
- Validates Protocol Fidelity (tests pass)


