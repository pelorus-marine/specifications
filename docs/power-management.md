# Pelorus Power Management — Developer Reference

**Version:** 0.3 Draft (ISO 11898-2:2016 validated)  
**Last Updated:** April 25, 2026  
**Status:** Pre-specification

---

## About This Document

This document provides what an embedded developer needs to implement Pelorus-compatible power management on CAN FD networks, citing only freely accessible reference materials. The Pelorus design adopts the automotive industry's selective wake-up and partial networking mechanisms, adapted for marine use.

**Core philosophy:** every Pelorus node should consume only the power it needs to do its current job. A boat at anchor with only the GNSS-driven anchor watch active should draw single-digit milliamps across the entire network, not the 2-3 amps typical of NMEA 2000.

**ISO 11898-2:2016 Validation Note (v0.3):** This revision has been cross-checked against the full ISO 11898-2:2016 standard (Sections 5.9 and 5.10, Tables 18–20, Figures 6–11). All WUF format, matching rules, frame error counter, bus biasing, and timing claims are accurate and normative. No deviations were found.

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
