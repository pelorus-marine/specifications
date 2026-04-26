# Pelorus Core — Signal Catalog Specification

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the Pelorus Signal Catalog — the canonical semantic data model for all Pelorus Core signals — using **COVESA VSS** under a **`Vessel.*`** root. It is protocol-agnostic and is the single source of truth for meaning, type, units, and instance binding. The locked catalog policy is summarized in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary); **normative** structure, overlays, and binding rules are defined in this document.

---

## 1. Catalog Structure

**Root:** `Vessel`

All signals live under `Vessel.*`. The tree is organized by marine functional areas (aligned with the power-management functional groups where applicable). The catalog is maintained as a standalone `Vessel.*` tree and is **not** contributed upstream to COVESA (v1.0 project policy).

- `Vessel.Propulsion` (engines, thrusters, sail drives)
- `Vessel.Navigation`
- `Vessel.Environment`
- `Vessel.Electrical`
- `Vessel.Tanks`
- `Vessel.Anchoring`
- `Vessel.Safety`
- `Vessel.Domestic`
- `Vessel.Network`

---

## 2. VSS Syntax Used

Pelorus uses standard VSS `.vspec` (YAML) format. Key elements:

- **Branches** (structural nodes)
- **Leaves** (actual signals) with mandatory attributes: `type`, `unit`, `description`, `min`/`max`/`enum`, etc.
- **Custom Pelorus attributes** (via vss-tools overlays): `pgn`, `instance-field`, `pelorus-priority`

---

## 3. Instance Handling and Binding (Critical)

**Instance Binding Problem** ([ARCHITECTURE.md](../ARCHITECTURE.md) §6.2) is solved here.

Pelorus uses **numeric indexed arrays** as the canonical form in the catalog:

- `Vessel.Propulsion.Engines[0].Speed`
- `Vessel.Propulsion.Engines[1].Speed`
- … (up to a practical maximum, e.g. `[0..15]`)

**Named branches (Port/Starboard)** are **not** used in the canonical catalog because they do not scale to boats with arbitrary numbers of identical devices.

**Binding table**  
The mapping `(Source Address + 64-bit NAME + PGN + PGN-internal instance field value) → VSS array index [n]` is stored in a **binding table**. Sailor-assigned friendly labels (“Port Main”, “Starboard”, “Wing Engine #3”, “Generator”) live as metadata on each entry.

---

## 4. Fault Tolerance — No Single Point of Failure

The binding table **must not** create a single point of failure (consistent with the rejection of a sole gateway authority in [ARCHITECTURE.md](../ARCHITECTURE.md) §5).

- The binding table is **published on the bus** via a dedicated Pelorus PGN (defined later in `07-pgn-registry.md`).
- Any authorized node (primary gateway, secondary display head, diagnostic tool, etc.) can act as binding authority and publish updates.
- Nodes that need semantics **cache the latest binding table in their own non-volatile memory**.
- The primary gateway provides the convenient web UI for editing/provisioning, but it is **not required** for continued operation.
- If the gateway is absent or failed:
  - Core raw PGN traffic continues unaffected.
  - Semantic consumers fall back to the last cached binding table (or to raw PGN + instance display if no cache exists).
  - New devices join and transmit data immediately (raw mode).
- On gateway return it can re-publish the authoritative table.

---

## 5. LMDE network compatibility

Pelorus signals are designed to carry the same semantic information that sailors already see on **LMDE** instrumentation networks.  

Where a Pelorus PGN transports data equivalent to fields commonly observed on LMDE buses, the correspondence is documented in `07-pgn-registry.md`. These mappings are derived from public observation of live networks and open-source reverse-engineering efforts (such as the canboat project). Proprietary or vendor-specific extensions are not carried forward.

This approach enables clean bridging via gateways while keeping Pelorus itself a fully independent, open standard.

---

## 6. Tooling and Implementation

- Catalog source: `catalog/vessel.vspec` (plus overlay files)
- Validation: `vss-tools` with Pelorus overlay profile
- Code generation: Rust structs, validation, and TypeScript definitions generated as part of `pgn-rs` and gateway crates
- Runtime: Only nodes that need semantics carry the binding cache; low-power sensors ignore it entirely

---

## 7. Open Items (to be resolved before v1.0 promotion)

- Exact tree structure and initial signal set (target: cover all common marine data observed on a representative liveaboard vessel)
- Binding table PGN format, publication cadence, and authority conflict rules
- Full binding table schema, provisioning UI, and conflict/drift recovery rules
- Custom attribute definitions and vss-tools overlay profile
- Catalog versioning and backward compatibility
- Whether to publish the catalog + binding tools as a separate repository (recommended)

---

## Example (excerpt from `vessel.vspec`)

```yaml
Vessel:
  type: branch
  description: Root of the Pelorus marine signal catalog

  Propulsion:
    type: branch
    description: Propulsion systems

    Engines:
      type: branch
      instances: [0,15]
      description: Individual propulsion engines (indexed by binding table)

      Speed:
        type: float
        unit: m/s
        description: Engine crankshaft rotational speed
        pgn: 0xF004
        instance-field: engine-instance