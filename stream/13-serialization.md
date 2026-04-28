# Pelorus Stream — Serialization

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document is the formal serialization profile for Pelorus Stream's control plane: the precise rules for how implementations turn in-memory values into bytes and back. The headline choice (deterministic CBOR per RFC 8949) is a **draft design target** in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary). This document fills in the testable details.

For the policy reasoning behind CBOR see [`10-encoding.md`](./10-encoding.md).

---

## 1. Profile Name

The Pelorus Stream control plane uses the **Pelorus-CBOR-1** profile, which is a strict subset of:

- RFC 8949 — *Concise Binary Object Representation (CBOR)*
- RFC 8949 §4.2.1 — *Core Deterministic Encoding Requirements*

A v1.1 profile may relax some restrictions; v1.0 takes the strictest reasonable subset to maximize interoperability between independent implementations.

---

## 2. Mandatory Encoding Rules

| Rule | Source | Pelorus-CBOR-1 |
|---|---|---|
| Definite-length encoding only | RFC 8949 §4.2.1 | **Required** |
| Smallest integer encoding | RFC 8949 §4.2.1 | **Required** |
| Smallest float encoding | RFC 8949 §4.2.1 | **Permitted but optional** for `f32`/`f64` choice; **required** that `f32` is not used when value is not exactly representable |
| Map keys in canonical order | RFC 8949 §4.2.1 (length-then-byte) | **Required** |
| No duplicate keys | RFC 8949 §4.2.1 | **Required** — encoder shall not emit; decoder shall reject |
| No tags-on-tags chain | — | **Required** maximum tag depth 1 |
| Tag 24 (encoded CBOR) | RFC 8949 §3.4.5 | **Forbidden** |
| Indefinite-length strings | RFC 8949 §3.2.3 | **Forbidden** |

---

## 3. Permitted CBOR Major Types

| Major | Type | Notes |
|---|---|---|
| 0 | Unsigned int | All sizes (0–8 bytes) per smallest-int rule. |
| 1 | Negative int | Same. |
| 2 | Byte string | Definite-length only. |
| 3 | Text string | Definite-length only. UTF-8 NFC recommended. |
| 4 | Array | Definite-length only. |
| 5 | Map | Definite-length only. Canonical key order. |
| 6 | Tag | Limited set, see §4. |
| 7 | Float / simple | `false`, `true`, `null`, `undefined` permitted; `f32`/`f64` permitted; `f16` (half) **forbidden**. |

---

## 4. Permitted Tags

| Tag | Meaning | Notes |
|---|---|---|
| 0 | Standard date/time string (RFC 3339) | Permitted in metadata. |
| 1 | Epoch-based date/time (numeric) | Permitted. Implementations should treat values as seconds (integer) or seconds-with-fraction (float). |
| 32 | URI | Permitted, e.g. for `extra.schema-uri`. |
| 37 | UUID | **Required** for Stream IDs ([`02-stream-id.md`](./02-stream-id.md)). |
| 55799 | "Self-described CBOR" | Permitted at the outermost level only; receivers shall accept and unwrap. |
| 55800–55899 | Reserved Pelorus | **Reserved** for future Pelorus extension; receivers shall reject. |

All other tags are **forbidden** in v1.0 and shall be rejected by decoders.

---

## 5. Map Key Conventions

Two key conventions coexist:

1. **Integer keys** are used in the envelope for compactness (see [`12-envelope.md` §2](./12-envelope.md)).
2. **Text keys** are used in message bodies and metadata for human-readability in packet captures and logs.

Within a single map, keys shall not mix integer and text. The canonical-order rule (length-then-byte-lex) treats integer keys first (encoded with major type 0/1, which sort before major type 3 by encoded byte sequence).

---

## 6. Decoder Robustness

Decoders shall:

- Reject any deviation from the rules in §2 with a `decode-error` ([`25-stream-error.md`](./25-stream-error.md)).
- Reject duplicate keys.
- Reject map keys in non-canonical order. (This is strict but enables byte-equality testing.)
- Ignore unknown text keys at the top level of message bodies.
- Ignore unknown integer keys in the envelope.
- Refuse to allocate large buffers blindly: a 32 MB byte-string in an unknown CBOR field shall not crash the decoder. Implementations shall enforce a configurable maximum decode size, default 64 KiB for control-plane datagrams.

---

## 7. CDDL Reference

The Pelorus Stream message taxonomy will be specified in CDDL (RFC 8610) in the reference implementation ([`27-lib.md`](./27-lib.md)). The CDDL is descriptive and is checked against test vectors during CI; it does not appear on the wire.

A skeleton (illustrative, non-normative):

```cddl
envelope = {
  1: uint,                      ; v
  2: uint,                      ; kind
  3: bstr .size 16,             ; sid
  4: uint,                      ; seq
  ? 5: uint,                    ; ts
  ? 6: uint,                    ; flags
  ? 7: uint,                    ; tts
  ? 8: bstr,                    ; caps echo
}

datagram = [envelope, body]
body = subscribe-body / event-body / metadata-update-body / payload-body / ...
payload-body = bstr
```

---

## 8. Test Vectors

Each Pelorus Stream protocol release publishes test vectors: hand-crafted hex strings of canonical encodings of representative messages. Implementations shall reproduce identical bytes when encoding the same in-memory value.

The vectors live in the reference repository under `tests/vectors/` and are updated whenever the protocol minor version changes.

---

## 9. Open Items

- Whether to fully reject `f32` in favor of `f64` always (currently both permitted; `f32` recommended for cabin temperatures and similar where precision is fine).
- COSE wrapping rules for v1.1 authenticated profile.
- Whether to embed CBOR-LD or similar typed-CBOR variants in the future telemetry payloads (currently no).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
