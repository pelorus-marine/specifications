# Pelorus Stream — Navigation Services

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

Concrete service definitions for navigation-relevant traffic: radar video, radar control, S-100 chart distribution, high-rate navigation data, stream health, and the replication node that fans out one-to-many traffic.

These services run over QUIC per [`04-transport.md`](./04-transport.md), use the datagram header in [`04-transport.md §5`](./04-transport.md), and participate in dual-fabric redundancy per [`07-redundancy.md`](./07-redundancy.md). Discovery is via the service catalog in [`08-discovery-and-registry.md §1`](./08-discovery-and-registry.md).

AIS is **not** a Stream service. AIS targets are low-bandwidth instrument data carried on Pelorus Core via the standard NMEA 2000 PGNs registered in [`core/07-dcid-registry.md §2.1`](../core/07-dcid-registry.md). Where a Stream-side ECDIS needs AIS targets, the Core→Stream gateway bridges the relevant DCIDs.

## 1. Radar Video

### 1.1 Service

`_pelorus-radar-video._quic.local` — QUIC datagram. One spoke per datagram (or one fragment of a split spoke).

Transmitted on both Fabric A and Fabric B connections simultaneously (active-active per [`07-redundancy.md`](./07-redundancy.md)). Receiver deduplicates via the Stream DDT using the sequence number in the datagram header.

### 1.2 Spoke Packet Format

Following the 16-byte Pelorus Stream Datagram Header:

| Offset | Size | Field |
|---|---|---|
| 16 | 2 | Radar instance (matches the Core DCID radar instance and the metadata `instance`) |
| 18 | 2 | Antenna bearing (0.01° resolution, heading-relative, 0–35999) |
| 20 | 1 | Bearing validity (0x01 = encoder valid, 0x00 = estimated) |
| 21 | 1 | Range scale index (index into per-radar-model range scale table) |
| 22 | 4 | Range scale metres (actual range represented by sample N) |
| 26 | 2 | Sample count (number of amplitude samples in this spoke) |
| 28 | 8 | Spoke timestamp (nanoseconds since gPTP epoch) |
| 36 | N | Amplitude samples (8-bit unsigned, N = sample count) |

Maximum datagram size 1200 bytes. For radars with > 1184 samples per spoke, spokes are split across multiple datagrams using a continuation flag in the datagram header `Flags` byte (bit 1 = continuation; bit 2 = final fragment of a split spoke).

### 1.3 Multi-Radar Support

Each radar antenna is a separate instance (0, 1, 2, …). Instance number is declared in the mDNS TXT record (`instance=`) and carried in both the datagram header and the spoke packet header. ECDIS displays subscribe to specific instances or all instances independently.

## 2. Radar Control

### 2.1 Service

`_pelorus-radar-ctrl._quic.local` — QUIC reliable stream. Commands are CBOR-framed control messages per [`05-control-protocol.md`](./05-control-protocol.md) using kind `0x0030` (`radar-control`).

### 2.2 Command Set

| Command | Direction | Description |
|---|---|---|
| `SET_RANGE` | Display → Radar | Set range scale |
| `SET_GAIN` | Display → Radar | Set receiver gain |
| `SET_SEA_CLUTTER` | Display → Radar | Sea clutter reduction level |
| `SET_RAIN_CLUTTER` | Display → Radar | Rain clutter reduction level |
| `SET_TRANSMIT` | Display → Radar | Transmit / standby |
| `STATUS` | Radar → Display | Periodic status (range, gain, heading, rotation rate) |

Command body shape:

```cbor
{
  "cmd": "set-range" | "set-gain" | …,
  "instance": <u16>,
  "value": <u32 or map>
}
```

QUIC reliable-stream delivery guarantees the radar receives every command exactly once. The radar acknowledges effect via a `state-update` reflecting the new setting.

## 3. S-100 Chart Distribution

### 3.1 Service

`_pelorus-chart._quic.local` — QUIC reliable stream carrying HTTP/3 (RFC 9114).

Pelorus Stream is a **transport only** — it carries S-100 files as opaque blobs and has no knowledge of their content.

### 3.2 Protocol

