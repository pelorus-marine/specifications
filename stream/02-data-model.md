# Pelorus Stream — Data Model

The wire data model: the identifier that names a stream, the metadata that describes it, the type and priority hints that classify it, and the payload-unit framing that carries its data. Wire transport is in [`04-transport.md`](./04-transport.md). Per-service payload formats are in [`10-services-nav.md`](./10-services-nav.md).

## 1. Stream Identifier

A Stream ID is a **128-bit UUIDv7** per draft-ietf-uuidrev-rfc4122bis.

UUIDv7 is chosen because it is time-sortable (leading 48 bits are Unix-epoch milliseconds), globally unique without coordination, privacy-preserving (no MAC address), and stable in size.

### 1.1 Layout

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       unix_ts_ms (48 bits)                    |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|  ver  |        rand_a         |  var  |    rand_b (62 bits)   |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          rand_b (cont.)                       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          rand_b (cont.)                       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Value |
| --- | --- | --- |
| `unix_ts_ms` | 48 bits | Unix-epoch milliseconds at stream creation, big-endian |
| `ver` | 4 bits | `0x7` |
| `rand_a` | 12 bits | Cryptographically-random or implementation-monotonic |
| `var` | 2 bits | `0b10` (RFC 4122 variant) |
| `rand_b` | 62 bits | Cryptographically-random |

### 1.2 Lifetime

- **Mint once.** A publisher mints a fresh UUIDv7 every time it opens a new stream session ([`06-session-and-state.md`](./06-session-and-state.md)).
- **Never reuse.** A Stream ID shall not be reused for any subsequent stream, even by the same publisher.
- **Survive transport reconnect.** If a stream's underlying QUIC connection drops and is re-established within the session lifetime, the Stream ID does not change. (This is what QUIC connection migration is for; see [`07-redundancy.md`](./07-redundancy.md).)
- **Die with the stream.** Once the stream closes (graceful close, publisher exit, lease expiry), the Stream ID is retired.
- **Persist in logs.** Subscribers and registries may retain Stream IDs for diagnostics indefinitely. This does not constitute "active" use.

A Stream ID **shall not** carry node identity, manufacturer ID, MAC address, or device serial number. Identity belongs in the `pub` metadata field (§4).

### 1.3 Textual Presentation

Canonical form is the standard 8-4-4-4-12 hex string with hyphens, lowercase: `018f3c2b-9a4d-7c80-b1e2-4f5d6a7b8c9d`. UI may truncate to the first 8 hex chars for display.

In CBOR ([`05-control-protocol.md`](./05-control-protocol.md)), Stream IDs are encoded as a 16-byte byte string with tag **37** (RFC 9581 — Tag for UUID).

---

## 2. Stream Type

Type is an 8-bit unsigned integer that classifies what a stream carries at a coarse level. It tells a subscriber whether it can decode a stream at all; concrete formats are in the per-service documents.

| Code | Name | v1.0 status | Description |
| --- | --- | --- | --- |
| `0x00` | `reserved` | Reserved | Never assigned; receivers shall reject. |
| `0x01` | `radar-video` | Specified ([`10`](./10-services-nav.md)) | Raw radar spoke datagrams. |
| `0x02` | `telemetry` | Specified | Periodic numeric or structured telemetry. |
| `0x03` | `file` | Specified ([`10`](./10-services-nav.md)) | Bulk file transfer (S-100 chart distribution and similar) over HTTP/3. |
| `0x04` | `control` | Specified | Soft control plane (radar control, etc.). |
| `0x05` | `nav` | Specified ([`10`](./10-services-nav.md)) | High-rate position/heading/attitude. |
| `0x06` | `health` | Specified ([`10`](./10-services-nav.md)) | Stream node health. |
| `0x07`–`0x7F` | `reserved-future` | Reserved | Reserved for future Pelorus assignment. |
| `0x80`–`0xEF` | `vendor` | Vendor | Vendor-specific; receivers shall ignore unless they advertise the same vendor capability. |
| `0xF0`–`0xFE` | `reserved-experimental` | Reserved | Local experimentation; never published outside a development bench. |
| `0xFF` | `reserved-sentinel` | Reserved | Reserved sentinel; receivers shall reject. |

