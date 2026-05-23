# Pelorus Stream — Discovery and Registry

mDNS-SD service catalog and the per-node observation registry. Discovery happens on each fabric independently; subscribers establish dual QUIC connections to publishers per [`04-transport.md §3`](./04-transport.md).

## 1. Service Catalog

Each Pelorus Stream service has its own mDNS service type. Service catalog (v1.0):

| Service type | Protocol | Description |
| --- | --- | --- |
| `_pelorus-radar-video._quic.local` | QUIC datagram | Raw radar spoke data |
| `_pelorus-radar-ctrl._quic.local` | QUIC reliable stream | Radar control commands |
| `_pelorus-chart._quic.local` | QUIC reliable stream (HTTP/3) | S-100 chart file distribution |
| `_pelorus-nav._quic.local` | QUIC datagram | High-rate navigation data |
| `_pelorus-stream-health._quic.local` | QUIC datagram | Stream node health reporting |
| `_pelorus-replicator._quic.local` | QUIC datagram | Replication node fan-out |
| `_pelorus-timesync._ptp.local` | IEEE 802.1AS | Time synchronisation (not QUIC) — see [`09-time-sync.md`](./09-time-sync.md) |

The browser-domain is `local.` per RFC 6762; v1.0 is link-local only.

## 2. Per-Fabric Advertisement

Each service is advertised on **both fabrics independently**. A consumer browsing on Fabric A and Fabric B will see the same logical service twice (once per fabric) and shall establish dual QUIC connections accordingly. The Stream ID in the TXT record (§3) is what allows the consumer to correlate the two advertisements as the same service instance.

## 3. Service Instance Naming

Each stream gets one mDNS service instance per its Stream ID:

```text
<stream-id-short>.<pub>.<service-type>.local.
```

- `<stream-id-short>` is the first 8 hex chars of the Stream ID
- `<pub>` is the publisher's `pub` metadata field, slug-safe (alphanumeric + hyphens; other characters replaced with `-`)

Example:

```text
018f3c2b.bow-radar._pelorus-radar-video._quic.local.
```

## 4. SRV Record

```text
SRV: 0 0 <port> <hostname>.local.
```

| Field | Value |
| --- | --- |
| `priority` | 0 |
| `weight` | 0 |
| `port` | The publisher's QUIC endpoint port for this service |
| `hostname` | Publisher's mDNS hostname |

## 5. TXT Record

The TXT record carries the stream's discovery metadata, serialised as classic mDNS TXT key=value strings. Required keys:

| TXT key | Source |
| --- | --- |
| `id=<full-uuid>` | Stream ID |
| `t=<type>` | Type code from [`02-data-model.md §2`](./02-data-model.md) |
| `pub=<pub>` | Publisher identifier |
| `class=<S\|D>` | Node class |
| `fabric=<A\|B>` | Which fabric this advertisement is on |
| `v=<u8>` | Protocol version |
| `caps=<hex>` | Capability bit-vector |

Optional keys map directly from metadata: `name`, `prio`, `lang`, `format`, `profile`, `sr`, `ch`, `cad`, `instance`, `vendor`, `tags`, `since`, `vss`.

Each TXT string is at most 255 bytes per RFC 6763 §6. Cumulative TXT data per RR shall not exceed 1200 bytes. `tags` is encoded as comma-separated values within a single TXT string.

## 6. PTR Record

Per RFC 6763 §4.1:

```text
PTR: <service-type>.local. → <instance>.<service-type>.local.
```

## 7. TTLs

| Record | TTL |
| --- | --- |
| PTR | 4500 (75 min) |
| SRV / TXT | 120 (2 min) |
| AAAA | 120 |

PTR has a long TTL because service-discovery responders cache it; SRV/TXT are short because they may change as metadata updates arrive.

When a stream closes, the publisher emits **goodbye packets** (TTL=0 unsolicited responses) for PTR and SRV records to evict caches promptly.

## 8. Browse Pattern

