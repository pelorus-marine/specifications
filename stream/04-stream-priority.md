# Pelorus Stream — Stream Priority

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines **Stream Priority**: a small, advisory hint about how a stream should be scheduled when contention arises. Priority is **not authority**. It does not preempt anything, it does not bypass the **Pelorus State subsystem**, and it has **no relationship whatsoever** to Pelorus Core arbitration priority defined in [`core/03-data-link-layer.md` §3.3](../core/03-data-link-layer.md).

The draft design target is summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary).

---

## 1. Scope

Stream Priority answers a narrow, mechanical question: *when two outbound packets are queued at the same NIC at the same instant, which goes first?*

It does **not** answer:

- Should this stream exist at all? — State decides ([`01-overview.md` §3.2](./01-overview.md)).
- Should I subscribe? — The application/State decides.
- Is this stream critical? — No Stream stream is critical. Anything critical lives on Core.
- Can this stream displace Core traffic? — No. Stream and Core run on different physical layers.

Implementations that try to use Priority for any of those questions are using it wrong.

---

## 2. Priority Encoding

Priority is a 4-bit unsigned integer (0–15) carried in:

- The stream announcement TXT record (`prio=<n>`)
- The control-message envelope ([`12-envelope.md`](./12-envelope.md)) for streams that announce variable priority
- The DSCP code-point mapping table (§3) when Stream packets reach the IP layer

| Range | Class | Typical use |
|---|---|---|
| 0–3 | Bulk | File transfer (when specified); telemetry that can wait. |
| 4–7 | Standard | Default for telemetry, generic media. |
| 8–11 | Interactive | Voice intercom, alarm-tone audio, navigator-display chrome. |
| 12–15 | Advisory-urgent | Reserved. See §4 — *advisory* not *authoritative*. |

The default for an unspecified stream is 7 (top of Standard). Publishers that want anything else shall publish a value explicitly.

---

## 3. DSCP Mapping

If a stream's underlying transport allows DSCP marking (UDP almost always does), a publisher may set the Differentiated Services Code Point on outbound packets according to RFC 4594 service classes:

| Stream priority | DSCP class | DSCP value |
|---|---|---|
| 0–3 | CS1 / Lower-effort | `001000` (8) |
| 4–7 | Default Forwarding | `000000` (0) |
| 8–11 | EF — Expedited Forwarding | `101110` (46) |
| 12–15 | EF — Expedited Forwarding | `101110` (46) |

Switches and routers in the vessel network are not required to honor DSCP. If they do not, Stream still works at best-effort. Implementations shall not assume DSCP enforcement.

---

## 4. The Advisory-Urgent Range (12–15)

Range 12–15 exists as an opt-in for streams whose loss is *operationally* annoying (alarm tone for low fuel, watch alert for AIS-CPA target). It is still **advisory**.

This range **shall not** be used for:

- Anything safety-critical. That belongs on Core.
- Bypassing State subsystem suppression. State may suppress 15-priority streams. Stream code shall not refuse such suppression.
- Holding QoS resources. There is no admission control in Stream.

A publisher claiming priority 12–15 on a stream that the **Pelorus State subsystem** determines should be silenced shall accept the silence. Stream is the messenger; State is the editor.

---

## 5. Local Scheduling

A node with multiple outbound Stream packets queued shall schedule them in priority order, ties broken by FIFO. Schedulers shall not starve lower-priority streams indefinitely; a simple weighted fair-queueing approach is sufficient and recommended.

A reference implementation may expose the scheduling algorithm via [`27-lib.md`](./27-lib.md). Custom schedulers are permitted.

---

## 6. Cross-stream Coordination Is Not Priority

A common request is "make sure the alarm tone takes precedence over the music". That is a **State** decision (mute music, ramp music down, raise alarm volume), not a Stream priority decision. Stream priority cannot reach across streams to mute one for another.

The correct pattern:

1. Alarm-tone publisher emits a high-priority `audio` stream and an associated `19-stream-event.md` event.
2. State subscribes to the event, decides what to do, and sends `17-playback-control.md` *pause* commands to the music stream.
3. Stream transports the pause command. Music stops.

Stream itself never decides to silence one stream because another exists.

---

## 7. Open Items

- Whether to map priority 12–15 to AF41 instead of EF, given EF is intended for low-latency low-jitter on a strict policer (currently EF — revisit if real-vessel measurements show it harms).
- Per-stream-type default priorities — to be settled in 15–18 once the audio profile is field-tested.
- Whether the State subsystem should publish a vessel-wide "priority budget" that nodes self-throttle against (currently no).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