A stream's type is fixed for the lifetime of the stream. To switch class, close the existing stream and open a new one with a new Stream ID.

**Forward compatibility.** Receivers that do not recognise a type shall ignore the announcement. They shall not log it as an error and shall not propagate it. This rule lets v1.0 receivers safely coexist with v1.1+ publishers.

**Discovery vs type.** Subscribers normally find streams via the mDNS service catalog ([`08-discovery-and-registry.md`](./08-discovery-and-registry.md)), which uses concrete service names like `_pelorus-radar-video._quic.local`. Type is the coarser classification carried inside metadata; the service name is the addressable entry point.

---

## 3. Stream Priority

Priority is a 4-bit unsigned integer (0–15) carried in metadata. It is a **hint**, not authority. It does not preempt anything, does not bypass the State subsystem, and has no relationship to Pelorus Core arbitration priority.

| Range | Class | Typical use |
| --- | --- | --- |
| 0–3 | Bulk | Chart file transfer, telemetry that can wait |
| 4–7 | Standard | Default for telemetry, stream health |
| 8–11 | Interactive | Radar video, high-rate nav |
| 12–15 | Advisory-urgent | Reserved — *advisory* not *authoritative* |

Default for an unspecified stream is 7. Publishers wanting anything else publish it explicitly.

### 3.1 DSCP Mapping

Implementations may set DSCP code points on outbound QUIC datagrams per RFC 4594:

| Stream priority | DSCP class | DSCP value |
| --- | --- | --- |
| 0–3 | CS1 / Lower-effort | `001000` (8) |
| 4–7 | Default Forwarding | `000000` (0) |
| 8–11 | EF — Expedited Forwarding | `101110` (46) |
| 12–15 | EF — Expedited Forwarding | `101110` (46) |

Switches in the vessel network are not required to honour DSCP. Implementations shall not assume DSCP enforcement; Stream still works at best-effort without it.

### 3.2 Local Scheduling

A node with multiple outbound Stream packets queued shall schedule them in priority order, ties broken by FIFO. Schedulers shall not starve lower-priority streams indefinitely; weighted fair-queueing is sufficient and recommended.

Cross-stream coordination is a State decision, not a Stream priority decision. Stream priority cannot reach across streams.

---

## 4. Stream Metadata

Metadata is a CBOR map ([`05-control-protocol.md`](./05-control-protocol.md)) describing a stream. It is published in mDNS TXT records and updated via stream-update messages ([`11-events-and-errors.md`](./11-events-and-errors.md)).

| Key | CBOR major | Type | Required | Notes |
| --- | --- | --- | --- | --- |
| `id` | byte string | UUIDv7 | Yes | Stream ID. §1. |
| `type` | uint | u8 | Yes | Type code. §2. |
| `prio` | uint | u8 (0–15) | No | Default 7 if absent. §3. |
| `pub` | text | string ≤ 64 | Yes | Publisher node identifier. See §4.1. |
| `name` | text or map | string ≤ 64, or BCP-47→string map | No | Human-readable name. |
| `format` | uint | u16 | Type-dependent | Format code per type's service document. |
| `profile` | uint | u8 | No | Profile selector inside the format. |
| `cad` | uint | u32 | Telemetry | Nominal cadence in ms; 0 = irregular. |
| `instance` | uint | u16 | Multi-instance services | Antenna index for multi-instance services (radar). |
| `class` | uint | u8 | Yes | Node class: 0 = Class S (single-fabric), 1 = Class D (dual-fabric). See [`07-redundancy.md`](./07-redundancy.md). |
| `caps` | byte string | bit vector | No | Capability bits. See [`05-control-protocol.md`](./05-control-protocol.md) §4. |
| `vendor` | text | string ≤ 32 | If type ≥ 0x80 | Reverse-DNS vendor identifier. |
| `tags` | array | text strings | No | Free-form tags for sailor-side filtering. |
| `vss` | text | string ≤ 256 | No | Canonical `Vessel.*` path from the Pelorus catalog ([`../catalog/`](../catalog/)) for telemetry mirroring catalog semantics. |
| `since` | uint | u64 | No | Unix-epoch milliseconds at session open. |
| `extra` | map | tstr→any | No | Type-specific extension fields. |

