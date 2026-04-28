# Pelorus Stream — Buffering

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specifies subscriber-side **buffering** for Pelorus Stream media: how much to buffer, how to absorb network jitter, and how to drop data when conditions force a choice between freshness and completeness. It is the policy companion to [`05-stream-payload.md`](./05-stream-payload.md) (which defines the abstract PU model).

The headline rule — **bounded latency over guaranteed delivery** — is specified in [`01-overview.md` §6](./01-overview.md).

---

## 1. The Jitter Buffer

A subscriber consumes PUs at a steady rate (one Opus frame every 20 ms for `audio`). The network delivers them with variable latency. A jitter buffer absorbs the variance.

```
     wire arrival                       playback
  irregular ────────▶ [ jitter buffer ] ────────▶ regular
   (bursts, jitter)                                (steady)
```

The buffer's **depth** in milliseconds determines:

- How much loss it can recover from (time available for re-arrival of late-but-not-lost PUs).
- The added end-to-end latency.

Pelorus Stream's buffers are **adaptive**: they grow when loss is observed, shrink when loss subsides.

---

## 2. Default Buffer Depths

| Stream type | Initial | Min | Max |
|---|---|---|---|
| `audio` (intercom, voice) | 40 ms | 20 ms | 120 ms |
| `audio` (entertainment, music) | 120 ms | 60 ms | 300 ms |
| `audio` (alarm tone) | 40 ms | 20 ms | 100 ms |
| `telemetry` | 0 (no buffer) | 0 | 0 |
| `control` | 0 (immediate) | 0 | 0 |

The intercom budget keeps end-to-end ≤ 100 ms typical ([`15-audio-stream.md` §3](./15-audio-stream.md)).

The entertainment budget trades latency for cleaner playback under congestion. Music can absorb 300 ms of buffering; a sailor pushing skip on a track will not feel that.

A subscriber may override these defaults but shall not exceed Max for the type.

---

## 3. Adaptation Algorithm

A simple, deterministic algorithm (reference implementation; alternatives permitted):

1. Maintain a sliding window of the last 100 PU arrival inter-times.
2. Compute the 99th percentile inter-time, `J`.
3. Target buffer depth = `min(MAX_for_type, max(MIN_for_type, 2 * J))`.
4. Adjust slowly: at most one frame per second of growth, at most one frame every five seconds of shrink.

Slow adjustment prevents oscillation. Real implementations may use Kalman filters or PIDs; the requirement is bounded behavior, not a specific algorithm.

---

## 4. Drop Policy

When the buffer overflows (PUs arrive faster than they play out, indicating a clock-drift mismatch or a brief burst), the subscriber shall drop the **oldest** PU.

When a PU arrives later than its scheduled playback time, the subscriber shall **discard it without surfacing as data loss** — the PU is late, not lost.

When a PU is detected missing (sequence gap), and the gap is not closed within the buffer depth, the subscriber:

1. Surfaces a `data-loss` event ([`19-stream-event.md`](./19-stream-event.md)).
2. For audio: invokes Opus PLC for one frame, then attenuates if loss persists, finally inserting comfort noise.
3. Resyncs sequence numbers when traffic resumes.

---

## 5. Discontinuities

The envelope's `discontinuity` flag ([`12-envelope.md` §3](./12-envelope.md)) signals a non-contiguous PU. The subscriber shall:

1. Drain the jitter buffer (play out remaining PUs).
2. Reset the inter-time statistics window.
3. Restart at initial depth for the type.

Discontinuities happen on session re-attachment, on multicast group rejoin, and on publisher-explicit reset.

---

## 6. Clock-Drift Compensation

Publisher and subscriber clocks drift relative to each other (ppm-scale on a typical onboard oscillator). Over minutes-to-hours, this causes the subscriber to either run out of buffered audio or overflow.

Subscribers shall compensate by **occasional sample insertion or deletion** at the output stage. For Opus, this is most cleanly done by speeding up or slowing down playback through a resampler (1–2 ppm correction).

A subscriber that cannot resample shall instead duplicate or drop one frame periodically; this introduces audible artifacts on music but is acceptable for voice.

A subscriber that cannot do either shall accept periodic buffer underruns and recover via [`§4`](#4-drop-policy).

---

## 7. Backpressure

Pelorus Stream has **no backpressure** at the application layer. A subscriber overwhelmed by incoming PUs cannot slow the publisher; it can only drop. This is a deliberate consequence of best-effort UDP.

For QUIC streams, the QUIC layer's flow control handles backpressure transparently. Application code does not see it.

---

## 8. Telemetry and Control: No Buffer

Telemetry PUs are delivered to the application as soon as they arrive. There is no buffering (the application may have its own ring buffer for analysis, but the Stream library does not).

Control commands are delivered immediately. A subscriber that holds a control PU has missed the point of the message.

---

## 9. Metrics and Observability

Each subscriber shall maintain, per active subscription, the following metrics, exposed via the reference library ([`27-lib.md`](./27-lib.md)):

- Current buffer depth (ms)
- Cumulative PUs dropped (overflow)
- Cumulative PUs lost (gap)
- Cumulative PUs discarded (late)
- Jitter 99th percentile (ms)
- Last-N adjusted-buffer transitions

These metrics are exposed read-only and may be republished by the subscriber as a `telemetry` stream of its own if a State subsystem aggregator wants them.

---

## 10. Open Items

- A reference jitter-buffer implementation in `pelorus-stream` ([`27-lib.md`](./27-lib.md)) — choice of algorithm and tunable parameters.
- Per-subscriber QUIC-vs-UDP buffering differences (currently same depths for both).
- Whether to specify a "panic-drain" mode for sustained overload (drop everything until buffer hits Min) — currently implementation choice.
- PTPv2-aware buffering for synchronized multi-zone audio (deferred to v1.1).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
