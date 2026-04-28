# Pelorus Stream — Versioning

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **versioning** of the Pelorus Stream wire protocol and the **capability negotiation** mechanism that lets v1.x publishers and subscribers cooperate when one knows features the other does not.

Draft design targets are summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary).

---

## 1. Version Numbers

The protocol version is **semantic** at the major-minor level: `vMAJOR.MINOR`.

| Component | Bumped when |
|---|---|
| **MAJOR** | An incompatible change occurs (e.g. envelope reshape, encoding change, port change). v1.x and v2.x cannot interoperate. |
| **MINOR** | A backward-compatible feature is added (new message kind, new metadata field, new capability bit). |

There is **no PATCH**. Patch-level fixes to the specification text that do not change wire behavior do not bump the version.

For v1.0, the on-wire `v` field in the envelope ([`12-envelope.md` §2](./12-envelope.md)) is **0** — interpreted as the v1.0 minor version. v1.1 will use `v = 1`. v2.0 will use a new `kind` code space and `v` will reset to `0` again.

---

## 2. Wire Field

The envelope's `v` field (CBOR map key 1) is a single unsigned byte (`u8`):

- High nibble: **MAJOR - 1** (so v1.x is `0`, v2.x is `1`, etc.)
- Low nibble: **MINOR** (so v1.0 is `0x00`, v1.1 is `0x01`, …, v1.15 is `0x0F`)

A 4-bit minor field caps each major series at 16 minor versions. If we exhaust them, that is itself a reason to bump major. This is intentional pressure to accumulate minor changes.

---

## 3. Cross-Major Behavior

A receiver that observes a different MAJOR than its own shall:

- Discard the datagram silently.
- **Not** emit an error (would amplify mismatches into log spam).
- Optionally rate-limit a `version-mismatch` log entry locally.

Cross-major coexistence on the same network is permitted. v1 publishers and v2 publishers can coexist on the same Ethernet plant; they will simply not subscribe to each other.

---

## 4. Cross-Minor Behavior

Within the same MAJOR, a higher-MINOR receiver:

- Accepts datagrams from lower-MINOR senders.
- Treats unknown envelope keys, unknown body keys, and unknown `kind` codes as ignorable (forward compatibility).
- Does not emit features or capability bits that the lower-MINOR sender did not advertise.

A lower-MINOR receiver:

- Accepts datagrams from higher-MINOR senders.
- Ignores envelope keys it does not understand.
- Ignores `kind` codes it does not understand (no reply, no error).
- Treats unknown capability bits as 0.

This works because:

- Map keys in CBOR are explicit and skippable.
- Kind codes are sparse and reserved blocks are ignored.
- Capabilities are bit-vectors with safe-to-ignore default semantics.

---

## 5. Capability Bits

Capabilities are advertised in:

- The mDNS TXT record (`caps=<hex>`) for discovery
- The `subscribe` body for negotiation
- The `subscribe-ack` echo for confirmation

Capabilities are a CBOR byte string of bit-vector form, big-endian, MSB of byte 0 = bit 0. Length is publisher-/subscriber-defined; receivers shall treat absent bytes as zero.

### 5.1 v1.0 Capability Bits

| Bit | Name | Semantics |
|---|---|---|
| 0 | `payload-cbor` | Sender supports CBOR control plane (always 1 in v1.0). |
| 1 | `mode-unicast` | Sender supports unicast UDP. |
| 2 | `mode-multicast-ssm` | Sender supports SSM multicast. |
| 3 | `mode-multicast-asm` | Sender supports ASM multicast. |
| 4 | `mode-quic` | Sender supports QUIC reliable mode. |
| 5 | `audio-opus-48k-mono` | Sender supports Opus 48 kHz mono. |
| 6 | `audio-opus-48k-stereo` | Sender supports Opus 48 kHz stereo. |
| 7 | `audio-opus-16k-narrowband` | Sender supports Opus 16 kHz narrowband (low-bandwidth voice). |
| 8 | `playback-control` | Sender accepts/sends playback control messages. |
| 9 | `metadata-update` | Sender emits/handles metadata updates. |
| 10 | `state-update` | Sender emits/handles state updates. |
| 11 | `event-stream` | Sender emits/handles `event` messages. |
| 12 | `mlds` | Sender supports MLDv2 (relevant for multicast subscribers). |
| 13–15 | Reserved future v1.x | |
| 16–31 | Reserved future v1.x | |
| 32–55 | Reserved future v1.x | |
| 56–63 | Reserved future v2.x preview | |
| 64+ | Vendor-defined | Paired with vendor identifier. |

A subscriber decides whether to attempt subscription based on the intersection of its caps and the publisher's caps. If a required cap is absent, it does not subscribe; this is not an error.

---

## 6. Negotiation

`subscribe` includes the subscriber's caps. `subscribe-ack` echoes the **intersection** as the negotiated cap set for the subscription.

Both sides shall behave only according to negotiated caps for the lifetime of the subscription. A capability not in the intersection is *not* available, even if both sides theoretically support it (one might be policy-disabled).

---

## 7. Vendor Capabilities

Vendor-defined caps live at bit 64 and above. A vendor cap is meaningful only when paired with the `vendor` metadata field ([`06-stream-metadata.md` §1](./06-stream-metadata.md)). Two distinct vendors must never share a bit position; this is unenforceable in v1.0 and is a non-issue in practice because vendors negotiate paired with their `vendor` identifier.

---

## 8. Document Numbering vs. Protocol Versioning

The 28 documents 00–27 ([`00-document-index.md`](./00-document-index.md)) are *spec document numbers*. They do **not** correspond to protocol version numbers. The same numbered document is amended across protocol minor versions; the document carries the protocol version it most recently describes in its frontmatter.

Sequential ordering of the 28 documents is part of the **specification contract** per Issue [#1](https://github.com/pelorus-marine/specifications/issues/1) and is independent of protocol versioning.

---

## 9. Open Items

- Whether the v field should be larger (`u16`) to allow more than 16 minors per major (currently u8 with high/low nibble split).
- Authoritative registry for vendor capability bits beyond bit 64 (currently informal).
- Sunset rules for deprecated capabilities (currently caps live forever; "deprecated" is a documentation concept only).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
