# Review log — core documents 05–07

**Version:** Living  
**Last Updated:** May 4, 2026  
**Status:** Non-normative (working notes for adoption of **05**, **06**, **07**)  
**Trust:** N/A

---

## About This Document

This file records a **read-only review pass** over [`05-addressing.md`](./05-addressing.md), [`06-signal-catalog.md`](./06-signal-catalog.md), and [`07-dcid-registry.md`](./07-dcid-registry.md) against the trusted Tier‑1 core (**01–04**). It exists so later edits can be **surgical** (revalidate / rewrite / delete specific sections) rather than wholesale rewrites.

**Does not change** trust labels on **05–07** or normative text in those documents.

---

## Cross-document gaps (01–04 ↔ 05–07)

| Finding | Where observed | Notes |
|--------|----------------|-------|
| **NAME bit layout** promised in **07** | [`05-addressing.md`](./05-addressing.md) §2 — “Exact bit field allocations … will be defined in `07-dcid-registry.md`”; [`05-addressing.md`](./05-addressing.md) Open Items | **[**07**](./07-dcid-registry.md)** does not define J1939 NAME subfields or preferred SA ranges; addressing remains high-level J1939‑81 parity. Either add a **07** subsection + tables or soften **05** §2 / Open Items to point at **SAE J1939‑81** / external annex as normative for NAME layout. |
| **Commanded Address (0xFED8)** | [`05-addressing.md`](./05-addressing.md) §4 — required | Numeric assignment is not registered or described in **[**07**](./07-dcid-registry.md)**. Either register wire layout/priority in **07** §2 or §3 or mark **05** §4 as “behavior per J1939; register when Pelorus-side layout is frozen.” |
| **Compatibility / field layouts “normative in 07”** | [`01-overview.md`](./01-overview.md) §9 bullet “DCID registry (07)” | **[**07**](./07-dcid-registry.md)** §2 registers a **small** set of J1939 compatibility DCIDs and points to **SAE J1939 DA** for bit layouts. Align overview wording with “registered in **07**; layouts per DA where cited” if tension confuses implementers. |
| **Binding table DCID** | [`06-signal-catalog.md`](./06-signal-catalog.md) §4 — “published on the bus via a dedicated Pelorus DCID (**defined later** in **07**)” | No binding-table DCID is assigned in **[**07**](./07-dcid-registry.md)**. **[**07**](./07-dcid-registry.md)** §4 explicitly leaves binding distribution **out of band** for v1.0. Treat as **rewrite or reconcile**: either reserve/register a DCID + format or revise **06** §4 to match **07** §4 until a DCID exists. |
| **Functional groups in WUF** | [`04-power-management.md`](./04-power-management.md) §7.2 ↔ **[**07**](./07-dcid-registry.md)** §1 **WUF** summary table | Byte **0** functional-group bitmask is normative in **04** §6 / §7.2; **[**07**](./07-dcid-registry.md)** §1 summary matches (**04** §6). OK — **07** defers detail to **04** on conflict (**07** About). |
| **`dcid-rs` toolchain name** | [`06-signal-catalog.md`](./06-signal-catalog.md) §7 | Implementations may use other crate names (e.g. workspace **`pelorus-core`**). Low priority: align **06** §7 with **[11-reference-implementations.md](./11-reference-implementations.md)** when that doc is refreshed. |

---

## [`05-addressing.md`](./05-addressing.md)

### Cross-references vs 01–04

- **01 §9** — Addressing / NAME / J1939‑81 parity: **05** is consistent at a summary level.
- **03** — SA in 29‑bit ID, **0x0EA00** request, **0x0EE00** address claim: **05** §3 cites **0x0EE00**; **03** §4 reserved table aligns.
- **Stale / missing:** §2 and Open Items promise **NAME** detail and preferred SA ranges in **07** — not present in **07** yet (see table above). §4 **Commanded Address** — **0xFED8** not in **07** (see table).

### Section disposition (for a follow-up PR)

| Section | Suggested action | Rationale |
|---------|------------------|-----------|
| §2 Device Identification — NAME | **Rewrite** (when **07** or external cite is chosen) | Remove or qualify the promise that **07** will define bit fields until content exists or pointer is to J1939‑81. |
| §3 Address Claim | **Revalidate** | Matches J1939‑81 story; no conflict spotted vs **03**. |
| §4 Commanded Address | **Rewrite** or **Revalidate** after **07** registration | Required feature stated without registry entry. |
| §5 Power Management | **Revalidate** | Pointer to **04** for wake sequencing is consistent. |
| §6 Instance Binding | **Revalidate** | Correctly defers semantics to **06**; does not claim **07** holds binding format. |
| Open Items | **Revalidate** | Already admit preferred ranges / **07** — fold into checklist below. |

---

## [`06-signal-catalog.md`](./06-signal-catalog.md)

### Cross-references vs 01–04

