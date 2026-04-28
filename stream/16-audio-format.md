# Pelorus Stream — Audio Format

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specifies the **on-wire audio format**: codec, sample rates, framing, packetization, and the format-code registry that audio stream metadata refers to via `format`.

The headline choice (Opus, 48 kHz, 20 ms) is a **draft design target** in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary).

---

## 1. Codec: Opus (RFC 6716)

Opus is the audio codec for Pelorus Stream v1.0. Reasons:

- **Free and open.** RFC 6716 is unencumbered; reference implementation is BSD-licensed.
- **One codec, many use cases.** Voice (narrowband 8 kHz) through music (fullband 48 kHz) without switching codecs.
- **Low latency.** 5 ms minimum frame size; 20 ms is the sweet spot.
- **Strong loss concealment.** Built-in PLC for short loss bursts.
- **Variable bitrate.** Bandwidth scales with content complexity.
- **Excellent ecosystem.** Hardware encoders/decoders exist; pure-software fits modern marine electronics.
- **Already a default for WebRTC, Discord, and broadcast** — implementations are battle-tested.

There is no second codec in v1.0. PCM, MP3, AAC, AC-3, and FLAC are not on-wire codecs in this specification. (PCM may exist in the publisher's input or the subscriber's output stage; the wire is Opus.)

---

## 2. Sample Rate

| Sample rate | Status | Use |
|---|---|---|
| 48 kHz | **Mandatory baseline.** All v1.0 implementations support. | All audio. |
| 16 kHz (Opus narrowband) | Capability `audio-opus-16k-narrowband`. Optional. | Low-bandwidth voice intercom over weak links. |
| Other Opus rates (8 / 12 / 24 kHz) | **Forbidden** in v1.0 announcements. | — |

Internally, Opus may switch its analysis bandwidth (e.g. SILK at narrowband, CELT at fullband). Implementations need not concern themselves; encoder selects based on `bitrate-kbps` and content.

---

## 3. Frame Size

The on-wire frame size is **20 ms** for v1.0:

| Sample rate | Samples per frame | Bytes (typical voice @ 32 kbps VBR) |
|---|---|---|
| 48 kHz | 960 | ~80 |
| 16 kHz | 320 | ~40 |

A subscriber that needs lower latency than one 20 ms frame buys it back from the jitter buffer ([`18-buffering.md`](./18-buffering.md)), not from a smaller frame size.

10 ms and 40 ms frames are reserved for v1.1; they are negotiated via capability bits not allocated in v1.0. v1.0 publishers always emit 20 ms frames.

---

## 4. Bitrate

Recommended bitrates (informative, not normative):

| Use | Bitrate | Notes |
|---|---|---|
| Voice intercom | 24–32 kbps | Mono, 48 kHz, Opus VBR. |
| Voice over weak link | 12–16 kbps | Mono, 16 kHz narrowband. |
| Music (general) | 64–128 kbps | Stereo, 48 kHz, VBR. |
| Music (audiophile) | 192 kbps | Stereo, 48 kHz, VBR. |
| Alarm tone | 16 kbps | Mono, 48 kHz, CBR for robustness. |

Publishers shall set `bitrate-kbps` in metadata to communicate their nominal bitrate; subscribers may use it for buffer sizing.

---

## 5. Packetization

One Opus packet **per PU** per UDP datagram ([`05-stream-payload.md` §3](./05-stream-payload.md)).

The Opus packet format is per RFC 6716 §3 — a Table-of-Contents byte followed by frame data. The PU byte string is the Opus packet, exactly. No additional headers, no Ogg framing, no RTP wrapping.

This keeps the wire minimal:

```
UDP datagram = IPv6 + UDP header + Stream envelope + Opus packet
             ≈ 40 +  8           +  ~34            +  ~80          = ~162 bytes
```

At 50 PUs/s that is ~65 kbps including all overhead per stream. A vessel running ten concurrent voice streams uses ~650 kbps — comfortable on 100 Mbit/s.

---

## 6. Format Code Registry

The `format` metadata field selects a precise audio profile:

| Code | Name | Sample rate | Channels | Frame ms |
|---|---|---|---|---|
| `0x0001` | `opus-48k-mono-20` | 48000 | 1 | 20 |
| `0x0002` | `opus-48k-stereo-20` | 48000 | 2 | 20 |
| `0x0003` | `opus-16k-mono-20` | 16000 | 1 | 20 |
| `0x0004`–`0x000F` | Reserved future Pelorus audio | — | — | — |
| `0x0010`–`0x00FF` | Reserved | | | |
| `0x0100`–`0x7FFF` | Future Pelorus codecs | | | |
| `0x8000`–`0xFFFE` | Vendor | | | |
| `0xFFFF` | Reserved sentinel | | | |

A subscriber that does not implement the announced `format` shall not subscribe; it is not an error.

---

## 7. Profile Selector

The `profile` metadata field, when present, allows fine-tuning within a format. v1.0 defines:

| Profile | Meaning | Applies to |
|---|---|---|
| `0x00` | Default — VBR, application=voip | Mono voice formats |
| `0x01` | VBR, application=audio | Music formats |
| `0x02` | CBR, application=audio | Alarm-tone use |
| `0x03` | Restricted-LowDelay — for mixers | Format-agnostic |

Implementations that do not understand a `profile` shall use `0x00`.

---

## 8. Discontinuous Transmission (DTX)

If the publisher has voice activity detection, it may emit "comfort noise" frames or skip frames during silence. The `dtx` metadata flag advertises this.

A subscriber receiving a missing PU during a DTX-enabled stream shall not surface a `data-loss` event; loss during silence is normal. The subscriber should generate comfort noise locally per RFC 6716 §4.2.

---

## 9. Multi-zone Music: Format Constraints

When the same music source feeds multiple zones via multicast, all subscribers receive identical Opus packets. Zone-specific volume and EQ are subscriber-side post-processing concerns and do not change the wire format.

---

## 10. Open Items

- Format codes for 10 ms / 40 ms Opus frames (v1.1+).
- Whether to register a CBR-only sub-format for severely constrained encoders (currently profile bit handles this).
- Lossless codec for studio-style use (deferred — outside marine scope).
- Surround / multichannel formats (deferred to multi-stream pattern).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
