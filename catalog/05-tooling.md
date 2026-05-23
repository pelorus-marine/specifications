# Pelorus Catalog — Tooling

VSS sources, validation, code generation, and runtime catalog handling.

## 1. Source Files

- `catalog/vessel.vspec` — root VSS specification for the `Vessel.*` tree.
- `catalog/contracts/*.yaml` — machine-readable Data Contract entries cross-linked to catalog leaves (when this artifact lands).
- Overlay files for Pelorus-specific attributes (`data_contract`, `instance-field`, `pelorus-priority`) per [`04-overlays.md`](./04-overlays.md).

Source files are CC BY 4.0 alongside the specification.

## 2. Validation

`vss-tools` validates the catalog with the Pelorus overlay profile. Validation passes are part of the conformance fixtures referenced in [`../core/11-conformance.md`](../core/11-conformance.md).

## 3. Code Generation

Reference crates in the Pelorus Platform repository (see [`../implementation/04-software.md`](../implementation/04-software.md)) generate:

- Rust structs and validation functions for catalog leaves
- TypeScript type definitions for tooling and inspectors
- Constant tables for DC ↔ catalog path lookup

Generated artifacts are committed; regeneration is run by the PR that changes the catalog.

## 4. Runtime Handling

- Nodes that need semantic awareness (gateways, plotters, diagnostic tools, the State subsystem) carry a catalog and binding cache.
- Low-power sensors do not carry the catalog at all — they transmit raw DCs and rely on downstream binding-aware nodes for semantic resolution.
- Binding-cache requirements for Pelorus Core nodes are normative in [`../core/06-instance-binding.md`](../core/06-instance-binding.md).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
