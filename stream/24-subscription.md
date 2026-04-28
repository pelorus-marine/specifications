# Pelorus Stream — Subscription

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **subscription protocol**: how a subscriber attaches to a publisher, how leases are renewed, and how subscriptions are torn down. Subscription messages are the most-exercised control-plane messages in the system.

---

## 1. Subscription Model

A **subscription** is a publisher-side bookkeeping entry that says "this subscriber is currently listening to this stream and wants traffic." It is created by a `subscribe` message ([`11-message.md` §1](./11-message.md)) and renewed periodically.

A subscription:

- Has a **lease** (default 60 seconds) after which it expires unless renewed.
- Implies that the publisher should keep the stream ACTIVE (it is at least one of the reasons to do so).
- For unicast streams, identifies a destination for PUs.
- For multicast streams, is informational — multicast traffic flows whether or not subscriptions exist.

A subscriber may have many simultaneous subscriptions to many publishers. There is no global subscription table.

---

## 2. `subscribe` Message

`kind = 0x0001`. Body:

```cbor
{
  ? "lease": <ms, default 60000>,
  "from": "fe80::dead:beef:cafe:1234",
  ? "port": <u16, default 5355>,
  ? "caps": h'<bit vector>',
  ? "extra": {<future use>}
}
```

| Key | Required | Notes |
|---|---|---|
| `lease` | No | Requested lease in ms. Publisher may grant less. |
| `from` | Yes | Subscriber's link-local IPv6 address; the publisher will deliver unicast PUs to it. |
| `port` | No | Subscriber's payload port; default `5355`. |
| `caps` | No | Subscriber's capability bits. Negotiation per [`14-versioning.md`](./14-versioning.md). |
| `extra` | No | Reserved for future fields; receivers ignore unknown keys. |

The `subscribe` envelope's `sid` names the target stream; the body does not need to repeat it.

---

## 3. `subscribe-ack` Message

`kind = 0x0003`. Body:

```cbor
{
  "result": "ok"|"rejected"|"already-subscribed",
  ? "lease": <granted ms>,
  ? "caps": h'<negotiated bit vector>',
  ? "reason": "<text — when rejected>"
}
```

The publisher emits `subscribe-ack` on receipt of `subscribe`. For multicast streams, ack still goes unicast back to the subscriber's address.

If the publisher rejects, `reason` should be a short identifier:

| Reason | Meaning |
|---|---|
| `subscriber-cap-exhausted` | Publisher's per-stream subscriber limit reached. |
| `caps-incompatible` | No usable intersection of capabilities. |
| `not-active` | Stream is in a state that does not accept subscriptions (rare). |
| `out-of-scope` | The subscriber lacks something policy-required. v1.0 has no policy; reserved. |

A subscriber receiving `rejected` shall not retry within 5 seconds and shall log the reason. After 5 seconds it may retry once with revised capabilities, then give up until the publisher's metadata changes.

---

## 4. Lease Renewal

A subscriber renews by re-sending `subscribe` before the granted lease expires. Recommended renewal at **2/3 of lease** to absorb network jitter.

A renewed `subscribe` is treated by the publisher as a no-op for an existing subscription, except that the lease timer resets. The `subscribe-ack` returns `already-subscribed` with the new effective expiry.

A publisher with no live (non-expired) subscriptions on a unicast stream after the configured idle window ([`07-session.md` §5](./07-session.md)) closes the stream.

---

## 5. `unsubscribe` Message

`kind = 0x0002`. Body:

```cbor
{
  ? "from": "fe80::dead:beef:cafe:1234"  ; optional, defaults to source address
}
```

Effect: publisher removes the subscription entry immediately. No `unsubscribe-ack` is required.

A subscriber that exits without sending `unsubscribe` simply lets its lease expire. This is acceptable (it costs the publisher up to one lease period of stale dispatch).

---

## 6. Subscriber Identity

The "subscriber" in the publisher's table is identified by the tuple:

```
(source-ipv6, source-port, sid)
```

A given physical host may have multiple subscribers to the same stream from different ports — e.g. a multi-process app — and they are tracked independently.

There is **no** subscriber-side identity beyond the network address. v1.0 trusts the source address as observed at the IP layer.

---

## 7. Multicast Subscription

For multicast streams (`mode=m`), subscription messages are still useful: they tell the publisher how many subscribers exist, allowing the publisher to:

- Maintain the ACTIVE state instead of falling to IDLE-ATTACHED.
- Surface `subscriber-joined` / `subscriber-left` events ([`19-stream-event.md`](./19-stream-event.md)).
- Decide whether to keep encoding (some publishers stop transmitting if no subscribers).

A subscriber that joins a multicast group but does **not** send `subscribe` will still receive PUs, but is invisible to the publisher and contributes no lease pressure. This is a normal, supported case (passive listeners).

---

## 8. Bulk and Wildcard Subscriptions

There is **no wildcard subscription** in v1.0. A subscriber that wants every audio stream subscribes one-by-one, browsing mDNS as new streams appear.

A future v1.1 may add a "match-by-type-and-tag" wildcard. v1.0 keeps the contract simple.

---

## 9. Cross-Cap Negotiation Worked Example

Publisher caps: `0b 0000 0011 0010 0011` (mode-unicast, mode-multicast-ssm, audio-opus-48k-mono, audio-opus-48k-stereo, playback-control)

Subscriber caps: `0b 0000 0001 0010 0011` (mode-unicast, audio-opus-48k-mono, audio-opus-48k-stereo, playback-control)

Subscribe carries the subscriber's caps. Publisher's `subscribe-ack` carries the **bitwise AND**: `0b 0000 0001 0010 0011`.

Both sides operate strictly under the AND. The subscriber knows this stream will be unicast (no multicast-ssm in intersection, even though publisher supports it).

---

## 10. Open Items

- Whether to surface subscriber-side reason for `unsubscribe` (currently fire-and-forget).
- Behavior on publisher restart with stale subscriptions (currently subscriber's lease expires, subscriber re-discovers via mDNS).
- A v1.1 "subscribe-many" message to attach to N streams in one request — currently per-stream.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
