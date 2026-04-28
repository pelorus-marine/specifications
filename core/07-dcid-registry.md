# Pelorus Core — DCID Registry

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the Pelorus DCID (Data Contract ID) registry — wire-level encoding on the Pelorus Core CAN FD bus. It is the transport counterpart to [06-signal-catalog.md](./06-signal-catalog.md). Stack-level decisions (J1939-style identifiers, no Fast Packet, Pelorus extension range) are summarized in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary); **bit-level** field definitions and DCID assignments are **normative** here and in [05-addressing.md](./05-addressing.md) where applicable.

---

## 1. Pelorus-Specific DCIDs

These DCIDs are defined exclusively for Pelorus and ratify the candidates from `04-power-management.md`.

### DCID 0x0FF80 — Wake-Up Frame (WUF)
- **Priority:** 0 (highest)
- **Type:** Single
- **Length:** 8 bytes
- **Transmission:** Broadcast on selective wake events
- **Purpose:** Triggers partial-network wake-up per ISO 11898-2:2016

**Fields:**

| Order | Name                  | Bit Length | Resolution | Unit | Signed | Description |
|-------|-----------------------|------------|------------|------|--------|-------------|
| 1     | PNC Mask              | 64         | 1          | -    | No     | 64-bit bitmask of Power Network Clusters to wake |

### DCID 0x0FF81 — Network Management (NM)
- **Priority:** 6
- **Type:** Single (200 ms cadence when active)
- **Length:** 8 bytes
- **Purpose:** Power state, functional group status, and binding table authority announcements

**Fields:** (detailed in `04-power-management.md` §9; summary here)

| Order | Name                  | Bit Length | Description |
|-------|-----------------------|------------|-------------|
| 1     | Source PNC            | 6          | Functional group of transmitter |
| 2     | Power State           | 2          | Active / Standby / Sleep / Deep Sleep |
| 3     | Binding Table Version | 16         | Monotonic version for cache invalidation |
| 4     | Authority Priority    | 8          | 0 = primary gateway, higher = secondary |
| 5     | Reserved              | 32         | - |

---

## 2. Compatibility DCIDs

Pelorus reuses selected DCID numbers from the **Legacy Marine Data Ecosystem** to enable seamless interoperability with existing LMDE instrumentation via gateways.

**Wire encoding:** On **LMDE**, those messages appear in **Classical CAN (CAN 2.0)** frames (8-byte data field per frame unless combined with LMDE multi-frame rules). On **Pelorus Core**, the **same numeric DCID values and field layouts** (where compatibility is claimed) are carried in **CAN FD** frames per **03**. This document registers Pelorus-side use; authoritative bit layouts for legacy messages remain in LMDE family standards.

The mapping from each DCID/field to the corresponding `Vessel.*` path in the signal catalog is maintained in `06-signal-catalog.md` and the machine-readable `catalog/vessel.vspec` file.

---

## 3. DCID Ranges and Assignment Rules

- **0x00000–0x0FF7F**: Standard marine DCIDs (used for compatibility as described above)
- **0x0FF80–0x0FFFF**: Pelorus extensions (defined in this document)
- **0x10000+**: Reserved for future manufacturer-specific or Pelorus v2+ extensions

Assignment authority: Pelorus DCIDs are allocated in this registry. Future additions require a pull request that updates this document and the corresponding entries in the signal catalog.

---

## 4. Relationship to Signal Catalog & Binding

- Every DCID field that carries an instance value is resolved to a `Vessel.*` path via the binding table (see `06-signal-catalog.md` §3–4).
- The binding table itself is broadcast as part of DCID 0x0FF81 updates.
- Low-power sensors only transmit raw DCIDs; semantic mapping is handled by any binding-aware node.

---

## 5. Open Items (to be resolved before v1.0 promotion)

- Exact list of compatibility DCIDs required for v1.0
- Full field-level definitions for Pelorus extensions beyond the two defined above
- Transmission rates and repetition rules for each DCID
- Conformance test fixtures
- Integration with the machine-readable `catalog/vessel.vspec` file

---

*This registry, together with documents 01–06, completes the minimum viable specification.*