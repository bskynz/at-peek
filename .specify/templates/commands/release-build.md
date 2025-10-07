# Command: Release Build

**Purpose:** Create optimized production builds

**Category:** Release

---

## Web UI Release Build

```bash
cd crates/at-peek-web

# Build optimized WASM
trunk build --release

# Output will be in dist/
# - index.html
# - at_peek_web_bg.wasm (WASM binary)
# - at_peek_web.js (JS glue code)

# Optimize WASM further with wasm-opt
wasm-opt -Oz -o dist/optimized.wasm dist/at_peek_web_bg.wasm
mv dist/optimized.wasm dist/at_peek_web_bg.wasm

# Check bundle size
du -h dist/*.wasm
```

---

## Full Workspace Release Build

```bash
# Build all crates in release mode
cargo build --workspace --release

# Binaries will be in target/release/
```

---

## Size Optimization

### WASM Size Reduction

```toml
# Add to Cargo.toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
panic = 'abort'     # Smaller binary
strip = true        # Remove debug symbols
```

### Check Binary Size

```bash
# WASM bundle
wasm-opt --version
ls -lh dist/*.wasm

# Target: < 500 KB for MVP
```

---

## Release Checklist

- [ ] All tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [ ] Code formatted (`cargo fmt --all -- --check`)
- [ ] CHANGELOG.md updated
- [ ] Version bumped in `Cargo.toml`
- [ ] Release notes written
- [ ] Security audit clean (`cargo audit`)
- [ ] Bundle size < 500 KB
- [ ] Smoke test on fresh browser (no cache)
- [ ] Accessibility audit (Lighthouse, axe-core)
- [ ] Cross-browser test (Chrome, Firefox, Safari)

---

## Deployment

```bash
# Deploy to static host (Netlify, Vercel, GitHub Pages, etc.)
# Simply upload contents of dist/ directory

# Example: GitHub Pages
cd dist
git init
git add .
git commit -m "Deploy $(date)"
git remote add origin git@github.com:user/at-peek.git
git push -f origin main:gh-pages
```

---

## Constitution Check

- WCAG 2.1 AA compliance (accessibility)
- No tracking or analytics in bundle
- HTTPS-only (configure hosting)
- License files included


