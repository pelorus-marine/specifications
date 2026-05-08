# Draft response — GitHub Issue #6 (Dual Bus Redundancy, Duplicate Discard, and Related Reliability Improvements)

**Status:** Draft. Not yet posted. Edit the wording to suit before publishing.  
**Last Updated:** May 4, 2026

---

## What this document is

This is a draft public response to **[Issue #6 — Dual Bus Redundancy, Duplicate Discard, and Related Reliability Improvements](https://github.com/pelorus-marine/specifications/issues/6)**. It maps every numbered section of the RFC onto the Pelorus Core spec sections that **absorb**, **refine**, or **reject** the proposal, so external readers can see exactly how the design landed.

The normative work lives in `specifications/core/02–17`. This file is informative and **not** part of the conformance set.

---

## Headline

Pelorus Core has adopted **path-redundant CAN FD** as a normative option, scoped by **criticality class (C0 / C1 / C2)** rather than mandated network-wide. Most of Issue #6 is incorporated, with three explicit changes from the issue's wording:

1. **Hybrid duplicate-discard model**, not a universal sequence-number byte. Pelorus-native broadcast DCIDs carry a **Pelorus Redundancy Header (PRH)**; J1939 / NMEA-2000 compatibility DCIDs use **payload-and-ID** dedup so byte layouts stay binary-compatible with the source standards.
2. **No length-vs-rate table** in v1.0. Pelorus Core stays at **250 kbit/s arbitration / 500 kbit/s data** with the **30 m / 6 m / 50-node** topology budget. Higher rates will be introduced as **named profiles** with their own complete topology + timing budget.
3. **Active-active without SYNC.** Pelorus does **not** define a SYNC channel between Bus A and Bus B; receivers reconcile via duplicate discard. This is also the design choice noted in the IP-landscape paragraph (see §13.7 below).

---

## Section-by-section mapping

References below use `core/NN-name.md §S` format and link to the corresponding repo path.

### 1. Need for path redundancy (RFC §1)

**Status:** Absorbed.

- Defined as a normative option in [`core/17-criticality-and-redundant-paths.md`](core/17-criticality-and-redundant-paths.md), `§§1–7`.
- Mandated for **C0** (safety-critical) and **C1** (mission-critical) zones, optional for **C2**.
- Reliability-and-durability principle pinned in [`core/01-overview.md §6`](core/01-overview.md) and `core/17 §0`.

### 2. Bit rate / segment length (RFC §2)

**Status:** Refined → effectively rejected for v1.0.

- See [`core/02-physical-layer.md §13.6`](core/02-physical-layer.md). v1.0 keeps the single LMDE-Micro-style profile (30 m backbone, 6 m stub, 50 nodes, 250 kbit/s arbitration / 500 kbit/s data).
- Future higher-rate profiles will be defined as **named profiles**, each with its own complete topology + timing budget, not via a generic length-vs-rate table.

### 3. Node classes (RFC §3 — Class S, Class D, Class H)

**Status:** Absorbed verbatim.

- Definitions in [`core/17-criticality-and-redundant-paths.md §4`](core/17-criticality-and-redundant-paths.md).
- Physical requirements in [`core/02-physical-layer.md §13`](core/02-physical-layer.md).
- Hub (Class H / RedBox-equivalent) behavior in [`core/10-repeater-specification.md §3`](core/10-repeater-specification.md).

### 4. Active-active replication and duplicate discard (RFC §4)

**Status:** Absorbed, refined.

- Active-active replication and the **Duplicate Discard Table (DDT)** algorithm are normative in [`core/03-data-link-layer.md §6`](core/03-data-link-layer.md), particularly §6.4.
- **Hybrid PRH model** (refinement): see §5 below.
- **Exemptions** (address claim, TP, WUF, NM) in [`core/03 §6.2`](core/03-data-link-layer.md).

### 5. Universal sequence-number byte in every frame (RFC §5)

**Status:** Refined / partially rejected.

- A universal sequence byte would break **bit-compatibility** with J1939 and NMEA-2000 layouts that Pelorus Core is committed to preserving in [`core/07 §2`](core/07-dcid-registry.md).
- Replaced with a **hybrid model**:
  - **Pelorus-native broadcast DCIDs** carry the **Pelorus Redundancy Header (PRH)** at bytes 0–2 — see [`core/03 §6.3`](core/03-data-link-layer.md). Future native DCIDs in `0x0FF84–0x0FFFF` with payload `>= 4 bytes` are required to use PRH.
  - **Compatibility DCIDs** are deduplicated by **payload-and-ID** key per [`core/03 §6.4.2`](core/03-data-link-layer.md). No layout change.

### 6. `DISCARD_WINDOW` and clock drift (RFC §6)

**Status:** Absorbed, formalized.

- `DISCARD_WINDOW` floor of **50 ms** in [`core/03 §6.4.1`](core/03-data-link-layer.md).
- Lower-bound formula `DISCARD_WINDOW >= 2 * H * L_hop + 2 * D_clk + safety_margin` in [`core/03 §6.4.3`](core/03-data-link-layer.md).
- Clock-drift bound `D_clk <= 10 ms` recommended for C0 zones in [`core/07 §1.4`](core/07-dcid-registry.md), realised via the optional **Time Sync DCID 0x0FF83**.
- Installation checklist line in [`core/14 §8`](core/14-installation-guide.md).

### 7. Bus Health and diagnostics (RFC §7)

**Status:** Absorbed.

- **Bus Health DCID 0x0FF82** normative in [`core/07 §1.3`](core/07-dcid-registry.md): TX/RX error counters, bus-off events, duplicate count, missed-frame count, node class, bus state (including `Degraded-Single`).
- Cadence: nominal 2 s, tolerance ± 500 ms.

### 8. Time synchronisation (RFC §8)

**Status:** Absorbed (optional).

- **Time Sync DCID 0x0FF83** optional, normative layout in [`core/07 §1.4`](core/07-dcid-registry.md).
- Stream-side time sync remains **IEEE 802.1AS** and is out of Core scope (cross-link in [`core/09 §8.2`](core/09-gateway-specification.md)).

### 9. Address claiming on dual buses (RFC §9)

**Status:** Absorbed.

- Dual-bus address claim procedures in [`core/05-addressing.md §7`](core/05-addressing.md). Class D and Class H must claim simultaneously on both buses; data transmission is gated until both claims succeed.

### 10. Power-management interaction (RFC §10)

**Status:** Absorbed.

- 4-bit **wake generation** counter, DDT invalidation on wake, in [`core/04 §13`](core/04-power-management.md) and [`core/03 §6.6`](core/03-data-link-layer.md).
- Bus-return rule: receivers accept the returning bus and apply normal dedup; no re-sync handshake in v1.0.

### 11. Failover behaviour (RFC §11)

**Status:** Absorbed.

- Failover convergence requirement in [`core/17 §3.1`](core/17-criticality-and-redundant-paths.md): under steady producer traffic on both buses, single-bus failure shall not cause an application-layer gap larger than `DISCARD_WINDOW + max(producer_period)`.
- Bus return without false duplicates in [`core/03 §6.6`](core/03-data-link-layer.md).
- Conformance tests **P-03-005**, **P-03-006**, **P-17-002** in [`core/15`](core/15-conformance-test-plan.md).

### 12. Hub (RedBox-equivalent) behaviour (RFC §12)

**Status:** Absorbed, extended.

- Class H specification in [`core/10 §3`](core/10-repeater-specification.md).
- **Bidirectional duplicate discard** (hub-internal, on backbone ingress before downstream forwarding) in [`core/10 §3.4`](core/10-repeater-specification.md).
- **Hub backbone bus-off and degraded backbone** behaviour in [`core/10 §3.5`](core/10-repeater-specification.md).
- Conformance tests **P-10-002**, **P-10-003** in [`core/15`](core/15-conformance-test-plan.md).

### 13. Stream-side redundancy (RFC §13, where applicable)

**Status:** Out of Core scope.

- Stream-layer redundancy belongs to `stream/`. Cross-link only: [`core/09 §8.2`](core/09-gateway-specification.md) recommends **IEEE 802.1CB** (FRER) for Stream and **IEEE 802.1AS** for time sync.
- **Stream → Core injection on a dual-bus target zone** is normative in [`core/09 §8.1`](core/09-gateway-specification.md).

### 14. Common-mode and IP considerations (RFC §14)

**Status:** Absorbed, with explicit notice.

- Physical / electrical common-mode mitigations in [`core/17 §5`](core/17-criticality-and-redundant-paths.md).
- Software / firmware common-mode logged as an **open item** for a future C0 / SOLAS-aligned profile in [`core/17 §5.1`](core/17-criticality-and-redundant-paths.md).
- IP notice (active-active dual CAN FD; references US 12,567,994 SYNC-based variant) added to [`core/02 §13.7`](core/02-physical-layer.md). Pelorus's design is **active-active without SYNC**.

### 15. Conformance and self-declaration (RFC §15)

**Status:** Absorbed.

- Test plan in [`core/15-conformance-test-plan.md`](core/15-conformance-test-plan.md): P-02 / P-03 / P-04 / P-05 / P-07 / P-10 / P-17 categories.
- Self-declaration template extended in [`core/16-compliance-self-declaration.md`](core/16-compliance-self-declaration.md) with `Path redundancy` claims (node class, criticality tier, critical zone map attachment, dual-bus tests executed).

---

## Items the RFC raised that we explicitly did **not** adopt

| RFC theme | Why not |
|---|---|
| Universal sequence-number byte in every CAN payload | Would break compatibility DCID layouts in [`core/07 §2`](core/07-dcid-registry.md). Replaced by hybrid PRH + payload-and-ID dedup. |
| Generic length-vs-rate table for higher CAN FD rates | Topology / signal-integrity at dual-rate CAN FD depends on stub count, stub length, transceiver class, and termination quality, not bit rate alone. v1.0 stays on a single profile; future rates will ship as **named profiles**. |
| Mandatory Class D for every node | Conflicts with the **C2** non-critical class. Mandate is per-zone (C0 / C1), not network-wide. See [`core/17 §2`](core/17-criticality-and-redundant-paths.md). |
| Stream layer 802.1CB normative text inside Core | Stream specs (`stream/`) own this; Core only carries cross-links. |

---

## Where to read first

- For the design rationale and zone model: [`core/17-criticality-and-redundant-paths.md`](core/17-criticality-and-redundant-paths.md).
- For the wire mechanics: [`core/03-data-link-layer.md §6`](core/03-data-link-layer.md) and [`core/07-dcid-registry.md §1.3 / §1.4`](core/07-dcid-registry.md).
- For installation: [`core/14-installation-guide.md §8`](core/14-installation-guide.md).
- For testing: [`core/15-conformance-test-plan.md`](core/15-conformance-test-plan.md).

---

*This draft is informative; if any wording here disagrees with the normative documents, the normative documents win.*