Unknown keys at the top level shall be ignored by receivers. This is the forward-compatibility hinge.

### 4.1 Publisher Identifier (`pub`)

`<role>@<node-name>` — e.g. `helm-amp@saloon-stream-1`, `radar@bow-radar`, `nmea-bridge@gateway-port`.

For nodes that also participate in Core, `node-name` should derive from the Core NAME field's manufacturer/function; this is a recommendation, not a contract. A node uses the same `pub` value for every stream it publishes during a single boot session.

### 4.2 Static vs Mutable Fields

| Class | Examples | Update mechanism |
| --- | --- | --- |
| **Static** | `id`, `type`, `pub`, `since`, `format`, `instance`, `class` | Never change. Republished only on session open. |
| **Mutable** | `name`, `prio`, `tags`, `extra`, `caps` | May change. Republished via stream-update ([`11`](./11-events-and-errors.md)). |

A subscriber that observes a *static* field changing on the same Stream ID shall surface a `metadata-conflict` error and unsubscribe; the publisher should have closed and re-opened the stream with a new ID.

### 4.3 Size Budget

Total CBOR-encoded metadata in an mDNS announcement shall fit comfortably in a 255-byte TXT record string and shall not exceed 1200 bytes cumulative TXT data per RR. Implementations needing more shall split into a paired metadata stream rather than oversize the TXT.

---

## 5. Payload Units

A stream is an ordered sequence of **payload units** (PUs). A PU is the smallest complete unit a publisher emits and a subscriber consumes:

- For a `radar-video` stream: one spoke datagram (or one fragment of a split spoke).
- For a `telemetry` stream: one CBOR map (one logical sample).
- For a `control` stream: one control message.
- For a `file` stream: an HTTP/3 byte range chunk; framing is HTTP/3's, not Stream's.
- For a `nav` stream: one CBOR map keyed by `Vessel.*` paths (one logical position/attitude sample).

PUs are atomic at the application layer. A subscriber either gets a complete PU or a loss notification; there is no partial PU.

The Stream control plane treats PU bodies as **opaque bytes** — it does not interpret payload contents. Per-type interpretation lives in the service documents.

### 5.1 Datagram Framing

For unreliable services (radar video, telemetry, nav, health), each PU rides one QUIC datagram (RFC 9221). The datagram carries the standard Pelorus Stream Datagram Header followed by the PU body. The header is defined normatively in [`04-transport.md §5`](./04-transport.md) — it carries service type, instance, sequence number, fabric ID, flags, and a gPTP timestamp.

One PU per datagram. Multiple PUs per datagram are not permitted in v1.0; this keeps loss recovery simple.

If a future service requires sub-PU fragmentation (very-large telemetry, oversized radar spokes), that service document defines its own continuation flag in the header's reserved bits. Radar video already does this for spokes that exceed the 1200-byte QUIC datagram limit ([`10-services-nav.md`](./10-services-nav.md)).

### 5.2 Reliable Stream Framing

For reliable services (control commands, chart file transfer), each PU is one HTTP/3 request/response or one CBOR-framed control message on a QUIC reliable stream. Sequence and ordering are guaranteed by QUIC; no Stream-level sequence number is required.

### 5.3 Loss

Lossy delivery is the default for datagram services. A subscriber that misses a sequence number shall:

1. Note the gap.
2. Continue with the next PU received.
3. Optionally surface a `data-loss` event ([`11-events-and-errors.md`](./11-events-and-errors.md)).

The publisher does not retransmit datagrams. For reliable services, QUIC handles retransmission and the application sees an ordered, gap-free sequence.

### 5.4 Empty PUs

A PU with zero bytes of payload is **not** permitted. Publishers signaling "alive but no data" use a stream event or a transport keepalive, never a zero-byte PU. This prevents ambiguity between "empty PU" and "lost PU" at the wire layer.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
