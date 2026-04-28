# Pelorus Stream — Playback Control

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **playback control** message subset: soft commands that adjust how a media stream behaves. Playback control covers `play`, `pause`, `seek`, and `set-volume`.

These commands are **soft** — they target stream behavior, not safety-critical actuators. The playback-control surface is intentionally limited so it cannot be repurposed as a backdoor command channel for Core-controlled equipment. See [`01-overview.md` §2](./01-overview.md) — Stream does not command actuators.

---

## 1. Scope

Playback control may target:

- An audio stream (typically): change volume, pause, resume, seek (for streams with seekable backing media).
- A future video stream (reserved): same operations.

Playback control shall **not** target:

- Pelorus Core entities (engines, autopilot, helm, navigation lights, alarm horns).
- Any device outside the Stream subsystem.
- Anything described as "actuator" or "controller" on Core.

A receiver of a playback-control message that names a non-Stream entity shall reject the message with an `out-of-scope` error ([`25-stream-error.md`](./25-stream-error.md)) and ignore the command.

---

## 2. Direction and Authority

Playback control flows from a **controller** (a UI head, a remote, an automation rule) to the **publisher** of the target stream:

```
   ┌────────────┐                          ┌────────────────┐
   │ Controller │  play / pause / seek /   │ Stream         │
   │ (UI / app) │ ──────────────────────▶  │ Publisher      │
   └────────────┘  set-volume              │ (media source) │
                                            └────────────────┘
```

The publisher is the authority on its own stream. A controller asks; a publisher decides whether to comply.

A subscriber (a speaker) does **not** control a publisher; a subscriber may, however, locally apply a `set-volume` effect at its own output stage (subscriber-local volume — see §6).

**State decides which controllers exist.** Stream does not enforce access control in v1.0; State chooses what messages get sent. An onboard adversary who fakes control messages is out of scope of v1.0 threat modeling.

---

## 3. Message Bodies

All bodies are CBOR maps, ordinary control envelope ([`12-envelope.md`](./12-envelope.md)).

### 3.1 `play` (`kind=0x0020`)

```cbor
{
  "target": h'<sid of target stream>',
  ? "from": <epoch ms or stream-relative ms — for seekable sources>
}
```

Effect: publisher resumes emission from the requested point (or from current position if `from` absent).

For non-seekable live streams (intercom mic, alarm tone), `play` is implicit on session open and `from` is ignored.

### 3.2 `pause` (`kind=0x0021`)

```cbor
{
  "target": h'<sid of target stream>'
}
```

Effect: publisher stops emitting PUs and transitions to ANNOUNCED-paused (a sub-state of ANNOUNCED for media sources). The session remains open. A subsequent `play` resumes.

For live streams, `pause` is interpreted as "mute output" — the publisher continues to maintain its session but emits silence frames or comfort noise. Implementations may also choose to emit an event indicating the pause.

### 3.3 `seek` (`kind=0x0022`)

```cbor
{
  "target": h'<sid of target stream>',
  "to": <epoch ms or stream-relative ms>
}
```

Effect: applicable only to seekable streams (file-backed media). Publisher repositions and resumes from the requested point. Publishers that do not support seek shall reject with `not-seekable`.

### 3.4 `set-volume` (`kind=0x0023`)

```cbor
{
  "target": h'<sid of target stream>',
  "level": <0..255>,           ; 0 = silence, 255 = unity gain
  ? "scope": "publisher"|"subscriber"  ; default "publisher"
}
```

`scope=publisher`: the publisher attenuates its emitted PUs. All subscribers receive the attenuated stream.

`scope=subscriber`: this message is **directed at a specific subscriber** and applies post-decode at that subscriber's output stage. Multiple subscribers may have different per-subscriber volumes for the same multicast stream.

The volume scale is **linear**, not dB. A reference implementation maps 0..255 to a perceptual curve at the output stage; the wire form is linear so conversion is unambiguous.

---

## 4. Acknowledgement

Playback control messages are **fire-and-forget**. The controller observes the effect via the target stream's `state-update` ([`20-stream-state.md`](./20-stream-state.md)) or by listening to the stream itself. There is no `play-ack`; if the controller needs confirmation, it observes.

This avoids ACK timing issues and round-trips on multicast control. Idempotency ([`11-message.md` §4](./11-message.md)) handles retries.

---

## 5. Order and Concurrency

If two controllers send conflicting commands (one `play`, one `pause`) at nearly the same time, the publisher applies them in arrival order; the latter wins. Stream provides **last-writer-wins**.

State is responsible for not generating conflicting commands. Stream cannot mediate between controllers.

---

## 6. Subscriber-Local Volume

A subscriber that hosts a speaker has an output volume independent of any publisher-scope volume. The subscriber's volume is a **subscriber-private** setting; it may be exposed on the subscriber's own (subscriber-as-publisher) `state-update` stream so a UI can read it.

A subscriber receiving a `set-volume` with `scope=subscriber` and the subscriber's identity in the body applies the volume locally and updates its own state. The publisher of the audio stream is not informed.

---

## 7. Interaction with Mute / Suppress

Mute is **not** a Stream primitive. To silence audio, send `set-volume` with `level=0` or `pause`. The **Pelorus State subsystem** may generate either depending on context.

Mute-as-silent-button-in-a-UI is a State subsystem concern; State chooses how to translate the user's intent into Stream messages.

---

## 8. Open Items

- Whether to add a `step-volume` (relative ±N) command for hardware rotary encoders (currently no — controller computes absolute level).
- Whether to specify a reference perceptual curve for level mapping (currently subscriber-private).
- Behavior on `seek` past end-of-media (currently publisher chooses: stop or wrap).
- Multi-target playback control (e.g. "pause everything") — currently one target per message; bulk operations are State's job.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
