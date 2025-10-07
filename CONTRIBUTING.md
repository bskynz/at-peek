# Contributing to at-peek

Thank you for your interest in contributing to **at-peek**! We welcome contributions from everyone, whether you're fixing a typo, adding a feature, or proposing architectural changes.

---

## Getting Started

1. **Read the [Constitution](.specify/memory/constitution.md)**  
   Our project is guided by five core principles. All contributions must align with these values, especially:
   - Rust safety and performance
   - User privacy by design
   - Protocol fidelity
   - UI clarity
   - Open source transparency

2. **Set up your development environment**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Clone the repository
   git clone https://github.com/yourusername/at-peek.git
   cd at-peek
   
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   
   # Check code quality
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

3. **Find something to work on**
   - Check [open issues](https://github.com/yourusername/at-peek/issues)
   - Look for issues labeled `good-first-issue` or `help-wanted`
   - Propose new features by opening an issue first for discussion

---

## Contribution Workflow

### 1. Fork and Branch

```bash
# Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/at-peek.git
cd at-peek

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write clean, idiomatic Rust code
- Add tests for new functionality
- Update documentation (inline docs, README, or specs as needed)
- Run `cargo fmt` to format code
- Run `cargo clippy` and fix any warnings
- Ensure all tests pass with `cargo test`

### 3. Commit

Use clear, descriptive commit messages following [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add DID resolution UI component
fix: handle malformed ATproto records gracefully
docs: update README with installation steps
chore: update dependencies to latest versions
```

### 4. Submit a Pull Request

- Push your branch to your fork
- Open a PR against the `main` branch of the upstream repo
- Fill out the PR template (if provided) with:
  - **What**: Describe the change
  - **Why**: Explain the motivation
  - **Constitution Check**: Note which principles are impacted
  - **Testing**: Describe how you tested the change
- Link related issues (e.g., "Closes #42")

### 5. Code Review

- Maintainers will review your PR within 14 days
- Address any feedback by pushing new commits to your branch
- Once approved, a maintainer will merge your PR

---

## Code Standards

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `#![forbid(unsafe_code)]` by default; justify any unsafe blocks inline
- Prefer explicit error handling (`Result<T, E>`) over panics
- Write comprehensive doc comments for public APIs
- Use meaningful variable and function names

### Testing

- Unit tests for business logic
- Integration tests for cross-module interactions
- UI tests for critical user flows (framework TBD)
- Aim for meaningful coverage, not just high percentages

### Documentation

- All public functions and types MUST have `///` doc comments
- Include examples in doc comments where helpful
- Update README.md for user-facing changes
- Add entries to CHANGELOG.md (we follow [Keep a Changelog](https://keepachangelog.com/))

---

## Constitutional Compliance

Before submitting, ensure your contribution aligns with our principles:

- [ ] **No unsafe code** without explicit justification
- [ ] **No telemetry or third-party tracking** introduced
- [ ] **ATproto spec version compatibility** documented if parsing logic changed
- [ ] **UI changes** include accessibility considerations
- [ ] **New dependencies** are reviewed for licensing and security

Violations of constitutional principles may result in PR rejection unless a constitutional amendment is proposed.

---

## Reporting Issues

Found a bug? Have a feature request?

1. **Search existing issues** to avoid duplicates
2. Open a new issue with:
   - **Title**: Clear, concise summary
   - **Description**: Steps to reproduce (for bugs) or use case (for features)
   - **Environment**: OS, Rust version, browser (if UI-related)
   - **Screenshots/Logs**: If applicable
3. Tag appropriately: `bug`, `enhancement`, `question`, `constitution-compliance`, etc.

---

## Security Disclosures

If you discover a security vulnerability, **do not open a public issue**. Instead:

- Email the maintainer(s) directly (TBD: add email address)
- Include details, reproduction steps, and potential impact
- We'll respond within 72 hours and work with you on a fix

---

## Community Guidelines

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and constructive in discussions
- Follow our [Code of Conduct](CODE_OF_CONDUCT.md)
- Help newcomers and share knowledge generously
- Assume good intent; disagree with ideas, not people

---

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0 dual license). You affirm that you have the right to submit your contribution under these licenses.

---

## Questions?

- Open a [GitHub Discussion](https://github.com/yourusername/at-peek/discussions)
- Comment on relevant issues
- Reach out to maintainers (contact info TBD)

Thank you for helping make at-peek better! 🎉

