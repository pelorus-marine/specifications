# Pelorus Stream — Events and Errors

Stream events, metadata/state/capability updates, and the application-layer and transport-layer error taxonomies. Control-message envelopes and CBOR rules are in [`05-control-protocol.md`](./05-control-protocol.md).

## 1. Events

### 1.1 What an Event Is

A timestamped, typed notification a publisher (or in some cases a subscriber) emits to inform observers of something noteworthy that happened to a stream. Events are **emitted, never authoritative**: a notification, not a command, not a state, not a fact-of-record.

| An event **is** | An event **isn't** |
| --- | --- |
| A timestamped, typed notification | A request to act |
| Best-effort delivered (over the QUIC reliable control stream once established) | Persisted by Stream itself |
| Idempotent at the receiver (de-dup by `seq`) | Repeatable for retry semantics |
| Consumed by State for decisions | Acted upon by Stream itself |

### 1.2 Event Body

Envelope `kind = 0x0010` per [`05-control-protocol.md §2`](./05-control-protocol.md). Body:

```cbor
{
  "name": "<event-name>",        ; required, see registry
  "level": "info"|"warn"|"err",  ; advisory severity, default "info"
  ? "details": {<event-specific>},
  ? "since": <epoch ms>           ; when the underlying condition began
}
```

### 1.3 Event Name Registry

| Name | Level | Emitted by | When |
| --- | --- | --- | --- |
| `opened` | info | Publisher | Session moves IDLE → ANNOUNCED |
| `activated` | info | Publisher | First subscriber attached, ANNOUNCED → ACTIVE |
| `deactivated` | info | Publisher | Last subscriber left, ACTIVE → IDLE-ATTACHED |
| `keepalive` | info | Publisher | No payloads to send; lifeline |
| `closing` | info | Publisher | Graceful close imminent |
| `subscriber-joined` | info | Publisher | A subscriber attached |
| `subscriber-left` | info | Publisher | A subscriber detached |
| `subscribers-quiet` | warn | Publisher | Lease lapsed without renewal, none active |
| `data-loss` | warn | Subscriber | Detected sequence gap not closed within buffer |
| `buffer-underrun` | warn | Subscriber | Output starved |
| `buffer-overrun` | warn | Subscriber | Output flooded; PUs dropped |
| `late-pu` | info | Subscriber | A PU arrived after its scheduled play time |
| `discontinuity` | info | Either | Discontinuity flag observed; buffer reset |
| `format-mismatch` | err | Subscriber | Received PU does not match negotiated format |
| `metadata-conflict` | err | Subscriber | Static metadata changed mid-session |
| `payload-too-large` | err | Subscriber | PU exceeded subscriber's MTU |
| `decode-error` | err | Subscriber | Codec or CBOR decode failed |
| `transport-stalled` | warn | Subscriber | No traffic for the configured stall window |
| `publisher-disappeared` | warn | Subscriber | Lease expired with no traffic |
| `clock-drift` | info | Subscriber | Drift compensation engaged |
| `sequence-reset` | warn | Subscriber | Source's datagram sequence reset (likely node reboot — see [`07-redundancy.md §4.2`](./07-redundancy.md)) |
| `fabric-degraded` | warn | Either | Dual-fabric session entered DEGRADED ([`07-redundancy.md §3`](./07-redundancy.md)) |
| `fabric-recovering` | info | Either | Dual-fabric session entered RECOVERING |
| `fabric-restored` | info | Either | Dual-fabric session returned to DUAL_ACTIVE |
| `vendor:<reverse-dns>:<name>` | * | Either | Vendor-defined |

Unknown event names shall be ignored by receivers (forward compat). Logging an unknown name is permitted but shall not propagate as an error.

### 1.4 Cadence

Events are emitted **once per occurrence**, not periodically. A publisher emitting the same event many times per second for the same condition is broken. Aggregation (one event per second carrying counts) is acceptable.

`keepalive` is the exception: every 5 s when there is no other control traffic.

### 1.5 Severity

- `info` — normal operation, routine.
- `warn` — transient, recoverable.
- `err` — non-recoverable for this PU/subscriber/session; resolution requires action.

Level is advisory. A State subsystem aggregator may re-classify based on stream context.

### 1.6 Persistence

Stream does not persist events. Subscribers, the registry ([`08-discovery-and-registry.md §11`](./08-discovery-and-registry.md)), or the Pelorus State subsystem may keep ring buffers; Stream itself emits and forgets. The reference library shall offer at least a 256-event ring buffer for diagnostic UIs ([`12-lib.md`](./12-lib.md)).

## 2. Updates

There are three update kinds:

