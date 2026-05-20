# Pelorus Stream — Control Protocol

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

The control plane that runs over QUIC reliable streams: message taxonomy, envelope, deterministic CBOR encoding, and versioning. Datagram services (radar, nav, telemetry, health) use the datagram header in [`04-transport.md §5`](./04-transport.md) directly and do not carry this envelope.

## 1. Two Encoding Planes

| Plane | What it carries | Encoding |
|---|---|---|
| **Control plane** | Subscribe / unsubscribe, events, updates, soft control commands | Deterministic CBOR (this document) over QUIC reliable streams |
| **Payload plane** | Service datagrams (radar spokes, nav, health, telemetry) | Service-specific binary payload behind the datagram header in [`04-transport.md §5`](./04-transport.md) |

Reliable file/asset transfers (S-100 charts, firmware) use HTTP/3 over QUIC and do not carry this envelope.

## 2. Message Taxonomy

| Message | Direction | Purpose |
|---|---|---|
| `subscribe` | Subscriber → Publisher | Open or renew a subscription |
| `unsubscribe` | Subscriber → Publisher | Voluntary teardown |
| `subscribe-ack` | Publisher → Subscriber | Accept or reject a subscribe |
| `keepalive` | Publisher → Subscriber | Indicates publisher alive when no traffic flows |
| `event` | Publisher → Subscriber | Stream-level event |
| `state-update` | Publisher → Subscriber | Observable state change |
| `metadata-update` | Publisher → Subscriber | Mutable metadata change |
| `closing` | Publisher → Subscriber | Graceful session close |
| `radar-control` | Sender → radar publisher | Radar control commands; payload defined in [`10-services-nav.md`](./10-services-nav.md) |
| `error` | Either direction | Out-of-band error notification |

Messages not listed are reserved and shall be ignored. Implementations shall not invent ad-hoc kinds outside the vendor capability range (§5).

### 2.1 Kind Codes

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
| `0x0020`–`0x002F` | Reserved (formerly soft media playback) |
| `0x0030` | `radar-control` |
| `0x00FE` | `error` |
| `0x0100`–`0x7FFF` | Reserved future Pelorus |
| `0x8000`–`0xFFFE` | Vendor (paired with `vendor` capability bit) |
| `0xFFFF` | Reserved sentinel |

Kind codes are stable. Once assigned, a code is never reused for a different meaning.

## 3. Envelope

A control message on a QUIC reliable stream is a single CBOR value: a 2-element array `[envelope, body]`.

```cbor
[
  {
    1: 0,                ; v (protocol minor version)
    2: 0x0001,           ; kind code
    3: h'<16 bytes>',    ; sid (Stream ID, UUIDv7)
    4: 12345,            ; seq (per-sender monotonic)
    5: 1714200000000,    ; ts (wall-clock ms, advisory)
    6: 0                 ; flags
  },
  <body>                 ; CBOR map with kind-specific keys
]
```

### 3.1 Envelope Field Map

| Key | Field | Type | Required | Notes |
|---|---|---|---|---|
| 1 | `v` | u8 | Yes | Protocol minor version (§5) |
| 2 | `kind` | u16 | Yes | Kind code (§2.1) |
| 3 | `sid` | byte string (16 B) | Yes | Stream ID (UUIDv7) |
| 4 | `seq` | u32 | Yes | Per-(sender, sid) sequence |
| 5 | `ts` | u64 | No | Wall-clock millisecond timestamp; advisory |
| 6 | `flags` | u32 | No | Bit flags (§3.2). Default 0. |
| 8 | `caps` | byte string | No | Echo of negotiated capability bits — present only on `subscribe-ack` |

Unknown integer keys shall be ignored by receivers (forward compatibility).

### 3.2 Flags

| Bit | Name | Meaning |
|---|---|---|
| 0 | `discontinuity` | Receiver should reset jitter buffer; this PU is not contiguous with prior |
| 1 | `keyframe` | Self-contained start-of-stream anchor (video and similar) |
| 2 | `final` | Last message of the session (paired with `closing`) |
| 3 | `redundant` | Re-emission of a prior `seq`; subscribers may de-duplicate |
| 4–31 | Reserved | Set to 0; receivers shall ignore |

## 4. Idempotency and Acknowledgement

`subscribe`, `unsubscribe`, and `metadata-update` are **idempotent** at the protocol level. A receiver applying a duplicate shall apply it as if it were the first.

`event` and `state-update` are **monotonic** — receivers de-duplicate by `seq`.

| Message | Ack |
|---|---|
| `subscribe` | Yes — `subscribe-ack` within 1 s |
| `unsubscribe` | No |
| `radar-control` | Yes — QUIC reliable stream guarantees delivery; explicit `state-update` echo expected |
| `closing` | No |
| Others | No |

## 5. Versioning

Protocol version is semantic at the major-minor level: `vMAJOR.MINOR`.

| Component | Bumped when |
|---|---|
| **MAJOR** | Incompatible change (envelope reshape, encoding change, transport change). Different majors do not interoperate. |
| **MINOR** | Backward-compatible feature added (new kind, new metadata field, new capability bit). |

There is no PATCH — text-only fixes do not bump the version.

The envelope `v` field is `u8`:

- High nibble: MAJOR − 1 (v1.x is `0`, v2.x is `1`)
- Low nibble: MINOR (v1.0 is `0x00`, v1.15 is `0x0F`)

For v0.2 of this specification, on-wire `v = 0`. v1.0 and onward will increment.

### 5.1 Cross-Major Behaviour

