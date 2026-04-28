# Pelorus Stream — Control Message

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **control message taxonomy**: the on-wire messages that publishers and subscribers exchange to manage sessions, subscriptions, events, and updates. Payload PUs (audio frames, telemetry samples) are **not** control messages and are framed differently ([`05-stream-payload.md`](./05-stream-payload.md), [`12-envelope.md`](./12-envelope.md)).

All control messages share the envelope from [`12-envelope.md`](./12-envelope.md) and are serialized per [`13-serialization.md`](./13-serialization.md).

---

## 1. Message Taxonomy

| Message | Direction | Purpose | Reference |
|---|---|---|---|
| `subscribe` | Subscriber → Publisher | Open or renew a subscription | [`24-subscription.md`](./24-subscription.md) |
| `unsubscribe` | Subscriber → Publisher | Voluntary teardown | [`24-subscription.md`](./24-subscription.md) |
| `subscribe-ack` | Publisher → Subscriber | Accept or reject a subscribe | [`24-subscription.md`](./24-subscription.md) |
| `keepalive` | Publisher → Subscriber | Indicates publisher is alive when no PUs flow | [`08-connection.md` §7](./08-connection.md) |
| `event` | Publisher → Subscriber | Stream-level event | [`19-stream-event.md`](./19-stream-event.md) |
| `state-update` | Publisher → Subscriber | Observable state change | [`20-stream-state.md`](./20-stream-state.md) |
| `metadata-update` | Publisher → Subscriber | Mutable metadata change | [`21-stream-update.md`](./21-stream-update.md) |
| `closing` | Publisher → Subscriber | Graceful session close | [`07-session.md` §6](./07-session.md) |
| `play` / `pause` / `seek` / `set-volume` | Sender → Target stream's publisher | Soft playback control | [`17-playback-control.md`](./17-playback-control.md) |
| `error` | Either direction | Out-of-band error notification | [`25-stream-error.md`](./25-stream-error.md) |

Messages not listed here are **reserved** and shall be ignored by receivers. Implementations shall not invent ad-hoc message kinds outside the vendor capability range described in [`14-versioning.md`](./14-versioning.md).

---

## 2. Common Required Fields

Every control message envelope carries:

| Field | Type | Notes |
|---|---|---|
| `v` | u8 | Protocol minor version. See [`14-versioning.md`](./14-versioning.md). |
| `kind` | u16 | Message kind code (table in §3). |
| `sid` | byte string (16 B) | Stream ID this message refers to. |
| `seq` | u32 | Per-(sender, sid) monotonic counter. |
| `ts` | u64 | Sender wall-clock millisecond timestamp (advisory). |
| `body` | map | Message-specific body. |

Receivers shall reject any message missing `v`, `kind`, or `sid` with a `protocol-error` ([`25-stream-error.md`](./25-stream-error.md)).

---

## 3. Kind Codes

| Code | Kind |
|---|---|
| `0x0001` | `subscribe` |
| `0x0002` | `unsubscribe` |
| `0x0003` | `subscribe-ack` |
| `0x0004` | `keepalive` |
| `0x0005` | `closing` |
| `0x0010` | `event` |
| `0x0011` | `state-update` |
| `0x0012` | `metadata-update` |
| `0x0020` | `play` |
| `0x0021` | `pause` |
| `0x0022` | `seek` |
| `0x0023` | `set-volume` |
| `0x00FE` | `error` |
| `0x0100`–`0x7FFF` | Reserved future Pelorus |
| `0x8000`–`0xFFFE` | Vendor (paired with `vendor` capability bit) |
| `0xFFFF` | Reserved sentinel |

Kind codes are stable. Once assigned, a kind code is never reused for a different meaning.

---

## 4. Idempotency

`subscribe`, `unsubscribe`, `play`, `pause`, `set-volume`, and `metadata-update` are **idempotent** at the protocol level. A publisher receiving a duplicate shall apply it as if it were the first.

`seek` is **not** idempotent (the same seek-target arriving twice may or may not be a no-op depending on intervening state changes).

`event` and `state-update` are **monotonic** — receivers de-duplicate by `seq`.

This matters because UDP loses and reorders packets. Idempotent senders simply retry on no-ack within a short window; non-idempotent senders need application-level reasoning.

---

## 5. Acknowledgement Policy

Most control messages are unacknowledged. The exceptions:

| Message | Ack required? |
|---|---|
| `subscribe` | Yes — `subscribe-ack` within 1 s, retry up to 3 times |
| `unsubscribe` | No — best effort |
| `play` / `pause` / `seek` / `set-volume` | No — observed via `state-update` if present |
| `closing` | No |
| Others | No |

A publisher that does not receive a `subscribe` for a multicast stream still serves multicast traffic; the acknowledgement is per the unicast control channel only.

---

## 6. Origin and Authentication

v1.0 control messages are unauthenticated. The receiver trusts the source address advertised in the envelope and observed at the IP layer.

This is acceptable in v1.0 because:

- Stream is non-safety-critical. State, not Stream, makes any decision based on a control message.
- Onboard LAN is treated as trusted (locked door, sailor-controlled hardware).
- Authentication adds a key-management problem that is itself out of scope for v1.0 ([`01-overview.md` §5](./01-overview.md)).

A v1.1 authenticated profile (COSE_Sign1 wrappers, pre-shared keys per vessel) is reserved.

---

## 7. Multi-recipient Messages

`event`, `state-update`, `metadata-update`, and `keepalive` may be sent to:

- Each known unicast subscriber (for unicast streams)
- The stream's multicast group (for multicast streams)
- Both (a publisher may multicast events even on a unicast payload stream)

The publisher chooses; subscribers shall accept either.

---

## 8. Open Items

- Whether to introduce a `request` / `response` pair distinct from `subscribe` / `subscribe-ack` for one-shot queries (currently no — keep the surface small).
- A signed/authenticated profile for v1.1.
- Whether to add a `flow-control` message kind for QUIC-mode reliable streams (currently QUIC handles flow control natively).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
