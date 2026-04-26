# CONTRIBUTING.md

**Pelorus Core Specification and Reference Implementations**

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Living document

---

## About This Document

Thank you for considering a contribution to Pelorus.

This is an open, sailor-first project. The specification (documents 01–16) is the single source of truth. All code, hardware, and documentation must follow it exactly.

**Locked rule (do not relitigate):** The architectural decisions in `ARCHITECTURE.md` and documents 02–10 are final for v1.0. Do not propose changes to bit rates, connector type, power-management model, instance binding strategy, or any other locked item unless the project maintainer explicitly invites reconsideration.

---

## Ways to Contribute

We welcome contributions in these areas, in rough order of priority:

1. **Specification improvements** — fixing errors, clarifying language, adding missing test cases
2. **Reference implementations** — Rust crates (`pelorus-pgn`, `pelorus-pm`, etc.)
3. **Test fixtures and conformance tools**
4. **Documentation** — installation guides, examples, troubleshooting
5. **Real-world testing and feedback** — especially on actual vessels
6. **Hardware reference designs** (KiCad schematics, BOMs)

---

## Development Workflow

1. Fork the repository
2. Create a branch named `feature/xxx` or `fix/yyy`
3. Make your changes
4. Ensure all tests pass (`cargo test`)
5. Update any affected specification documents if your change impacts normative behavior
6. Submit a Pull Request with a clear description

All pull requests must include:
- A clear reason for the change
- Updated tests (where applicable)
- No `unsafe` code unless explicitly approved

---

## Coding Standards (Rust)

- Rust 2021 edition
- `forbid(unsafe_code)` in all crates
- `no_std` + `alloc` preferred for embedded targets
- `clippy` and `rustfmt` must pass with default settings
- All public APIs must be documented
- Every state machine transition must have unit tests

---

## Specification Changes

- Specification changes are treated as normative and require careful review.
- If your change affects any numbered document (01–16), you must also update the corresponding section in `00-document-index.md`.
- Major architectural proposals must be discussed in an issue first.

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