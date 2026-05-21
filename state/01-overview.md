# Pelorus State — Overview

Entry point to the Pelorus State specification. Normative requirements live in [`02-system-model.md`](./02-system-model.md) onward.

## 1. What Pelorus State Is

The vessel's fused world-state subsystem, layered above Pelorus Core and Pelorus Stream. State subscribes to facts emitted by Core and Stream, produces a coherent vessel snapshot, lays a semantic overlay on top, and runs deterministic policy that emits intents back to executors.

Concretely, State provides:

- Time-aligned fact ingestion from Core and Stream events ([`03-event-ingestion-and-time.md`](./03-event-ingestion-and-time.md))
- A numeric world snapshot — kinematics, geometry, uncertainty — with no semantic labels ([`04-world-snapshot.md`](./04-world-snapshot.md))
- A situation model that overlays identity on the snapshot — vessel names, track IDs, "this radar contact is that AIS target" ([`05-situation-model.md`](./05-situation-model.md))
- Policy and intents — rules over snapshot + situation producing alerts, suppressions, behavioural intents — no I/O ([`06-policy-and-intents.md`](./06-policy-and-intents.md))
- Optional multi-node distribution with eventual consistency ([`07-distribution-and-consistency.md`](./07-distribution-and-consistency.md))

## 2. Three Things Called "State"

The word "state" is overloaded across Pelorus. They are not the same:

| Term | What it is | Where defined |
|---|---|---|
| **Pelorus State subsystem** | This subsystem — the vessel-wide fused world model | `state/` (this directory) |
| **Stream session state** | The lifecycle of one Stream session (`ANNOUNCED → ACTIVE → CLOSED`) | [`stream/06-session-and-state.md §1`](../stream/06-session-and-state.md) |
| **Stream per-stream state object** | The CBOR map `{id, state, since, subscribers, …}` emitted by a publisher in `state-update` messages | [`stream/06-session-and-state.md §7`](../stream/06-session-and-state.md) |

State (the subsystem) **subscribes to** the second and third. It is not either of them. A publisher reporting "this stream is ACTIVE with 3 subscribers" is one of many inputs State consumes; State's own outputs are about the vessel, not transport sessions.

## 3. Boundary with Core and Stream

State **imports** Core and Stream APIs. Core and Stream do not import State.

- A failed State subsystem shall leave Core and Stream fully functional. Each device retains local autonomy when State is unreachable.
- State does not actuate hardware. It emits intents; **executors** translate intents into concrete Core or Stream operations.
- State does not redefine signal semantics. It consumes `Vessel.*` paths from Core's signal catalog ([`core/06-signal-catalog.md`](../core/06-signal-catalog.md)) and Stream events as-is.
- The pipeline is one-directional. Earlier stages do not reach into later ones; later stages do not re-run fusion already performed.

## 4. Pipeline

```
Core events  ─┐
              ├─▶ ingest ─▶ snapshot ─▶ situation ─▶ policy ─▶ intents
Stream events ┘                                                  │
                                                                 ▼
                                              executors (Core, Stream, UI)
```

Data flows left to right only. The interface that publishes intents to executors is in [`09-subsystem-interface.md`](./09-subsystem-interface.md).

## 5. Design Principles

- **One-directional pipeline.** Ingest produces facts; snapshot fuses numbers; situation labels them; policy decides.
- **Numbers before names.** The snapshot is geometric (position, velocity, uncertainty as numbers). Names, track IDs, and COLREG roles live in the situation model.
- **Deterministic intents.** Policy outputs are idempotent and replayable: the same snapshot + situation produces the same intents.
- **No I/O in policy.** Intents are data, not wire commands. Executors handle transport.
- **Open all the way down.** Specification, reference implementation, test fixtures.

## 6. Status

v0.1 is pre-release. The pipeline shape is fixed; per-stage formats are draft and subject to revision until at least one end-to-end reference (ingest → snapshot → situation → policy) passes the smoke tests.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
