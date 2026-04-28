# Pelorus Stream — Envelope

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **envelope**: the small, fixed-shape header that wraps every Pelorus Stream message and PU on the wire. The envelope is the same for control messages and payload PUs; the body is what differs.

The envelope is encoded per [`13-serialization.md`](./13-serialization.md) (deterministic CBOR).

---

## 1. Wire Form

A Pelorus Stream UDP datagram is a single CBOR value: a 2-element array `[envelope, body]`.

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
  <body>                 ; control map for control messages,
                         ; byte string for payload PUs
]
```

The body shape depends on `kind`:

- For control kinds (≤ `0x00FE`), body is a CBOR map with kind-specific keys.
- For payload PUs (sent on port 5355), the envelope `kind` is `0xF000` ("payload") and body is a CBOR byte string of opaque payload bytes.

---

## 2. Envelope Field Map

The envelope is a CBOR map with **integer keys** (not string keys, for compactness):

| Key | Field | Type | Required | Notes |
|---|---|---|---|---|
| 1 | `v` | u8 | Yes | Protocol minor version. See [`14-versioning.md`](./14-versioning.md). |
| 2 | `kind` | u16 | Yes | Kind code, including `0xF000` for payload. |
| 3 | `sid` | byte string (16 B) | Yes | Stream ID (UUIDv7). |
| 4 | `seq` | u32 | Yes | Per-(sender, sid) sequence. Wraps at 2³². |
| 5 | `ts` | u64 | No | Wall-clock millisecond timestamp. Advisory only. |
| 6 | `flags` | u32 | No | Bit flags (table §3). Default 0. |
| 7 | `tts` | u64 | No | Publisher-monotonic microseconds since session open (for jitter). |
| 8 | `caps` | byte string | No | Echo of negotiated capability bits — present only on `subscribe-ack`. |

Unknown integer keys shall be **ignored** by receivers (forward compatibility).

---

## 3. Flags

| Bit | Name | Meaning |
|---|---|---|
| 0 | `discontinuity` | Receiver should reset jitter buffer; this PU is not contiguous with prior. |
| 1 | `keyframe` | This PU is a self-contained start-of-stream anchor (audio not applicable; video and others). |
| 2 | `final` | This is the last PU of the session (paired with `closing`). |
| 3 | `redundant` | This PU is a re-emission of a prior `seq`; subscribers may de-duplicate. |
| 4–31 | Reserved | Set to 0; receivers shall ignore. |

---

## 4. Size Budget

Envelope CBOR weight (worst case with all optional fields):

| Field | Approx bytes |
|---|---|
| `v` | 2 (key + value) |
| `kind` | 4 |
| `sid` | 19 (key + tag + 16 bytes) |
| `seq` | 6 |
| `ts` | 9 |
| `flags` | 6 |
| `tts` | 9 |
| Map overhead | 2 |
| **Total** | **~57 bytes** |

A typical payload datagram envelope (no `tts`, no `ts`) is around **34 bytes**. This is the budget [`05-stream-payload.md` §3.1](./05-stream-payload.md) deducts from MTU.

---

## 5. Integrity

The UDP checksum protects the entire envelope and body. Stream does not add an additional CRC at the application layer for v1.0.

QUIC streams have built-in TLS integrity. Plain UDP relies on the UDP checksum (which is mandatory for IPv6).

A v1.1 signed envelope profile is reserved: an additional `9` key carrying a COSE_Sign1 detached signature over the canonical CBOR encoding of the envelope. v1.0 receivers shall ignore key `9` if present.

---

## 6. Encoding Determinism

Envelopes shall be encoded with deterministic CBOR per [`13-serialization.md`](./13-serialization.md). Specifically:

- Integer map keys appear in numerical ascending order.
- Smallest-int encoding (e.g. `seq = 5` is encoded in 1 byte, not 4).
- No indefinite-length items.

This matters for any future signed-envelope profile and for cheap byte-equality checks during testing.

---

## 7. Envelope Versioning

The `v` field is the **protocol minor version**. Major versions are bumped only on incompatible wire changes. Minor versions carry capability bits per [`14-versioning.md`](./14-versioning.md).

For v1.0, `v = 0`. Future v1.x increments `v`. A v2.0 would use a different `kind` code space and require capability negotiation up front — that is a v2 problem.

A receiver that sees `v` greater than its highest known version shall treat the envelope as best-effort: keep what it can parse, ignore unknown keys, and surface no protocol error unless required fields are missing.

---

## 8. Examples

### 8.1 Audio PU envelope (typical)

```cbor
[
  {1: 0, 2: 0xF000, 3: h'<16-byte UUID>', 4: 542, 7: 10840000},
  h'<~80 bytes Opus packet>'
]
```

### 8.2 Subscribe message

```cbor
[
  {1: 0, 2: 0x0001, 3: h'<sid>', 4: 1, 5: 1714200000000},
  {
    "lease": 60000,
    "caps": h'01000000',
    "from": "fe80::dead:beef:cafe:1234"
  }
]
```

### 8.3 Closing event

```cbor
[
  {1: 0, 2: 0x0005, 3: h'<sid>', 4: 9999, 6: 4},  ; flags bit 2 = final
  {"reason": "publisher-shutdown"}
]
```

---

## 9. Open Items

- Whether to assign integer body-keys (parallel to envelope keys) for the most common control bodies (currently using text keys for human-readability of pcap captures).
- Padding/reserved-bytes field for fixed-size hardware encoders (currently no — variable-length CBOR is fine).
- Whether the `tts` field should be 32-bit microseconds with explicit wrap rules (currently 64-bit).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