A receiver that observes a different MAJOR than its own discards silently — no error emission (would amplify mismatches into log spam). Cross-major coexistence is permitted; v1 and v2 publishers can share an Ethernet plant and simply not subscribe to each other.

### 5.2 Cross-Minor Behaviour

Within the same MAJOR:

- Higher-MINOR receivers accept lower-MINOR senders; treat unknown envelope keys, unknown body keys, and unknown kind codes as ignorable.
- Lower-MINOR receivers accept higher-MINOR senders; ignore envelope keys, kind codes, and capability bits they do not understand.

This works because CBOR map keys are explicit and skippable, kind codes are sparse with reserved blocks, and capabilities are bit-vectors with safe-to-ignore default semantics.

### 5.3 Capability Bits

Capabilities are advertised in the mDNS TXT record (`caps=<hex>`), the `subscribe` body, and the `subscribe-ack` echo. Encoded as a CBOR byte string of bit-vector form, big-endian, MSB of byte 0 = bit 0. Receivers shall treat absent bytes as zero.

| Bit | Name | Semantics |
|---|---|---|
| 0 | `payload-cbor` | Sender supports CBOR control plane (always 1 in v1.0) |
| 1 | `quic-datagrams` | Sender supports QUIC datagrams (RFC 9221) — always 1 in v1.0 |
| 2 | `quic-reliable` | Sender supports QUIC reliable streams — always 1 in v1.0 |
| 3 | Reserved | Formerly `audio-opus-48k-mono`; audio is out of scope ([`01-overview.md §1`](./01-overview.md)) |
| 4 | Reserved | Formerly `audio-opus-48k-stereo`; audio is out of scope |
| 5 | Reserved | Formerly `audio-opus-16k-narrowband`; audio is out of scope |
| 6 | Reserved | Formerly `playback-control`; audio/media playback is out of scope |
| 7 | `metadata-update` | Sender emits/handles metadata updates |
| 8 | `state-update` | Sender emits/handles state updates |
| 9 | `event-stream` | Sender emits/handles `event` messages |
| 10 | `radar-video` | Sender produces or consumes radar video service |
| 11 | `radar-control` | Sender accepts radar control commands |
| 12 | `s100-charts` | Sender produces or consumes S-100 chart distribution |
| 13 | Reserved | Formerly `ais-targets`; AIS lives on Core ([`10-services-nav.md`](./10-services-nav.md)) |
| 14 | `nav-high-rate` | Sender produces or consumes high-rate nav |
| 15 | `replication-node` | Sender is or supports a replication node ([`10-services-nav.md`](./10-services-nav.md)) |
| 16–55 | Reserved future v1.x | |
| 56–63 | Reserved future v2.x preview | |
| 64+ | Vendor-defined | Paired with vendor identifier |

`subscribe` includes the subscriber's caps; `subscribe-ack` echoes the **intersection** as the negotiated cap set for the subscription. Both sides behave only according to negotiated caps for the lifetime of the subscription.

Vendor caps at bit 64+ are meaningful only when paired with the `vendor` metadata field. Two distinct vendors must never share a bit position; this is unenforceable in v1.0 and is a non-issue in practice because vendors negotiate paired with their `vendor` identifier.

## 6. CBOR Encoding Profile (Pelorus-CBOR-1)

A strict subset of RFC 8949 with deterministic encoding requirements per RFC 8949 §4.2.1.

| Rule | Pelorus-CBOR-1 |
|---|---|
| Definite-length encoding only | Required |
| Smallest integer encoding | Required |
| Map keys in canonical order (length-then-byte) | Required |
| No duplicate keys | Required — encoder shall not emit; decoder shall reject |
| Maximum tag depth 1 | Required |
| Tag 24 (encoded CBOR) | Forbidden |
| Indefinite-length strings | Forbidden |
| Half-floats (f16) | Forbidden |
| f32 / f64 | Permitted |

### 6.1 Permitted Tags

| Tag | Meaning | Notes |
|---|---|---|
| 0 | Standard date/time string (RFC 3339) | Permitted in metadata |
| 1 | Epoch-based date/time (numeric) | Permitted; seconds (integer) or seconds-with-fraction (float) |
| 32 | URI | Permitted (e.g. `extra.schema-uri`) |
| 37 | UUID | Required for Stream IDs ([`02-data-model.md`](./02-data-model.md)) |
| 55799 | Self-described CBOR | Permitted at outermost level only |
| 55800–55899 | Reserved Pelorus | Reserved for future Pelorus extension; receivers shall reject |

All other tags are forbidden in v1.0 and shall be rejected by decoders.

### 6.2 Map Key Conventions

Two key conventions coexist:

1. **Integer keys** in the envelope for compactness (§3.1).
2. **Text keys** in message bodies and metadata for human-readability in packet captures.

Within a single map, keys shall not mix integer and text.

### 6.3 Decoder Robustness

Decoders shall:

- Reject any deviation from the rules in this section with a `decode-error` ([`11-events-and-errors.md`](./11-events-and-errors.md)).
- Reject duplicate keys.
- Reject map keys in non-canonical order.
- Ignore unknown text keys at the top level of message bodies.
- Ignore unknown integer keys in the envelope.
- Enforce a configurable maximum decode size, default 64 KiB for control-plane messages.

## 7. Origin and Authentication

v1.0 control messages rely on the QUIC TLS 1.3 session for transport authentication. The receiver trusts the publisher's link-local identity at the QUIC layer (subject of self-signed certificate). Trust evaluation policy is in [`04-transport.md §3.2`](./04-transport.md).

A v1.1 application-layer authenticated profile (COSE_Sign1 wrappers, pre-shared keys per vessel) is reserved.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
