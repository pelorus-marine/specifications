# Pelorus Power Management — Developer Reference

**Version:** 0.2 Draft
**Last Updated:** April 25, 2026
**Status:** Pre-specification

---

## About This Document

This document provides what an embedded developer needs to implement Pelorus-compatible power management on CAN FD networks, citing only freely accessible reference materials. The Pelorus design adopts the automotive industry's selective wake-up and partial networking mechanisms, adapted for marine use.

**Core philosophy:** every Pelorus node should consume only the power it needs to do its current job. A boat at anchor with only the GNSS-driven anchor watch active should draw single-digit milliamps across the entire network, not the 2-3 amps typical of NMEA 2000.

---

## 1. The Problem

NMEA 2000 inherited its always-on power model from SAE J1939, designed for trucks where the alternator runs continuously. The standard provides no selective device power management.

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

Each node stores a target ID and an ID mask. A `1` in the mask means "don't care" for that bit position; a `0` means the bit must match.

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

```
Bit 0: anchor_watch group
Bit 1: underway group
Bit 2: engine group
Bit 3: comms group
Bit 4: domestic group
Bit 5: storm group
Bit 6: alarm group
Bit 7: maintenance group
```

A node belonging to anchor_watch and alarm groups configures data mask `0b01000001`. To wake anchor_watch, transmit a WUF with data byte `0b00000001`. To wake everything, transmit `0xFF`.

**Configuration trap:** if DLC ≠ 0 but all data mask bits are `0`, the node cannot be woken. By default, all data mask bits are `1`.

---

## 6. The Frame Error Counter (Safety-Critical)

ISO 11898-2:2016 Section 5.9.4.5 specifies a frame error counter mechanism that prevents nodes from staying asleep when the bus is in persistent error state. **This is mandatory for any compliant selective wake-up implementation.**

**Behavior:**

- Counter initialized to zero on selective wake-up activation and on `tsilence` expiration
- Incremented by 1 on bit stuffing, CRC, or CRC delimiter form errors
- Decremented by 1 on receipt of a valid Classical CAN frame (when counter > 0)
- Default threshold value is 32 (other values may be configurable)
- When counter reaches threshold, a wake-up shall happen immediately or on the next received WUP

**Why this matters:** Without this mechanism, a node could remain asleep indefinitely while the bus is unusable due to errors. The wake-up on error-counter overflow ensures the node can be activated to investigate or recover.

**The 4-frame ignore window:** After bias reaction time (`tBias`) elapses, an implementation may ignore up to 4 frames (or up to 8 when bit rate > 500 kbit/s) in CBFF and CEFF before it must start processing them as WUF candidates. This handles the bias settling period gracefully.

---

## 7. Bus Biasing State Machine

ISO 11898-2:2016 Section 5.10 defines the automatic voltage biasing state machine that any selective-wake-up-capable implementation must implement. The states are:

| State | Bus Biasing | Meaning |
|---|---|---|
| Ini | Inactive | Initial state on power-up |
| Wait | Inactive | Waiting after timeout |
| State 1 | Inactive | After dominant detected |
| State 2 | Inactive | After recessive following dominant |
| State 3 | Active | Normal operation |
| State 4 | Active | After receiving recessive |

### Critical Timing Parameters (from ISO 11898-2:2016 Table 20)

| Parameter | Symbol | Min | Max | Notes |
|---|---|---|---|---|
| CAN activity filter time, long | tFilter | 0.5 µs | 5.0 µs | |
| CAN activity filter time, short | tFilter | 0.15 µs | 1.8 µs | At higher bit rates |
| Wake-up timeout (optional) | tWake | 800 µs | 10000 µs | |
| Timeout for bus inactivity | tSilence | 0.6 s | 1.2 s | Triggers state reset |
| Bus bias reaction time | tBias | — | 250 µs | Until Vsym ≥ 0.1 |

**For Pelorus:** these timing values are normative requirements from ISO 11898-2:2016. Pelorus implementations must respect them to maintain interoperability with commercial CAN FD transceivers. The Pelorus specification will reference these parameters by symbol; the underlying values come from the standard.

### Wake-Up Filter Caveat

At higher bit rates, the activity filter time has practical implications. For example, at 500 kbit/s a wake-up message must carry at least three similar bit levels in a row to pass the filter reliably. Shorter filter times reduce wake-up message constraints but increase risk of unwanted wake-ups due to bus noise.

---

## 8. Node Power States

This section defines the Pelorus node power state model. It is informed by the ISO 11898-2 transceiver state machine and AUTOSAR CanNm, adapted for marine operational realities.

### Defined States

