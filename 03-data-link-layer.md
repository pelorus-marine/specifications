# Pelorus Core — Data Link Layer Specification

**Version:** 0.1 Draft
**Last Updated:** April 26, 2026
**Status:** Pre-specification

---

## About This Document

This document specifies the data link layer for Pelorus Core: how CAN FD frames are formatted, how identifiers carry addressing and message-type information, how multi-frame messages are constructed when payloads exceed 64 bytes, and how bus errors are handled. It sits between [02-physical-layer.md](./02-physical-layer.md) (the wire) and [05-addressing.md](./05-addressing.md), [06-signal-catalog.md](./06-signal-catalog.md), [07-pgn-registry.md](./07-pgn-registry.md) (the application data).

**Design philosophy:** Pelorus Core inherits J1939's identifier and PGN model with two changes — the data phase runs at 500 kbit/s, and the payload extends to 64 bytes. Everything else (priority, PDU1/PDU2 distinction, source-address claiming) is preserved so that gateway translation to NMEA 2000 stays mechanical.

---

## 1. Scope

This document defines:

- The CAN FD frame format Pelorus Core uses
- The 29-bit identifier layout and PGN derivation
- Addressing modes (broadcast PDU2, peer-to-peer PDU1)
- Multi-frame message construction for payloads exceeding 64 bytes
- Error handling at the data link layer
- Reserved identifier ranges

This document does **not** define:

- Specific PGN assignments (see [07-pgn-registry.md](./07-pgn-registry.md))
- Source address values or claiming protocol (see [05-addressing.md](./05-addressing.md))
- Signal definitions or units (see [06-signal-catalog.md](./06-signal-catalog.md))
- Wake-up frame matching (see [04-power-management.md §5](./04-power-management.md))

---

## 2. Frame Format

Pelorus Core uses CAN FD frames per ISO 11898-1:2015 with the following constraints:

| Field | Pelorus Core Value |
|---|---|
| Identifier length | 29 bits (extended) only |
| BRS (Bit Rate Switch) | Set (`recessive`) on all data frames |
| ESI (Error State Indicator) | Per ISO 11898-1:2015 |
| FDF (FD Format) | Set (`recessive`) — CAN FD frame |
| RRS (Remote Request Substitution) | Set (`dominant`) — Pelorus does not use Remote Frames |
| DLC | 0–15 per ISO 11898-1 (0–8 bytes for DLC ≤ 8, 12/16/20/24/32/48/64 for DLC > 8) |

### 2.1 No Classical CAN Data Frames

Pelorus Core nodes shall not transmit Classical CAN data frames except where ISO 11898-2:2016 requires Classical CAN encoding for Wake-Up Frames (see [04-power-management.md §5](./04-power-management.md)). All application traffic uses CAN FD format.

This is a deliberate divergence from coexistence-with-NMEA 2000 thinking. NMEA 2000 traffic is on a separate physical bus per [01-overview.md §4](./01-overview.md); Pelorus Core does not need to emit Classical CAN data frames for any compatibility purpose.

### 2.2 No Remote Frames

Pelorus Core does not use Remote Transmission Request (RTR) frames. Data is push-only or request-via-PGN. Receivers needing data either:

- Subscribe by listening for the relevant broadcast PGN, or
- Send a request PGN (PGN 0xEA00 per J1939, ratification pending in [07-pgn-registry.md](./07-pgn-registry.md))

### 2.3 Frame Size Selection

Senders should fit messages into the smallest practical CAN FD frame size. The DLC-to-size mapping (per ISO 11898-1):

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

Padding bytes in oversized frames shall be transmitted as `0xFF` and shall be ignored on receive.

---

## 3. Identifier Structure

Pelorus Core uses a 29-bit identifier laid out per SAE J1939-21:

```
 28 27 26 | 25 | 24 | 23 .... 16 | 15 .... 8  | 7 ...... 0
   PRIO   |  R | DP |     PF     |     PS      |    SA
```

| Bits | Field | Description |
|---|---|---|
| 28–26 | Priority (PRIO) | 0 (highest) to 7 (lowest) |
| 25 | Reserved (R) | Transmitted as 0; ignored on receive |
| 24 | Data Page (DP) | Selects PGN page; v1.0 of Pelorus uses DP=0 only |
| 23–16 | PDU Format (PF) | Determines PDU type and PGN |
| 15–8 | PDU Specific (PS) | Destination address (PDU1) or group extension (PDU2) |
| 7–0 | Source Address (SA) | Sender's claimed address per [05-addressing.md](./05-addressing.md) |

### 3.1 PDU1 vs PDU2

| PF Range | PDU Type | PS Field Means | Addressing |
|---|---|---|---|
| 0x00 – 0xEF | PDU1 | Destination Address | Peer-to-peer |
| 0xF0 – 0xFF | PDU2 | Group Extension | Broadcast |