| Kind | Carries |
| --- | --- |
| `0x0011` `state-update` | The state object from [`06-session-and-state.md §7`](./06-session-and-state.md) |
| `0x0012` `metadata-update` | Subset of mutable metadata fields from [`02-data-model.md §4`](./02-data-model.md) |
| `0x0013` `capability-update` | Capability bit-vector replacement |

### 2.1 `state-update` Body

Full snapshot or delta. See [`06-session-and-state.md §7`](./06-session-and-state.md).

### 2.2 `metadata-update` Body

Carries the new value of each mutable metadata field that has changed:

```cbor
{
  "id": h'<sid>',
  ? "name": "Bow Radar",
  ? "prio": 9,
  ? "tags": ["nav", "bow"],
  ? "extra": {<merged extras>}
}
```

Static fields (`type`, `pub`, `format`, `sr`, `ch`, `instance`, `class`, etc., per [`02-data-model.md §4.2`](./02-data-model.md)) shall not appear in `metadata-update`. A publisher that needs to change a static field shall close and re-open the session.

### 2.3 `capability-update` Body

Replaces the current capability bit-vector entirely:

```cbor
{ "id": h'<sid>', "caps": h'<bit-vector>' }
```

Subscribers negotiated against a capability newly **removed** shall continue with the negotiated cap as long as the publisher honours it; if the publisher stops honouring it, surface `format-mismatch` and unsubscribe. Capabilities **added** mid-session require re-subscription to negotiate.

### 2.4 Cadence and Coalescing

Updates are change-driven. A publisher shall not emit an unchanged update. When multiple changes arrive in a short window (~100 ms), publishers should coalesce them into a single update message rather than emitting one per change.

`state-update` has the periodic 30 s heartbeat described in [`06-session-and-state.md §7.2`](./06-session-and-state.md). Other update kinds have no heartbeat — fresh subscribers receive current values via the snapshot-on-subscribe flag.

### 2.5 Ordering

Within a single `(sender, sid)`, updates are ordered by envelope `seq`. Receivers de-duplicate and reorder by `seq`. Out-of-order arrival of a higher `seq` followed by a lower `seq` shall result in the lower being dropped (it represents an older view).

Across kinds (state vs metadata vs capability), there is no cross-kind ordering — receivers process each kind's updates independently.

## 3. Stream Errors (Application Layer)

### 3.1 Where Errors Surface

| Surface | When |
| --- | --- |
| Local `Result::Err` in the reference library | For the local application that initiated an operation |
| `error` control message (kind `0x00FE`) | Sent across the wire when the local error is relevant to a remote peer |
| Event of a specific name (§1.3) | When the error is a per-stream condition observable by all subscribers |

### 3.2 `error` Body

```cbor
{
  "code": "<error-code>",
  "level": "warn"|"err"|"fatal",
  ? "details": {<context-specific>},
  ? "ref_seq": <u32>            ; sequence number that caused the error
}
```

Severity:

- `warn` — recoverable; the offending operation will be retried or dropped
- `err` — non-recoverable for this operation/PU; the stream continues
- `fatal` — non-recoverable for this **session**; the sender will close

### 3.3 Stream Error Code Registry

Codes are short stable strings, kebab-case. Receivers shall ignore unknown codes (log only).

| Code | Level | Meaning | Source |
| --- | --- | --- | --- |
| `protocol-error` | err | Malformed envelope; required field missing | Either |
| `decode-error` | err | CBOR decode failure or codec decode failure | Receiver |
| `format-mismatch` | err | Received PU does not match negotiated format | Subscriber |
| `metadata-conflict` | err | Static metadata changed mid-session | Subscriber |
| `payload-too-large` | err | PU exceeded subscriber's configured maximum | Subscriber |
| `caps-incompatible` | warn | Subscribe negotiation failed; no usable caps | Publisher |
| `subscriber-cap-exhausted` | warn | Per-stream subscriber limit reached | Publisher |
| `not-active` | warn | Operation requested on an inactive stream | Either |
| `not-seekable` | warn | `seek` requested on a non-seekable stream | Publisher |
| `out-of-scope` | err | Control command targets a non-Stream entity | Publisher |
| `vendor-required` | warn | Vendor capability is required but not advertised | Publisher |
| `data-loss` | warn | Detected loss not closed within buffer | Subscriber |
| `clock-discipline` | warn | Severe clock drift detected | Either |
| `closing` | warn | The peer is shutting down | Either |
| `internal` | err | Unspecified internal error; details should describe | Either |

Codes prefixed with `vendor:<reverse-dns>:<name>` are vendor-defined and ignored unless the vendor capability is negotiated.

### 3.4 `out-of-scope` — Boundary Enforcement