| State | Microcontroller | Transceiver | Bus Activity | Typical Current | Wake Latency |
|---|---|---|---|---|---|
| **Active** | Running | Normal mode | TX/RX | 20-100 mA | N/A |
| **Standby** | Stop mode, RAM retained | Low-power, selective wake | Monitoring for WUF | 1-5 mA | <1 ms |
| **Sleep** | Powered off or deep sleep | Low-power, selective wake | Monitoring for WUF | 10-100 µA | <10 ms |
| **Deep Sleep** | Powered off | Low-power, basic wake only | Bus biasing only | 1-10 µA | 50-300 µs + tBias |

### Pelorus Current Budget Targets

- Active: ≤200 mA at 12V (typical instrumentation node)
- Standby: ≤5 mA
- Sleep: ≤100 µA
- Deep Sleep: ≤10 µA

**Implication:** A 16-node Pelorus network in anchor watch mode (1 active node, 15 sleeping) draws approximately 200 mA + 15 × 0.1 mA = **201.5 mA**. Compared to NMEA 2000's typical 2-3 A, that is a **10-15× improvement** in overnight current draw.

---

## 9. Network Management Layer

Pelorus does not need full AUTOSAR — that is automotive complexity for problems Pelorus does not have. The core state machine and coordination concepts translate directly.

### Three Top-Level States

- **Bus-Sleep Mode** — all participating nodes are sleeping; transceiver wake-up monitoring is active
- **Network Mode** — at least one node is active and communicating; NM messages exchanged periodically
- **Prepare Bus-Sleep Mode** — transitional; all nodes have indicated readiness to sleep but a settle period must elapse before transitioning to Bus-Sleep

### Distributed Consensus

Any node can request the network to remain awake by transmitting NM messages. Any node can release the network when it no longer needs communication. The network transitions to Bus-Sleep Mode only when all participating nodes have released. **No single node is master** — this matches CAN's multi-master nature and is critical for safety. Loss of any single node never prevents others from continuing operation or sleeping.

### Partial Network Clusters (PNCs)

A PNC is a group of nodes that need to wake and sleep together. Marine examples:

- **PNC_ANCHOR_WATCH** — GNSS receiver, anchor alarm output, swing radius calculator
- **PNC_NAV_SUITE** — chartplotter, depth, wind, log, heading, autopilot
- **PNC_ENGINE** — engine monitor, fuel, alternator, starter battery
- **PNC_COMMS** — VHF, AIS, satellite

A node may belong to multiple PNCs. The GNSS receiver might be in both PNC_ANCHOR_WATCH and PNC_NAV_SUITE.

**Activation:** A PNC is active if any node in it has requested communication. All nodes in an active PNC must be awake.

**Release:** A PNC is inactive when all nodes have released. Nodes belonging only to inactive PNCs may sleep.

### Operational Modes

| Mode | Active PNCs | Description |
|---|---|---|
| `OFF` | None | Vessel in storage |
| `ANCHOR_WATCH` | ANCHOR_WATCH | At anchor, monitoring position |
| `HARBOR` | DOMESTIC | At dock, only domestic monitoring |
| `UNDERWAY_SAIL` | NAV_SUITE, COMMS | Sailing |
| `UNDERWAY_MOTOR` | NAV_SUITE, COMMS, ENGINE | Motoring |
| `STORM` | STORM, COMMS | Storm conditions, minimum essential |
| `MAINTENANCE` | All | Diagnostic and configuration mode |

Modes are operator-selected (typically via the gateway web interface) or automatically transitioned based on conditions (e.g., GPS speed > 0.5 knots transitions ANCHOR_WATCH to UNDERWAY).

---

## 10. Implementation Guidance

### Recommended Hardware

**CAN FD transceivers with partial networking:**
- NXP TJA1145 / TJA1145/FD — 2 Mbit/s CAN FD, ISO 11898-2:2016 compliant, mature
- Microchip ATA6570 — up to 5 Mbit/s, second-source for design diversity
- NXP UJA1168 — system basis chip (transceiver + voltage regulator + watchdog)

**Microcontrollers with CAN FD and low-power modes:**
- STM32L5/U5 series — Cortex-M33, hardware CAN FD, sub-µA standby with retention, mature Embassy support
- NXP S32K3 series — automotive grade, hardware CAN FD with native partial networking
- Nordic nRF54 — newer, extreme low-power capability

### Firmware Architecture

Each Pelorus node firmware implements:

1. **Boot path** — minimal initialization, read configuration from NV memory, configure transceiver for partial networking, enter Standby
2. **Wake path** — transceiver interrupt wakes MCU, brief work, return to Standby or Sleep
3. **Active path** — periodic NM message transmission while active, monitor for release conditions
4. **Configuration path** — NM-mediated configuration changes (group membership, mode preferences)
5. **Frame error counter** — implement per Section 6

The Rust embedded ecosystem with Embassy async naturally models the event-driven wake/work/sleep cycle.

### Pelorus Reference Implementation (Planned)

The `pelorus-pm` crate will provide:

