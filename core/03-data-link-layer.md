# Pelorus Core — Data Link Layer Specification

**Version:** 0.1 Draft  
**Last Updated:** May 4, 2026  
**Status:** Pre-specification  
**Trust:** Trusted

---

## About This Document

This document specifies the data link layer for Pelorus Core: CAN FD frame usage, 29-bit identifier layout, multi-frame transport when payloads exceed 64 bytes, and error handling. It sits between [02-physical-layer.md](./02-physical-layer.md) and the application-layer documents [05-addressing.md](./05-addressing.md), [06-signal-catalog.md](./06-signal-catalog.md), [07-dcid-registry.md](./07-dcid-registry.md). For the J1939-derived identifier model, Fast Packet policy, and other suite-wide stack decisions stated once for the whole suite, see [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary).

**J1939 vs CAN FD (normative scope of this document):** Pelorus adopts **SAE J1939**–family *semantics* for the CAN identifier, DCIDs, and transport/addressing references used in this suite, but **all Pelorus Core application data** uses **CAN FD** frames as defined below. **LMDE** segments use **Classical CAN (CAN 2.0)** for those semantics; they are **not** bit-compatible with Pelorus on a **single shared segment**. See [01-overview.md §4](./01-overview.md#4-coexistence-with-the-legacy-marine-data-ecosystem).

---

## 1. Scope

This document defines:

- The CAN FD frame format Pelorus Core uses
- The 29-bit identifier layout and DCID derivation
- Addressing modes (broadcast PDU2, peer-to-peer PDU1)
- Multi-frame message construction for payloads exceeding 64 bytes
- Error handling at the data link layer
- Reserved identifier ranges

This document does **not** define:

- Specific DCID assignments (see [07-dcid-registry.md](./07-dcid-registry.md))
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

This is a deliberate divergence from a coexistence-on-the-same-wire mindset. LMDE traffic lives on a separate physical bus per [01-overview.md §4](./01-overview.md#4-coexistence-with-the-legacy-marine-data-ecosystem); Pelorus Core does not need to emit Classical CAN data frames for any compatibility purpose.

### 2.2 No Remote Frames

Pelorus Core does not use Remote Transmission Request (RTR) frames. Data is push-only or request-via-DCID. Receivers needing data either:

- Subscribe by listening for the relevant broadcast DCID, or
- Send a request message (DCID 0xEA00 per J1939, ratification pending in [07-dcid-registry.md](./07-dcid-registry.md))

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
| 24 | Data Page (DP) | Selects DCID page; v1.0 of Pelorus uses DP=0 only |
| 23–16 | PDU Format (PF) | Determines PDU type and DCID |
| 15–8 | PDU Specific (PS) | Destination address (PDU1) or group extension (PDU2) |
| 7–0 | Source Address (SA) | Sender's claimed address per [05-addressing.md](./05-addressing.md) |

### 3.1 PDU1 vs PDU2

| PF Range | PDU Type | PS Field Means | Addressing |
|---|---|---|---|
| 0x00 – 0xEF | PDU1 | Destination Address | Peer-to-peer |
| 0xF0 – 0xFF | PDU2 | Group Extension | Broadcast |

This is the J1939 convention, preserved unchanged.

### 3.2 DCID Derivation

The Data Contract ID is derived from R, DP, PF, and (for PDU2 only) PS:

- **PDU1 (PF ≤ 0xEF):** `DCID = (R << 17) | (DP << 16) | (PF << 8)` — the destination address is **not** part of the DCID
- **PDU2 (PF ≥ 0xF0):** `DCID = (R << 17) | (DP << 16) | (PF << 8) | PS`

Examples:

| Identifier (29-bit) | PRIO | PF | PS | SA | DCID | Type |
|---|---|---|---|---|---|---|
| `0x0CF80401` | 3 | 0xF8 | 0x04 | 0x01 | 0x0F804 | PDU2 broadcast |
| `0x18EE0102` | 6 | 0xEE | 0x01 | 0x02 | 0x0EE00 | PDU1 to addr 0x01 |
| `0x18FF8003` | 6 | 0xFF | 0x80 | 0x03 | 0x0FF80 | PDU2 (Pelorus WUF) |

### 3.3 Priority Allocation

Priority is the most-significant 3 bits of the identifier and dominates bus arbitration. Pelorus assigns priority by message class:

| Priority | Class | Examples |
|---|---|---|
| 0 | Wake-up frames | WUF (DCID 0x0FF80) |
| 1 | Safety-critical real-time | Steering, autopilot commands, alarm assertions |
| 2 | Critical real-time | Heading, attitude, propulsion control |
| 3 | Real-time navigation | GNSS position, depth, speed, wind |
| 4 | Standard navigation | AIS targets, route data |
| 5 | Engine and machinery | Engine RPM, fuel, alternator |
| 6 | Network management, diagnostics | NM (DCID 0x0FF81), address claim, status |
| 7 | Bulk and non-critical | Logs, configuration, historical data |

These are guidelines. Specific DCID priority assignments are recorded in [07-dcid-registry.md](./07-dcid-registry.md). Vendor-specific or proprietary DCIDs must respect the same priority bands.

---

## 4. Reserved Identifier Ranges

The following DCIDs and ranges are reserved for Pelorus Core protocol use and shall not be assigned to application data.

| DCID / Range | Purpose | Document |
|---|---|---|
| 0x0EE00 | Address Claimed | [05-addressing.md](./05-addressing.md) |
| 0x0EA00 | Request | This document §5.3 |
| 0x0EB00 | Transport Protocol – Data Transfer | This document §5 |
| 0x0EC00 | Transport Protocol – Connection Mgmt | This document §5 |
| 0x0FF80 | Pelorus Wake-Up Group Frame | [04-power-management.md §7.1](./04-power-management.md) |
| 0x0FF81 | Pelorus Network Management | [04-power-management.md §7.3](./04-power-management.md) |
| 0x0FF82 | Pelorus Bus Health (per **[07 §1.3](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)**) | [07](./07-dcid-registry.md), [17](./17-criticality-and-redundant-paths.md) |
| 0x0FF83 | Pelorus Time Sync (optional; per **[07 §1.4](./07-dcid-registry.md#14-time-sync-dcid-0x0ff83-optional)**) | [07](./07-dcid-registry.md), [17](./17-criticality-and-redundant-paths.md) |
| 0x0FF84 – 0x0FF8F | Reserved for future Pelorus protocol use | — |
| 0x0EF00 – 0x0EFFF | Proprietary A (per-vendor, peer-to-peer) | Vendor-managed |
| 0x0FF00 – 0x0FF7F | Proprietary B (per-vendor, broadcast) | Vendor-managed |

DCID 0x0FF80 and onward in the 0x0FF8x range are **carved out** of what would otherwise be Proprietary B space. Vendors shall not assign proprietary messages in these ranges.

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

1. Transmits DCID 0x0EC00 BAM with the destination DCID, total length, and frame count
2. Transmits DCID 0x0EB00 data frames sequentially, with sequence number in byte 0 and up to 63 bytes of payload in bytes 1–63
3. Receivers reassemble by sequence number

Frame spacing: minimum 50 ms between data frames per J1939-21. Receivers shall accept frames as fast as the bus delivers them.

### 5.3 Request Mechanism

A receiver requesting a specific DCID sends DCID 0x0EA00 with the requested DCID encoded in the data field (3 bytes, little-endian). The target node replies with the requested DCID if it is the producer. If multiple producers exist (instance handling), the receiver may need to disambiguate via DCID 0x0EE00 address-claim records.

### 5.4 No Fast Packet

Pelorus Core does not implement the LMDE Fast Packet protocol. Senders shall not use Fast Packet framing. Gateways translating from LMDE to Pelorus Core must reassemble Fast Packet payloads into single CAN FD frames where possible, or fall back to J1939 TP for payloads exceeding 64 bytes.

---

## 6. Path redundancy (dual bus)

Pelorus **path redundancy** uses two independent CAN FD media — **Bus A** and **Bus B** — in a **dual-bus domain** as defined in **[17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md)**. **Segmentation** (repeaters, multiple segments) is orthogonal: each of Bus A and Bus B may comprise one or more segments per **08** / **10**.

### 6.1 Normative goals

- **Active-active:** Producers on **Class D** or **Class H** nodes **shall** transmit the same logical frame on both buses (same 29-bit identifier and same data field for that logical message), subject to **§6.5** bus-ID bit where applicable.
- **Single logical delivery:** Receivers **shall** apply **duplicate discard** so each logical message is delivered to the application layer **once** per transmission instant.

### 6.2 Exemptions from duplicate discard

The following **shall not** pass through the duplicate-discard algorithm (each bus is processed independently for network-management correctness):

- **Address Claimed** and all **address-management** traffic (DCID **0x0EE00**, Commanded Address **0xFED8**, and any future address-management DCIDs registered in **07**).
- **Transport Protocol** frames (**0x0EB00**, **0x0EC00**) — reassembly **shall** occur per **§5** on each bus independently until a future revision defines TP-level deduplication.
- **Wake-Up** (**0x0FF80**) and **Network Management** (**0x0FF81**) frames — processed independently on each bus.

All other application DCIDs **shall** be subject to **§6.4** on receivers in dual-bus domains.

### 6.3 PRH — Pelorus redundancy header (Pelorus-native DCIDs)

The **Pelorus Redundancy Header (PRH)** is a fixed 3-byte preamble used at the **start** of the CAN FD data field for **Pelorus-native broadcast** DCIDs that participate in **path redundancy**.

**Layout (normative):**

| Byte(s) | Field |
|---------|--------|
| 0–1 | **Sequence** — `uint16` little-endian; rolling counter per `(SA, DCID)`. |
| 2 | **BusId_WakeGen** — bit **0**: Bus ID (**0** = Bus A, **1** = Bus B); bits **4–1**: Wake generation (**0–15**, per **[04 §13](./04-power-management.md#13-path-redundancy-and-duplicate-discard-interaction-with-power-states)**); bits **7–5**: reserved — transmit **`0`**, ignore on receive. |

**Scope (normative):**

- DCIDs **0x0FF82** (Bus Health) and **0x0FF83** (Time Sync) **shall** use this PRH; their full payload layouts are in **[07 §1.3 / §1.4](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)**.
- Any **future** Pelorus-native broadcast DCID assigned in the range **`0x0FF84`–`0x0FFFF`** with a payload of **4 bytes or more** **shall** include this PRH at bytes **0–2** before its application fields. Pelorus-native DCIDs with payload **< 4 bytes** **may** omit the PRH (such DCIDs use **§6.4.2** payload-and-ID dedup).
- **Compatibility DCIDs** (per **[07 §2](./07-dcid-registry.md)** — J1939 / NMEA-2000 heritage layouts) **shall not** carry a PRH; their bit layouts remain bound to the SAE J1939 Digital Annex and are deduplicated via **§6.4.2**.
- **Reserved transport DCIDs** (**0x0EA00** request, **0x0EB00 / 0x0EC00** TP, **0x0EE00** address claim, **0x0FF80** WUF, **0x0FF81** NM) **shall not** carry a PRH; they are exempt from duplicate discard (**§6.2**).

Receivers **shall** use the 16-bit sequence in the PRH for duplicate discard per **§6.4.1**.

### 6.4 Duplicate discard algorithm

Receivers in a dual-bus domain **shall** maintain a **Duplicate Discard Table (DDT)** with at most one entry per **source address** `S` that has been heard on either bus.

#### 6.4.1 Entries keyed with PRH (DCIDs 0x0FF82, 0x0FF83)

For each received frame of DCID **0x0FF82** or **0x0FF83** from source `S` with PRH sequence `N` received on bus `B` (`A` or `B`):

1. If no DDT entry for `S` for this DCID: **accept**; create entry `(S, DCID, N, B, now)`.
2. If entry exists:
   - If `N == entry.last_sequence` **and** `(now - entry.last_seen_time) < DISCARD_WINDOW`: **discard** (duplicate).
   - Else: **accept**; update entry to `(S, DCID, N, B, now)`.

| Parameter | Value | Notes |
|-----------|-------|--------|
| `DISCARD_WINDOW` | **50 ms** | **Minimum**; per-installation value **shall** satisfy the formula in **§6.4.3**. |
| `NODE_FORGET_TIME` | **60 s** | Remove entry if no frame from `(S, DCID)` on either bus. |

#### 6.4.2 Compatibility and other application DCIDs (no PRH)

For frames **without** a PRH (including all **§2** compatibility layouts in **07** where Digital Annex byte positions are preserved), receivers **shall** use **payload-and-ID duplicate discard**:

- Key: `(S, DCID, DLC, data[0..DLC-1])`.
- On receive on bus `B`: if an entry exists for the same key from the **other** bus within `DISCARD_WINDOW`, **discard** this frame; else **accept** and record `(key, B, now)`.

**Note:** Identical legitimate retransmissions of the **same** payload within `DISCARD_WINDOW` on one bus are rare for marine periodic data; if an application requires identical back-to-back payloads faster than `DISCARD_WINDOW`, it **shall** use a DCID or transport that carries an explicit sequence (future catalog overlay or Pelorus-native DCID).

#### 6.4.3 `DISCARD_WINDOW` lower-bound formula

For an installation with a maximum of `H` repeater / hub hops between any producer and any consumer on **either** bus (per **[08 §2](./08-network-architecture.md#2-multi-segment-networks-and-repeaters)**), declared per-hop maximum forwarding latency `L_hop`, and bounded inter-node clock drift `D_clk`, the configured `DISCARD_WINDOW` **shall** satisfy:

```
DISCARD_WINDOW >= 2 * H * L_hop  +  2 * D_clk  +  safety_margin
```

with **default** `safety_margin = 10 ms`. The **50 ms** value above is the absolute floor; a deeper or higher-drift installation **shall** use a larger value and document it in the **critical zone map** (**[17 §6](./17-criticality-and-redundant-paths.md#6-critical-zone-map-and-conformance)**).

`D_clk` defaults to **10 ms** when **0x0FF83** Time Sync is implemented in the dual-bus domain per **[07 §1.4](./07-dcid-registry.md#14-time-sync-dcid-0x0ff83-optional)**; otherwise the installation **shall** declare a measured or worst-case `D_clk` and use the formula.

### 6.5 Multi-frame (J1939 TP) and duplicate discard

Until a future revision specifies TP-level deduplication, receivers **shall** run **§5** reassembly **independently per bus** for TP traffic. Application consumers **should** merge only after complete reassembly and **may** treat identical completed messages from A and B within `DISCARD_WINDOW` as one logical delivery.

### 6.6 Interaction with power management and bus return

**Power-state transitions.** After any transition from **Sleep** or **Deep Sleep** to **Active** (per **04**), the node **shall** increment a **4-bit wake generation** counter (stored in non-volatile or retained RAM) exposed in Bus Health (**07**) when implemented; receivers **shall** invalidate all DDT entries for that source when the generation value changes. If generation is not yet implemented, receivers **shall** invalidate DDT entries for a source on **first** NM **Normal-Operation** indication after bus activity resumes (**04** §9). See **[04 §13](./04-power-management.md#13-path-redundancy-and-duplicate-discard-interaction-with-power-states)** (Path redundancy and duplicate-discard interaction with power states).

**Bus return after a failed-bus interval.** When a previously failed bus (Bus A or Bus B) recovers and resumes carrying valid CAN FD traffic, receivers **shall**:

1. **Accept** frames on the returning bus immediately and apply the normal **§6.4** duplicate-discard rules — there is **no** re-sync handshake or replay protocol in v1.0.
2. **Not** treat sequence numbers or payloads observed on the returning bus as duplicates of frames already delivered from the surviving bus during the outage **unless** they fall inside the active `DISCARD_WINDOW` for the same `(S, DCID)` per **§6.4.1** / **§6.4.2**.
3. Continue the existing **DDT** entries for sources whose **wake generation** has not changed; only invalidate when generation changes per the rule above.

This makes bus return **transparent** to the application layer when failover convergence is met (**[17 §3.1](./17-criticality-and-redundant-paths.md#31-failover-convergence-c0--c1)**).

---

## 7. Error Handling

CAN FD's error handling mechanisms (CRC, ACK, error frames, error counters per ISO 11898-1) operate unchanged on Pelorus Core. Application-level error handling is layered on top.

### 7.1 Bus-level errors

The CAN controller handles:

- **CRC errors** — receiver detects payload corruption; transmitter retries automatically
- **Form errors** — fixed-format bits violated; transmitter retries
- **Stuff errors** — bit-stuffing rule violated; transmitter retries
- **ACK errors** — no receiver acknowledged; transmitter retries
- **Bit errors** — transmitted vs. monitored bit mismatch; transmitter retries

A node observing repeated errors transitions to error-passive and eventually bus-off per ISO 11898-1.

### 7.2 Bus-off recovery

A node entering bus-off shall:

1. Wait for 128 occurrences of 11 consecutive recessive bits (per ISO 11898-1)
2. Transition to error-active and resume transmission
3. Log the bus-off event for diagnostics

Pelorus does not specify additional bus-off recovery procedures. Implementations may add backoff or alarm signaling at the application layer.

### 7.3 Application-layer error handling

The data link layer does not guarantee delivery of any single frame. Applications requiring guaranteed delivery must implement acknowledgement at the application layer. Pelorus does not provide a generic ACK mechanism.

For periodic data (GNSS position, wind speed, depth), the natural retransmit cadence absorbs occasional frame loss. For commanded actions (autopilot setpoint changes, alarm acknowledgement), the originator should expect and verify a status response.

### 7.4 Transmit retry policy

Pelorus Core nodes shall use the CAN controller's automatic retransmission. Manual retry suppression is permitted only for time-critical messages where stale data is worse than no data (e.g., heading updates older than 100 ms should be discarded rather than retransmitted).

### 7.5 Error counters and diagnostics

Each Pelorus Core node shall expose, via **[DCID 0x0FF82](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)** (Bus Health) when in a dual-bus domain, and **may** use additional diagnostic DCIDs registered in [07-dcid-registry.md](./07-dcid-registry.md):

- TX error count
- RX error count
- Bus-off event count
- Last bus-off timestamp

This makes the network sailor-debuggable as required by [01-overview.md §6](./01-overview.md).

---

## 8. Bus arbitration

CAN FD arbitration uses the standard CSMA/CR (Carrier Sense Multiple Access with Collision Resolution by bitwise priority). The lowest numerical identifier wins arbitration.

### 8.1 Arbitration implications for Pelorus

- Priority 0 messages (WUF) preempt all other traffic
- Priority 1–2 messages preempt routine sensor traffic
- A node with low-priority bulk traffic must not starve higher-priority messages — implementations shall not back-pressure higher priorities

### 8.2 Identifier uniqueness

The full 29-bit identifier (priority + reserved + DP + PF + PS + SA) must be unique among messages active on the bus at any moment. Two nodes simultaneously transmitting the same identifier with different data is a CAN-level error and indicates an addressing conflict (handled per [05-addressing.md](./05-addressing.md)).

---

## 9. Open items

The following remain unresolved:

- Final priority assignments for specific DCIDs (in [07-dcid-registry.md](./07-dcid-registry.md))
- Additional diagnostic DCIDs beyond **0x0FF82** / **0x0FF83** for vendor-specific maintenance
- Behavior under sustained bus saturation (denial-of-service mitigation)
- Multi-frame message reassembly behavior across repeater hops (depends on [10-repeater-specification.md](./10-repeater-specification.md))
- Whether Pelorus adopts J1939-21 TP unchanged or defines a CAN-FD-aware variant that uses 64-byte data frames natively

---

## Appendix A: Comparison with the Legacy Marine Data Ecosystem data link

| Aspect | Legacy Marine Data Ecosystem | Pelorus Core |
|---|---|---|
| Frame format | Classical CAN | CAN FD |
| Identifier length | 29 bit | 29 bit (identical layout) |
| Maximum single-frame payload | 8 bytes | 64 bytes |
| Multi-frame protocol | LMDE Fast Packet | J1939 TP |
| Remote frames | Permitted | Not used |
| Priority field | 3 bits | 3 bits (identical) |
| PDU1/PDU2 split | At PF=0xF0 | At PF=0xF0 (identical) |
| Source address | 8 bit | 8 bit (identical) |

Pelorus Core preserves LMDE / J1939 identifier and DCID **semantics**. The differences are the **physical frame format** (**Classical CAN** on LMDE vs **CAN FD** on Pelorus) and the **multi-frame rules** (LMDE Fast Packet vs J1939 TP on Pelorus as specified).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
