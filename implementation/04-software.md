# Pelorus Implementation — Software

Reference implementations of Pelorus subsystems live in the Pelorus Platform repository:

> <https://github.com/pelorus-marine/platform>

This document is intentionally short. The platform repository is the canonical home for Rust crates, firmware references, and software tooling; its layout and naming are under active development and will change before v1.0. Refer to the platform repository's own README and per-crate documentation for the current shape — this specification does not mirror them.

## 1. Scope of the Platform Repository

The platform repository covers reference implementations across all Pelorus subsystems:

- Pelorus Core ([`../core/`](../core/)) — wire, addressing, power, multi-frame transport, dual-bus, binding cache, firmware update
- Pelorus Stream ([`../stream/`](../stream/)) — QUIC transport, control protocol, discovery, services
- Pelorus State ([`../state/`](../state/)) — ingest, snapshot, situation, policy, intents
- Pelorus Catalog ([`../catalog/`](../catalog/)) — VSS overlay tooling, code generation
- Conformance test fixtures and harnesses
- Reference firmware for gateway, repeater, and VDR devices

## 2. Language and Stack

The platform standardises on Rust, with `no_std` and `forbid(unsafe_code)` as the default posture for embedded crates. Specific dependency choices, crate boundaries, and testing requirements are documented inside the platform repository.

## 3. Conformance

A device or software component is **conformant** only if it passes the applicable tests in [`../core/11-conformance.md`](../core/11-conformance.md) and publishes the self-declaration template defined there. Platform crates serve as the reference behaviour during self-testing.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
