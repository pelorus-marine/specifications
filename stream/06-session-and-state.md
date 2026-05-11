# Pelorus Stream — Session and State

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

Session lifecycle, observable per-stream state, and subscription protocol. Control-message envelopes for the messages defined here are in [`05-control-protocol.md`](./05-control-protocol.md).

## 1. Session States

Every stream session moves through this state machine on the publisher side:

```
   ┌──────────┐    open      ┌──────────┐
   │   IDLE   │──────────────▶ ANNOUNCED │
   └──────────┘              └────┬─────┘
                                  │ first subscriber attaches
                                  ▼
                            ┌──────────┐
                            │  ACTIVE  │ ◀────┐
                            └────┬─────┘      │ play
                                 │            │
                                 │ pause      │
                                 ▼            │
                          ┌──────────────┐    │
                          │ ACTIVE-      │────┘
                          │ PAUSED       │
                          └──────────────┘
                                 │
                            (last subscriber detaches OR
                             publisher policy holds open)
                                 ▼
                            ┌──────────────┐
                            │ IDLE-        │ optional, configurable lifetime
                            │ ATTACHED     │
                            └────┬─────────┘
                                 │ close()  /  lease expiry
                                 ▼
                            ┌──────────┐
                            │  CLOSED  │ (terminal)
                            └──────────┘
```

| State | Meaning |
|---|---|
| `IDLE` | Pre-announce. Stream object exists locally but is not on the wire. |
| `ANNOUNCED` | Discoverable; mDNS record live; no payload traffic yet. |
| `ACTIVE` | Emitting payloads; ≥ 1 subscriber attached. |
| `ACTIVE-PAUSED` | Media-source state: session live, output suppressed by `pause` command. |
| `IDLE-ATTACHED` | No active subscribers, but session held open per publisher policy. |
| `CLOSED` | Terminal. Stream ID retired. |

`IDLE` is invisible — never published. The wire-visible states begin at `ANNOUNCED`.

## 2. Open

A publisher opens a session by:

1. Minting a fresh UUIDv7 Stream ID.
2. Constructing the metadata record ([`02-data-model.md §4`](./02-data-model.md)).
3. Allocating local transport resources (QUIC endpoint).
4. Publishing an mDNS-SD announcement ([`08-discovery-and-registry.md`](./08-discovery-and-registry.md)) with the metadata in its TXT record.

After step 4 the session is in **ANNOUNCED**. The stream is visible in the registry but no payload is being transmitted. A publisher shall not transmit payload while ANNOUNCED.

## 3. Subscriber Attach

A subscriber attaches by opening a QUIC connection to the publisher's link-local address (one connection per fabric for Class D — see [`04-transport.md §3`](./04-transport.md)) and sending a `subscribe` message on a reliable stream. On accepting, the publisher:

- Adds the subscriber to its dispatch set.
- Transitions to `ACTIVE` if this is the first subscriber.
- Begins or continues payload emission on the QUIC connection.

The first payload after a transition into `ACTIVE` may be a key/anchor PU defined per service.

## 4. Active

In `ACTIVE`, the publisher emits payloads at the cadence appropriate to the service. The session remains `ACTIVE` while at least one subscriber is attached or the publisher's policy keeps the stream alive without subscribers (default: drop to `IDLE-ATTACHED` after the last subscriber leaves).

For high-fan-out services (radar video to multiple ECDIS displays), the publisher publishes once to a **Replication Node** ([`10-services-nav.md`](./10-services-nav.md)) and the Replication Node fans out to display subscribers; from the original publisher's perspective there is only one downstream subscriber.

## 5. Lease and Renewal

A subscription is leased. Default lease is **60 seconds**. A subscriber renews by re-sending `subscribe` before lease expiry. Recommended renewal at **2/3 of lease** to absorb network jitter.

A publisher with no live subscriptions and no other policy reason to remain open transitions to `IDLE-ATTACHED`, then `CLOSED` after a publisher-defined idle window (default **5 minutes**).

## 6. Subscribe / Subscribe-Ack / Unsubscribe

### 6.1 `subscribe` (kind `0x0001`)

```cbor
{
  ? "lease": <ms, default 60000>,
  ? "caps": h'<bit vector>',
  ? "extra": {<future use>}
}
```

