# Pelorus State — Specification Document Index

Authoritative list of Pelorus State specification documents. Pelorus State is the vessel's fused world-state subsystem layered above Pelorus Core and Pelorus Stream.

State is a one-directional pipeline: ingest → snapshot → situation → policy. See [`01-overview.md`](./01-overview.md) for what State is and is not.

## Documents

Entries written in inline code without a link are planned but not yet drafted.

| # | Filename | Purpose |
| --- | --- | --- |
| 00 | [`00-document-index.md`](./00-document-index.md) | This index |
| 01 | [`01-overview.md`](./01-overview.md) | What State is and is not; boundaries with Core and Stream; pipeline shape |
| 02 | [`02-system-model.md`](./02-system-model.md) | Entities and coordinate frames; static transforms |
| 03 | `03-event-ingestion-and-time.md` | Ingest Core and Stream events; normalize, order, time-align |
| 04 | `04-world-snapshot.md` | Fused numeric world state — kinematics, geometry, uncertainty; no labels |
| 05 | `05-situation-model.md` | Semantic overlay — identity resolution, correlated contacts, labels; no re-fusion |
| 06 | `06-policy-and-intents.md` | Rules over snapshot + situation → alerts, suppressions, intents; no I/O |
| 07 | `07-distribution-and-consistency.md` | Multi-node State: replication, eventual consistency, split-brain |
| 08 | `08-errors-and-degraded-mode.md` | Faults, stale data, missing publishers; per-stage degradation |
| 09 | `09-subsystem-interface.md` | Public inputs and outputs; version negotiation with Core and Stream consumers |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
