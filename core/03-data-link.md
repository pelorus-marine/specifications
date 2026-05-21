# Pelorus Core — Data Link Layer

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

Data is push-only or request-via-`Pelorus.Request`; receivers needing data either subscribe by listening for the relevant broadcast DC or send a request message (`Pelorus.Request`, see [§4.8](#48-request-mechanism)). Pelorus does not use Remote Transmission Request frames.

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

29-bit identifier, Pelorus-native layout:

```
 28 27 26 | 25 ............................... 8 | 7 ...... 0
   PRIO   |              DC_ID                   |    SA
   (3 b)  |              (18 b)                  |   (8 b)
```

| Bits | Field | Description |
|---|---|---|
| 28–26 | PRIO | 0 (highest) to 7 (lowest) |
| 25–8 | DC_ID | 18-bit Pelorus Data Contract ID. Allocated in [`07-dcid-registry.md`](./07-dcid-registry.md). Written as 5 hex digits (range `0x00000`–`0x3FFFF`) |
| 7–0 | SA | Source Address per [`05-addressing.md`](./05-addressing.md) |

There is no PDU1/PDU2 distinction, no R bit, no DP bit. A Data Contract that needs peer-to-peer targeting carries the destination address in its payload, not in the wire identifier.

### 2.1 Identifier Examples

| 29-bit identifier | PRIO | DC_ID | SA | Pelorus DC |
|---|---|---|---|---|
| `0b000_000000000000000001_00000011` | 0 | `0x00001` | `0x03` | `Pelorus.WakeUp` |
| `0b110_000000000000000101_00000010` | 6 | `0x00005` | `0x02` | `Pelorus.AddressClaim` |
| `0b100_000000000100010000_00000111` | 4 | `0x00110` | `0x07` | `Pelorus.AISClassAPosition` |

### 2.2 Priority Allocation

| Priority | Class | Examples |
|---|---|---|
| 0 | Wake-up frames | `Pelorus.WakeUp` |
| 1 | Safety-critical real-time | Steering, autopilot commands, alarm assertions |
| 2 | Critical real-time | Heading, attitude, propulsion control |
| 3 | Real-time navigation | GNSS position, depth, speed, wind |
| 4 | Standard navigation | AIS targets, route data |
| 5 | Engine and machinery | Engine RPM, fuel, alternator |
| 6 | Network management, diagnostics | `Pelorus.NetworkManagement`, `Pelorus.AddressClaim`, `Pelorus.BusHealth` |
| 7 | Bulk and non-critical | Multi-frame transport, firmware update, logs, configuration |

Specific DC priority assignments are recorded in [`07-dcid-registry.md`](./07-dcid-registry.md).

## 3. DC_ID Namespace

Numeric DC_ID allocation is normative in [`07-dcid-registry.md §3`](./07-dcid-registry.md). The Pelorus 18-bit DC_ID space is partitioned into a Pelorus-protocol band, a compatibility (bridged) band, a general-contract band, a reserved expansion band, and a vendor-proprietary band. The data link layer treats all DC_ID values uniformly — partitioning is a registry concern, not a wire concern.

Pelorus does not carve out any range from a third-party identifier space; the entire 18-bit DC_ID namespace is Pelorus-owned.

## 4. Multi-Frame Transport

Messages exceeding 64 bytes use Pelorus-native multi-frame transport. Two Data Contracts carry the mechanism:

- `Pelorus.MultiFrameControl` (`DC_ID = 0x00008`, priority 7) — session control
- `Pelorus.MultiFrameData` (`DC_ID = 0x00009`, priority 7) — payload data frames

The transport supports both targeted (windowed-ack) and broadcast (unacked) sessions. The primary motivating use case is the open firmware update protocol in [`12-firmware-update.md`](./12-firmware-update.md).

### 4.1 Session Identifiers and Limits

- `session_id` is a 16-bit value scoped to the sender's source address. A node shall not have two open sessions sharing a `session_id` toward the same destination.
- A node shall not have more than one open ingress and one open egress multi-frame session at any time in v1.0. Concurrent sessions are deferred to v2.
- Session content is bounded by `total_size` (32-bit). At 58 payload bytes per `MultiFrameData` frame, sessions support up to ~4 GB of content; firmware updates of any practical size fit within one session.

### 4.2 `Pelorus.MultiFrameControl` Frame

Byte 0 is an opcode. Remaining bytes carry opcode-specific fields:

| Opcode | Direction | Fields |
|---|---|---|
| `Open` | sender → receiver | `session_id` (2 B), `dst_SA` (1 B), `content_DC_ID` (3 B), `total_size` (4 B), `total_frames` (4 B), `window_size_requested` (2 B), `content_crc32` (4 B) |
| `OpenAck` | receiver → sender | `session_id` (2 B), `window_size_granted` (2 B), `next_expected_seq` (4 B) |
| `OpenNak` | receiver → sender | `session_id` (2 B), `reason_code` (1 B) |
| `BroadcastOpen` | sender → all | `session_id` (2 B), `content_DC_ID` (3 B), `total_size` (4 B), `total_frames` (4 B), `content_crc32` (4 B) |
| `Window` | receiver → sender | `session_id` (2 B), `next_expected_seq` (4 B), `last_received_seq` (4 B), `missing_count` (1 B), `missing_list` (4 B × missing_count, up to ~12 entries per CAN FD frame) |
| `Close` | sender → receiver | `session_id` (2 B), `status` (1 B) |
| `Abort` | either | `session_id` (2 B), `reason_code` (1 B) |

`content_DC_ID` names the Data Contract that the multi-frame session is delivering — for firmware update sessions, this is the DC_ID of the image content channel referenced by `Pelorus.FirmwareUpdateBegin`.

Reason codes and status codes are enumerated in [`12-firmware-update.md`](./12-firmware-update.md) for firmware update; general transport-level codes are listed in [§4.6](#46-reason-and-status-codes).

### 4.3 `Pelorus.MultiFrameData` Frame

| Bytes | Field |
|---|---|
| 0–1 | `session_id` |
| 2–5 | `sequence_number` (first data frame in a session is `0`) |
| 6–63 | `data` (up to 58 bytes; final frame may be shorter — actual byte count derived from `total_size`) |

### 4.4 Targeted Sessions

1. Sender transmits `MultiFrameControl{Open, ...}` carrying `total_size`, `total_frames`, requested window size, and content CRC32.
2. Receiver responds with `OpenAck` (granting a window size ≤ requested) or `OpenNak`.
3. Sender streams up to `window_size_granted` `MultiFrameData` frames, then waits for a `Window` acknowledgement.
4. Receiver sends `Window` carrying `next_expected_seq`, the highest sequence received, and a list of missing sequence numbers (NAK list).
5. Sender retransmits any missing frames, then continues from `next_expected_seq + window_size`.
6. After the final frame, sender transmits `Close{status=Complete}`. Receiver verifies CRC32 over reassembled content and responds with an out-of-band status (via the content DC's own response channel, e.g. `Pelorus.FirmwareUpdateProgress`).

If CRC verification fails at the receiver, it transmits `Abort{reason=CRCMismatch}` and discards the session content.

### 4.5 Broadcast Sessions

1. Sender transmits `MultiFrameControl{BroadcastOpen, ...}`.
2. Sender streams `MultiFrameData` frames sequentially with no flow control.
3. Receivers reassemble by `sequence_number` and verify `content_crc32` at completion.
4. Receivers report errors via diagnostics (not via the transport itself). Senders shall not assume delivery.

Broadcast sessions are appropriate for catalog snapshots, logs, and bulk data where any-receiver reception is sufficient and per-receiver acknowledgement is not required.

### 4.6 Reason and Status Codes

General multi-frame transport codes:

| Code | Meaning |
|---|---|
| `0x00` | Complete / no error |
| `0x01` | CRCMismatch — reassembled content CRC32 does not match `content_crc32` |
| `0x02` | SessionExists — `session_id` already in use |
| `0x03` | NoResources — receiver lacks buffer for a session of this size |
| `0x04` | Timeout — no progress within implementation-defined timeout |
| `0x05` | UnknownContent — `content_DC_ID` is unrecognised at receiver |
| `0x06` | Cancelled — initiator cancelled |
| `0x07` | ProtocolError — malformed control frame |

Application-layer codes (e.g. firmware-update-specific errors) live in their respective Data Contract specifications.

### 4.7 Resumability

A receiver shall retain partial-session state through `Abort` and may accept a subsequent `Open` with the same `session_id`, advancing `next_expected_seq` past sequence numbers already received. Receivers shall retain partial state for at least 60 seconds following the last received frame; longer retention is implementation-defined. Resumability allows interrupted firmware updates to continue without re-transmitting the full image.

### 4.8 Request Mechanism

A receiver requesting a specific DC sends `Pelorus.Request` (`DC_ID = 0x00007`, priority 6) with the requested `DC_ID` in the first 3 bytes of payload (little-endian). The target node replies with the requested DC if it is the producer. If multiple producers exist, the requester disambiguates via `Pelorus.AddressClaim` records.

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

Each node exposes TX error count, RX error count, bus-off event count, and last bus-off timestamp via `Pelorus.BusHealth` when in a dual-bus domain ([`08-redundancy.md`](./08-redundancy.md)). Additional diagnostic DCs may be registered in [`07-dcid-registry.md`](./07-dcid-registry.md).

## 6. Arbitration

Standard CSMA/CR — lowest numerical identifier wins. Implementations shall not back-pressure higher priorities with low-priority bulk traffic.

The full 29-bit identifier must be unique among messages active on the bus at any moment. Two nodes simultaneously transmitting the same identifier with different data is a CAN-level error indicating an addressing conflict (handled per [`05-addressing.md`](./05-addressing.md)).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
