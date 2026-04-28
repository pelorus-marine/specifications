# Pelorus Stream — Stream Metadata

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **metadata** that accompanies every Pelorus stream: the human- and machine-readable description of what a stream is, where it came from, and what it is currently doing. Metadata is published in announcements (mDNS TXT, [`23-discovery.md`](./23-discovery.md)) and updated via stream-update messages ([`21-stream-update.md`](./21-stream-update.md)).

Metadata is **descriptive**. It is not authority and it is not state in the sense of [`20-stream-state.md`](./20-stream-state.md).

---

## 1. Metadata Fields

The v1.0 metadata schema is a CBOR map ([`13-serialization.md`](./13-serialization.md)) with the following keys. Required keys must be present in every announcement; optional keys may be omitted (subscribers must tolerate absence).

| Key (text) | CBOR major | Type | Required | Notes |
|---|---|---|---|---|
| `id` | byte string | UUIDv7 | Yes | Stream ID. See [`02-stream-id.md`](./02-stream-id.md). |
| `type` | uint | u8 | Yes | Type code. See [`03-stream-type.md`](./03-stream-type.md). |
| `prio` | uint | u8 (0–15) | No | Default 7 if absent. See [`04-stream-priority.md`](./04-stream-priority.md). |
| `pub` | text | string ≤ 64 chars | Yes | Publisher node identifier (see §2). |
| `name` | text | string ≤ 64 chars | No | Human-readable stream name, e.g. "Saloon Speakers". |
| `lang` | text | BCP-47 tag | No | For `audio`/`text` payloads, e.g. `en`, `fr`. |
| `format` | uint | u16 | Type-dependent | Format code per the type's format document. |
| `profile` | uint | u8 | No | Profile selector inside the format. |
| `sr` | uint | u32 | Audio only | Sample rate Hz, e.g. 48000. |
| `ch` | uint | u8 | Audio only | Channel count: 1 mono, 2 stereo. |
| `cad` | uint | u32 | Telemetry | Nominal cadence in milliseconds; 0 = irregular. |
| `caps` | byte string | bit vector | No | Capability bits. See [`14-versioning.md`](./14-versioning.md). |
| `vendor` | text | string ≤ 32 chars | If type ≥ 0x80 | Reverse-DNS vendor identifier. |
| `tags` | array | text strings | No | Free-form tags for sailor-side filtering. |
| `vss` | text | string ≤ 256 chars | No | If present: canonical **`Vessel.*`** path for telemetry whose samples mirror Core semantics (see [`core/06-signal-catalog.md`](../core/06-signal-catalog.md) §6). Ignored for non-`telemetry` types in v1.0 unless extended later. |
| `since` | uint | u64 | No | Unix-epoch milliseconds at session open. |
| `extra` | map | tstr→any | No | Type-specific extension fields. |

Unknown keys at the top level shall be ignored by receivers. This is the forward-compatibility hinge.

---

## 2. Publisher Identifier (`pub`)

The publisher identifier is a stable, short string that names the node that originated the stream. Format:

```
<role>@<node-name>
```

Examples:

- `helm-amp@saloon-stream-1`
- `nmea-bridge@gateway-port`
- `intercom-mic@cockpit`

For nodes that are also Pelorus Core participants, `node-name` should be derived from the Core NAME field's manufacturer/function, but the exact derivation is a *recommendation*, not a contract — Stream identity is intentionally decoupled from Core identity ([`02-stream-id.md` §2.2](./02-stream-id.md)).

A node shall use the same `pub` value for every stream it publishes during a single boot session. On reboot, `pub` may change (e.g. if a sailor renames the node), but it should be stable across power cycles when the configuration is unchanged.

---

## 3. Mutability and Update Cadence

Metadata fields fall into two classes:

| Class | Examples | Update mechanism |
|---|---|---|
| **Static** | `id`, `type`, `pub`, `since`, `format`, `sr`, `ch` | Never change. Republished only on session open. |
| **Mutable** | `name`, `prio`, `tags`, `extra`, `caps` | May change. Republished via [`21-stream-update.md`](./21-stream-update.md). |

A subscriber that observes a *static* field changing on the same Stream ID shall treat the stream as inconsistent, surface a `metadata-conflict` error ([`25-stream-error.md`](./25-stream-error.md)), and unsubscribe. The publisher should have closed and re-opened the stream with a new ID instead.

Mutable fields propagate through the registry ([`22-stream-registry.md`](./22-stream-registry.md)) on a best-effort basis. Convergence is eventual; a subscriber may see stale `name` for seconds after a rename.

---

## 4. Size Budget

Total CBOR-encoded metadata for an mDNS announcement shall fit comfortably in a 255-byte TXT record string and shall not exceed 1200 bytes of cumulative TXT data per RR. Implementations that need more than that for `extra` shall split into a separate metadata stream (recommended pattern: a paired `telemetry` stream of type `extra`).

The 255-byte-per-string limit forces metadata to be terse. Long human-readable descriptions belong in a paired text stream, not in TXT.

---

## 5. Localization

`name` may be a single string or a CBOR map of BCP-47 → string for multi-language UIs:

```cbor
"name": "Saloon Speakers"
```

or

```cbor
"name": {"en": "Saloon Speakers", "fr": "Haut-parleurs salon"}
```

Subscribers shall accept either form. The registry stores either form unchanged.

---

## 6. Tags

`tags` is a free-form array of short (≤ 24 char) strings that helps sailor-side UI filter long stream lists. There is no central tag registry. Conventional tags:

- `intercom`, `entertainment`, `alarm`, `nav`, `engine`
- `cabin`, `cockpit`, `helm`, `saloon`, `galley`
- `bridge` (for Stream-to-Core bridge-published streams)

Tags are not used for routing decisions and shall not be interpreted as security labels.

---

## 7. The `extra` Field

`extra` is a type-specific extension map. Each payload-format document (15–18 and beyond) may define keys that appear here. Examples:

- For `audio`: `bitrate-kbps`, `voice-activity-detection`, `discontinuous-transmission`
- For `telemetry`: `schema-uri`, `compression`

Unknown keys inside `extra` shall be ignored. There is no global registry; keys are scoped to the type.

---

## 8. Open Items

- Whether to introduce a checksum field for the static portion of metadata so subscribers can detect inconsistency cheaply (currently no — relying on the format/type/sr/ch tuple to flag mismatch).
- Convention for the cross-vessel federation case where multiple streams from different vessels share `name` (deferred — v1.0 is single-vessel scope).
- A formal schema language for `extra` payloads — currently informal; CDDL is the leading candidate for v1.1.
- Mirror `vss` in mDNS TXT when present (see §1 `vss` key) and verify cumulative TXT size with long paths.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
