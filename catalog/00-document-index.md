# Pelorus Catalog — Specification Document Index

Authoritative list of Pelorus Catalog specification documents. The Pelorus Catalog is the protocol-agnostic semantic data model for Pelorus, expressed in COVESA VSS under a `Vessel.*` root. It is shared infrastructure consumed by Pelorus Core, Pelorus Stream, and Pelorus State.

The catalog defines **meaning** — signal names, types, units, instance handling, overlay attributes. It does not define wire encoding (that lives in Pelorus Core's Data Contract registry at [`../core/07-dcid-registry.md`](../core/07-dcid-registry.md)) or transport (that lives in Pelorus Stream at [`../stream/`](../stream/)).

## Trust Levels

- **Trusted** — written deliberately against external sources; cited content verified.
- **Unverified** — provisional draft; not validated.
- **Final** — frozen.

## Documents

| # | Filename | Purpose | Status | Trust |
| --- | --- | --- | --- | --- |
| 00 | [`00-document-index.md`](./00-document-index.md) | This index | Living | Trusted |
| 01 | [`01-overview.md`](./01-overview.md) | What the `Vessel.*` catalog is; who consumes it; subsystem boundaries; three-layer roles | Draft | Unverified |
| 02 | [`02-structure.md`](./02-structure.md) | Root, top-level branches, VSS `.vspec` format, leaf attributes, worked examples | Draft | Unverified |
| 03 | [`03-instances.md`](./03-instances.md) | Indexed-array canonical form; subsystem resolution of instance identities | Draft | Unverified |
| 04 | [`04-overlays.md`](./04-overlays.md) | Pelorus overlay attributes; Stream `vss` metadata linkage; LMDE bridge linkage | Draft | Unverified |
| 05 | [`05-tooling.md`](./05-tooling.md) | VSS sources, validation, code generation, runtime catalog handling | Draft | Unverified |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
