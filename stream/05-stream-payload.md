# Pelorus Stream — Stream Payload

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines **Stream Payload**: the abstract notion of "what a stream carries" and the rules for framing it on the wire. Concrete payload formats live in the Media subsystem (15–18); this document defines the model that all of those format documents specialize.

---

## 1. Payload Model

A stream is, conceptually, an ordered sequence of **payload units** (PUs). A PU is the smallest complete unit a publisher emits and a subscriber consumes:

- For an `audio` stream: one Opus packet (~20 ms of audio).
- For a `telemetry` stream: one CBOR map (one logical sample).
- For a `control` stream: one control message ([`17-playback-control.md`](./17-playback-control.md)).

PUs are atomic at the application layer. A subscriber either gets a complete PU or a loss notification. There is no partial PU.

---

## 2. Opaque to the Control Plane

The Stream control plane (announcements, capability negotiation, registry, discovery) treats PUs as **opaque bytes**. Control plane code does not interpret the contents of a PU. This separation lets the control plane stay small and stable while payload formats evolve.

Concretely:

- The on-wire envelope ([`12-envelope.md`](./12-envelope.md)) carries the payload as a single CBOR byte string of length up to MTU-minus-headers.
- The envelope's `type` field selects which downstream document defines the contents of that byte string.
- Type-specific documents (15–18) define the PU layout *inside* that byte string.

A reference library ([`27-lib.md`](./27-lib.md)) may expose typed accessors per stream type, but the wire format is byte-string-of-opaque-PU.

---

## 3. Framing

Each network packet on a Stream session carries **exactly one** PU at the application layer for v1.0. Multiple PUs in a single UDP datagram are not permitted in v1.0; this keeps loss recovery and jitter buffering simple.

Justification:

- Audio at 20 ms / Opus / 48 kHz mono with typical bitrates produces datagrams well under MTU.
- Telemetry samples are small (tens to a few hundred bytes).
- Control messages are tiny.

If a future payload format requires sub-PU fragmentation (very-large telemetry sample, file transfer), that format document will define its own fragmentation header **above** the envelope. The envelope itself remains one-PU-per-datagram in v1.0.

### 3.1 MTU Assumption

Stream assumes a 1500-byte Ethernet MTU and computes its safety budget against that:

| Component | Bytes (typ.) |
|---|---|
| IPv6 header | 40 |
| UDP header | 8 |
| Stream envelope ([`12-envelope.md`](./12-envelope.md)) | 16–48 |
| Payload | up to ~1404 |

PUs that approach 1404 bytes shall be the exception, not the rule. Implementations shall set the IPV6_DONTFRAG socket option where available; fragmentation is a symptom of misuse.

---

## 4. Sequence and Time

Each PU carries a 32-bit **sequence number** in the envelope, incremented by 1 per PU emitted on a session. Wraparound at 2³² is permitted but only after >49 days at 1 ms PU cadence; for the v1.0 audio profile this never happens during a single session.

PUs may also carry a publisher-local **monotonic timestamp** (microseconds since session open). This is for jitter-buffer scheduling ([`18-buffering.md`](./18-buffering.md)). It is **not** wall-clock time.

Wall-clock timestamps, where needed, live in payload-format-specific fields, not in the generic envelope.

---

## 5. Loss

Lossy delivery is the default. A subscriber that misses a sequence number shall:

1. Note the gap.
2. Continue with the next PU received.
3. Optionally surface a `data-loss` event ([`19-stream-event.md`](./19-stream-event.md)).

The publisher is **not** required to retransmit. For reliable streams (QUIC, [`08-connection.md`](./08-connection.md)), the transport handles retransmission; the application still sees an ordered sequence with no gaps.

---

## 6. Ordering

Within a single session, sequence numbers define the canonical order. Out-of-order arrival is permitted on UDP transports; the subscriber's jitter buffer reorders within its window and drops anything older than its buffer.

For QUIC transports, the QUIC stream layer guarantees in-order delivery and the Stream sequence number is redundant but still emitted for parity.

---

## 7. Variable PU Sizes

Most payload formats produce variable-size PUs (Opus VBR, telemetry maps with optional fields). Implementations shall not assume fixed PU size. The envelope's CBOR byte-string length prefix communicates the actual size of each PU.

A subscriber that has been sized for a worst-case PU and finds itself receiving PUs larger than that shall surface a `payload-too-large` error ([`25-stream-error.md`](./25-stream-error.md)) and may unsubscribe.

---

## 8. Empty PUs

A PU with zero bytes of payload is **not** permitted in v1.0. Publishers that wish to signal "alive but no data" shall use a stream event ([`19-stream-event.md`](./19-stream-event.md)) or a transport keepalive, never a zero-byte PU.

This rule prevents ambiguity between "empty PU" and "lost PU" at the wire layer.

---

## 9. Open Items

- Whether to add an opt-in PU CRC distinct from the UDP checksum (currently no — UDP checksum is sufficient on a typical onboard switch fabric).
- Whether to permit PU batching (multiple PUs per datagram) for high-cadence telemetry streams (deferred to v1.1+).
- Behavior when a publisher exceeds MTU mid-stream — `payload-too-large` reaction is specified, but graceful renegotiation is not (currently the publisher must close and re-open with a smaller-PU profile).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