- **01 §9** — VSS / `Vessel.*`: aligned.
- **07** — §5–§6: semantics vs wire vs binding roles are clear; **Issue [#3](https://github.com/pelorus-marine/specifications/issues/3)** tracks DCID evolution — still open.
- **Stale:** §4 binding publication via “dedicated DCID” vs **[**07**](./07-dcid-registry.md)** §4 out-of-band binding for v1.0 (see gaps table).
- **09** — §4 fault tolerance references gateway spec; consistent with “no single point of failure.”

### Section disposition

| Section | Suggested action | Rationale |
|---------|------------------|-----------|
| §3–§4 Instance / binding / fault tolerance | **Rewrite** (binding DCID vs out-of-band) | **06** §4 and **07** §4 need one story for v1.0. |
| §6 DCID and VSS | **Revalidate** | Coherent; depends on Issue **#3** outcome — track issue. |
| §7 Tooling | **Revalidate** (optional tidy of crate names) | Non-blocking. |
| §8 Open Items | **Revalidate** | Captures catalog + binding + Issue **#3** — mirrored below. |

---

## [`07-dcid-registry.md`](./07-dcid-registry.md)

### Cross-references vs 01–04

- **03** — DCID derivation §3.2, reserved ranges §4: **07** §3 references **03** §3.2 / §4 — OK.
- **04** — WUF **0x0FF80**, NM **0x0FF81**: **07** §1 defers layout to **04** §7 / §6 / §9 — OK; **07** About states **04** wins on conflict.
- **05** — Address claim **0x0EE00** appears in **05** / **03**; not duplicated as full **07** entry (acceptable if **03**/ **05** remain normative for TP/claim messages).
- **06** — §2 compatibility table names `Dcid` lanes; full signal extraction remains catalog/DBC work — consistent with **06**.
- **Gap:** **05** §4 **0xFED8** and **NAME** tables not registered here yet.

### Section disposition

| Section | Suggested action | Rationale |
|---------|------------------|-----------|
| §1 Pelorus-specific (WUF / NM) | **Revalidate** | Matches **04**; summary tables aligned. |
| §2 Compatibility DCIDs | **Revalidate** + **expand** when ready | Initial table is small; **Open Items** already say “expand.” |
| §3 Ranges and rules | **Revalidate** | Aligns with **03**. |
| §4 Binding distribution | **Revalidate** | Explicit v1.0 out-of-band — use to fix **06** §4 drift. |
| §5 Open Items | **Revalidate** | Mirrors global checklist. |

---

## Consolidated checklist (**07** §5 + **06** §8 + **05** Open Items)

Use this as a single backlog before promoting **05–07** toward higher trust.

**Registry & wire**

- [ ] Expand compatibility DCIDs beyond **[**07**](./07-dcid-registry.md)** §2 (propulsion, navigation, environment; NMEA2000/gateway profiles as needed).
- [ ] Register **Commanded Address** / **0xFED8** (and any Pelorus-specific constraints) or explicitly defer with pointer to J1939.
- [ ] Decide use of **WUF** / **NM** reserved bytes for extended masks, binding hints, or authority (**07** §5; **04** §7).
- [ ] Transmission rates / repetition rules per DCID (**NM** cadence ratified in **04** §9.1).
- [ ] Diagnostic DCID for error counters (**03** §6.5 references **07**).
- [ ] Conformance test fixtures (**07** §5; **15** when ready).

**Catalog & binding**

- [ ] Binding table: DCID format, publication cadence, authority/conflict rules (**06** §8; reconcile with **07** §4).
- [ ] Full binding schema, provisioning UI, drift recovery (**06** §8).
- [ ] Initial `Vessel.*` tree and signal set (**06** §8); integration with `catalog/vessel.vspec` (**07** §5).
- [ ] Custom Pelorus attributes + vss-tools overlay profile (**06** §8).
- [ ] Catalog versioning and backward compatibility (**06** §8).
- [ ] Optional: separate repo for catalog + binding tools (**06** §8).
- [ ] Instance binding design supported by real LMDE traffic capture (see **[00-document-index.md](./00-document-index.md)** priorities).

**NAME / addressing**

- [ ] Preferred SA ranges or device-class tables (**05** Open Items; **07** if placed there).
- [ ] Pelorus-specific NAME extensions (**05** — “none planned” — confirm and close).
- [ ] Multi-segment / repeater / gateway address spaces (**05** Open Items; **08** / **09**).

**Tracked GitHub issues (specifications repo)**

| Topic | Issue |
|-------|--------|
| DCID model evolution (versioning, namespaces, transport) | [#3 — Evolution of PGN-Based Systems Toward DCID](https://github.com/pelorus-marine/specifications/issues/3) |
| Machine-readable `dcid-contract` artifact | [#4](https://github.com/pelorus-marine/specifications/issues/4) |
| Dual-bus redundancy, duplicate discard, reliability RFC | [#6 — RFC: Dual Bus Redundancy, Duplicate Discard, and Related Reliability Improvements](https://github.com/pelorus-marine/specifications/issues/6) |

**Related (not specifications-repo issues)**

- **Instance binding** and capture prerequisites are coordinated via **[Pelorus Specifications — GitHub Issues](https://github.com/pelorus-marine/specifications/issues)** (no separate issue number assumed in this log).

---

## Suggested order of follow-up edits

1. **Reconcile 06 §4 with 07 §4** (binding distribution story) — smallest conflict.
2. **Register or qualify 05 §4 (0xFED8) and 05 §2 (NAME)** vs **07** and/or external J1939 cites.
3. **Expand 07 §2** compatibility table and rates/fixtures per Open Items.
4. Fold **Issue #6** (redundancy RFC) into **03** / **07** / **08** / **10** / **15** in a **separate** vertical slice after Core adoption advances.

---

*This review log is not part of the normative v1.0 document set; update it when **05–07** change materially.*
