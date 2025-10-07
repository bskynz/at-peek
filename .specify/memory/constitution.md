<!--
Sync Impact Report:
Version: 0.0.0 → 1.0.0
Initial constitution creation for at-peek project.

Modified principles: N/A (initial creation)
Added sections:
  - Core Identity
  - Founding Principles (5 principles)
  - Governance
Removed sections: None

Templates status:
  ⚠ .specify/templates/plan-template.md - pending (not yet created)
  ⚠ .specify/templates/spec-template.md - pending (not yet created)
  ⚠ .specify/templates/tasks-template.md - pending (not yet created)
  ⚠ .specify/templates/commands/*.md - pending (not yet created)

Follow-up TODOs: Create supporting template files to align with constitution principles.
-->

# Project Constitution

**Version:** 1.0.0  
**Ratification Date:** 2025-10-07  
**Last Amended:** 2025-10-07

---

## Core Identity

**Project Name:** at-peek

**Mission Statement:**  
at-peek is a Rust-based web UI tool designed to investigate and visualize content moderation labels applied to ATproto (AT Protocol) users, posts, and records. By providing transparent visibility into moderation decisions and labels from various labeler services, the tool empowers moderators, researchers, and users to understand moderation actions in the decentralized Bluesky/ATproto ecosystem with clarity, privacy, and performance.

**Target Audience:**  
- Bluesky community moderators investigating reported content
- Trust & Safety teams analyzing moderation label patterns
- Content creators checking moderation status of their posts
- Researchers studying decentralized moderation systems
- End users seeking transparency into moderation decisions affecting their content

---

## Founding Principles

### Principle 1: Rust Safety & Performance

**Statement:**  
All core logic MUST be written in idiomatic, safe Rust. Unsafe code blocks MUST be explicitly justified, documented, and minimized. The tool MUST prioritize memory safety, zero-cost abstractions, and efficient resource usage to handle large datasets without degradation.

**Rationale:**  
Rust's ownership model prevents entire classes of bugs (memory leaks, data races) while delivering native performance. For a tool processing potentially large volumes of ATproto records, safety and speed are non-negotiable to ensure reliability and user trust.

**Implementation Requirements:**
- All code MUST pass `cargo clippy` with no warnings (or explicitly documented exceptions)
- All public APIs MUST have comprehensive documentation
- Performance-critical paths MUST be benchmarked
- MUST use `#![forbid(unsafe_code)]` by default; unsafe blocks require inline rationale

### Principle 2: User Privacy by Design

**Statement:**  
The tool MUST NOT store, log, or transmit any user DIDs, handles, or record data to third parties without explicit user consent. All network requests MUST be transparently displayed to the user. Data fetched from ATproto MUST be processed locally and MUST be clearable on demand.

**Rationale:**  
Users investigating their own or others' public records deserve assurance that the investigation tool itself is not harvesting data. Privacy-first design builds trust and aligns with the decentralized ethos of ATproto.

**Implementation Requirements:**
- No telemetry or analytics by default
- All ATproto API calls MUST be logged in the UI (request log panel)
- Session data MUST be stored in browser local storage only (Web UI)
- MUST provide a "Clear All Data" button that wipes local state
- MUST NOT include tracking scripts or third-party dependencies that phone home

### Principle 3: Protocol Fidelity & Data Accuracy

**Statement:**  
All ATproto record parsing, validation, and display MUST conform strictly to the official ATproto specifications. When specifications are ambiguous or evolving, the tool MUST document assumptions and version compatibility. Invalid or malformed records MUST be clearly flagged without silent failures.

**Rationale:**  
Users rely on at-peek to understand ground truth about ATproto data. Misrepresenting record structure or silently ignoring errors undermines the tool's core value proposition.

**Implementation Requirements:**
- MUST use official ATproto lexicons and schemas where available
- MUST clearly display ATproto spec version compatibility (e.g., "Compatible with atproto v0.3.x")
- MUST show validation errors inline with record data
- MUST provide raw JSON view alongside parsed representations
- MUST update dependencies when ATproto specifications change (tracked in changelog)

### Principle 4: Clarity & Discoverability in UI

**Statement:**  
The web UI MUST present complex ATproto structures in an intuitive, hierarchical manner. Every UI element MUST have clear labels and tooltips explaining ATproto-specific terminology. Users MUST be able to navigate from high-level summaries to deep record inspection with minimal friction.

**Rationale:**  
ATproto records can be deeply nested and technical. A good investigation tool acts as a teaching aid, making the protocol accessible to non-experts while remaining powerful for advanced users.

**Implementation Requirements:**
- MUST provide expandable/collapsible tree views for nested records
- MUST include inline help tooltips for ATproto jargon (DID, CID, record keys, etc.)
- MUST support search/filter across record fields
- MUST offer dark mode and accessibility compliance (WCAG 2.1 AA minimum)
- MUST show loading states and error messages that suggest remediation steps

### Principle 5: Open Source Transparency & Community

**Statement:**  
The project MUST remain open source (permissive license recommended: MIT or Apache 2.0). All architectural decisions, roadmap items, and security considerations MUST be documented publicly. Community contributions MUST be welcomed with clear contribution guidelines and respectful code review.

**Rationale:**  
Trust in an investigation tool comes from verifiability. Open source ensures users can audit the codebase, propose improvements, and fork if needed. Transparency aligns with ATproto's decentralized philosophy.

**Implementation Requirements:**
- MUST maintain a public repository with clear README, LICENSE, and CONTRIBUTING.md
- MUST document build instructions for all major platforms
- MUST use semantic versioning and publish release notes
- MUST respond to issues and pull requests within 14 days (or mark as triaged)
- MUST include a CODE_OF_CONDUCT.md fostering inclusive participation

---

## Governance

### Amendment Procedure

This constitution may be amended by the project maintainer(s) through the following process:

1. Propose amendment via GitHub issue or pull request, clearly stating:
   - Which principle(s) or section(s) are affected
   - Rationale for change
   - Backward compatibility impact
2. Allow a minimum 7-day comment period for community feedback
3. Incorporate feedback or document reasons for declining suggestions
4. Update `CONSTITUTION_VERSION` according to semantic versioning:
   - **MAJOR**: Principle removed, redefined incompatibly, or governance authority restructured
   - **MINOR**: New principle added or existing principle materially expanded
   - **PATCH**: Clarifications, typo fixes, non-semantic wording improvements
5. Update `LAST_AMENDED_DATE` and prepend a new Sync Impact Report comment
6. Merge the amendment and publish in release notes

### Versioning Policy

- Constitution versions follow **semantic versioning** (MAJOR.MINOR.PATCH)
- All versions MUST be tagged in git (e.g., `constitution-v1.2.0`)
- Each version change MUST include a Sync Impact Report documenting what changed and which templates/docs were updated

### Compliance Review

- Maintainers MUST audit the codebase against this constitution quarterly
- Any violations MUST be logged as GitHub issues labeled `constitution-compliance`
- Pull requests introducing violations SHOULD be rejected unless they propose a constitutional amendment

### Dispute Resolution

- Technical disputes are resolved by project maintainer(s)
- Community concerns can be escalated via GitHub Discussions
- If maintainers are unresponsive for 90+ days, trusted contributors may fork and propose a governance transition

---

## Adoption

This constitution was ratified on **2025-10-07** and takes effect immediately for all new contributions. Existing code should be brought into compliance incrementally, with non-compliant areas documented as known technical debt.

**Signed (Initial Ratification):**  
Project Maintainer(s) – 2025-10-07
