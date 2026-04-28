# Pelorus Stream — Encoding

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **encoding policy** for Pelorus Stream: which serializers are used where, what the constraints are, and why. It is the policy companion to [`13-serialization.md`](./13-serialization.md) (which is the formal CBOR profile for the control plane) and the format documents in 16 (audio) and beyond.

---

## 1. Two Encoding Planes

Pelorus Stream has two distinct encoding planes with different requirements:

| Plane | What it carries | Encoding | Why |
|---|---|---|---|
| **Control plane** | Announcements, subscribe/unsubscribe, events, updates | **Deterministic CBOR** (RFC 8949) | Compact, schema-flexible, deterministic, embedded-friendly |
| **Payload plane** | Stream PUs (audio frames, telemetry samples, control commands) | **Type-specific** | Each payload type uses its native most-efficient encoding |

The two planes are **never** mixed. A control message never contains a raw audio frame; an audio packet never contains a CBOR map.

---

## 2. Control Plane: CBOR

Reasons CBOR is the control-plane encoding:

- **Compactness.** ~2× smaller than equivalent JSON for typical control messages, often more.
- **Determinism.** RFC 8949 §4.2.1 specifies a deterministic encoding mode (canonical key order, smallest int encoding, no tags-on-tags). This makes hashes and signatures reproducible and prevents encoder-divergence bugs.
- **Embedded-friendly.** Multiple no_std Rust crates exist (`minicbor`, `ciborium`); decode size is small enough for STM32-class controllers.
- **Schema-optional.** CDDL is available when we want it but not required to parse.
- **Well-supported across language ecosystems.** Rust, C, Python, JS, Go.

### 2.1 Profile

The Pelorus control plane uses a **strict subset** of CBOR:

| Feature | Status | Notes |
|---|---|---|
| Definite-length encoding | **Required** | No indefinite-length items on the wire |
| Deterministic key order (RFC 8949 §4.2.1) | **Required** | Map keys sorted by length-then-byte |
| Tag 37 (UUID) | **Required** | For Stream IDs |
| Tag 0/1 (date/time) | **Permitted** | For metadata timestamps |
| Custom tags | **Reserved** | Range 55799–55899 reserved for Pelorus |
| Floating-point | **Permitted (single, double)** | No half-floats |
| Indefinite-length strings | **Forbidden** | |
| Tag 24 (encoded CBOR) | **Forbidden** | Avoids nested decode quirks |

Encoders shall write the strict subset; decoders shall accept it and reject anything else with a `decode-error` ([`25-stream-error.md`](./25-stream-error.md)).

### 2.2 Why Not JSON

- Larger on the wire.
- No native byte-string type — base64 padding for the 16-byte UUID and similar fields is wasteful.
- No deterministic encoding without an extension (canonical-JSON variants are not standardized).
- Floating-point precision concerns for telemetry.

### 2.3 Why Not Protobuf / FlatBuffers / Cap'n Proto

- Schema requirement is heavier than necessary for a small control plane.
- Cross-language tooling is more complex than CBOR's.
- Forward compatibility for unknown keys is more awkward than CBOR's "ignore unknown".
- Embedded support exists but is not as broadly proven for `no_std` Rust as CBOR.

These options are not bad; they are not best for *this* use case.

---

## 3. Payload Plane: Type-Specific

Each payload type uses its native encoding. The control plane does not care what is inside.

| Type | Encoding | Reference |
|---|---|---|
| `audio` | Opus packet bytes (RFC 6716) | [`16-audio-format.md`](./16-audio-format.md) |
| `telemetry` | One CBOR map per PU (deterministic profile from §2.1) | This document, §4 |
| `control` | One CBOR map per PU (deterministic profile from §2.1) | [`17-playback-control.md`](./17-playback-control.md) |
| `video` (reserved) | TBD | v2.0+ |
| `file` (reserved) | Raw bytes with framing per the file format | v1.1+ |

Telemetry and control happen to use CBOR for their PUs — this is convenient, not required by the architecture. A future payload type could choose any encoding it wishes.

---

## 4. Telemetry Payload Encoding

A telemetry PU is a single CBOR map. Recommended top-level keys:

| Key | Type | Required | Purpose |
|---|---|---|---|
| `t` | uint (uepoch ms) | Yes | Sample timestamp |
| `seq` | uint | Inferred from envelope | Already in envelope; not required in payload |
| `v` | map\|number\|array | Yes | The value(s) being sampled |
| `q` | uint | No | Quality / confidence indicator (0–255) |
| `src` | tstr | No | Sub-source within the publisher |

Schema for `v` is publisher-defined and may be referenced via `extra.schema-uri` in metadata ([`06-stream-metadata.md` §7](./06-stream-metadata.md)).

---

## 5. Endianness

CBOR is byte-oriented and endianness-agnostic at the wire level (definite-length integers are big-endian per RFC 8949). Implementations have no endianness choice for control-plane data.

For payload plane, endianness is per format. Opus is byte-stream defined and not affected.

---

## 6. Compression

Stream payloads are **not** further compressed at the Stream layer. The codecs and serializers chosen are already efficient for their payload classes:

- Opus is a compressed audio codec.
- CBOR with deterministic encoding is comparable in size to gzipped JSON without the gzip cost.
- File transfers (deferred to v1.1) may apply per-file compression as part of the file format.

A general-purpose compression layer (zstd, brotli) is not part of v1.0 Stream.

---

## 7. Open Items

- Whether to add an explicit version byte on the wire envelope (currently version is in the announcement only — see [`14-versioning.md`](./14-versioning.md)).
- Whether to support COSE-signed control messages for future authentication (currently no — v1.0 trusts onboard LAN).
- Whether telemetry should permit per-publisher choice of CBOR vs. another encoding for payload (currently CBOR-only, change requires v1.1).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
