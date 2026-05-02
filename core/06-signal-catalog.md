# Pelorus Core — Signal Catalog Specification

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the Pelorus Signal Catalog — the canonical semantic data model for all Pelorus Core signals — using **COVESA VSS** under a **`Vessel.*`** root. It is protocol-agnostic and is the single source of truth for meaning, type, units, and instance binding. The catalog policy is summarized in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary); **normative** structure, overlays, and binding rules are defined in this document.

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
- **Custom Pelorus attributes** (via vss-tools overlays): `dcid`, `instance-field`, `pelorus-priority`

---

## 3. Instance Handling and Binding (Critical)

**Instance Binding Problem** — LMDE traces, binding-table authority, and gateway coordination are tracked on [Pelorus Specifications — GitHub Issues](https://github.com/pelorus-marine/specifications/issues); catalog rules normative below resolve the wire vs semantics story.

Pelorus uses **numeric indexed arrays** as the canonical form in the catalog:

- `Vessel.Propulsion.Engines[0].Speed`
- `Vessel.Propulsion.Engines[1].Speed`
- … (up to a practical maximum, e.g. `[0..15]`)

**Named branches (Port/Starboard)** are **not** used in the canonical catalog because they do not scale to boats with arbitrary numbers of identical devices.

**Binding table**  
The mapping `(Source Address + 64-bit NAME + DCID + DCID-internal instance field value) → VSS array index [n]` is stored in a **binding table**. Sailor-assigned friendly labels (“Port Main”, “Starboard”, “Wing Engine #3”, “Generator”) live as metadata on each entry.

---

## 4. Fault Tolerance — No Single Point of Failure

The binding table **must not** create a single point of failure (consistent with [09-gateway-specification.md](./09-gateway-specification.md): gateways are not mandatory sole authorities; multiple gateways are supported).

- The binding table is **published on the bus** via a dedicated Pelorus DCID (defined later in `07-dcid-registry.md`).
- Any authorized node (primary gateway, secondary display head, diagnostic tool, etc.) can act as binding authority and publish updates.
- Nodes that need semantics **cache the latest binding table in their own non-volatile memory**.
- The primary gateway provides the convenient web UI for editing/provisioning, but it is **not required** for continued operation.
- If the gateway is absent or failed:
  - Core raw DCID traffic continues unaffected.
  - Semantic consumers fall back to the last cached binding table (or to raw DCID + instance display if no cache exists).
  - New devices join and transmit data immediately (raw mode).
- On gateway return it can re-publish the authoritative table.

---

## 5. LMDE network compatibility

Pelorus signals are designed to carry the same semantic information that sailors already see on **LMDE** instrumentation networks.

Those semantics are today exchanged on **Classical CAN (CAN 2.0)** segments; **Pelorus Core** carries equivalent catalog meaning in **CAN FD** frames where mapped. **Gateways** perform frame translation; the signal catalog and binding table describe meaning independent of which side produced the frame.

Where a Pelorus DCID transports data equivalent to fields commonly observed on LMDE buses, the correspondence is documented in `07-dcid-registry.md`. These mappings are derived from public observation of live networks and open-source reverse-engineering efforts (such as the canboat project). Proprietary or vendor-specific extensions are not carried forward.

This approach enables clean bridging via gateways while keeping Pelorus itself a fully independent, open standard.

---

## 6. DCID and VSS — roles and evolution

**Purpose.** Implementers need one clear story for *meaning* vs *wire contracts*. Today the catalog uses **VSS** for semantics and an overlay attribute **`dcid`** for the Core wire identifier. **[Issue #3 — DCID exploration](https://github.com/pelorus-marine/specifications/issues/3)** tracks an evolved **Data Contract ID** model (versioning, namespaces, transport independence). Until that work converges with **`core/07-dcid-registry.md`**, treat the following as the **documentation contract**:

| Layer | Responsibility | Authoritative in |
|---|---|---|
| **Semantics** | Units, types, valid range, human meaning, relationships between signals | This document — **`Vessel.*`** tree, COVESA VSS syntax |
| **Core wire contract** | Which CAN FD (Pelorus) or Classical CAN (LMDE) message layout carries bits for a signal | **`core/07-dcid-registry.md`**, compatibility tables, gateway behavior |
| **Instance binding** | Which physical device / bus instance maps to `Vessel.*[n]` | **Binding table** (published on Core; see §3–4) |
| **Pelorus Stream session** | High-bandwidth or media *sessions* (UUIDv7 stream ID), not CAN frames | **`stream/`** — metadata may **reference** `Vessel.*` paths or future DCID forms; no merge of identifier spaces without an explicit cross-spec |

**Rules of the road**

1. **VSS does not define the CAN bitpacking.** The catalog points to a DCID (or future contract ID); bitfields live in the DCID registry / LMDE references.
2. **DCID does not define nautical meaning.** Two signals with the same rough name in different namespaces must still resolve to distinct `Vessel.*` leaves or explicit aliases documented here.
3. **Stream telemetry** that mirrors Core quantities should carry the optional metadata key **`vss`** (full `Vessel.*` path) per [`stream/06-stream-metadata.md`](../stream/06-stream-metadata.md) §1 until Issue #3 defines a single cross-transport contract ID usable on both CAN and IP.
4. When Issue #3 **supersedes** the current flat DCID model, update **`07-dcid-registry.md`**, this section, and **`stream/01-overview.md` §3.3** in one coordinated edit so gateways, compilers, and tooling do not fork.

---

## 7. Tooling and Implementation

- Catalog source: `catalog/vessel.vspec` (plus overlay files)
- Validation: `vss-tools` with Pelorus overlay profile
- Code generation: Rust structs, validation, and TypeScript definitions generated as part of `dcid-rs` and gateway crates
- Runtime: Only nodes that need semantics carry the binding cache; low-power sensors ignore it entirely

---

## 8. Open Items (to be resolved before v1.0 promotion)

- Exact tree structure and initial signal set (target: cover all common marine data observed on a representative liveaboard vessel)
- Binding table DCID format, publication cadence, and authority conflict rules
- Full binding table schema, provisioning UI, and conflict/drift recovery rules
- Custom attribute definitions and vss-tools overlay profile
- Catalog versioning and backward compatibility
- Whether to publish the catalog + binding tools as a separate repository (recommended)
- Align §6 with the outcome of [Issue #3](https://github.com/pelorus-marine/specifications/issues/3) (DCID header, versioning, TLV extensions) and norm Stream metadata linkage to `Vessel.*`

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
        dcid: 0xF004
        instance-field: engine-instance