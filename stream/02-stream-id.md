# Pelorus Stream — Stream Identifier

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **Stream ID**: the identifier that uniquely names a stream across the Pelorus Stream network for as long as that stream exists. Stream IDs are referenced by every other Stream document. The draft design target (UUIDv7, 128-bit) is summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary); this document specifies the format, lifetime rules, and presentation forms.

---

## 1. Identifier Format

A Stream ID is a **128-bit UUIDv7** per draft-ietf-uuidrev-rfc4122bis (Universally Unique IDentifiers).

UUIDv7 is chosen over v4 (random) and v1 (MAC-based) because it is:

- **Time-sortable.** The leading bits are a 48-bit Unix-epoch millisecond timestamp. Streams sort naturally by start time without parsing metadata.
- **Globally unique without coordination.** No central allocator is required.
- **Privacy-preserving.** No MAC address is encoded.
- **Stable in size.** Always 128 bits, never larger.

The 128-bit value is what is transmitted on the wire and stored in registries. Textual presentation is defined in [§4](#4-textual-presentation).

---

## 2. Layout

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
|---|---|---|
| `unix_ts_ms` | 48 bits | Unix-epoch milliseconds at stream creation, big-endian |
| `ver` | 4 bits | `0x7` |
| `rand_a` | 12 bits | Cryptographically-random or implementation-monotonic |
| `var` | 2 bits | `0b10` (RFC 4122 variant) |
| `rand_b` | 62 bits | Cryptographically-random |

### 2.1 Clock Discipline

The `unix_ts_ms` field is sourced from the publishing node's best available time. Onboard a vessel, this is typically a GNSS-disciplined clock provided by a Pelorus Core GNSS sensor and bridged to the Stream stack via the gateway.

Time skew between publishers does not break uniqueness — only the random bits enforce global uniqueness — but it does affect natural sort order. Implementations should not rely on Stream ID timestamps for any safety, billing, or audit purpose; use a separate timestamp metadata field for that (see [`06-stream-metadata.md`](./06-stream-metadata.md)).

### 2.2 No Embedded Identity

A Stream ID **shall not** carry node identity, manufacturer ID, MAC address, or device serial number. Identity is carried in metadata, not in the stream identifier itself. This allows the same physical node to host many streams and to retire and re-issue streams without exposing routing information.

---

## 3. Lifetime Rules

A Stream ID is bound to a single, specific stream instance:

- **Mint once.** A publisher mints a fresh UUIDv7 every time it opens a new stream session ([`07-session.md`](./07-session.md)).
- **Never reuse.** A Stream ID shall **not** be reused for any subsequent stream, even by the same publisher, even after the original stream has closed.
- **Survive transport reconnect.** If a stream's underlying connection drops and is re-established within the session lifetime, the Stream ID does not change.
- **Die with the stream.** Once the stream closes (graceful close, publisher exit, lease expiry), the Stream ID is retired.
- **Persist in logs.** Subscribers and registries may retain Stream IDs for diagnostics indefinitely. This does not constitute "active" use.

A node that observes the same Stream ID claimed by two distinct publishers shall treat the second claim as an error ([`25-stream-error.md`](./25-stream-error.md)) and refuse to subscribe to either until the conflict is resolved by State.

---

## 4. Textual Presentation

The canonical text form is the standard 8-4-4-4-12 hex string with hyphens, lowercase:

```
018f3c2b-9a4d-7c80-b1e2-4f5d6a7b8c9d
```

Stream IDs in user-facing UI **may** be truncated to the first 8 hex characters for display (e.g. `018f3c2b…`), but the on-wire and in-API form is always the full 128-bit value.

For brevity in CBOR ([`13-serialization.md`](./13-serialization.md)), Stream IDs are encoded as a 16-byte byte string with tag **37** (RFC 9581 — Tag for UUID).

---

## 5. Comparison and Equality

Two Stream IDs are equal if and only if all 128 bits are equal. There is no "stream family" or "stream group" relation expressed in the Stream ID itself; group membership lives in [`06-stream-metadata.md`](./06-stream-metadata.md).

Implementations shall use constant-time comparison only where they are comparing against an attacker-controlled value (none in v1.0); otherwise byte-wise equality is sufficient.

---

## 6. Rationale: Why Not …

Documented for completeness so this is not relitigated:

- **Not UUIDv4.** Random-only IDs are not time-sortable; sorting is useful for log analysis and de-duplication.
- **Not UUIDv1.** Encodes a MAC address; leaks identity and topology.
- **Not a 64-bit identifier.** Birthday-collision risk at the scale of vessel-fleet-wide log aggregation is non-trivial; 128 bits is cheap.
- **Not a publisher-local sequence number.** Requires a coordinator to compose with publisher identity, which State does not provide for Stream.
- **Not the underlying NAME from Core.** Stream identity is intentionally separate from Core node identity. One node may publish many streams.

---

## 7. Open Items

- Whether to assign a Pelorus-specific UUIDv7 sub-version reserving some `rand_a` bits for stream-class hints (currently *no* — full random `rand_a` per the standard).
- Whether stream restart after publisher reboot mints a new ID (currently *yes*) or attempts to recover the old ID from registry cache (currently *no*).
- Diagnostic representation in logs — full UUID vs. truncated form — to be settled in [`27-lib.md`](./27-lib.md).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