The `out-of-scope` error is the wire-level enforcement of the boundary in [`01-overview.md §2`](./01-overview.md). A publisher receiving any control message that targets a Pelorus Core entity, attempts to actuate hardware outside Stream's purview, or requests Stream to influence Core, shall reject with `code=out-of-scope, level=err`. The message is **not** acted upon. Implementations shall log the offending message and the source identity so a misbehaving controller can be identified and fixed.

### 3.5 Recovery

| Code | Recovery |
| --- | --- |
| `protocol-error`, `decode-error`, `format-mismatch` | Drop the offending PU. Continue. |
| `metadata-conflict`, `payload-too-large` | Unsubscribe; re-subscribe if metadata reconverges |
| `caps-incompatible`, `subscriber-cap-exhausted` | Wait, retry after metadata update |
| `not-active`, `not-seekable` | Surface to controller; State decides next action |
| `out-of-scope` | Log, do not retry. Bug at the controller |
| `data-loss` | Apply concealment; continue |
| `clock-discipline` | Engage drift compensation |
| `closing` | Tear down; consider re-attaching to a successor stream when one appears |
| `internal` | Surface; next steps are publisher-specific |

## 4. Transport Errors

Transport errors arise from the wire substrate (QUIC, IPv6, sockets, mDNS) rather than from the meaning of stream operations. Most are handled by the OS or by the QUIC stack; this section specifies how they are surfaced to Stream-aware code.

### 4.1 Where They Surface

- As OS socket errors during send/receive
- As QUIC connection-level events (RFC 9000 §20)
- As Stream events (§1) when a transport condition is observable from the application's perspective

### 4.2 Transport Error Code Registry

| Code | Level | Meaning |
| --- | --- | --- |
| `transport-stalled` | warn | No QUIC traffic for the configured stall window |
| `mtu-exceeded` | err | Outbound datagram exceeds path MTU; fragmentation forbidden |
| `socket-bind` | fatal | Failed to bind UDP socket to required port |
| `quic-handshake-failed` | err | QUIC TLS handshake failed during connect |
| `quic-connection-closed` | warn | QUIC connection closed unexpectedly |
| `quic-stream-reset` | warn | QUIC stream-level reset |
| `link-down` | err | Underlying network interface link is down |
| `address-unreachable` | err | Subscriber address no longer reachable (ICMPv6 unreachable) |
| `route-disappeared` | err | Local routing table no longer has a route to the peer |
| `mdns-conflict` | warn | mDNS service-instance name conflict ([`08-discovery-and-registry.md §10`](./08-discovery-and-registry.md)) |
| `interface-bounce` | warn | A Stream interface flipped down/up; sessions reset |
| `internal` | fatal | Unspecified transport-layer internal error |

### 4.3 QUIC Errors

QUIC carries rich error information per RFC 9000 §20:

- **Handshake failure** → `quic-handshake-failed`. Common in v1.0 due to self-signed certificate validation policy mismatch.
- **Stream reset** → `quic-stream-reset`. Stream-level recovery is QUIC's job; the Pelorus Stream session may continue if other streams in the connection are healthy.
- **Connection close** → `quic-connection-closed`. The Stream session is unrecoverable on this connection; the subscriber must reconnect.

QUIC TLS error codes shall not be exposed verbatim in `details`; map to the abstract code above. Specific TLS alerts are debug-only.

### 4.4 MTU and Fragmentation

Pelorus Stream is single-PU-per-datagram in v1.0 ([`02-data-model.md §5.1`](./02-data-model.md)). Fragmentation is forbidden:

- Publishers shall set `IPV6_DONTFRAG` (or platform equivalent) where available.
- A datagram that would fragment is dropped, and `mtu-exceeded` is surfaced.

Path MTU discovery is not performed by Stream itself in v1.0. Onboard Ethernet has a stable 1500-byte MTU; the assumption is safe.

### 4.5 Recovery Patterns

| Error | Reference recovery |
| --- | --- |
| `transport-stalled` | Receiver re-checks subscription; sends fresh `subscribe` |
| `quic-connection-closed` | Subscriber re-discovers, re-connects (new QUIC connection) |
| `link-down` | Hold sessions; on link-up surface `interface-bounce`, then re-discover |
| `address-unreachable` | Drop the unreachable subscriber; expire its lease |
| `mtu-exceeded` | Publisher closes; operator adjusts encoder profile |

### 4.6 Sustained Failure

A node that observes the same transport error continuously for over **30 seconds** shall:

1. Reduce log cadence (one line per 30 s, not per occurrence).
2. Stop attempting recovery for the offending session.
3. Mark the affected subscription/stream as failed and let State decide.

Stream code shall not enter a tight retry loop.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
