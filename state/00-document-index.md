# Pelorus State — Specification Document Index

**Version:** Living
**Last Updated:** May 10, 2026
**Trust:** Trusted (index only; individual docs carry their own trust levels)

Authoritative list of Pelorus State specification documents. Pelorus State is the vessel's fused world-state subsystem layered above Pelorus Core and Pelorus Stream.

State is a one-directional pipeline: ingest → snapshot → situation → policy. See [`01-overview.md`](./01-overview.md) for what State is and is not.

## Trust Levels

- **Trusted** — written deliberately against external sources; cited content verified.
- **Unverified** — provisional draft; not validated.
- **Final** — frozen.

## Documents

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 00 | [`00-document-index.md`](./00-document-index.md) | This index | Living | Trusted |
| 01 | [`01-overview.md`](./01-overview.md) | What State is and is not; boundaries with Core and Stream; pipeline shape | Draft | Unverified |
| 02 | [`02-system-model.md`](./02-system-model.md) | Entities and coordinate frames; static transforms | Draft | Unverified |
| 03 | `03-event-ingestion-and-time.md` | Ingest Core and Stream events; normalize, order, time-align | Planned | — |
| 04 | `04-world-snapshot.md` | Fused numeric world state — kinematics, geometry, uncertainty; no labels | Planned | — |
| 05 | `05-situation-model.md` | Semantic overlay — identity resolution, correlated contacts, labels; no re-fusion | Planned | — |
| 06 | `06-policy-and-intents.md` | Rules over snapshot + situation → alerts, suppressions, intents; no I/O | Planned | — |
| 07 | `07-distribution-and-consistency.md` | Multi-node State: replication, eventual consistency, split-brain | Planned | — |
| 08 | `08-errors-and-degraded-mode.md` | Faults, stale data, missing publishers; per-stage degradation | Planned | — |
| 09 | `09-subsystem-interface.md` | Public inputs and outputs; version negotiation with Core and Stream consumers | Planned | — |

## Numbering

Numbers are assigned at document creation and not reused.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
