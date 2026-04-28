# Pelorus Core — Reference Implementations

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document lists the official reference implementations for Pelorus Core and defines rules implementations should follow. The language and safety posture are summarized in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary); crate inventory, versioning, and contribution expectations are **normative** here.

---

## 1. Official Reference Crates

The following crates/repositories are the canonical reference implementations:

| Crate / Repository | Purpose | Status | Key Characteristics |
|--------------------|---------|--------|---------------------|
| `pelorus-dcid`      | DCID encoding/decoding and validation | Planned | Full support for all DCIDs in `07-dcid-registry.md` |
| `pelorus-pm`       | Power management state machine and selective wake-up | Planned | Implements `04-power-management.md` exactly |
| `pelorus-address`  | Address claiming and NAME handling | Planned | Implements `05-addressing.md` |
| `pelorus-catalog`  | VSS catalog parsing, binding table, and runtime mapping | Planned | Implements `06-signal-catalog.md` |
| `pelorus-gateway`  | Reference gateway firmware (bridge + web UI) | Planned | Implements `09-gateway-specification.md` |
| `pelorus-repeater` | Reference repeater firmware | Planned | Implements `10-repeater-specification.md` |

All crates will be published under the `pelorus-marine` GitHub organization.

---

## 2. Implementation Principles

Language, edition, `no_std` posture, and `forbid(unsafe_code)` are fixed at the program level in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary). **Reference crates and third-party implementations** shall also meet:

- **Determinism:** No heap allocation in real-time paths (static buffers where needed)
- **Testing:** Full unit and integration test coverage against the conformance test fixtures (to be provided in `15-conformance-test-plan.md`)
- **Licensing:** MIT or Apache 2.0 (chosen per crate)
- **Versioning:** Semantic versioning that tracks the specification version (e.g. `pelorus-dcid` v0.1.x matches spec v0.1)

---

## 3. Conformance

A device or software component is considered Pelorus Core conformant only if it satisfies the process in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary), passes the applicable tests in [15-conformance-test-plan.md](./15-conformance-test-plan.md) when that document is authored, and publishes [16-compliance-self-declaration.md](./16-compliance-self-declaration.md) as required.

The reference implementations serve as the gold standard for what “correct behavior” means during self-testing.

---

## 4. Open Items (to be resolved before v1.0 promotion)

- Exact crate structure and public API surface
- Initial release versions and dependency graph
- Conformance test fixtures and automated test harness
- Rust coding style guide and `clippy` configuration
- Procedure for community contributions to reference crates

---

*This document supports the full numbered specification set (01–16); reference crates implement subsets of that set as each matures.*