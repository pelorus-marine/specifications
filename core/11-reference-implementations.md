# Pelorus Core — Reference Implementations

**Version:** 0.1 Draft  
**Last Updated:** May 2, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document lists the official reference implementations for Pelorus Core and defines rules implementations should follow. The language and safety posture are summarized in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary); crate inventory, versioning, and contribution expectations are **normative** here.

**Where the code lives (today):** The canonical Rust tree is the **`pelorus-marine/platform`** workspace — especially **`pelorus-core`**, **`pelorus-stream`**, **`pelorus-state`**, and host tooling such as **`pelorus-inspector`**. The names in the table below are **logical** components; some may later become **separate** `pelorus-*` crates on crates.io, but they are **developed in `platform` first** (or the noted sibling repo) — not in standalone repositories until explicitly split.

---

## 1. Official Reference Crates

The following **logical** components map to the **current** reference code location:

| Component | Purpose | Home (repository / path) | Maturity | Key characteristics |
|------------|---------|---------------------------|----------|---------------------|
| `pelorus-dcid`      | DCID encoding/decoding and validation | [`platform` / `pelorus-core` / `dcid`](https://github.com/pelorus-marine/platform/tree/main/pelorus-core/src/dcid) | Evolving | `registry`, `protocol` (**03**), `wire` (**04** §7 v1.0), `mapping` — not every **07** lane enumerated yet |
| `pelorus-pm`        | Power management state machine and selective wake-up | Same — [`dcid::wire`](https://github.com/pelorus-marine/platform/tree/main/pelorus-core/src/dcid/wire.rs), full **04** state machine **TBD** | Partial | WUF/NM **payload** codec; coordinated NM / selective-wake **SM** not complete |
| `pelorus-address`   | Address claiming and NAME handling | Same — [`dcid::protocol`](https://github.com/pelorus-marine/platform/tree/main/pelorus-core/src/dcid/protocol.rs) (reserved DCIDs); **05** protocol **TBD** | Partial | **0x0EE00** and request framing hooks; no full claim sequence yet |
| `pelorus-catalog`   | VSS catalog parsing, binding table, and runtime mapping | [`pelorus-core` / `correlation` + `semantics`](https://github.com/pelorus-marine/platform/tree/main/pelorus-core/src); VSS editor in [`pelorus-inspector`](https://github.com/pelorus-marine/platform/tree/main/pelorus-inspector) | Partial | Correlation paths + tooling; full **06** binding engine **TBD** |
| `pelorus-gateway`  | Reference gateway firmware (bridge + web UI) | Scaffold — [`reference-implementations/pelorus-gateway`](https://github.com/pelorus-marine/reference-implementations/tree/main/pelorus-gateway) (will use **`platform`** / `pelorus-core` when implemented) | Scaffold | [`09-gateway-specification.md`](./09-gateway-specification.md) |
| `pelorus-repeater` | Reference repeater firmware | **TBD** in `platform` or hardware repo | Planned | [`10-repeater-specification.md`](./10-repeater-specification.md) |

Optional **standalone** `pelorus-*` package splits and crates.io releases are **TBD**; until then, consumers depend on **`pelorus-marine/platform`** and **semantic versioning of `pelorus-core`** (and siblings) as the release unit.

---

## 2. Implementation Principles

Language, edition, `no_std` posture, and `forbid(unsafe_code)` are fixed at the program level in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary). **Reference crates and third-party implementations** shall also meet:

- **Determinism:** No heap allocation in real-time paths (static buffers where needed)
- **Testing:** Full unit and integration test coverage against the conformance test fixtures (to be provided in `15-conformance-test-plan.md`)
- **Licensing:** MIT or Apache 2.0 (chosen per crate)
- **Versioning:** Semantic versioning that tracks the specification version (e.g. `pelorus-core` **0.1.x** with spec **v0.1**; or a future split `pelorus-dcid` **0.1.x** if published separately)

---

## 3. Conformance

A device or software component is considered Pelorus Core conformant only if it satisfies the process in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary), passes the applicable tests in [15-conformance-test-plan.md](./15-conformance-test-plan.md) when that document is authored, and publishes [16-compliance-self-declaration.md](./16-compliance-self-declaration.md) as required.

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