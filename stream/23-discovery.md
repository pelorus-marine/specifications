# Pelorus Stream — Discovery

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specifies **discovery** — how nodes find Pelorus Stream publishers and subscribers without static configuration. v1.0 uses mDNS-SD (RFC 6762, RFC 6763). Draft design targets are summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary).

---

## 1. Service Type

The mDNS service type for Pelorus Stream publishers is:

```
_pelorus-stream._udp
```

Subtype-based selection (RFC 6763 §7.1) is used to filter by stream type:

| Subtype | Meaning |
|---|---|
| `_audio._sub._pelorus-stream._udp` | Audio streams |
| `_telemetry._sub._pelorus-stream._udp` | Telemetry streams |
| `_control._sub._pelorus-stream._udp` | Control streams |
| `_video._sub._pelorus-stream._udp` | Reserved |
| `_file._sub._pelorus-stream._udp` | Reserved |

A subscriber that only cares about audio can browse `_audio._sub._pelorus-stream._udp` and ignore everything else. Browsing the bare `_pelorus-stream._udp` returns all streams.

The browser-domain is `local.` per RFC 6762; v1.0 is link-local only.

---

## 2. Service Instance Naming

Each stream gets one mDNS service instance per its Stream ID. The service-instance name is:

```
<stream-id-short>.<pub>._pelorus-stream._udp.local.
```

Where:

- `<stream-id-short>` is the first 8 hex chars of the Stream ID (collisions are possible but rare; the full ID disambiguates in TXT).
- `<pub>` is the publisher's `pub` metadata field, slug-safe (alphanumeric + hyphens; other characters replaced with `-`).

Example:

```
018f3c2b.intercom-mic-cockpit._pelorus-stream._udp.local.
```

Service-instance names are sailor-debuggable in any standard mDNS browser.

---

## 3. SRV Record

The SRV record points at the host and port serving this stream's control endpoint:

```
SRV: <priority> <weight> <port> <hostname>.local.
```

| Field | Value |
|---|---|
| `priority` | 0 |
| `weight` | 0 |
| `port` | The publisher's stream control port (default `5354`) |
| `hostname` | Publisher's mDNS hostname |

Weight and priority are not used in v1.0; both 0.

---

## 4. TXT Record

The TXT record carries the stream's discovery metadata. The schema is the metadata map from [`06-stream-metadata.md`](./06-stream-metadata.md), serialized as classic mDNS TXT key=value strings.

Required keys in TXT:

| TXT key | Source |
|---|---|
| `id=<full-uuid>` | `id` |
| `t=<type>` | `type` |
| `pub=<pub>` | `pub` |
| `mode=<u|m|q>` | Transport mode |
| `addr=<ipv6>` | Transport address (link-local or multicast) |
| `port=<u16>` | Transport payload port (default `5355`) |
| `v=<u8>` | Protocol version |
| `caps=<hex>` | Capability bit-vector |

Optional keys map directly from metadata: `name`, `prio`, `lang`, `format`, `profile`, `sr`, `ch`, `cad`, `vendor`, `tags`, `since`, `vss`.

Each TXT string is at most 255 bytes per RFC 6763 §6. Stream's metadata budget ([`06-stream-metadata.md` §4](./06-stream-metadata.md)) is sized to fit comfortably.

`tags` is encoded as comma-separated values within a single TXT string: `tags=intercom,cabin,saloon`. Decoders shall split on `,` and trim whitespace.

---

## 5. PTR Record

The PTR record links the service type to the service instance name per RFC 6763 §4.1.

```
PTR: _pelorus-stream._udp.local. → 018f3c2b.intercom-mic-cockpit._pelorus-stream._udp.local.
```

If subtypes are advertised:

```
PTR: _audio._sub._pelorus-stream._udp.local. → 018f3c2b.intercom-mic-cockpit._pelorus-stream._udp.local.
```

---

## 6. TTLs

Per RFC 6762, Pelorus Stream uses these TTLs:

| Record | TTL |
|---|---|
| PTR | 4500 (75 min) |
| SRV / TXT | 120 (2 min) |
| A / AAAA | 120 |

PTR is given a long TTL because service-discovery responders cache it; SRV/TXT are short because they may change as metadata updates arrive.

When a stream closes, the publisher emits **goodbye packets** (TTL=0 unsolicited responses) for PTR and SRV records to evict caches promptly.

---

## 7. Browse Patterns

A typical subscriber browses:

```
_audio._sub._pelorus-stream._udp.local.   -> PTR list
   for each instance:
     resolve SRV  -> hostname + port
     resolve TXT  -> metadata
     resolve AAAA -> IPv6 address
```

A typical registry node browses the bare `_pelorus-stream._udp.local.` to capture all streams.

---

## 8. Interaction with Existing mDNS Plant

Pelorus Stream nodes shall use the standard onboard mDNS responder. Implementations:

- Use the system responder where available (Avahi on Linux, `mdnsResponder` on Apple, `mdnsd` on FreeBSD).
- Fall back to an embedded responder on `no_std` platforms — `pelorus-mdns` reference crate.

Pelorus Stream **shall not** re-implement mDNS port handling. Multiple responders on one host produce mDNS conflicts. If the system responder is present, use it.

---

## 9. Conflict Detection

Two publishers attempting to claim the same service-instance name (different streams that happen to collide on the 8-char short ID + same `pub` slug — vanishingly rare) shall, per RFC 6762 §9, run probe-and-conflict resolution. The losing publisher renames itself by appending `-2`, `-3`, etc., to its `pub` slug. The Stream ID does not change.

This is purely an mDNS-layer concern; sailors never see it.

---

## 10. Open Items

- DNS-SD over the gateway's wider DNS for cross-segment discovery in v1.1.
- A "manual peer" mechanism for vessels with broken mDNS (currently no — fix the network).
- Whether to publish a vessel-wide `_pelorus._tcp` parent-service for all Pelorus subsystems (currently no; Stream and Core are independent).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