This is the J1939 convention, preserved unchanged.

### 3.2 PGN Derivation

The Parameter Group Number is derived from R, DP, PF, and (for PDU2 only) PS:

- **PDU1 (PF ≤ 0xEF):** `PGN = (R << 17) | (DP << 16) | (PF << 8)` — the destination address is **not** part of the PGN
- **PDU2 (PF ≥ 0xF0):** `PGN = (R << 17) | (DP << 16) | (PF << 8) | PS`

Examples:

| Identifier (29-bit) | PRIO | PF | PS | SA | PGN | Type |
|---|---|---|---|---|---|---|
| `0x0CF80401` | 3 | 0xF8 | 0x04 | 0x01 | 0x0F804 | PDU2 broadcast |
| `0x18EE0102` | 6 | 0xEE | 0x01 | 0x02 | 0x0EE00 | PDU1 to addr 0x01 |
| `0x18FF8003` | 6 | 0xFF | 0x80 | 0x03 | 0x0FF80 | PDU2 (Pelorus WUF) |

### 3.3 Priority Allocation

Priority is the most-significant 3 bits of the identifier and dominates bus arbitration. Pelorus assigns priority by message class:

| Priority | Class | Examples |
|---|---|---|
| 0 | Wake-up frames | WUF (PGN 0x0FF80) |
| 1 | Safety-critical real-time | Steering, autopilot commands, alarm assertions |
| 2 | Critical real-time | Heading, attitude, propulsion control |
| 3 | Real-time navigation | GNSS position, depth, speed, wind |
| 4 | Standard navigation | AIS targets, route data |
| 5 | Engine and machinery | Engine RPM, fuel, alternator |
| 6 | Network management, diagnostics | NM (PGN 0x0FF81), address claim, status |
| 7 | Bulk and non-critical | Logs, configuration, historical data |

These are guidelines. Specific PGN priority assignments are recorded in [07-pgn-registry.md](./07-pgn-registry.md). Vendor-specific or proprietary PGNs must respect the same priority bands.

---

## 4. Reserved Identifier Ranges

The following PGNs and ranges are reserved for Pelorus Core protocol use and shall not be assigned to application data.

| PGN / Range | Purpose | Document |
|---|---|---|
| 0x0EE00 | Address Claimed | [05-addressing.md](./05-addressing.md) |
| 0x0EA00 | Request | This document §5.3 |
| 0x0EB00 | Transport Protocol – Data Transfer | This document §5 |
| 0x0EC00 | Transport Protocol – Connection Mgmt | This document §5 |
| 0x0FF80 | Pelorus Wake-Up Group Frame | [04-power-management.md §7.1](./04-power-management.md) |
| 0x0FF81 | Pelorus Network Management | [04-power-management.md §7.3](./04-power-management.md) |
| 0x0FF82 – 0x0FF8F | Reserved for future Pelorus protocol use | — |
| 0x0EF00 – 0x0EFFF | Proprietary A (per-vendor, peer-to-peer) | Vendor-managed |
| 0x0FF00 – 0x0FF7F | Proprietary B (per-vendor, broadcast) | Vendor-managed |

PGN 0x0FF80 and onward in the 0x0FF8x range are **carved out** of what would otherwise be Proprietary B space. Vendors shall not assign proprietary messages in these ranges.

---

## 5. Multi-Frame Messages

CAN FD frames carry up to 64 bytes. Messages larger than 64 bytes use a transport protocol modelled on J1939-21 TP, adapted for CAN FD:

### 5.1 When to Use Transport Protocol

| Payload Size | Transport |
|---|---|
| ≤ 64 bytes | Single CAN FD frame |
| 65 – 1785 bytes | J1939 TP (BAM for broadcast, CMDT for peer-to-peer) |
| > 1785 bytes | Application must segment at higher layer or use Pelorus Stream |

### 5.2 BAM (Broadcast Announce Message)

For broadcast messages exceeding 64 bytes, the sender:

1. Transmits PGN 0x0EC00 BAM with the destination PGN, total length, and frame count
2. Transmits PGN 0x0EB00 data frames sequentially, with sequence number in byte 0 and up to 63 bytes of payload in bytes 1–63
3. Receivers reassemble by sequence number

Frame spacing: minimum 50 ms between data frames per J1939-21. Receivers shall accept frames as fast as the bus delivers them.

### 5.3 Request Mechanism

A receiver requesting a specific PGN sends PGN 0x0EA00 with the requested PGN encoded in the data field (3 bytes, little-endian). The target node replies with the requested PGN if it is the producer. If multiple producers exist (instance handling), the receiver may need to disambiguate via PGN 0x0EE00 address-claim records.