A typical subscriber browses one or more service types:

```text
_pelorus-radar-video._quic.local.   -> PTR list
   for each instance:
     resolve SRV  -> hostname + port
     resolve TXT  -> metadata
     resolve AAAA -> IPv6 address
     for each fabric advertisement:
       open QUIC connection to (hostname, port) on that fabric
```

A registry node browses every service type in §1 to capture all streams.

## 9. Existing mDNS Plant

Pelorus Stream nodes shall use the standard onboard mDNS responder where available (Avahi on Linux, `mdnsResponder` on Apple, `mdnsd` on FreeBSD). Fall back to an embedded responder on `no_std` platforms — the `pelorus-mdns` reference crate.

Pelorus Stream shall not re-implement mDNS port handling. Multiple responders on one host produce mDNS conflicts.

## 10. Conflict Detection

Two publishers attempting to claim the same service-instance name (different streams that happen to collide on the 8-char short ID and same `pub` slug — vanishingly rare) shall, per RFC 6762 §9, run probe-and-conflict resolution. The losing publisher renames itself by appending `-2`, `-3`, etc., to its `pub` slug. The Stream ID does not change.

## 11. Stream Registry

A per-node software construct that:

1. Listens to mDNS-SD for advertisements of every Pelorus service type.
2. Subscribes opportunistically to `metadata-update` and `state-update` messages ([`11-events-and-errors.md`](./11-events-and-errors.md)) for streams it cares about.
3. Maintains a local table keyed by Stream ID with the most recently observed metadata and state.
4. Exposes that table to local applications, UIs, and the Pelorus State subsystem.

Heads, gateways, and dedicated UI displays typically run a registry; bare publishers (e.g. single-purpose embedded sensors) may not.

### 11.1 Authority

There is **no authoritative registry**. Statements made by a publisher about its own streams take precedence over any registry's cache. If a registry's cached metadata for stream X disagrees with what the publisher of X is currently advertising, the publisher wins.

A node that "knows" about a stream because its registry told it so shall verify by listening for the stream itself before subscribing. The registry is for discovery and UI, not for resolution.

### 11.2 Schema

```cbor
RegistryEntry = {
  "id": h'<sid>',
  "metadata": <metadata map per 02-data-model>,
  "state": <state object per 06-session-and-state>,
  "service_type": "_pelorus-...",
  "addresses": {
    "fabric_a": <ipv6 string or absent>,
    "fabric_b": <ipv6 string or absent>
  },
  "port": <u16>,
  "last_seen_ms": <epoch ms>,
  "source": "mdns"|"update"|"direct"
}
```

`source` indicates how this entry was acquired:

- `mdns` — initial discovery
- `update` — `state-update` or `metadata-update`
- `direct` — direct subscription handshake

Entries are merged across sources; the latest data wins (per `last_seen_ms`).

### 11.3 Eviction

A registry shall evict an entry when:

- The mDNS advertisement has expired (TTL passed without refresh).
- A `closing` event was observed for that Stream ID.
- A user-configurable maximum age (default **1 hour**) has elapsed without any update or refresh.

Eviction is local. Other registries on the network are not informed; each evicts independently.

### 11.4 No Replication

There is no replication of registries in v1.0. Each registry constructs its own view from observations. Two registries on the same vessel will, in steady state, have identical views (modulo mDNS query timing); they do not actively gossip their tables.

### 11.5 Application API

The registry exposes a read-only API to local applications via the reference library ([`12-lib.md`](./12-lib.md)):

| Operation | Description |
| --- | --- |
| `list()` | All current entries, sorted by `last_seen_ms` |
| `get(sid)` | Lookup by Stream ID |
| `find_by_service(type)` | Filter by service type |
| `find_by_tag(tag)` | Filter by metadata tag |
| `subscribe_changes(callback)` | Notify on entry add/remove/update |

A registry has no write API. Applications cannot inject entries; entries enter only via observation. This preserves the principle that publishers are authoritative for their own streams.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
