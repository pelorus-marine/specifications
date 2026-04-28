# Pelorus Stream — Stream Type

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **Stream Type** taxonomy: the closed enumeration that describes the *class* of payload a stream carries. Type is what tells a subscriber *whether* it can decode a stream; it does not describe *how* (that is the role of [`16-audio-format.md`](./16-audio-format.md) and other format-specific documents).

The draft enumeration is summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary). The full registry and forward-compatibility rules are normative here.

---

## 1. Type Registry

Stream Type is encoded as an 8-bit unsigned integer on the wire. The registry for v1.0:

| Code | Name | v1.0 status | Description |
|---|---|---|---|
| `0x00` | `reserved` | Reserved | Never assigned; receivers shall reject. |
| `0x01` | `audio` | Specified ([15–18](./15-audio-stream.md)) | PCM-equivalent audio carried as Opus frames. |
| `0x02` | `video` | Reserved | Image sequences. Format unspecified in v1.0. |
| `0x03` | `telemetry` | Specified | Periodic numeric or structured non-safety telemetry. |
| `0x04` | `file` | Reserved | Bulk file transfer. v1.1+. |
| `0x05` | `control` | Specified | Soft control plane (playback, volume). See [`17-playback-control.md`](./17-playback-control.md). |
| `0x06` | `text` | Reserved | Human-readable log or chat. v1.1+. |
| `0x07`–`0x7F` | `reserved-future` | Reserved | Reserved for future Pelorus assignment. |
| `0x80`–`0xEF` | `vendor` | Vendor | Vendor-specific; receivers shall ignore unless they advertise the same vendor capability. |
| `0xF0`–`0xFE` | `reserved-experimental` | Reserved | Local experimentation; never published outside a development bench. |
| `0xFF` | `reserved-sentinel` | Reserved | Reserved sentinel; receivers shall reject. |

A Stream's type is fixed for the lifetime of the stream. A publisher that wants to switch class shall close the existing stream and open a new one with a new Stream ID.

---

## 2. Type vs. Format

| Concept | Document | Cardinality |
|---|---|---|
| **Type** (this document) | 03 | One of a small enumeration |
| **Format** | 10, 16 | Many per type |
| **Profile** | 16 | Many per format |

A subscriber decides whether to subscribe based on **type**. Once subscribed, the subscriber negotiates **format** and **profile** via the announcement TXT and the open-session handshake ([`07-session.md`](./07-session.md)).

A stream of type `audio` always carries audio. A stream of type `audio` may carry Opus mono at 48 kHz 20 ms, or Opus stereo at 48 kHz 10 ms, or a future codec; type does not pin format.

---

## 3. Mandatory and Optional Types

A v1.0 conformant Pelorus Stream **publisher** shall implement at least one of:

- `audio`
- `telemetry`

A v1.0 conformant Pelorus Stream **subscriber library** ([`27-lib.md`](./27-lib.md)) shall accept and decode all v1.0 *Specified* types it advertises support for in its capability bits ([`14-versioning.md`](./14-versioning.md)).

A subscriber that does not advertise support for a type shall ignore announcements of that type and shall not attempt to subscribe.

---

## 4. Forward Compatibility

Stream Type is the primary forward-compatibility hinge of the protocol.

- New Pelorus types are added by amendment to this document; the next free code in the `reserved-future` range is taken.
- Receivers that do not recognize a type **shall** ignore the announcement. They shall not log it as an error and shall not propagate it.
- Vendor types in `0x80`–`0xEF` are **not** registered here. Vendors choose any code in that range and pair it with their vendor capability bit ([`14-versioning.md`](./14-versioning.md)) and TXT record (`vendor=...`).

This rule is what makes v1.0 receivers safe to deploy alongside v1.1 publishers: unknown is not a fault.

---

## 5. Telemetry Type Detail

Telemetry streams carry non-safety-critical numeric or structured data published as CBOR maps ([`13-serialization.md`](./13-serialization.md)). Examples: cabin temperature history, watermaker output flow rate, battery monitor charge counts, satellite-modem signal-strength rolling window.

Telemetry on Stream is **always non-authoritative**. A receiver shall not act on telemetry stream values to drive a safety decision. Authoritative numeric data lives on Pelorus Core.

The telemetry stream payload is one CBOR map per logical sample. Cadence is publisher-defined; subscribers absorb cadence via [`18-buffering.md`](./18-buffering.md).

---

## 6. Control Type Detail

Control streams carry soft commands targeting a **specific other Stream** (almost always an audio stream). Examples: pause, resume, set volume, seek.

Control streams **shall not** target Pelorus Core entities. A control stream message that names a Core entity shall be rejected by the receiver. Helm, autopilot, engine, thrusters, and similar actuators are off-limits.

The full message taxonomy lives in [`17-playback-control.md`](./17-playback-control.md).

---

## 7. Open Items

- Whether `telemetry` should be split into push and pull subtypes (currently push-only).
- Whether to expose a "structured event log" subtype distinct from generic `telemetry` (currently no — that is what [`19-stream-event.md`](./19-stream-event.md) describes).
- Reserved code blocks for sonar and radar — to be assigned in v1.1+ when those are specified.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
