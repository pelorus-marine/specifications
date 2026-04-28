# Pelorus Stream — Stream Registry

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **Stream Registry**: the distributed, eventually-consistent index of streams currently advertised on the vessel network. The registry is a **view**, not a database; no node is the canonical source of truth, and no Pelorus Stream feature *requires* the registry to function.

The draft design target (distributed, eventually-consistent, advisory registry) is summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary).

---

## 1. What the Registry Is

The registry is a per-node software construct that:

1. Listens to mDNS-SD ([`23-discovery.md`](./23-discovery.md)) for advertisements of `_pelorus-stream._udp`.
2. Subscribes opportunistically to `metadata-update` and `state-update` messages ([`21-stream-update.md`](./21-stream-update.md)) for streams it cares about.
3. Maintains a local table keyed by Stream ID, with the most recently observed metadata and state.
4. Exposes that table to local applications, UIs, and the State subsystem.

Every Stream-aware node may run a registry. Heads, gateways, and dedicated UI displays typically do; bare audio publishers may not.

---

## 2. Authority Model

There is **no authoritative registry**. Statements made by a publisher about its own streams take precedence over any registry's cache. If a registry's cached metadata for stream X disagrees with what the publisher of X is currently advertising, the publisher wins.

This aligns with Core gateway policy ([`core/09-gateway-specification.md`](../core/09-gateway-specification.md)): no mandatory single authority—no single point of failure, anywhere, in either Core or Stream.

A node that "knows" about a stream because its registry told it so shall verify by listening for the stream itself before subscribing. The registry is for *discovery and UI*, not for *resolution*.

---

## 3. Schema

The registry table is logically a map of `Stream ID → RegistryEntry`:

```cbor
RegistryEntry = {
  "id": h'<sid>',
  "metadata": <metadata map per 06>,
  "state": <state object per 20>,
  "transport": {
    "mode": "u"|"m"|"q",
    "addr": <ipv6 string>,
    "port": <u16>
  },
  "last_seen_ms": <epoch ms>,
  "source": "mdns"|"update"|"direct"
}
```

`source` indicates how this entry was acquired:

- `mdns`: from an mDNS advertisement (initial discovery).
- `update`: from a `state-update` or `metadata-update` message.
- `direct`: from a direct subscription handshake.

Entries are merged across sources; the latest data wins (per `last_seen_ms`).

---

## 4. Eviction

A registry shall evict an entry when:

- The mDNS advertisement has expired (TTL passed without refresh).
- A `closing` event was observed for that Stream ID.
- A user-configurable maximum age (default 1 hour) has elapsed without any update or refresh.

Eviction is local. Other registries on the network are not informed; each evicts independently based on its own observations.

A stream that recovers from a transient mDNS advertisement loss will be re-discovered and re-added by registries that had evicted it. The Stream ID remaining the same is enough to merge with any local UI bindings (favorites, last-volume settings, etc.).

---

## 5. Replication

There is **no replication** of registries in v1.0. Each registry constructs its own view from observations.

Two registries on the same vessel will, in steady state, have identical views (modulo mDNS query timing). They will not actively gossip their tables; if the network partitions and reconverges, each registry simply re-discovers what it has lost.

A future v1.1 may add an opt-in registry-gossip mechanism for very large vessels with many segments. v1.0 keeps it simple.

---

## 6. Capacity

A vessel-wide registry should expect on the order of:

- 5–20 audio streams (cabins, alarm zones, intercom lanes).
- 5–10 telemetry streams (battery, watermaker, weather sensor packages).
- 1–5 control streams.

Total ~30 active streams. Registries shall comfortably handle 1000 entries (history plus active) so the `mDNS` cache plus the live table plus a few minutes of recently-closed streams fits.

A registry approaching 10000 entries is misbehaving; emit a `registry-capacity` event at the warn level (vendor-name `pelorus.stream:registry-capacity` is reserved for this).

---

## 7. Application API

The registry exposes a read API to local applications via the reference library ([`27-lib.md`](./27-lib.md)):

| Operation | Description |
|---|---|
| `list()` | All current entries, sorted by `last_seen_ms`. |
| `get(sid)` | Lookup by Stream ID. |
| `find_by_type(t)` | Filter by stream type. |
| `find_by_tag(tag)` | Filter by metadata tag. |
| `subscribe_changes(callback)` | Notify on entry add/remove/update. |

A registry has **no** write API. Applications cannot inject entries; entries enter only via observation. This preserves the principle that publishers are authoritative for their own streams.

---

## 8. Privacy and Visibility

There is no access control on the registry in v1.0. Any node on the LAN can run a registry and see all streams. This is acceptable because:

- Vessel LAN is treated as trusted.
- mDNS advertisements are publicly broadcast anyway.
- Stream metadata is not sensitive.

If sensitive metadata exists, do not place it in `name` or `tags`; use a paired authenticated stream — not a Stream-protocol concern in v1.0.

---

## 9. Interaction with Pelorus Core

The registry is a **Stream-only** construct. It does not know about Core DCIDs, Core NAMEs, or Core devices. Cross-bus correlation (matching a Stream publisher to a Core node by NAME) is a State subsystem task that uses both subsystems' data.

---

## 10. Open Items

- A small write-through "favorites" mechanism for sailor UI — currently a UI concern, may move to registry library.
- Federation across multi-segment vessels in v1.1.
- A formal schema for vendor-extension fields in `RegistryEntry` (currently they live in `metadata.extra`).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
