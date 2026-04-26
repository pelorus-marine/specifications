# Pelorus Power Management — Developer Reference

**Version:** 0.4 Draft
**Last Updated:** April 26, 2026
**Status:** Pre-specification

---

## About This Document

This document provides what an embedded developer needs to implement Pelorus-compatible power management on CAN FD networks, citing only freely accessible reference materials. The Pelorus design adopts the automotive industry's selective wake-up and partial networking mechanisms, adapted for marine use.

**Core philosophy:** every Pelorus node should consume only the power it needs to do its current job. A boat at anchor with only the GNSS-driven anchor watch active should draw single-digit milliamps across the entire network, not the 2-3 amps typical of the legacy marine data ecosystem.

**ISO 11898-2:2016 Validation Note:** Sections 1–5 of this document have been cross-checked against the full ISO 11898-2:2016 standard (Sections 5.9 and 5.10, Tables 18–20, Figures 6–11). All WUF format, matching rules, frame error counter, bus biasing, and timing claims are accurate and normative. Sections 6 onward describe Pelorus-specific allocations (functional group bit assignments, reserved identifiers, NM cadence) which are v0.1 proposals subject to revision after prototype validation.

---

## 1. The Problem

The legacy marine data ecosystem inherited its always-on power model from SAE J1939, designed for trucks where the alternator runs continuously. The standard provides no selective device power management.

A typical sailboat instrument suite draws 2-3 A continuously. Over 12 hours at anchor that consumes 24-36 Ah — a meaningful fraction of the 200-400 Ah usable battery capacity on most cruising boats, before refrigeration, lights, and other loads compete for the same capacity.

Pelorus solves this by making power management a first-class part of the protocol. Every Pelorus node has defined power states with documented current budgets, participates in coordinated network sleep and wake, and can be selectively woken based on functional group membership.

---

## 2. Solution Overview

Partial networking lets individual CAN nodes (or groups) sleep while the bus stays operational for other nodes. A specific CAN frame format — the Wake-Up Frame (WUF) — selectively wakes targeted nodes without disturbing those that should remain asleep.

The mechanism is implemented primarily in the CAN transceiver hardware. While a node is asleep:

- The microcontroller is fully powered down or in deep retention sleep
- Only the CAN transceiver remains powered, drawing typically 1-50 µA
- The transceiver's internal logic monitors the bus for a valid WUF
- A matching WUF triggers the transceiver to wake the microcontroller

This is shipping in millions of vehicles today. The reference standard is ISO 11898-2:2016, which integrated the earlier ISO 11898-5 (low-power mode) and ISO 11898-6 (selective wake-up) into a single document.

**Marine functional groups** map naturally to partial networking:

- **Anchor watch** — GNSS, depth, alarm output
- **Underway** — full navigation suite, autopilot, AIS
- **Engine** — engine monitoring, fuel, alternator
- **Comms** — VHF, AIS, satellite
- **Domestic** — tank levels, battery monitoring
- **Storm** — wind, AIS receive only, GNSS

---

## 3. ⚠️ Patent Notice (Read First)

**ISO 11898-2:2016 explicitly discloses that the selective wake-up function (Section 5.9.4) involves patents.** The spec lists the patent-holding organizations: Audi, BMW, Continental Teves, DENSO, Elmos Semiconductor, Freescale Semiconductor (now NXP), General Motors, NXP, Renesas Electronics, Robert Bosch, STMicroelectronics, and Volkswagen.

The patent holders have committed to ISO that they will license under reasonable and non-discriminatory (RAND) terms. This means:

- Implementing selective wake-up in commercial Pelorus hardware likely requires patent licensing
- Open source reference implementations and personal/research use are typically lower risk, but this is not legal advice
- Before commercial Pelorus products implementing selective wake-up ship, a maritime IP attorney must review the patent landscape and licensing situation
- Basic wake-up and wake-up pattern (WUP) wake-up — Sections 5.9.2 and 5.9.3 — do not have the same patent disclosure and may be viable lower-functionality alternatives

This is the single most important consideration in this document. **Selective wake-up is the technically superior approach but carries explicit IP exposure that the basic wake-up alternatives do not.**

