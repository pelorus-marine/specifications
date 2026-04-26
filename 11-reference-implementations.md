# Pelorus Core — Reference Implementations

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification (normative for v1.0)

---

## About This Document

This document lists the official reference implementations for Pelorus Core and defines the rules that all implementations must follow to be considered conformant.

The reference implementations are the canonical, authoritative source of how the specification should be realized in code. They serve as the basis for conformance testing, example code, and community contributions.

**Design decision (locked):** All reference implementations are written in Rust, `no_std` compatible where possible, and `forbid(unsafe_code)`.

---

## 1. Official Reference Crates

The following crates/repositories are the canonical reference implementations:

| Crate / Repository | Purpose | Status | Key Characteristics |
|--------------------|---------|--------|---------------------|
| `pelorus-pgn`      | PGN encoding/decoding and validation | Planned | Full support for all PGNs in `07-pgn-registry.md` |
| `pelorus-pm`       | Power management state machine and selective wake-up | Planned | Implements `04-power-management.md` exactly |
| `pelorus-address`  | Address claiming and NAME handling | Planned | Implements `05-addressing.md` |
| `pelorus-catalog`  | VSS catalog parsing, binding table, and runtime mapping | Planned | Implements `06-signal-catalog.md` |
| `pelorus-gateway`  | Reference gateway firmware (bridge + web UI) | Planned | Implements `09-gateway-specification.md` |
| `pelorus-repeater` | Reference repeater firmware | Planned | Implements `10-repeater-specification.md` |

All crates will be published under the `pelorus-marine` GitHub organization.

---

## 2. Implementation Principles

Every reference implementation and any conformant third-party implementation must follow these rules:

- **Language:** Rust (minimum edition 2021)
- **Safety:** `forbid(unsafe_code)` in all crates
- **Embedded suitability:** `no_std` + `alloc` where possible; optional `std` features for tools
- **Determinism:** No heap allocation in real-time paths (static buffers where needed)
- **Testing:** Full unit and integration test coverage against the conformance test fixtures (to be provided in `15-conformance-test-plan.md`)
- **Licensing:** MIT or Apache 2.0 (chosen per crate)
- **Versioning:** Semantic versioning that tracks the specification version (e.g. `pelorus-pgn` v0.1.x matches spec v0.1)

---

## 3. Conformance

A device or software component is considered Pelorus Core conformant only if it passes the official conformance test suite and correctly implements the behavior defined in documents 01–10.

The reference implementations serve as the gold standard for what “correct behavior” means.

---

## 4. Open Items (to be resolved before v1.0 promotion)

- Exact crate structure and public API surface
- Initial release versions and dependency graph
- Conformance test fixtures and automated test harness
- Rust coding style guide and `clippy` configuration
- Procedure for community contributions to reference crates

---

*This document, together with documents 01–10, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*