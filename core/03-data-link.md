# Pelorus Core — Data Link Layer

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Trusted

CAN FD frame usage, 29-bit identifier layout, multi-frame transport, error handling. Dual-bus duplicate discard lives in [`08-redundancy.md`](./08-redundancy.md).

## 1. Frame Format

CAN FD per ISO 11898-1:2015 with the following field constraints:

| Field | Pelorus value |
|---|---|
| Identifier length | 29 bits (extended) only |
| BRS | Set (recessive) on all data frames |
| ESI | Per ISO 11898-1:2015 |
| FDF | Set (recessive) — CAN FD frame |
| RRS | Set (dominant) — no Remote Frames |
| DLC | 0–15 per ISO 11898-1 |

Pelorus Core nodes shall not transmit Classical CAN data frames except where ISO 11898-2:2016 requires Classical CAN encoding for Wake-Up Frames (see [`04-power.md`](./04-power.md)).

Data is push-only or request-via-DCID; receivers needing data either subscribe by listening for the relevant broadcast DCID or send a request message (DCID 0x0EA00 per J1939). Pelorus does not use Remote Transmission Request frames.

DLC-to-size mapping:

| DLC | Bytes |
|---|---|
| 0–8 | 0–8 |
| 9 | 12 |
| 10 | 16 |
| 11 | 20 |
| 12 | 24 |
| 13 | 32 |
| 14 | 48 |
| 15 | 64 |

Padding bytes in oversized frames shall be `0xFF` and shall be ignored on receive.

## 2. Identifier Structure

29-bit identifier per SAE J1939-21:

```
 28 27 26 | 25 | 24 | 23 .... 16 | 15 .... 8  | 7 ...... 0
   PRIO   |  R | DP |     PF     |     PS      |    SA
```

| Bits | Field | Description |
|---|---|---|
| 28–26 | PRIO | 0 (highest) to 7 (lowest) |
| 25 | R | Transmitted as 0; ignored on receive |
| 24 | DP | Data Page; v1.0 uses DP=0 only |
| 23–16 | PF | PDU Format — determines PDU type and DCID |
| 15–8 | PS | PDU Specific (destination address for PDU1, group extension for PDU2) |
| 7–0 | SA | Source Address per [`05-addressing.md`](./05-addressing.md) |

### 2.1 PDU1 vs PDU2

| PF range | PDU type | PS field | Addressing |
|---|---|---|---|
| 0x00 – 0xEF | PDU1 | Destination Address | Peer-to-peer |
| 0xF0 – 0xFF | PDU2 | Group Extension | Broadcast |

### 2.2 DCID Derivation

- **PDU1 (PF ≤ 0xEF):** `DCID = (R << 17) | (DP << 16) | (PF << 8)` — destination address is not part of the DCID
- **PDU2 (PF ≥ 0xF0):** `DCID = (R << 17) | (DP << 16) | (PF << 8) | PS`

Examples:

| Identifier | PRIO | PF | PS | SA | DCID | Type |
|---|---|---|---|---|---|---|
| `0x0CF80401` | 3 | 0xF8 | 0x04 | 0x01 | 0x0F804 | PDU2 broadcast |
| `0x18EE0102` | 6 | 0xEE | 0x01 | 0x02 | 0x0EE00 | PDU1 to addr 0x01 |
| `0x18FF8003` | 6 | 0xFF | 0x80 | 0x03 | 0x0FF80 | PDU2 (Pelorus WUF) |

### 2.3 Priority Allocation

| Priority | Class | Examples |
|---|---|---|
| 0 | Wake-up frames | WUF (DCID 0x0FF80) |
| 1 | Safety-critical real-time | Steering, autopilot commands, alarm assertions |
| 2 | Critical real-time | Heading, attitude, propulsion control |
| 3 | Real-time navigation | GNSS position, depth, speed, wind |
| 4 | Standard navigation | AIS targets, route data |
| 5 | Engine and machinery | Engine RPM, fuel, alternator |
| 6 | Network management, diagnostics | NM (0x0FF81), address claim, status |
| 7 | Bulk and non-critical | Logs, configuration, historical data |

Specific DCID priority assignments are recorded in [`07-dcid-registry.md`](./07-dcid-registry.md). Vendor-specific DCIDs respect the same priority bands.

## 3. Reserved Identifier Ranges

Reserved for protocol use; shall not be assigned to application data.