---

## 4. Free Reference Materials

All sources below are freely downloadable, with no NDA or registration. Pelorus contributors can implement the entire specification without spending a dollar on standards documents.

### Primary Implementation References

**NXP AH1203 Application Note — "Partial Networking"**  
`https://community.nxp.com/pwmxy87654/attachments/pwmxy87654/other/7787/1/AH1203_Partial_Networking_v1.1.pdf`  
WUF format, identifier matching, data mask logic, timing parameters, state machine.

**NXP TJA1145 Datasheet**  
`https://www.nxp.com/docs/en/data-sheet/TJA1145.pdf`  
Register-level reference implementation. CAN FD up to 2 Mbit/s. The TJA1145/FD variants support CAN FD frame tolerance.

**Microchip ATA6570 Datasheet**  
`https://ww1.microchip.com/downloads/en/DeviceDoc/ATA6570-Data-Sheet-DS20005788D.pdf`  
Second-source partial networking implementation supporting CAN FD up to 5 Mbit/s.

**NXP "Getting Started With CAN FD"**  
`https://community.nxp.com/pwmxy87654/attachments/pwmxy87654/connects/50/1/AMF-AUT-T2788.pdf`  
Worked configuration examples including identifier ranges and 64-group data field addressing.

### Network Management Layer

**AUTOSAR Specification of CAN Network Management (CanNm) R23-11**  
`https://www.autosar.org/fileadmin/standards/R23-11/CP/AUTOSAR_CP_SWS_CANNetworkManagement.pdf`  
Network management state machine, coordinated sleep, partial network coordination.

**AUTOSAR Specification of NetworkManagement Interface**  
`https://www.autosar.org/fileadmin/standards/R18-10_R4.4.0_R1.5.0/CP/AUTOSAR_SWS_NetworkManagementInterface.pdf`  
Generic NM interface, multi-bus coordination.

### Linux Reference

**Linux Kernel SocketCAN Documentation**  
`https://docs.kernel.org/networking/can.html`  
Production CAN/CAN FD implementation including ISO-TP support since kernel 5.10.

**Rust SocketCAN Crate**  
`https://docs.rs/socketcan/latest/socketcan/`  
Rust bindings with CAN FD support and async runtimes.

**kal102/CanNm — Open Source AUTOSAR CanNm Implementation**  
`https://github.com/kal102/CanNm` (MIT licensed)

### Standards (For Maintainer Reference Only)

ISO 11898-2:2016 (~$200) and ISO 16845-2 (conformance test plan, ~$200) are recommended purchases for the project maintainer when finalizing the formal Pelorus specification. Not required for contributors.

---

## 5. The Wake-Up Frame Format

A WUF is a Classical CAN frame (per ISO 11898-1:2015) consisting of:

- **Identifier field** — standard 11-bit or extended 29-bit (selectable per node)
- **Data Length Code (DLC)** — 4 bits, value 0-8
- **Data field** — 0 to 8 bytes per the DLC
- **CRC field** — including CRC delimiter

A frame is a valid WUF only if it is free of CRC, stuff, and form errors through the CRC delimiter. Errors after the CRC delimiter are ignored.

### CAN FD Frames as WUFs — The Nuanced Answer

A CAN FD frame is not recognized as a valid WUF by basic ISO 11898-2 implementations. However, the standard defines optional CAN FD tolerance with two bit rate tiers:

- **Bitfilter option 1** — tolerates CAN FD frames with data phase ≤4× arbitration rate or 2 Mbit/s, whichever is lower
- **Bitfilter option 2** — tolerates CAN FD frames with data phase ≤10× arbitration rate or 5 Mbit/s, whichever is lower

CAN FD-tolerant transceivers (like the TJA1145/FD variants) detect the FDF=recessive followed by res=dominant pattern, then wait for `nBits_idle` recessive bits (range 6-10) before considering a new SOF.

**For Pelorus:** WUFs must be transmitted as Classical CAN frames. CAN FD data frames coexist on the same bus but cannot themselves serve as WUFs. The Pelorus specification will mandate CAN FD-tolerant transceivers for all nodes so that CAN FD data traffic does not interfere with WUF detection.