### 5.4 No Fast Packet

Pelorus Core does not implement NMEA 2000 Fast Packet. Senders shall not use Fast Packet framing. Gateways translating from NMEA 2000 to Pelorus Core must reassemble Fast Packet payloads into single CAN FD frames where possible, or fall back to J1939 TP for payloads exceeding 64 bytes.

---

## 6. Error Handling

CAN FD's error handling mechanisms (CRC, ACK, error frames, error counters per ISO 11898-1) operate unchanged on Pelorus Core. Application-level error handling is layered on top.

### 6.1 Bus-Level Errors

The CAN controller handles:

- **CRC errors** — receiver detects payload corruption; transmitter retries automatically
- **Form errors** — fixed-format bits violated; transmitter retries
- **Stuff errors** — bit-stuffing rule violated; transmitter retries
- **ACK errors** — no receiver acknowledged; transmitter retries
- **Bit errors** — transmitted vs. monitored bit mismatch; transmitter retries

A node observing repeated errors transitions to error-passive and eventually bus-off per ISO 11898-1.

### 6.2 Bus-Off Recovery

A node entering bus-off shall:

1. Wait for 128 occurrences of 11 consecutive recessive bits (per ISO 11898-1)
2. Transition to error-active and resume transmission
3. Log the bus-off event for diagnostics

Pelorus does not specify additional bus-off recovery procedures. Implementations may add backoff or alarm signaling at the application layer.

### 6.3 Application-Layer Error Handling

The data link layer does not guarantee delivery of any single frame. Applications requiring guaranteed delivery must implement acknowledgement at the application layer. Pelorus does not provide a generic ACK mechanism.

For periodic data (GNSS position, wind speed, depth), the natural retransmit cadence absorbs occasional frame loss. For commanded actions (autopilot setpoint changes, alarm acknowledgement), the originator should expect and verify a status response.

### 6.4 Transmit Retry Policy

Pelorus Core nodes shall use the CAN controller's automatic retransmission. Manual retry suppression is permitted only for time-critical messages where stale data is worse than no data (e.g., heading updates older than 100 ms should be discarded rather than retransmitted).

### 6.5 Error Counters and Diagnostics

Each Pelorus Core node shall expose, via a diagnostic PGN (assignment in [07-pgn-registry.md](./07-pgn-registry.md)):

- TX error count
- RX error count
- Bus-off event count
- Last bus-off timestamp

This makes the network sailor-debuggable as required by [01-overview.md §6](./01-overview.md).

---

## 7. Bus Arbitration

CAN FD arbitration uses the standard CSMA/CR (Carrier Sense Multiple Access with Collision Resolution by bitwise priority). The lowest numerical identifier wins arbitration.

### 7.1 Arbitration Implications for Pelorus

- Priority 0 messages (WUF) preempt all other traffic
- Priority 1–2 messages preempt routine sensor traffic
- A node with low-priority bulk traffic must not starve higher-priority messages — implementations shall not back-pressure higher priorities

### 7.2 Identifier Uniqueness

The full 29-bit identifier (priority + reserved + DP + PF + PS + SA) must be unique among messages active on the bus at any moment. Two nodes simultaneously transmitting the same identifier with different data is a CAN-level error and indicates an addressing conflict (handled per [05-addressing.md](./05-addressing.md)).

---

## 8. Open Items

The following are unresolved and tracked in [TODO.md](../TODO.md):

- Final priority assignments for specific PGNs (in [07-pgn-registry.md](./07-pgn-registry.md))
- Diagnostic PGN definitions for error counter exposure
- Behavior under sustained bus saturation (denial-of-service mitigation)
- Multi-frame message reassembly behavior across repeater hops (depends on [10-repeater-specification.md](./10-repeater-specification.md))
- Whether Pelorus adopts J1939-21 TP unchanged or defines a CAN-FD-aware variant that uses 64-byte data frames natively

---

## Appendix A: Comparison with NMEA 2000 Data Link

| Aspect | NMEA 2000 | Pelorus Core |
|---|---|---|
| Frame format | Classical CAN | CAN FD |
| Identifier length | 29 bit | 29 bit (identical layout) |
| Maximum single-frame payload | 8 bytes | 64 bytes |
| Multi-frame protocol | NMEA 2000 Fast Packet | J1939 TP |
| Remote frames | Permitted | Not used |
| Priority field | 3 bits | 3 bits (identical) |
| PDU1/PDU2 split | At PF=0xF0 | At PF=0xF0 (identical) |
| Source address | 8 bit | 8 bit (identical) |

Pelorus Core preserves the NMEA 2000 / J1939 identifier semantics exactly. The differences are the underlying frame format (CAN FD) and the multi-frame protocol (J1939 TP rather than Fast Packet).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).
