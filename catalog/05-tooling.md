# Pelorus Catalog — Tooling

VSS sources, validation, code generation, and runtime catalog handling.

## 1. Source Files

- `catalog/vessel.vspec` — root VSS specification for the `Vessel.*` tree.
- `catalog/units.yaml` — Pelorus custom units extending the COVESA VSS standard units (currently only `knots`; see §2.2).
- `catalog/contracts/*.yaml` — machine-readable Data Contract entries cross-linked to catalog leaves (when this artifact lands).
- Overlay files for Pelorus-specific attributes (`data_contract`, `dc-field`, `instance-field`, `pelorus-priority`) per [`04-overlays.md`](./04-overlays.md).

Source files are CC BY 4.0 alongside the specification.

## 2. Validation

`vss-tools` validates the catalog with the Pelorus overlay profile. Validation passes are part of the conformance fixtures referenced in [`../core/11-conformance.md`](../core/11-conformance.md).

### 2.1 Overlay Attributes

Pelorus extends the standard VSS leaf-attribute set with four overlay attributes (per [`04-overlays.md`](./04-overlays.md)):

- `data_contract` — names the Pelorus DC that projects to this leaf on Pelorus Core.
- `dc-field` — names the field within the DC payload this leaf reads (omitted when the DC carries a single quantity).
- `instance-field` — names the DC-internal payload field carrying instance value (omitted for NAME-instanced DCs).
- `pelorus-priority` — Pelorus Core arbitration priority hint (0 highest, 7 lowest).

`vss-tools` rejects unknown attributes by default. Validation invocations therefore declare the Pelorus extended-attribute set explicitly.

### 2.2 Custom Units

Catalog leaves use the COVESA VSS standard units (`m`, `m/s`, `km/h`, `deg`, `deg/s`, `rpm`, `celsius`, `K`, `Pa`, `kPa`, …) wherever they cover the case. The catalog adds the following Pelorus-defined units:

| Unit | Quantity | Definition |
| --- | --- | --- |
| `knots` | speed | International nautical miles per hour (≈ 0.5144444 m/s). Marine convention for vessel and wind speeds. |

All custom units live in `catalog/units.yaml` and are loaded via `vss-tools`' `--unit-file` flag.

### 2.3 Validation Invocation

```bash
vspec export json \
  --vspec specifications/catalog/vessel.vspec \
  --unit-file specifications/catalog/units.yaml \
  --extended-attributes data_contract,dc-field,instance-field,pelorus-priority \
  --output build/vessel.json
```

The invocation above produces a JSON dump of the resolved tree and exits non-zero on any structural, attribute, or unit error. CI runs this on every PR that touches `catalog/`. Specific `vss-tools` versions and flag spellings are tracked in the Platform repository's CI configuration; this document specifies the contract (overlay attribute set + custom units file) rather than the exact CLI binding.

Alternative exporters (`vspec export csv`, `vspec export ddsidl`, `vspec export protobuf`) accept the same inputs and are used downstream for code generation per §3.

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