### Identifier Matching

Each node stores a target ID and an ID mask. A `1` in the mask means "don't care" for that bit position; a `0` means the bit must match. (This convention matches common transceiver register implementations and ISO 11898-2:2016 Figure 9.)

**Example from NXP TJA1145 datasheet:**
- 11-bit ID configured: `0x1A0` (binary `001 1010 0000`)
- Mask: lowest 3 bits set to `1` (don't care)
- Result: any of `0x1A0` through `0x1A7` matches (8 different sources)

The IDE bit (which selects 11-bit vs 29-bit) is not part of the mask — it must match exactly.

### DLC Matching

The received DLC must equal the configured DLC. **Special case:** if configured DLC = 0, the data field is not evaluated and the data mask is ignored.

### Data Field Group Matching

When DLC ≥ 1, the data field implements group addressing. Up to 8 bytes × 8 bits = **64 distinct groups**.

**The matching rule (different from ID matching):** A wake-up occurs if **at least one bit position** has `1` in both the received frame's data field and the node's data mask. Multiple matching bits are fine — only one is required.

**Pelorus group addressing example:**

A GPS receiver participates in the `anchor_watch` group (bit 0) and the `underway` group (bit 1). Its data mask is `0x03 0x00 0x00 0x00 0x00 0x00 0x00 0x00`.

- WUF data `0x01 …` (anchor_watch) → bit 0 matches → GPS wakes
- WUF data `0x02 …` (underway) → bit 1 matches → GPS wakes
- WUF data `0x03 …` (both groups requested) → either bit matches → GPS wakes
- WUF data `0x04 …` (engine only) → no overlap → GPS stays asleep

This makes group membership additive: a node wakes for any group it belongs to, and a single WUF can wake multiple cooperating groups at once.

---

## 6. Pelorus Marine Functional Groups (PNCs)

Pelorus reserves the lowest six bits of byte 0 for the standard marine functional groups. Bits 6–63 are reserved for future Pelorus assignments and vendor-specific clusters; they shall not be used by v1.0 implementations.

| Bit | Group | Typical Members | Wake Trigger |
|---|---|---|---|
| 0 | `anchor_watch` | GNSS, depth, anchor alarm | At anchor; wakes periodically or on drift |
| 1 | `underway` | GNSS, heading, wind, AIS, autopilot, log | Vessel moving under sail or power |
| 2 | `engine` | Engine ECU, fuel, alternator, exhaust temp | Ignition on or engine running |
| 3 | `comms` | VHF, AIS transmit, satellite, legacy-marine bridge | DSC inbound, scheduled poll, or user request |
| 4 | `domestic` | Tank levels, battery monitors, refrigeration | Periodic housekeeping or user request |
| 5 | `storm` | Wind, AIS receive, GNSS, barometer | Severe weather mode; reduced bandwidth |
| 6–63 | Reserved | — | Shall not be used in v1.0 |

### 6.1 Group Membership

Group membership is configured per device, typically at provisioning time. A device may belong to any combination of groups. Examples:

- **GPS receiver** — `anchor_watch | underway | storm` (bits 0, 1, 5)
- **Wind transducer** — `underway | storm` (bits 1, 5)
- **Engine ECU** — `engine` (bit 2)
- **Tank sender** — `domestic` (bit 4)

### 6.2 Group Activation

A node sending a WUF asserts one or more group bits to wake the corresponding clusters. The gateway is the typical originator but any active node may transmit a WUF. Group activation is not exclusive: an autopilot waking the `underway` cluster may simultaneously assert `comms` to ensure AIS is available.

### 6.3 Mode Transitions

Vessel-wide mode transitions (e.g., "weighing anchor" — moving from anchor to underway) are coordinated by the gateway:

1. Gateway transmits WUF asserting the new groups
2. Newly-woken nodes initialize and begin transmitting their data
3. Nodes belonging only to the old groups eventually time out and re-sleep via normal NM behavior (see §9)

Mode transitions are not atomic. Sailors should expect new instruments to come online over a few seconds, not instantly.

---

## 7. Reserved Identifiers and Data Conventions

### 7.1 WUF Identifier

Pelorus WUFs use a single fixed 29-bit J1939-style identifier:

| Field | Value | Notes |
|---|---|---|
| Priority | 0 (binary `000`) | Highest priority on the bus |
| Reserved + DP | 0 | Standard J1939 |
| PF (PDU Format) | 0xFF | PDU2 broadcast (no destination) |
| PS (PDU Specific) | 0x80 | Pelorus WUF assignment |
| Source Address | originator's claimed address | See [05-addressing.md](./05-addressing.md) |
| Resulting PGN | 0x0FF80 (65408) | "Pelorus Wake-Up Group Frame" |

This is a candidate allocation. Final PGN assignment is recorded in [07-pgn-registry.md](./07-pgn-registry.md).

### 7.2 WUF Data Field

DLC = 8. Eight bytes of group mask, MSB byte first. Byte 0 carries the standard functional groups (§6); bytes 1–7 are reserved (must be transmitted as zero, must be ignored on receive).

### 7.3 NM Identifier

Pelorus Network Management messages use:

| Field | Value | Notes |
|---|---|---|
| Priority | 6 (binary `110`) | Below safety-critical traffic |
| PF | 0xFF | PDU2 broadcast |
| PS | 0x81 | Pelorus NM assignment |
| Resulting PGN | 0x0FF81 (65409) | "Pelorus Network Management" |

### 7.4 NM Data Field

DLC = 8. Layout:

| Byte | Field | Description |
|---|---|---|
| 0 | NM state | See §9.2 — `0x00` ready-sleep, `0x01` repeat, `0x02` normal-operation, `0x03` prepare-bus-sleep |
| 1 | Active groups (low byte) | Bitmap of group memberships the sender is keeping awake |
| 2–7 | Reserved | Transmitted zero, ignored on receive |

Implementations should not rely on the reserved bytes carrying information; future revisions may allocate them.

### 7.5 No Other PGNs Reserved by This Document

This document allocates only the WUF and NM identifiers. All other identifier allocations are deferred to [07-pgn-registry.md](./07-pgn-registry.md).

---

## 8. Power States and Transitions

### 8.1 State Definitions

A Pelorus node operates in one of four states:

| State | Microcontroller | Transceiver | Bus Monitoring | Typical Current (non-isolated) |
|---|---|---|---|---|
| **Active** | Running | Normal mode | Yes | Application-specific (declared) |
| **Standby** | Low-power running | Normal mode | Yes | < device-specific declared |
| **Sleep** | Off or retention | Selective wake mode | Yes (via WUF detection) | ≤ 100 µA |
| **Deep Sleep** | Off | Standby mode (no WUF) | No | ≤ 10 µA |

Sleep targets for galvanically isolated devices are documented in [02-physical-layer.md §9.5](./02-physical-layer.md).

### 8.2 State Transition Diagram

```
                ┌──────────────┐
                │              │
     ┌──────────► Deep Sleep   │
     │          │              │
     │          └──────┬───────┘
     │   external      │ external wake (timer, switch)
     │   trigger       ▼
     │          ┌──────────────┐
     │          │              │
     ├──────────►   Sleep      ◄──────────┐
     │          │              │          │ NM coordinated
     │          └──────┬───────┘          │ sleep
     │                 │ WUF match         │
     │                 ▼                   │
     │          ┌──────────────┐          │
     │          │              │          │
     │          │   Standby    ├──────────┤
     │          │              │          │
     │          └──────┬───────┘          │
     │                 │ application      │
     │                 │ wake             │
     │                 ▼                  │
     │          ┌──────────────┐          │
     │          │              │          │
     └──────────┤    Active    ├──────────┘
                │              │
                └──────────────┘
```

### 8.3 Transition Rules

| From | To | Trigger | Notes |
|---|---|---|---|
| Active | Standby | Idle timeout (application-defined) | Application must drain pending transmissions first |
| Active | Sleep | Coordinated cluster sleep (see §9) | Initiated by NM, not unilateral |
| Standby | Active | Application event (sensor reading, RX traffic) | No bus signaling required |
| Standby | Sleep | Coordinated cluster sleep (see §9) | |
| Sleep | Active | WUF group match | Via Standby; transceiver wakes MCU which initializes |
| Sleep | Deep Sleep | Vessel-wide power-down command | Optional; not all devices support Deep Sleep |
| Deep Sleep | Active | External wake (RTC, manual switch, hardwired event) | Bus traffic does not wake from Deep Sleep |

Unilateral sleep is forbidden for any node that other nodes depend on. A node may only enter Sleep if (a) all consumers of its data have indicated they no longer need it, or (b) the cluster has reached coordinated sleep through NM.

### 8.4 Wake-up Latency

Wake-up latency is the elapsed time from WUF detection to the node being able to transmit useful data.

| Phase | Typical Duration |
|---|---|
| Transceiver INH/MCU wake | 100–500 µs |
| MCU boot and CAN init | 5–50 ms |
| Application initialization | 10–500 ms |
| First valid data transmission | 50 ms – 2 s (sensor-dependent) |

Pelorus does not mandate a wake-up latency target. Device manufacturers shall declare the typical and maximum elapsed time from WUF reception to first valid data, in the device's data sheet, for each functional group it supports.

---

## 9. Network Management Behavior

Pelorus NM is modelled on AUTOSAR CanNm (R23-11), simplified for marine use. Each node periodically transmits an NM message to indicate its intent to keep the cluster active. When all nodes stop transmitting NM, the cluster transitions to coordinated sleep.

### 9.1 NM Cadence

| Parameter | Value | Notes |
|---|---|---|
| NM message period | 200 ms ± 20 ms | Per node, while cluster active |
| Repeat-message duration | 1.0 s | Initial active phase to confirm cluster membership |
| Wait-bus-sleep duration | 2.0 s | Quiet time before transceivers enter selective wake mode |
| Total transition to Sleep | ~3.0 s after last keep-active | |

These values are starting points. Final cadence is subject to wake-up latency measurements from prototype hardware.

### 9.2 NM States

Each node implements the following NM state machine:

| State | Behavior |
|---|---|
| **Bus-Sleep** | Transceiver in selective wake mode. No NM transmission. |
| **Prepare-Bus-Sleep** | No NM transmission. Wait 2.0 s for any node to break the silence. If a frame is observed, return to Repeat. Else transition to Bus-Sleep. |
| **Ready-Sleep** | No NM transmission, but listening. Other nodes' NM messages keep cluster alive. After 1.0 s with no NM traffic, enter Prepare-Bus-Sleep. |
| **Normal-Operation** | Transmit NM message every 200 ms. Application is operating. |
| **Repeat-Message** | Transmit NM message every 200 ms for 1.0 s after waking, regardless of application state. Ensures cluster membership announcement. |

### 9.3 Wake-Up to Active Sequence

1. Sleeping nodes' transceivers detect a matching WUF
2. Transceivers wake their MCUs
3. Each woken node enters Repeat-Message and transmits NM for 1.0 s
4. Each node either:
   - Continues to Normal-Operation if its application has work to do, or
   - Transitions to Ready-Sleep if it has no pending work
5. If all woken nodes reach Ready-Sleep with no traffic for 1.0 s, the cluster proceeds toward Bus-Sleep again

### 9.4 Sleep Coordination Failure Modes

If a node fails to transmit NM but has pending application work, other nodes may incorrectly initiate cluster sleep. Mitigations:

- Application work that requires the bus shall keep the node in Normal-Operation
- Watchdog timers detect stuck Ready-Sleep states
- The gateway acts as cluster monitor and may rebroadcast a WUF if it detects premature sleep

This is a known weakness of the AUTOSAR model. Field validation on prototype hardware will determine whether additional safeguards are needed.

---

## 10. Frame Error Counter

Per ISO 11898-2:2016 §5.9.4.4, a transceiver in selective wake mode increments a Frame Error Counter (FEC) when it observes a CAN frame that fails validation (CRC, stuff, or form errors through the CRC delimiter). When FEC reaches the configured threshold, the transceiver suspends WUF detection until the bus quiets.

### 10.1 Pelorus FEC Configuration

Pelorus implementations shall configure the transceiver FEC threshold to **31** errors. This is the default for most compliant transceivers and provides adequate margin against transient bus disturbances without false-positive sleep faults.

### 10.2 FEC Reset Conditions

The FEC resets to zero when:

- A valid CAN frame is observed (regardless of WUF match)
- The host MCU explicitly resets the counter via the transceiver's SPI interface
- The transceiver re-enters normal mode

### 10.3 Recovery from FEC Saturation

If the FEC saturates (typically because the bus is severely degraded), the transceiver enters a fault-tolerant state that does not wake the MCU. The bus must quiet, and the host must explicitly reset the counter, before WUF detection resumes. This is handled automatically by transceiver hardware.

Implementers should not attempt to bypass FEC. It is the primary defense against errant noise causing battery drain through repeated false wake-ups.

---

## 11. Bus Biasing

A CAN bus requires active biasing — at least one node driving the bus toward the recessive state — for proper signal integrity.

### 11.1 Biasing Responsibility

In a Pelorus network with at least one Active node, that node's transceiver provides bus biasing.

When **all** nodes are in Sleep or Deep Sleep, the bus is unbiased. WUFs still propagate correctly because:

- The transmitting node (the one originating the WUF) provides biasing during its transmission
- Receiving transceivers in selective wake mode synchronize to the WUF's edges from the unbiased state
- ISO 11898-2:2016 §5.10 specifies the unpowered-bus reception requirements

### 11.2 Biasing Implications for Repeaters

Repeater nodes (see [02-physical-layer.md §5.3](./02-physical-layer.md)) bridge two segments. Each segment requires independent biasing. A repeater shall maintain at least one transceiver in Standby (not Sleep) on each segment whenever any node on either segment is Active. If both segments reach coordinated Bus-Sleep, the repeater itself may sleep.

### 11.3 No Pelorus-Specific Biasing Components

Pelorus does not specify external bias resistors beyond what compliant CAN FD transceivers integrate. The split termination defined in [02-physical-layer.md §6](./02-physical-layer.md) is sufficient.

---

## 12. Implementation Checklist

For developers integrating Pelorus power management into a node:

**Hardware**

- [ ] Selected transceiver complies with ISO 11898-2:2016 §5.9.4 selective wake-up
- [ ] Transceiver supports CAN FD passive at ≥1 Mbit/s data phase
- [ ] Standby current ≤ 10 µA at the transceiver
- [ ] MCU sleep current is consistent with Sleep state target (≤100 µA non-isolated, ≤200 µA isolated)
- [ ] Reverse polarity protection per [02-physical-layer.md §8.2](./02-physical-layer.md)
- [ ] Voltage regulation supports 9–32 V continuous (40 V transient)

**Firmware**

- [ ] Configures transceiver WUF identifier to PGN 0x0FF80
- [ ] Configures data mask for the device's functional group memberships
- [ ] Implements NM state machine per §9
- [ ] Transmits NM at 200 ms cadence in Normal-Operation
- [ ] Transitions to Sleep only via coordinated cluster sleep
- [ ] Resets FEC and recovers from saturation per §10
- [ ] Declares wake-up latency for each supported group

**Validation**

- [ ] Measured Sleep current matches declared budget within ±10%
- [ ] Wake-up latency for each group is measured and documented
- [ ] Cluster sleep coordination verified with at least three nodes
- [ ] Behavior under bus-error storms verified (FEC saturation recovery)

---

## 13. Open Items

These remain unresolved and are tracked in [TODO.md](../TODO.md).

- Final PGN assignments for WUF and NM (currently candidates 0x0FF80 / 0x0FF81; ratification in [07-pgn-registry.md](./07-pgn-registry.md))
- NM cadence values pending prototype-hardware wake-up latency measurement
- Vendor-specific PNC bit allocation policy (bits 6–63)
- Multi-segment cluster coordination across repeaters (interaction with [10-repeater-specification.md](./10-repeater-specification.md))
- Behavior under partial bus failure (one segment alive, another silent)
- Conformance test fixtures (reference WUF generator, reference NM master)

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).
