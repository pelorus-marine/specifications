# Pelorus Stream — Stream State

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines **Stream State**: the per-stream observable status that a publisher exposes for **Pelorus State subsystem** consumption. **Stream State** means *the current condition of one specific media/telemetry session*. It is **not** the same thing as the **Pelorus State subsystem** (the vessel-wide behavioral layer). This document uses *Stream State* for the former only.

Per-stream state is **observed**, not authoritative. Stream State is what *is* for that stream, not what *should be* vessel-wide. The **Pelorus State subsystem** reads Stream State (and other inputs) and decides what *should be*.

---

## 1. The Per-Stream State Machine

```
   ┌──────────┐   ┌──────────────┐   ┌──────────┐   ┌──────────────┐   ┌──────────┐
   │   IDLE   ├──▶│   ANNOUNCED  ├──▶│  ACTIVE  ├──▶│ IDLE-ATTACHED├──▶│  CLOSED  │
   └──────────┘   └──────────────┘   └─────┬────┘   └──────────────┘   └──────────┘
                                           │
                                           ▼
                                     ┌──────────────┐
                                     │ ACTIVE-PAUSED│  (media only, via `pause`)
                                     └──────┬───────┘
                                            │ play
                                            └──────▶ ACTIVE
```

Only the publisher transitions between these states. Subscribers observe the publisher-reported state via `state-update` messages ([`21-stream-update.md`](./21-stream-update.md)).

| State | Meaning |
|---|---|
| `IDLE` | Pre-announce. The stream object exists but is not on the wire. Visible only locally. |
| `ANNOUNCED` | Discoverable; mDNS record live; no PU traffic yet. |
| `ACTIVE` | Emitting PUs; ≥ 1 subscriber attached. |
| `ACTIVE-PAUSED` | Media-source state: session live, output suppressed by `pause` command. |
| `IDLE-ATTACHED` | No active subscribers, but session held open per publisher policy. |
| `CLOSED` | Terminal. No further PUs. Stream ID retired. |

`IDLE` is invisible — it is the state immediately before the first announcement and is never published. The wire-visible states begin at `ANNOUNCED`.

---

## 2. State Object

Per-stream state is exposed as a CBOR map:

```cbor
{
  "id": h'<sid>',
  "state": "announced"|"active"|"active-paused"|"idle-attached"|"closed",
  "since": <epoch ms when this state was entered>,
  ? "subscribers": <count>,
  ? "pus_emitted": <u64>,
  ? "bytes_emitted": <u64>,
  ? "last_event": "<name>",
  ? "extra": {<type-specific>}
}
```

This is the canonical state representation. It is published in two places:

- **Snapshot:** in any `state-update` message ([`21-stream-update.md`](./21-stream-update.md)) with the full map.
- **Delta:** in `state-update` with only changed keys (per-key omission interpreted as "unchanged").

A State subsystem subscriber reconciles the two by maintaining a local copy.

---

## 3. Pull and Push

State is **pushable**: the publisher emits `state-update` on its own cadence, primarily on transitions.

State is also **pullable**: a subscriber can subscribe to the stream's "state" via the standard `subscribe` mechanism with a metadata flag (`extra: {snap: true}`). On accept, the publisher emits a one-shot `state-update` with the full snapshot, then transitions to delta mode.

Multicast streams broadcast state on the same multicast group (port `5354`), making per-stream state visible to all observers without a unicast probe.

---

## 4. Cadence

State updates are **change-driven**, not periodic, with two exceptions:

- A periodic `state-update` heartbeat once every 30 s for ACTIVE streams, carrying the snapshot (so a late-joining observer eventually catches up).
- An immediate state-update on `subscribers` count change (≤ 1 Hz throttle to avoid storms on rapid join/leave).

Counts (`pus_emitted`, `bytes_emitted`) are emitted only on the periodic heartbeat, not on every transition. They are advisory diagnostic data.

---

## 5. Stream State vs. Stream Event

| Stream Event | Stream State |
|---|---|
| Discrete moment | Continuous condition |
| "Something happened" | "What is currently true" |
| `kind = 0x0010` | `kind = 0x0011` |
| Best read as a notification | Best read as a value |
| Emitted once per occurrence | Emitted on change (with periodic heartbeat) |

Both serve State; they answer different questions. A typical State subsystem node subscribes to both.

---

## 6. Stream State and Pelorus Core State

Stream State is **not** Pelorus Core state. Stream State describes *one media stream*; Core state describes vessel-level conditions (engine RPM, depth, position).

A common confusion: "stream is paused" and "engine is on" are completely different state spaces. Stream State concerns only Stream-subsystem entities. The State *layer* aggregates both.

A node implementing Stream **shall not** publish or claim Core state via Stream channels. Doing so would violate the boundary in [`01-overview.md` §2](./01-overview.md).

---

## 7. Eventual Consistency

A subscriber may observe stale state during a network glitch. State updates are best-effort (UDP); the periodic heartbeat eventually reconverges all observers. Convergence guarantee is **bounded by the heartbeat period plus one round-trip**.

A subscriber that has been disconnected for longer than the heartbeat period may have missed transitions. On reconnect, it should request a fresh snapshot via the `extra: {snap: true}` subscribe flag.

---

## 8. State and the Registry

The registry ([`22-stream-registry.md`](./22-stream-registry.md)) caches the most recent state observation for each stream. It does **not** become authoritative. Authority always remains with the publisher.

A registry node observing a stream in `ACTIVE` for 30 minutes with no new updates and no payload traffic shall not change its cached state — but it may emit a `transport-stalled` event of its own; State decides what to do with it.

---

## 9. Open Items

- Whether `ACTIVE-PAUSED` should be a top-level state or a flag on `ACTIVE` (currently a top-level state for clarity).
- Cadence tuning for the 30 s heartbeat — likely too frequent for low-event streams, too infrequent for fast UIs. v1.1 may make it negotiable.
- Per-stream "cumulative loss rate" exposure — currently in `extra`, may promote to top-level.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
