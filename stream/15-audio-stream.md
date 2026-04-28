# Pelorus Stream — Audio Stream

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specializes the generic Stream model for **audio**: the channel model, the latency budget, and the integration points with discovery, format ([`16-audio-format.md`](./16-audio-format.md)), playback control ([`17-playback-control.md`](./17-playback-control.md)), and buffering ([`18-buffering.md`](./18-buffering.md)).

Audio is the only media class specified in v1.0. Video, sonar, and radar are deferred per [`01-overview.md` §5](./01-overview.md).

---

## 1. What an Audio Stream Is

An audio stream is a Pelorus Stream of `type = audio` (`0x01`) carrying a sequence of Opus packets representing real-time audio. Use cases:

- **Intercom:** mic-to-speaker(s) voice between cabins, helm, or with a tender.
- **Alarms and announcements:** alert tones routed to all enabled speaker zones.
- **Entertainment:** music distribution from a media source to amplifiers and speakers.
- **Audible navigation:** synthesized speech for AIS-CPA or course alerts.

An audio stream is **non-safety-critical**. Audible alarms whose loss would compromise safety shall **also** be raised via Pelorus Core mechanisms; Stream is the loud-and-rich channel, not the only channel.

---

## 2. Channel Model

| Channel count | Use | Notes |
|---|---|---|
| 1 (mono) | Voice intercom, navigation speech, alarm tones | Default. Smallest bandwidth. |
| 2 (stereo) | Music, ambient | Permitted via capability `audio-opus-48k-stereo`. |
| > 2 | — | **Not in v1.0.** Multi-channel audio (5.1, ambisonic) is not specified. |

Multi-zone distribution is achieved with **multiple parallel streams**, not multi-channel within a stream. A music source emitting to four cabins emits four streams (or one multicast stream that all four cabins join — preferred).

---

## 3. Latency Budget

The end-to-end latency budget for v1.0 audio:

| Component | Target | Notes |
|---|---|---|
| Capture and Opus encode | ≤ 25 ms | One 20 ms frame plus encoder lookahead |
| Network (in-vessel) | ≤ 5 ms | LAN one-way |
| Jitter buffer (subscriber) | ≤ 60 ms typical | Adaptive, see [`18-buffering.md`](./18-buffering.md) |
| Opus decode and DAC | ≤ 10 ms | Decoder + audio-out path |
| **End-to-end** | **≤ 100 ms typical** | Audible-but-acceptable for intercom |
| **Worst-case** | **≤ 200 ms** | Above this, intercom feels broken |

Streams that need lower latency than 100 ms (e.g. live music collaboration) are **out of scope** for v1.0. v1.0 targets practical onboard voice and music distribution; sub-100 ms specialty applications can use a future profile.

---

## 4. Roles

A stream node may act as one or more of:

| Role | Description | Examples |
|---|---|---|
| **Publisher** (source) | Encodes and emits an audio stream. | Microphone preamp, media-source headend |
| **Subscriber** (sink) | Receives, decodes, and renders audio. | Powered speaker, amplifier zone |
| **Mixer** | Subscribes to multiple streams, emits a derived stream. | "Saloon background mix" headend |
| **Bridge** | Translates between Pelorus Stream audio and an external audio system. | Alexa / DLNA / Sonos bridge (if any) |

Mixers and bridges are subscribers + publishers from the protocol's perspective; there is no separate primitive.

---

## 5. Stream Metadata for Audio

In addition to common metadata ([`06-stream-metadata.md`](./06-stream-metadata.md)), audio streams carry:

| Key | Type | Required | Notes |
|---|---|---|---|
| `format` | u16 | Yes | Format code per [`16-audio-format.md`](./16-audio-format.md). |
| `sr` | u32 | Yes | Sample rate in Hz; 48000 default. |
| `ch` | u8 | Yes | 1 mono, 2 stereo. |
| `bitrate-kbps` | u16 | No | Nominal Opus bitrate. |
| `vad` | bool | No | Voice-activity-detection enabled. |
| `dtx` | bool | No | Discontinuous transmission enabled. |
| `zone` | tstr | No | Sailor-friendly zone label (e.g. `"saloon"`). |
| `purpose` | tstr | No | One of `intercom`, `alarm`, `entertainment`, `navigation`, `other`. |

`zone` and `purpose` are advisory; the **Pelorus State subsystem** uses them to construct multi-zone routing UIs. Stream itself does not interpret them.

---

## 6. Multicast vs. Unicast for Audio

| Pattern | Recommended mode |
|---|---|
| One mic to one speaker (1:1 intercom) | Unicast |
| One mic to many speakers (PA, multi-zone alarm) | Multicast SSM |
| One source to a configurable subset of zones | Multicast SSM with subscriber filtering |
| Mixer with N inputs and M outputs | Subscribe unicast or multicast to inputs; multicast outputs |

The publisher chooses based on its expected subscriber count and the network's multicast support. Subscribers do not pick the mode; they observe `mode=` in the announcement and join accordingly.

---

## 7. Synchronization Across Speakers

v1.0 does **not** specify multi-speaker synchronization. Two speakers playing the same multicast audio stream will drift relative to each other by up to ±20 ms depending on jitter buffer choices. This is acceptable for safety alarms and acceptable for casual music distribution; it is not acceptable for true whole-vessel synchronized audio.

A v1.1+ profile may add PTPv2-based audio synchronization (RFC 8173 / IEEE 1588) for streams that opt in via a `sync` capability bit.

---

## 8. Interaction with State Layer

**Pelorus State subsystem** responsibilities for audio (informative — normative State text will live under [`state/`](../state/00-document-index.md)):

- Decide which audio sources are "on" at a given time (suppress music when the engine alarm announces).
- Drive volume and zone-routing UI.
- Implement priority dimming (e.g. duck music 12 dB during navigation speech).

Stream layer responsibilities for audio:

- Carry Opus packets reliably enough.
- Expose state and metadata so State can decide.
- Apply soft playback control commands ([`17-playback-control.md`](./17-playback-control.md)) when received.

The audio publisher does **not** decide on its own to silence itself for another stream. The audio subscriber does **not** decide on its own to attenuate one source for another. State decides; Stream applies.

---

## 9. Open Items

- A formal end-to-end conformance test for the 100 ms typical latency budget — currently aspirational, awaits hardware.
- Multi-zone synchronization profile (PTPv2 or otherwise).
- Whether to specify a "voice priority" capability that auto-ducks entertainment audio (currently no — that is State's job).
- Echo cancellation requirements for full-duplex intercom (currently publisher's local concern).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
