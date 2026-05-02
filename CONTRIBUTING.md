# CONTRIBUTING.md

**Pelorus Core Specification and Reference Implementations**

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Living document

---

## About This Document

Thank you for considering a contribution to Pelorus.

This is an open, sailor-first project. The specification (`core/00-document-index.md` through `core/16-…`) is the single source of truth. All code, hardware, and documentation must follow it exactly.

**Governance:** Major architectural choices documented in `ARCHITECTURE.md` and the trusted core under `core/` (especially **01–04**) should not be reopened for v1.0 unless the project maintainer explicitly invites reconsideration.

---

## Ways to Contribute

We welcome contributions in these areas, in rough order of priority:

1. **Specification improvements** — fixing errors, clarifying language, adding missing test cases
2. **Reference implementations** — Rust crates (`pelorus-dcid`, `pelorus-pm`, etc.)
3. **Test fixtures and conformance tools**
4. **Documentation** — installation guides, examples, troubleshooting
5. **Real-world testing and feedback** — especially on actual vessels
6. **Hardware reference designs** (KiCad schematics, BOMs)

---

## Development Workflow

1. Fork the repository
2. Create a branch named `feature/xxx` or `fix/yyy`
3. Make your changes
4. Ensure checks pass — `cargo fmt`, `cargo clippy`, and **`cargo test --workspace`** at repository root (Rust workspace); if you change **`core/`** or **`stream/`**, run **`cargo run -p xtask -- book-build`** once ([`SPEC_BOOK.md`](./SPEC_BOOK.md))
5. Update any affected specification documents if your change impacts normative behavior
6. Submit a Pull Request with a clear description

All pull requests must include:
- A clear reason for the change
- Updated tests (where applicable)
- No `unsafe` code unless explicitly approved

---

## Markdown link checks

Changes touching Markdown files trigger [`.github/workflows/markdown-links.yml`](.github/workflows/markdown-links.yml) — external URL checks with [`.markdown-link-check.json`](.markdown-link-check.json). Extend **`ignorePatterns`** there for benign flaps (redirect quirks, authenticated endpoints).

---

## Specification library (HTML book)

The **`xtask`** binary mirrors `core/` and `stream/` into mdBook and runs **`mdbook build`**. Install **`mdbook`** once (`cargo install mdbook`), then see **[`SPEC_BOOK.md`](./SPEC_BOOK.md)**. Pull requests that touch corpus paths run [`.github/workflows/book.yml`](.github/workflows/book.yml).

---

## Coding Standards (Rust)

- Workspace crates (**`pelorus-spec`**, **`xtask`**) use edition **2024** (`rust-toolchain.toml`). Separate **reference implementation** crates may remain on Rust 2021 until migrated.
- `forbid(unsafe_code)` in all crates
- `no_std` + `alloc` preferred for embedded targets
- `clippy` and `rustfmt` must pass with default settings
- All public APIs must be documented
- Every state machine transition must have unit tests

---

## Specification Changes

- Specification changes are treated as normative and require careful review.
- If your change affects any numbered document (01–16), you must also update the corresponding section in `core/00-document-index.md`.
- Major architectural proposals must be discussed in an issue first.

### DCID numbering, registry, and VSS linkage

Any change that **alters DCID assignment, DCID versioning, or the semantic meaning of the `dcid` overlay attribute** must be submitted as **one coherent Pull Request** that updates together:

- [`core/07-dcid-registry.md`](./core/07-dcid-registry.md)
- [`core/06-signal-catalog.md`](./core/06-signal-catalog.md) §6 (and any affected `catalog/` entries)
- [`stream/01-overview.md`](./stream/01-overview.md) §3.3 if Stream–Core identifier narrative is affected
- Any machine-readable **DCID contract** artifact that ships in-repo (schema, registry snapshot, generated bundle for compilers or gateways) — **update or re-version it in this same PR** when DCID assignment or semantics change. There is no separate generator committed yet; when one lands, this rule still applies.

Rationale: prevents gateways, compilers, and docs from drifting out of sync. Exploratory DCID structure belongs in [Issue #3](https://github.com/pelorus-marine/specifications/issues/3) until promoted into **07**.

---

## Issue Reporting

Please open an issue for:
- Bugs in the specification
- Ambiguities or missing details
- Conformance failures
- Real-world interoperability problems

Include as much detail as possible (logs, bus traces, hardware used, etc.).

---

## Community Guidelines

- Be respectful and constructive
- Focus on technical merit, not personal opinion
- Prioritize sailor reliability and long-term maintainability
- See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for full details

---

## Questions?

The preferred channel is a GitHub issue in the relevant repository.

Thank you for helping make Pelorus a genuine open alternative for marine data networking.

*This document is part of the Pelorus Core specification. Changes to this file must follow the same review process as any other specification document.*