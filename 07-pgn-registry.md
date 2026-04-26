# Pelorus Core — PGN Registry

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the Pelorus PGN (Parameter Group Number) registry — wire-level encoding on the Pelorus Core CAN FD bus. It is the transport counterpart to [06-signal-catalog.md](./06-signal-catalog.md). Stack-level decisions (J1939-style identifiers, no Fast Packet, Pelorus extension range) are summarized in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary); **bit-level** field definitions and PGN assignments are **normative** here and in [05-addressing.md](./05-addressing.md) where applicable.

---

## 1. Pelorus-Specific PGNs

These PGNs are defined exclusively for Pelorus and ratify the candidates from `04-power-management.md`.

### PGN 0x0FF80 — Wake-Up Frame (WUF)
- **Priority:** 0 (highest)
- **Type:** Single
- **Length:** 8 bytes
- **Transmission:** Broadcast on selective wake events
- **Purpose:** Triggers partial-network wake-up per ISO 11898-2:2016

**Fields:**

| Order | Name                  | Bit Length | Resolution | Unit | Signed | Description |
|-------|-----------------------|------------|------------|------|--------|-------------|
| 1     | PNC Mask              | 64         | 1          | -    | No     | 64-bit bitmask of Power Network Clusters to wake |

### PGN 0x0FF81 — Network Management (NM)
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

## 2. Compatibility PGNs

Pelorus re-uses selected PGN numbers from the **legacy marine data ecosystem** to enable seamless interoperability with existing legacy marine instrumentation via gateways.  

This allows Pelorus to serve as a modern, open replacement for outdated closed systems while preserving compatibility during the transition period.

Detailed bit-level field layouts for these compatibility PGNs are defined by the relevant legacy marine standards and are not duplicated in this document.

The mapping from each PGN/field to the corresponding `Vessel.*` path in the signal catalog is maintained in `06-signal-catalog.md` and the machine-readable `catalog/vessel.vspec` file.

---

## 3. PGN Ranges and Assignment Rules

- **0x00000–0x0FF7F**: Standard marine PGNs (used for compatibility as described above)
- **0x0FF80–0x0FFFF**: Pelorus extensions (defined in this document)
- **0x10000+**: Reserved for future manufacturer-specific or Pelorus v2+ extensions

Assignment authority: Pelorus PGNs are allocated in this registry. Future additions require a pull request that updates this document and the corresponding entries in the signal catalog.

---

## 4. Relationship to Signal Catalog & Binding

- Every PGN field that carries an instance value is resolved to a `Vessel.*` path via the binding table (see `06-signal-catalog.md` §3–4).
- The binding table itself is broadcast as part of PGN 0x0FF81 updates.
- Low-power sensors only transmit raw PGNs; semantic mapping is handled by any binding-aware node.

---

## 5. Open Items (to be resolved before v1.0 promotion)

- Exact list of compatibility PGNs required for v1.0
- Full field-level definitions for Pelorus extensions beyond the two defined above
- Transmission rates and repetition rules for each PGN
- Conformance test fixtures
- Integration with the machine-readable `catalog/vessel.vspec` file

---

*This registry, together with documents 01–06, completes the minimum viable specification.*