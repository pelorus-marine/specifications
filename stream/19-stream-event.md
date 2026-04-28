# Pelorus Stream — Stream Event

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **stream-event model**: discrete, named occurrences that a publisher (or in some cases a subscriber) emits to inform observers of something noteworthy that happened to a stream. Events are the primary way the Stream subsystem talks to the **Pelorus State subsystem**.

Events are **emitted, never authoritative**. An event is a notification, not a command, not a state, not a fact-of-record.

---

## 1. What an Event Is — and Isn't

| An event **is** | An event **isn't** |
|---|---|
| A timestamped, typed notification | A request to act |
| Best-effort delivered | Guaranteed delivered |
| Idempotent at the receiver (de-dup by `seq`) | Repeatable for retry semantics |
| Emitted on a control endpoint (UDP `5354`) or unicast/multicast | Persisted by Stream itself |
| Consumed by State for decisions | Acted upon by Stream itself |

Stream emits events. State decides. If a sailor needs to be alerted, State decides how (UI flash, audio alarm publication, …).

---

## 2. Event Envelope

An event reuses the standard Stream envelope ([`12-envelope.md`](./12-envelope.md)) with `kind = 0x0010`. Body shape:

```cbor
{
  "name": "<event-name>",        ; required, see registry
  "level": "info"|"warn"|"err",  ; advisory severity, default "info"
  ? "details": {<event-specific>},
  ? "since": <epoch ms>,         ; when the underlying condition began
}
```

`name` is the canonical identifier; `details` is event-specific structure. Receivers consult the registry (§3) to interpret `details` for known names.

---

## 3. Event Name Registry

| Name | Level | Emitted by | When |
|---|---|---|---|
| `opened` | info | Publisher | Session moves IDLE → ANNOUNCED. |
| `activated` | info | Publisher | First subscriber attached, ANNOUNCED → ACTIVE. |
| `deactivated` | info | Publisher | Last subscriber left, ACTIVE → IDLE-ATTACHED. |
| `keepalive` | info | Publisher | No PUs to send; lifeline. |
| `closing` | info | Publisher | Graceful close imminent. |
| `subscriber-joined` | info | Publisher | A unicast subscriber attached. |
| `subscriber-left` | info | Publisher | A unicast subscriber detached. |
| `subscribers-quiet` | warn | Publisher | Lease lapsed without renewal, none active. |
| `data-loss` | warn | Subscriber | Detected sequence gap not closed within buffer. |
| `buffer-underrun` | warn | Subscriber | Output starved. |
| `buffer-overrun` | warn | Subscriber | Output flooded; PUs dropped. |
| `late-pu` | info | Subscriber | A PU arrived after its scheduled play time (logged, not user-visible by default). |
| `discontinuity` | info | Either | Discontinuity flag observed; buffer reset. |
| `format-mismatch` | err | Subscriber | Received PU does not match negotiated format. |
| `metadata-conflict` | err | Subscriber | Static metadata changed mid-session. |
| `payload-too-large` | err | Subscriber | PU exceeded subscriber's MTU. |
| `decode-error` | err | Subscriber | Codec or CBOR decode failed. |
| `transport-stalled` | warn | Subscriber | Heartbeats absent for 3× interval. |
| `publisher-disappeared` | warn | Subscriber | Lease expired with no traffic. |
| `clock-drift` | info | Subscriber | Drift compensation engaged. |
| `vendor:<reverse-dns>:<name>` | * | Either | Vendor-defined event. Receivers ignore unless they recognize the vendor. |

Unknown event names shall be **ignored** by receivers (forward compat). Logging an unknown name is permitted but shall not propagate as an error.

---

## 4. Cardinality and Cadence

Events shall be emitted **once per occurrence**, not on a periodic schedule. Specifically:

- `opened`, `activated`, `deactivated`, `closing`: once per state transition.
- `subscriber-joined`/`left`: once per subscriber transition.
- `data-loss`, `buffer-underrun`/`overrun`: once per gap, with optional summary fields (e.g. `details.frames_lost`).
- `keepalive`: every 5 s when there is no other traffic ([`08-connection.md` §7](./08-connection.md)).

A publisher emitting the same event many times per second for the same condition is broken. Aggregation (one event per second carrying counts) is acceptable.

---

## 5. Severity (`level`)

Three levels:

- `info` — normal operation, routine.
- `warn` — transient, recoverable, sailor-visible if the State subsystem chooses.
- `err` — non-recoverable for this PU/subscriber/session; resolution requires action by the publisher or operator.

Level is **advisory**. A State subsystem aggregator may re-classify; it may decide that 100 `data-loss` warns/min is "ok" for music and "alarm-worthy" for voice intercom.

---

## 6. Delivery

Events go to:

- All known unicast subscribers (for unicast streams).
- The control multicast group (if defined) for the stream's type. **In v1.0 there is no global control multicast group;** events on multicast streams reach observers via the multicast payload group on port `5354` adjacent to the payload group on `5355`.

Publisher-emitted events for multicast streams travel on the multicast group; subscriber-emitted events travel on the publisher's unicast control endpoint. Cross-subscriber event visibility (one subscriber seeing another's events) is **not** offered by Stream — that is a **State subsystem** aggregation concern.

---

## 7. Persistence

Stream does not persist events. Subscribers, the registry ([`22-stream-registry.md`](./22-stream-registry.md)), or the **Pelorus State subsystem** may keep ring buffers; Stream itself emits and forgets.

Stream emits each event with a **schema context** implied by the envelope: protocol **`v`** ([`12-envelope.md`](./12-envelope.md)) and event **name**/payload shape ([§3](#3-event-names)). Consumers shall treat unknown names or payload shapes as ignorable unless a capability bit opts in.

**Timestamps** in envelopes (`ts`, `tts`) are **publisher-local** unless a vessel-wide time policy applies (planned under **`state/`**, see [state/00-document-index.md](../state/00-document-index.md)); Stream v1.0 does **not** require IEEE 1588 (PTP). When correlating Stream data with Core or multiple publishers, **State** is responsible for clock interpretation—not Stream alone.

Implementations exposing events to user code (via the reference library, [`27-lib.md`](./27-lib.md)) shall offer at least a 256-event ring buffer for diagnostic UIs.

---

## 8. Open Items

- Whether `late-pu` should be promoted from "log only" to a counted summary event (currently log-only).
- A formal threshold-based "loss-rate-elevated" event distinct from per-gap `data-loss` (currently aggregator's job).
- Per-event cardinality budgets for high-cadence streams.
- Whether to expose a publisher-side "events from all my streams" multicast for fleet-wide diagnostics (currently no — too noisy).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
