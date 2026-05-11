# Pelorus Stream — Transport

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

QUIC over IPv6 link-local on a dual-fabric Ethernet plant. Datagram framing for unreliable streams; reliable QUIC streams for everything else. Dual-fabric state machine, deduplication, and recovery are in [`07-redundancy.md`](./07-redundancy.md). Service-level use of these primitives is in [`10-services-nav.md`](./10-services-nav.md).

## 1. QUIC as Single Transport

| Capability | What Stream uses it for |
|---|---|
| TLS 1.3 mandatory | Built-in security; no custom security header |
| Multiplexed streams | Independent radar, chart, nav streams; head-of-line blocking eliminated |
| Reliable streams | Chart files, control commands |
| Unreliable datagrams (RFC 9221) | Radar spokes, high-rate nav, telemetry, health |
| Connection migration | Fabric failover preserves the connection ID across IP address change |
| 0-RTT reconnection | Sub-millisecond reconnection for known peers after transient interruption |
| Pluggable congestion control | BBR for low-latency radar, CUBIC for chart throughput |
| Userspace implementation | No kernel modifications; runs on embedded Linux and bare metal |

Pelorus Stream uses **QUIC v1 (RFC 9000)**. QUIC v2 (RFC 9369) is permitted for negotiation; v1 support is mandatory.

ALPN identifier: `pelorus-stream/0.2`.

There is no plain-UDP control plane and no multicast in v1.0. Fan-out (one publisher, many subscribers) is provided by **replication nodes** ([`10-services-nav.md`](./10-services-nav.md)), not by IP multicast.

## 2. IPv6 Addressing

| Address class | Source |
|---|---|
| IPv6 link-local (`fe80::/10`) | RFC 4862 SLAAC; modified EUI-64 from interface MAC |
| IPv6 ULA (`fd00::/8`) | Optional, vessel-wide static prefix; configured by gateway |

A v1.0 Stream node shall auto-configure a link-local address on every interface. ULA is optional; if present, ULA addresses may also be used for unicast streams whose subscribers are on the vessel-wide segment beyond a single link.

Global IPv6 addressing is out of scope for v1.0. Cross-Internet streaming is not specified here.

## 3. Connection Model

### 3.1 Dual Connections per Peer

Every Class D Stream node maintains **two simultaneous QUIC connections** to each peer:

- **Connection A** — via Fabric A
- **Connection B** — via Fabric B

These are independent QUIC connections, each with its own TLS 1.3 session, congestion controller, and flow control state. They are not aware of each other at the QUIC layer; redundancy management is in the Stream Redundancy Manager above QUIC ([`07-redundancy.md`](./07-redundancy.md)).

Class S nodes maintain one QUIC connection to each peer over their single attached fabric.

### 3.2 TLS 1.3 Profile

- TLS 1.3 negotiated as part of the QUIC handshake
- Self-signed certificates whose subject is the publisher's link-local address
- Trust evaluation is policy of the subscriber; "accept any onboard certificate" is permitted for v1.0

### 3.3 Single QUIC Connection per `(publisher, subscriber)` Pair

Use a single QUIC connection per `(publisher, subscriber)` pair regardless of how many Stream-streams are multiplexed over it. Multiplexing is at the QUIC stream and datagram layer.

## 4. Reliable Streams vs Datagrams

| Data type | QUIC mechanism | Rationale |
|---|---|---|
| Radar spoke video | QUIC datagram (RFC 9221) | No retransmission — stale spoke is worse than gap |
| High-rate position/heading | QUIC datagram | Old position data is actively harmful |
| Telemetry samples | QUIC datagram | Periodic; stale not interesting |
| Stream health | QUIC datagram | Periodic; stale not interesting |
| S-100 chart files | QUIC reliable stream | Integrity critical; every byte must arrive |
| Firmware updates | QUIC reliable stream | As above |
| Radar control commands | QUIC reliable stream | Commands shall not be lost or reordered |
| Configuration | QUIC reliable stream | As above |
| Subscribe / state-update / event | QUIC reliable stream | Control plane survives loss |

## 5. Pelorus Stream Datagram Header

All Pelorus Stream QUIC datagrams carry a 16-byte header before the service payload.

| Offset | Size | Field |
|---|---|---|
| 0 | 2 | Service type (registry in [`08-discovery-and-registry.md`](./08-discovery-and-registry.md)) |
| 2 | 2 | Instance (e.g. radar antenna 0/1) |
| 4 | 2 | Sequence number (16-bit rolling counter, per source per service per instance) |
| 6 | 1 | Fabric ID (`0x00` = Fabric A, `0x01` = Fabric B) |
| 7 | 1 | Flags (bit 0: time sync valid; bits 1–7: reserved, transmit `0`, ignore on receive) |
| 8 | 8 | Timestamp (nanoseconds since gPTP epoch, IEEE 802.1AS — see [`09-time-sync.md`](./09-time-sync.md)) |
| 16+ | N | Service payload |

**Sequence number scope.** Per `(source node, service type, instance)` tuple. A radar node with two antennas has independent sequence counters for instance 0 and instance 1. A node reboot resets sequence counters; receivers detect this via the RECOVERING state transition ([`07-redundancy.md`](./07-redundancy.md)).

**16-bit sequence numbers are mandatory.** 8-bit counters wrap in under 30 seconds at typical datagram rates — insufficient for the RECOVERING state verification period.

### 5.1 Maximum Datagram Size

1200 bytes (QUIC recommended maximum to avoid IP fragmentation). Services with payloads exceeding this (e.g. radar spokes wider than 1184 samples) split across multiple datagrams using a continuation flag in the per-service payload header — defined in the service document.

## 6. Reliable Stream Framing

Reliable services run over standard QUIC bidirectional streams. Two reliable-stream profiles:

- **HTTP/3 (RFC 9114)** for resource-style transfers — chart files, firmware updates, configuration GET/PUT. Native support for resumable transfers via range requests.
- **CBOR control framing** for command-style traffic — radar control, subscribe / unsubscribe / event / state-update messages. Detail in [`05-control-protocol.md`](./05-control-protocol.md).

QUIC handles ordering, retransmission, and flow control. No application-layer sequence number is required on reliable streams.

## 7. Interface Selection

Most Stream nodes have one Fabric A interface and one Fabric B interface (Class D), or one interface only (Class S). Stream sockets bind to the configured Pelorus Stream interfaces. A node shall not act as an IP router between fabrics — multi-fabric connectivity is the dual-connection model in §3, not packet forwarding.

## 8. Source Address Stability

A publisher's link-local address is derived from its MAC address. Hardware replacement changes the MAC; the publisher's QUIC connection identity changes; subscribers reconnect via mDNS-SD ([`08-discovery-and-registry.md`](./08-discovery-and-registry.md)).

ULA addresses, if used, are stable across MAC changes and provide a vessel-wide identity. ULA usage is policy of the gateway, not Stream-mandated.

## 9. Loopback and Self-Subscribe

A publisher subscribing to its own stream is permitted. Implementations should loop traffic locally without round-tripping through the network when possible. This pattern is useful for the Pelorus State subsystem to observe locally what is also being published over the network.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
