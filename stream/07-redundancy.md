# Pelorus Stream — Dual-Fabric Redundancy

Dual-fabric state machine, datagram deduplication, Class S vs Class D nodes, and the Stream RedBox. Physical installation rules for the dual fabric live in [`03-physical.md §4`](./03-physical.md). The transport-layer connection model is in [`04-transport.md §3`](./04-transport.md).

## 1. Failure Modes Addressed

| Failure | Detection |
|---|---|
| Cable break | QUIC connection loss on affected fabric |
| Switch failure | All QUIC connections via that fabric drop simultaneously |
| Switch port failure | Single node loses one fabric connection |
| NIC failure | Node loses one fabric connection |
| Power rail failure | All connections via affected fabric drop simultaneously |
| Partial failure / high loss | QUIC detects via ACK gaps and RTT increase |
| Byzantine / corrupt data | TLS 1.3 MAC failure — QUIC discards frame |
| Temporary congestion | QUIC congestion controller backs off |

All failure modes manifest as QUIC-observable events. The redundancy mechanism requires no knowledge of physical cause.

## 2. Node Classes

| Class | Connectivity |
|---|---|
| **Class D** | Dual transceivers; attaches to both Fabric A and Fabric B. Standard for safety-relevant services. |
| **Class S** | Single transceiver; attaches to one fabric only. Permitted for non-safety auxiliary services. |

Class S nodes shall not be the sole source of safety-relevant data (radar video, position). Safety-relevant data sources shall be Class D.

## 3. State Machine

Each pair of QUIC connections (A+B) to a given peer operates as a three-state machine:

```
              ┌─────────────────┐
              │   DUAL_ACTIVE   │◄──────────────────────┐
              │ TX on both      │                        │
              │ RX from first   │                        │ Verification
              └────────┬────────┘                        │ passes (5s clean)
                       │                                 │
              One connection fails                       │
                       │                                 │
                       ▼                                 │
              ┌─────────────────┐              ┌─────────┴───────┐
              │    DEGRADED     │              │   RECOVERING    │
              │ TX on survivor  │              │ Monitor only    │
              │ RX from survivor│              │ Do not promote  │
              └────────┬────────┘              └─────────────────┘
                       │                                 ▲
              Failed connection                          │
              re-establishes                             │
                       │                                 │
                       └─────────────────────────────────┘
```

### 3.1 DUAL_ACTIVE

- **Unreliable (datagrams):** transmit identical datagrams on both connections simultaneously. Receiver accepts first arrival, discards duplicate via DDT (§4).
- **Reliable (streams):** transmit on Fabric A only. Fabric B connection is hot-standby.

### 3.2 DEGRADED

- All data transmits on the surviving connection only.
- Reliable streams: QUIC handles retransmission of unacknowledged data via connection migration. Application sees a brief stall (~1 RTT) then continuity. No data loss.
- Alert generated: a fabric-failure event is published on the Stream Health service ([`10-services-nav.md`](./10-services-nav.md)).

### 3.3 RECOVERING

- Failed connection has re-established at the QUIC layer.
- Do **not** immediately promote to DUAL_ACTIVE.
- Monitor for `STREAM_VERIFY_PERIOD` (**5 seconds**) of clean traffic: no sequence gaps, no TLS errors, RTT within 2× baseline.
- If verification passes → promote to DUAL_ACTIVE.
- If verification fails → drop connection, enter backoff, retry.

This prevents a flapping fabric from causing worse behaviour than a cleanly failed one.

## 4. Datagram Deduplication Table (DDT)

Each receiver maintains:

```rust
struct StreamDDTEntry {
    source_node:    NodeId,       // Pelorus Stream node identifier
    service_type:   ServiceType,  // From datagram header
    instance:       u16,          // From datagram header
    last_sequence:  u16,          // Last accepted sequence number
    last_fabric:    FabricId,     // Fabric A or B
    last_seen:      Timestamp,    // Local monotonic time
}
```

The datagram header fields (service type, instance, sequence, fabric ID) are defined in [`04-transport.md §5`](./04-transport.md).

| Parameter | Value |
|---|---|
| **Discard window** | **10 ms** — wider than necessary (Ethernet switch latency between two ports on a vessel is microseconds; 10 ms is ample margin without risking suppression of legitimate new datagrams) |
| **Forget timeout** | **60 s** — DDT entry removed if no datagrams received from `(source, service, instance)` for this duration |

### 4.1 Algorithm

For each received datagram from `(source, service, instance)` with sequence `N` on fabric `F`:

1. If no DDT entry: **accept**; create entry with last_sequence = N, last_fabric = F.
2. If entry exists:
   - If `N == entry.last_sequence` and `(now − entry.last_seen) < DISCARD_WINDOW`: **discard** as duplicate.
   - If `N == entry.last_sequence + 1` (mod 2¹⁶): **accept**; advance entry.
   - If `N` is far ahead of `entry.last_sequence`: **accept**; record sequence gap.
   - If `N` is far below `entry.last_sequence`: see §4.2.

### 4.2 Node Reboot Detection

When a node reboots, its sequence numbers reset to 0. A receiver sees a sequence number far below the last accepted value:

- If `last_seen` is older than `FORGET_TIMEOUT`: **accept** as a fresh start.
- If `last_seen` is recent: flag as unexpected reset — may indicate a node fault. Log a `sequence-reset` event ([`11-events-and-errors.md`](./11-events-and-errors.md)) and accept the new sequence.

## 5. Stream RedBox

A Stream RedBox proxies for Class S nodes: it receives their transmissions on one fabric and re-transmits on both. This allows legacy or cost-constrained devices to participate without dual-NIC hardware.

The RedBox function may be implemented as a standalone node or co-located with the Pelorus Stream Hub.

A RedBox shall:

- Maintain DUAL_ACTIVE QUIC connections to upstream peers.
- For each downstream Class S node, transmit identical datagrams on both fabrics with the same sequence numbers as the originator.
- Apply DDT on its own ingress to avoid forwarding duplicates back.
- Surface RedBox health on the Stream Health service ([`10-services-nav.md`](./10-services-nav.md)).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
