# Command: Code Quality Checks

**Purpose:** Run linting, formatting, and code quality tools

**Category:** Development

---

## Quick Commands

```bash
# Format all code
cargo fmt --all

# Check formatting (CI mode)
cargo fmt --all -- --check

# Run clippy (linter)
cargo clippy --workspace -- -D warnings

# Fix clippy warnings automatically (where possible)
cargo clippy --workspace --fix --allow-dirty

# Check for unused dependencies
cargo +nightly udeps --workspace
```

---

## Detailed Checks

### Clippy Configuration

```bash
# Run clippy with pedantic warnings
cargo clippy --workspace -- \
  -W clippy::pedantic \
  -D warnings

# Run clippy on specific crate
cargo clippy --package atproto_client -- -D warnings
```

### Security Audit

```bash
# Check for security vulnerabilities
cargo audit

# Update advisory database
cargo audit fetch
```

### Dependency Check

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Check dependency tree
cargo tree
```

---

## Pre-Commit Checklist

```bash
#!/bin/bash
# Run before committing

set -e

echo "🔍 Checking formatting..."
cargo fmt --all -- --check

echo "🔍 Running clippy..."
cargo clippy --workspace -- -D warnings

echo "🧪 Running tests..."
cargo test --workspace

echo "✅ All checks passed!"
```

---

## Constitution Check

- Enforces Rust Safety & Performance (clippy rules)
- Maintains code quality standards
- No unsafe code without justification