- Transceiver driver abstraction (TJA1145 first)
- WUF construction and parsing
- Network management state machine
- PNC membership and release tracking
- Power state transitions
- Frame error counter mechanism
- Test fixtures using vcan virtual buses

Same standards as `dbc-rs`: `no_std` first, zero-copy where possible, `forbid(unsafe_code)`, comprehensive testing with property-based tests against the WUF specification.

---

## 11. Compliance Strategy

### Why Free Sources Suffice for Implementation

The NXP and Microchip transceivers must implement ISO 11898-2:2016 correctly to interoperate. Their datasheets describe implementable behavior in detail. Two independent vendors implementing the same standard identically provides strong cross-validation that the format described matches the ISO standard.

This mirrors how the Linux kernel CAN subsystem was developed — implemented from chip datasheets and application notes, with formal standards as background context but not as direct source material.

### What Pelorus Will Not Do

- Redistribute ISO specification text
- Derive specification language from ISO documents
- Require contributors to purchase standards

Pelorus specification text is original — it describes Pelorus behavior in Pelorus terms, informed by but not copied from external standards.

### What the Maintainer Will Do

- Purchase ISO 11898-2:2016 (~$200) for normative validation when finalizing v1.0
- Purchase ISO 16845-2 (~$200) if formal conformance testing is pursued
- Engage maritime IP counsel before any commercial product implementing selective wake-up ships (see Section 3)

---

## 12. Open Questions

The following items require resolution before the Pelorus power management specification reaches v1.0:

### Specification Items

- Reserved CAN identifier ranges for WUF transmissions
- PNC numbering scheme and registration process
- Operational mode definitions and transition rules
- NM message format and transmission cadence
- Configuration interface for PNC membership
- Decision on selective wake-up vs basic wake-up given patent landscape

### Hardware Validation

- Real-world current measurements with prototype nodes
- Wake-up latency characterization across transceiver families
- EMC behavior of partial networking on marine cabling
- Bus biasing performance with NMEA 2000 standard cables

### Marine-Specific Concerns

- Behavior during voltage transients (engine starting, alternator load dump)
- Recovery from extended power loss with NV state preservation
- Operator interface for mode selection
- Failure modes when individual nodes fail to honor sleep requests

### NMEA 2000 Bridge Integration

- The NMEA 2000 bridge cannot impose partial networking on legacy NMEA 2000 devices
- The Pelorus side of the bridge must respect partial networking
- Bridge node power management is a hybrid case requiring explicit design

### Patent Resolution

- Maritime IP counsel review of selective wake-up patent landscape
- Decision on commercial licensing approach
- Documentation of which Pelorus tier (open source / personal / commercial) uses which wake-up mechanism

---

## Appendix A: Source Cross-Validation

Each technical claim in this document is grounded in at least two independent sources:

- WUF format described identically in NXP TJA1145 datasheet, Microchip ATA6570 datasheet, NXP AH1203 application note, and NXP "Getting Started With CAN FD" presentation
- Frame error counter mechanism specified in ISO 11898-2:2016 Section 5.9.4.5 and implemented in TJA1145 (see datasheet error counter behavior)
- Bus biasing state machine described in ISO 11898-2:2016 Section 5.10, TJA1145 datasheet, and ATA6570 datasheet
- AUTOSAR CanNm state machine validated across three AUTOSAR releases (R20-11, R21-11, R23-11)

The patent disclosure in Section 3 is sourced directly from ISO 11898-2:2016 Introduction.

---

## Appendix B: Glossary

| Term | Definition |
|---|---|
| **CAN** | Controller Area Network — multi-master serial bus, ISO 11898 |
| **CAN FD** | CAN with Flexible Data-Rate — 64-byte frames, faster data phase, ISO 11898-1:2015 |
| **CanNm** | AUTOSAR CAN Network Management module |
| **CBFF** | Classical Base Frame Format (11-bit ID) |
| **CEFF** | Classical Extended Frame Format (29-bit ID) |
| **DLC** | Data Length Code — 4-bit field indicating frame data length |
| **FBFF/FEFF** | FD Base/Extended Frame Format |
| **FDF** | Flexible Data-rate Format bit |
| **HS-PMA** | High-Speed Physical Media Attachment |
| **NM** | Network Management |
| **PNC** | Partial Network Cluster — group of nodes that wake/sleep together |
| **RAND** | Reasonable And Non-Discriminatory (patent licensing terms) |
| **TRX** | Transceiver |
| **WUF** | Wake-Up Frame — CAN frame triggering selective node wake-up |
| **WUP** | Wake-Up Pattern — bus activity pattern triggering basic (non-selective) wake-up |

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).

Reference materials cited remain under their respective copyrights and licenses. URLs are for reader convenience; the Pelorus project does not host or redistribute external materials.