The envelope's `sid` names the target stream; the body does not need to repeat it. Subscriber identity comes from the QUIC connection — there is no separate `from` address field.

### 6.2 `subscribe-ack` (kind `0x0003`)

```cbor
{
  "result": "ok"|"rejected"|"already-subscribed",
  ? "lease": <granted ms>,
  ? "caps": h'<negotiated bit vector>',
  ? "reason": "<text when rejected>"
}
```

Reject reasons:

| Reason | Meaning |
|---|---|
| `subscriber-cap-exhausted` | Publisher's per-stream subscriber limit reached |
| `caps-incompatible` | No usable intersection of capabilities |
| `not-active` | Stream is in a state that does not accept subscriptions |
| `out-of-scope` | Subscriber lacks something policy-required (v1.0: reserved) |

A subscriber receiving `rejected` shall not retry within 5 seconds and shall log the reason.

### 6.3 `unsubscribe` (kind `0x0002`)

Empty body. Effect: publisher removes the subscription entry immediately. No `unsubscribe-ack` required.

A subscriber that exits without sending `unsubscribe` simply lets its lease expire. This is acceptable (it costs the publisher up to one lease period of stale dispatch).

### 6.4 Subscriber Identity

The "subscriber" in the publisher's table is identified by the QUIC connection. A given physical host may have multiple subscribers to the same stream from different connections (multi-process app); they are tracked independently.

### 6.5 No Wildcard Subscriptions in v1.0

A subscriber that wants every stream of a given service type subscribes one-by-one, browsing mDNS as new streams appear.

## 7. Per-Stream State Object

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
  ? "extra": {<service-specific>}
}
```

This is the canonical state representation, published in two ways:

- **Snapshot:** in any `state-update` message ([`11-events-and-errors.md`](./11-events-and-errors.md)) with the full map.
- **Delta:** `state-update` with only changed keys (omission means unchanged).

### 7.1 Push and Pull

State is **pushable** — the publisher emits `state-update` on transitions.

State is **pullable** — a subscriber requests a fresh snapshot via `subscribe` with `extra: {snap: true}`. On accept, the publisher emits a one-shot `state-update` with the full snapshot, then transitions to delta mode.

### 7.2 Cadence

Change-driven, with two exceptions:

- Periodic `state-update` heartbeat once every **30 s** for `ACTIVE` streams, carrying the snapshot — so a late-joining observer eventually catches up.
- Immediate `state-update` on `subscribers` count change (≤ 1 Hz throttle to avoid storms on rapid join/leave).

Counts (`pus_emitted`, `bytes_emitted`) are emitted only on the periodic heartbeat. They are advisory diagnostic data.

### 7.3 Eventual Consistency

A subscriber may observe stale state during a network glitch. The periodic heartbeat eventually reconverges all observers; convergence is bounded by the heartbeat period plus one round-trip. A subscriber disconnected longer than the heartbeat period should request a fresh snapshot via `extra: {snap: true}`.

### 7.4 State and the Registry

The registry ([`08-discovery-and-registry.md`](./08-discovery-and-registry.md)) caches the most recent state observation. It does not become authoritative — authority always remains with the publisher.

## 8. Close

A publisher closes the session by:

1. Sending a `closing` event to all attached subscribers.
2. Withdrawing the mDNS announcement (TTL=0).
3. Releasing transport resources.

The Stream ID is now retired and shall not be reused.

A publisher that exits without sending `closing` is permitted; subscribers detect this through QUIC connection close and the loss of mDNS announcements. Subscribers shall surface `publisher-disappeared` ([`11-events-and-errors.md`](./11-events-and-errors.md)) only after the lease has expired with no renewal traffic.

## 9. Crash and Recovery

If a publisher crashes and restarts, the new process shall mint a new Stream ID — never recover the old one. Subscribers see the old stream age out of the registry and a new stream appear. The State subsystem, not Stream, decides whether to auto-attach to the replacement.

If the network partitions, sessions on either side continue independently. Reconverge is handled by the registry and is best-effort.

## 10. Multiple Sessions per Publisher

A publisher may host an arbitrary number of concurrent sessions, each with its own Stream ID and state machine. The publisher's `pub` field (in metadata) is shared across all sessions; the Stream IDs differentiate them. There is no aggregation primitive in v1.0.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