- **HTTP/3** over QUIC. Native support for resumable transfers via range requests, content negotiation, and `If-Match`/`If-None-Match` for cache validation.
- **Integrity:** SHA-256 hash of each file in HTTP response headers (`Content-Digest: sha-256=…` per RFC 9530). Verified by the ECDIS before acceptance.
- **S-100 data protection** (S-100 Part 15: cell permits, encrypted ENCs) is validated by the ECDIS after receipt — not by Pelorus Stream.

### 3.3 Resumable Transfers

HTTP/3 range requests allow interrupted transfers to resume from the last received byte. Relevant for large S-102 bathymetric grids over slow satellite connections.

### 3.4 Service Discovery

ECDIS discovers chart distribution via `_pelorus-chart._quic.local` on both fabrics. The TXT record advertises the HTTP/3 endpoint base URL:

```
endpoint=https://chart-server.local./s100/
```

## 4. High-Rate Navigation Data

### 4.1 Service

`_pelorus-nav._quic.local` — QUIC datagram. Position, heading, attitude, and rate signals at rates higher than CAN FD comfortably carries (e.g. 50–100 Hz GNSS, IMU, gyro).

### 4.2 Datagram

Following the 16-byte datagram header, a CBOR map keyed by signal name. Recommended signal names map to `Vessel.*` paths from [`core/06-signal-catalog.md`](../core/06-signal-catalog.md):

```cbor
{
  "Vessel.CurrentLocation.Latitude": 47.6062345,
  "Vessel.CurrentLocation.Longitude": -122.3320708,
  "Vessel.Speed": 6.2,
  "Vessel.Heading.True": 047.5,
  "Vessel.Attitude.Pitch": 1.2,
  "Vessel.Attitude.Roll": -0.4
}
```

Signals not relevant to a sample may be omitted; receivers shall accept partial maps.

## 5. Replication Node

### 5.1 Purpose

For services with one publisher and many subscribers (radar video to multiple ECDIS displays; nav to multiple plotters), a Replication Node decouples the source from knowledge of how many displays exist:

```
[Radar Processor] ──QUIC──▶ [Replication Node] ──QUIC──▶ [ECDIS 1]
                                              ──QUIC──▶ [ECDIS 2]
                                              ──QUIC──▶ [ECDIS 3]
```

### 5.2 Service

`_pelorus-replicator._quic.local` — QUIC datagram. The Replication Node accepts upstream connections from a publisher and downstream connections from subscribers; it copies datagrams from upstream to all downstream connections without modification (preserving the datagram header so DDT and timestamps are end-to-end).

- The radar processor establishes one QUIC connection to the Replication Node per fabric.
- Displays establish QUIC connections to the Replication Node (not directly to the radar).
- Adding a display requires no change to radar processor configuration.
- Replication Node may run as a software process on the Pelorus Stream Hub.

## 6. Stream Health Service

### 6.1 Service

`_pelorus-stream-health._quic.local` — QUIC datagram. Every Class D Stream node periodically transmits a health datagram on both fabrics independently every 2 seconds.

### 6.2 Datagram

Following the 16-byte datagram header:

| Offset | Size | Field |
|---|---|---|
| 16 | 1 | Node class (S=0, D=1) |
| 17 | 1 | Fabric A state (DUAL_ACTIVE=0, DEGRADED=1, RECOVERING=2, FAILED=3) |
| 18 | 1 | Fabric B state (same encoding) |
| 19 | 2 | Fabric A duplicates discarded since power-on (saturating u16) |
| 21 | 2 | Fabric B duplicates discarded since power-on (saturating u16) |
| 23 | 2 | Sequence gaps detected on Fabric A since power-on (saturating u16) |
| 25 | 2 | Sequence gaps detected on Fabric B since power-on (saturating u16) |
| 27 | 1 | PoE status (bit 0: Port A powered, bit 1: Port B powered) |
| 28 | 2 | Port A PoE power draw in 0.1 W units |
| 30 | 2 | Port B PoE power draw in 0.1 W units |

Class S nodes transmit on their attached fabric only with the corresponding peer-fabric counters set to 0xFFFF (not applicable).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
