# Pelorus State — Specification Document Index (draft)

**Version:** Living draft  
**Last Updated:** April 27, 2026  
**Status:** Placeholder — see [**Issue #2 — Define Pelorus State subsystem**](https://github.com/pelorus-marine/specifications/issues/2) for the original task list. This index **replaces** that list with a **non-overlapping** document set: each file has a single, exclusive charter at the heading below.

**Trust:** Unverified (no `state/NN-*.md` bodies authored yet except this index).

**Related:** [`core/`](../core/00-document-index.md), [`stream/`](../stream/00-document-index.md), [`core/06-signal-catalog.md`](../core/06-signal-catalog.md) §6 (VSS / DCID roles).

---

## 1. Why this layout (no overlap)

The State subsystem is a **pipeline**. Data may only move **forward** through the stages below. Earlier stages **do not** assign “meaning” reserved for later stages, and later stages **do not** re-run sensor fusion reserved for earlier ones.

| Stage | Output (exclusive) |
|------|---------------------|
| Ingestion + time | Normalized **facts** with timestamps and source labels |
| World snapshot | **Numeric / geometric** fused state only (positions, rates, health bits) |
| Situation model | **Semantic overlay** on that snapshot (vessel names, track IDs, “this contact is that AIS target”) |
| Policy + intents | **Rule firings** and **behavioral intents** for executors (Core, Stream publishers, UI)—no I/O |

Documents are **merged** from the older Issue #2 sketch so that **reconstruction vs interpretation** and **policy vs decision** cannot blur across files.

---

## 2. Planned documents (`state/`)

| # | Filename | Exclusive charter (one sentence) |
|---|----------|-----------------------------------|
| 00 | [`00-document-index.md`](./00-document-index.md) | This map and trust posture. |
| 01 | `01-overview.md` | What State is/is not; boundaries vs Core and Stream; reading order. |
| 02 | `02-system-model.md` | **Entities and coordinate frames only** (vessel, sensors, contacts as *things*—no live values). |
| 03 | `03-event-ingestion-and-time.md` | **Ingest Core + Stream events**, normalize, order, **clock and skew policy** (including optional PTP/IEEE 1588 as a *vessel configuration*, not a Stream requirement). |
| 04 | `04-world-snapshot.md` | **Fused numeric world state** from facts only—kinematics, continuous fields, uncertainty as numbers—**no** narrative labels or COLREG roles. |
| 05 | `05-situation-model.md` | **All semantics on top of the snapshot**: identity resolution, correlated contacts, labels, confidence *as classification*—**no** re-fusion of raw sensor math. |
| 06 | `06-policy-and-intents.md` | **Rules** over snapshot + situation → **alerts, suppressions, behavioral intents**; deterministic, idempotent intent generation—**no** transport and **no** device I/O. |
| 07 | `07-distribution-and-consistency.md` | **Multi-node** State: what is replicated, eventual consistency, split-brain behavior, recovery—**no** sensor fusion (that is §4). |
| 08 | `08-errors-and-degraded-mode.md` | Faults, stale data, missing publishers—how each pipeline stage degrades. |
| 09 | `09-subsystem-interface.md` | **Public inputs/outputs**: event types State accepts; intent types State emits; version negotiation with Core/Stream consumers. |

**Renumbering note:** If GitHub Issue #2 is updated, point it at **this** table as authoritative rather than the original 12-file list.

---

## 3. Clock / synchronization (plain-language)

**Marine fusion** (radar + AIS + own-ship GNSS, etc.) only works if events can be aligned in time. **Pelorus Stream** may carry timestamps per datagram, but **does not** own vessel-wide time ([`stream/19-stream-event.md`](../stream/19-stream-event.md) §7). **`state/03`** will define: which clocks are trusted, how to reconcile Stream `ts`/`tts` with Core timestamps, and whether IEEE 1588 (PTP) is **recommended** on the Stream Ethernet plant for serious fusion—without making PTP mandatory for v1.0 Stream.

---

## 4. Relationship to DCID and VSS

State consumes **semantic** paths from **`Vessel.*`** and **wire contracts** from **Core DCIDs** as *inputs*, but does not redefine them. DCID evolution is tracked in [**Issue #3**](https://github.com/pelorus-marine/specifications/issues/3); see [`core/06-signal-catalog.md`](../core/06-signal-catalog.md) §6.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