| DCID / range | Purpose | Document |
|---|---|---|
| 0x0EA00 | Request | §4.3 |
| 0x0EB00 | Transport Protocol – Data Transfer | §4 |
| 0x0EC00 | Transport Protocol – Connection Management | §4 |
| 0x0EE00 | Address Claimed | [`05-addressing.md`](./05-addressing.md) |
| 0x0FF80 | Pelorus Wake-Up Group Frame | [`04-power.md`](./04-power.md) |
| 0x0FF81 | Pelorus Network Management | [`04-power.md`](./04-power.md) |
| 0x0FF82 | Pelorus Bus Health | [`08-redundancy.md`](./08-redundancy.md) |
| 0x0FF83 | Pelorus Time Sync (optional) | [`08-redundancy.md`](./08-redundancy.md) |
| 0x0FF84 – 0x0FF8F | Reserved for future Pelorus protocol use | — |
| 0x0EF00 – 0x0EFFF | Proprietary A (per-vendor, peer-to-peer) | Vendor-managed |
| 0x0FF00 – 0x0FF7F | Proprietary B (per-vendor, broadcast) | Vendor-managed |

DCIDs 0x0FF80+ in the 0x0FF8x range are carved out of what would otherwise be Proprietary B space. Vendors shall not assign proprietary messages in these ranges.

## 4. Multi-Frame Messages

Messages exceeding 64 bytes use J1939-21 Transport Protocol, adapted for CAN FD:

| Payload size | Transport |
|---|---|
| ≤ 64 bytes | Single CAN FD frame |
| 65 – 1785 bytes | J1939 TP (BAM for broadcast, CMDT for peer-to-peer) |
| > 1785 bytes | Application must segment at higher layer or use Pelorus Stream |

### 4.1 BAM (Broadcast Announce Message)

For broadcast >64 bytes, the sender:

1. Transmits DCID 0x0EC00 BAM with the destination DCID, total length, and frame count.
2. Transmits DCID 0x0EB00 data frames sequentially, with sequence number in byte 0 and up to 63 bytes of payload in bytes 1–63.
3. Receivers reassemble by sequence number.

Frame spacing: minimum 50 ms between data frames per J1939-21. Receivers shall accept frames as fast as the bus delivers them.

### 4.2 Request Mechanism

A receiver requesting a specific DCID sends DCID 0x0EA00 with the requested DCID encoded in the data field (3 bytes, little-endian). The target node replies with the requested DCID if it is the producer. If multiple producers exist, the receiver disambiguates via DCID 0x0EE00 address-claim records.

### 4.3 No Fast Packet

Pelorus Core does not implement LMDE Fast Packet. Gateways translating from LMDE reassemble Fast Packet payloads into single CAN FD frames where possible, or fall back to J1939 TP.

## 5. Error Handling

CAN FD's CRC, ACK, error frames, and error counters per ISO 11898-1 operate unchanged.

### 5.1 Bus-Level Errors

The CAN controller handles CRC, form, stuff, ACK, and bit errors; transmitter retries automatically. A node observing repeated errors transitions to error-passive and eventually bus-off per ISO 11898-1.

### 5.2 Bus-Off Recovery

A node entering bus-off shall wait for 128 occurrences of 11 consecutive recessive bits per ISO 11898-1, transition to error-active, resume transmission, and log the bus-off event for diagnostics.

### 5.3 Application-Layer Delivery

The data link layer does not guarantee delivery of any single frame. Applications requiring guaranteed delivery implement acknowledgement at the application layer; Pelorus does not provide a generic ACK mechanism. For periodic data (GNSS, wind, depth), the natural retransmit cadence absorbs occasional loss. For commanded actions (autopilot setpoint, alarm acknowledgement), the originator expects and verifies a status response.

### 5.4 Transmit Retry Policy

Use the CAN controller's automatic retransmission. Manual retry suppression is permitted only for time-critical messages where stale data is worse than no data (e.g., heading updates older than 100 ms should be discarded rather than retransmitted).

### 5.5 Error Diagnostics

Each node exposes TX error count, RX error count, bus-off event count, and last bus-off timestamp via Bus Health DCID 0x0FF82 when in a dual-bus domain ([`08-redundancy.md`](./08-redundancy.md)). Additional diagnostic DCIDs may be registered in [`07-dcid-registry.md`](./07-dcid-registry.md).

## 6. Arbitration

Standard CSMA/CR — lowest numerical identifier wins. Implementations shall not back-pressure higher priorities with low-priority bulk traffic.

The full 29-bit identifier must be unique among messages active on the bus at any moment. Two nodes simultaneously transmitting the same identifier with different data is a CAN-level error indicating an addressing conflict (handled per [`05-addressing.md`](./05-addressing.md)).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
