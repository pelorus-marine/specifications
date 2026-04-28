# Pelorus Stream — Stream Update

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **stream-update** message: the wire mechanism that publishes mutable changes — to metadata, to capability bits, and to per-stream state ([`20-stream-state.md`](./20-stream-state.md)) — without closing and re-announcing the stream.

Stream-update is the *how* for the *what* defined in 06 (mutable metadata) and 20 (state).

---

## 1. Update Kinds

There are three update kinds, each with its own envelope `kind` code:

| Kind code | Name | Carries |
|---|---|---|
| `0x0011` | `state-update` | The state object from [`20-stream-state.md`](./20-stream-state.md). |
| `0x0012` | `metadata-update` | Subset of mutable metadata fields from [`06-stream-metadata.md`](./06-stream-metadata.md). |
| `0x0013` | `capability-update` | Capability bit-vector replacement. |

Each is a normal control envelope ([`12-envelope.md`](./12-envelope.md)) with a body specific to the kind.

---

## 2. `state-update` Body

Full snapshot:

```cbor
{
  "id": h'<sid>',
  "state": "active",
  "since": 1714200000000,
  "subscribers": 3,
  "pus_emitted": 184500,
  "bytes_emitted": 14760000,
  "last_event": "subscriber-joined"
}
```

Delta (each key optional; absent means "unchanged"):

```cbor
{
  "id": h'<sid>',
  "subscribers": 4
}
```

Receivers that have never seen a snapshot for this `id` shall request one via the snapshot subscribe flag ([`20-stream-state.md` §3](./20-stream-state.md)). Receivers shall not reject deltas referring to streams they have not yet snapshotted; they may simply ignore or queue.

---

## 3. `metadata-update` Body

Carries the new value of each mutable metadata field that has changed:

```cbor
{
  "id": h'<sid>',
  ? "name": "Saloon Speakers",
  ? "prio": 9,
  ? "tags": ["entertainment", "saloon"],
  ? "extra": {<merged extras>}
}
```

Static fields (`type`, `pub`, `format`, `sr`, `ch`, etc., per [`06-stream-metadata.md` §3](./06-stream-metadata.md)) **shall not** appear in `metadata-update`. A publisher that needs to change a static field shall close and re-open the session.

Receivers shall apply the update to their cached metadata and propagate to UI.

---

## 4. `capability-update` Body

Replaces the current capability bit-vector entirely:

```cbor
{
  "id": h'<sid>',
  "caps": h'<bit-vector>'
}
```

Capability changes during a session are rare (typically signaling that a publisher has spun up an additional codec or audio profile). Subscribers that are negotiated against a capability newly **removed** shall:

1. Continue with the negotiated cap as long as the publisher continues to honor it.
2. If the publisher stops honoring it, surface `format-mismatch` and unsubscribe.

Capabilities **added** mid-session are advertised but require re-subscription to negotiate; the existing subscription is unaffected.

---

## 5. Cadence and Coalescing

Updates are **change-driven**. A publisher shall not emit an unchanged update.

When multiple changes arrive in a short window (~100 ms), publishers should **coalesce** them into a single update message rather than emitting one per change. The receiver of a coalesced update applies all keys atomically.

State-update has the periodic 30 s heartbeat described in [`20-stream-state.md` §4](./20-stream-state.md). Metadata-update and capability-update have no heartbeat — fresh subscribers receive current values via snapshot on subscribe.

---

## 6. Targeting

Updates are routed:

- **Unicast streams:** to each known unicast subscriber on port `5354`.
- **Multicast streams:** to the multicast group on port `5354`.
- **Always also** to the registry, if a registry node is subscribed (the registry is just a subscriber).

A publisher that has only multicast subscribers does not need to enumerate them; group delivery suffices.

---

## 7. Reliability Considerations

Updates are best-effort UDP. Loss is recovered through:

- Periodic state-update heartbeat (every 30 s).
- Subscriber-driven snapshot request on observed inconsistency.
- Registry's eventually-consistent view ([`22-stream-registry.md`](./22-stream-registry.md)).

The control plane **shall not** retransmit lost update messages directly. Retransmit logic invites complexity that is poorly matched to the best-effort wire model.

For QUIC-mode streams, updates carried over the QUIC control stream are reliable for free. Implementations may push update messages over the QUIC control stream when one exists; the wire format is identical.

---

## 8. Ordering

Within a single sender + sid, updates are ordered by their envelope `seq`. Receivers de-dup and reorder by `seq`. Out-of-order arrival of a higher `seq` followed by a lower `seq` shall result in the lower `seq` being **dropped** (it represents an older view).

Across kinds (state vs. metadata vs. capability), there is no cross-kind ordering. Receivers process each kind's updates independently.

---

## 9. Vendor Extensions

A vendor may emit additional updates carrying vendor-specific fields. Vendors shall:

- Use a vendor-bit capability so receivers can opt in.
- Place vendor data inside the `extra` map, not at the top level.
- Document the schema of their vendor data.

Stream itself does not validate vendor `extra` content.

---

## 10. Open Items

- Whether to coalesce state and metadata into a single "update" envelope (currently separate kinds for clarity).
- Whether to expose update sequence numbers to user code via the reference library (currently internal).
- Backpressure: how a busy registry handles a flood of updates from many publishers (currently no protocol-level rate limit; reference library may local-rate-limit).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
