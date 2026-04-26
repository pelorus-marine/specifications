# Pelorus — Overview

**Version:** 0.1 Draft
**Last Updated:** April 26, 2026
**Status:** Pre-specification

---

## About This Document

This is the entry point to the Pelorus specification. It states what Pelorus is, what it replaces, how the protocol stack is divided, and which documents to read next. It is non-normative — every concrete requirement lives in a downstream document.

Read this first. Then go to [02-physical-layer.md](./02-physical-layer.md) for hardware-level requirements or [04-power-management.md](./04-power-management.md) for selective wake-up and power state behavior.

---

## 1. What Pelorus Is

Pelorus is an open marine data network standard. It is CAN FD-based for safety-critical instrumentation and Ethernet-based for high-bandwidth media. It is designed to install on existing NMEA 2000 cabling and connectors, to operate cleanly without an internet connection, and to draw single-digit milliamps at anchor.

Pelorus is a specification, not a product. Reference implementations exist in Rust. Commercial products that comply with the specification can use the "Pelorus Core Compatible" branding without a license fee, an NDA, or third-party certification.

The specification, the reference implementations, the test fixtures, and the documentation are all released under permissive open licenses. Documents are CC BY 4.0; code is MIT or Apache 2.0.

---

## 2. The Problem

NMEA 2000 is technically sound at its core but trapped by twenty years of backward compatibility and closed-ecosystem incentives. The specific weaknesses Pelorus addresses:

- **Closed protocol** — proprietary PGNs, paid certification, NDA requirements to obtain the specification
- **Always-on power** — no selective device sleep; instrument suite draws 2-3 A continuously even at anchor, consuming 24-36 Ah overnight on a typical vessel
- **Single-segment topology** — backbone failure takes down the entire network
- **Aging physical layer** — locked to classical CAN at 250 kbit/s; cannot migrate without breaking installed equipment
- **No diagnostic transparency** — sailors cannot debug their own networks
- **Vendor lock-in tactics** — proprietary extensions break interoperability between brands

Bandwidth is **not** the headline problem. NMEA 2000 at 250 kbit/s is adequate for GPS, depth, wind, heading, AIS, and engine data. Pelorus does not exist to make instrument data faster. It exists to make it open, reliable, power-aware, and debuggable. Higher bandwidth is delivered by a separate Ethernet layer for the use cases that need it.

---

## 3. Two-Layer Architecture

Pelorus has two distinct physical layers serving different traffic classes. They are bridged by gateway nodes but operate as independent physical buses.

### 3.1 Pelorus Core (CAN FD)

Safety-critical instrumentation backbone. Real-time, deterministic, reliable. Replaces NMEA 2000 functionality on a vessel.

- CAN FD at 250 kbit/s arbitration / 500 kbit/s data phase
- 64-byte frames eliminate Fast Packet for current NMEA 2000 message types
- ISO 11898-2:2016 partial networking with selective wake-up
- M12 A-coded 5-pin connectors, NMEA 2000 micro cable, NMEA 2000-style installation practice
- Linear bus per segment, repeater nodes for vessels exceeding 30 m
- Segmented architecture for fault containment

### 3.2 Pelorus Stream (Ethernet)

High-bandwidth, non-safety-critical layer. Radar, sonar, video, cloud connectivity.

- M12 D-coded 4-pin (100 Mbit/s) recommended, X-coded reserved for future Gigabit profiles
- Connector compatibility with OneNet cable infrastructure (without protocol compatibility)
- Protocol stack, PoE strategy, and switch architecture are largely undecided as of v0.1

This document and the rest of the v0.1 specification focus on Pelorus Core. Pelorus Stream design is deferred until Core is stable and validated on real hardware.

---

## 4. Coexistence with NMEA 2000

Pelorus Core uses identical connectors and cable to NMEA 2000 micro. The two networks are **not** electrically interoperable on the same wire — different bit rates and CAN FD frames are not recognized by classical CAN transceivers. Cross-connecting cables between the two networks results in a non-functional bus but does not damage equipment.

A vessel typically runs both networks during transition: NMEA 2000 for legacy devices, Pelorus Core for new equipment, with a gateway node bridging selected messages between them. The gateway handles PGN translation, instance binding, and rate adaptation. See [09-gateway-specification.md](./09-gateway-specification.md) when drafted.

Visual differentiation (Pelorus marine blue cable jackets, port labeling, distinctive terminator caps) is recommended but not mandatory.

---

## 5. v1.0 Scope

The v1.0 specification covers Pelorus Core only. The minimum viable specification consists of:

| # | Document | Purpose |
|---|---|---|
| 01 | [01-overview.md](./01-overview.md) | This document |
| 02 | [02-physical-layer.md](./02-physical-layer.md) | Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation |
| 03 | 03-data-link-layer.md | CAN FD frame format usage, message addressing, error handling |
| 04 | [04-power-management.md](./04-power-management.md) | Selective wake-up, partial network clusters, power states, network management |
| 05 | 05-addressing.md | Source address claiming, conflict resolution, device identification |
| 06 | 06-signal-catalog.md | VSS-syntax catalog format, `Vessel.*` data model, instance handling |
| 07 | 07-pgn-registry.md | Specific PGN assignments and definitions |

Tier 2 (network architecture, gateway, repeater) and Tier 3 (implementation guidance) documents extend the core but are not required for an interoperable v1.0 device. See [00-document-index.md](./00-document-index.md) for the full document list.

### Explicitly Deferred From v1.0

The following were considered and held for later versions. Rationale is captured in [ARCHITECTURE.md](../ARCHITECTURE.md) §5.

- Higher data phase rates (1 Mbit/s, 2 Mbit/s) — held for v2.0+
- Auto-negotiation of bit rates — held indefinitely; static profile is correct for v1.0
- Fast Packet support — not adopted; 64-byte CAN FD frames cover existing NMEA 2000 PGNs
- Mandated universal galvanic isolation — replaced by tiered requirement (mandatory for high-power, optional for low-power sensors)
- Signal K as core component — treated as one possible app-level consumer, not part of the core stack
- Pelorus Stream protocol stack — design deferred until Core is validated

---

## 6. Design Principles

These guide every concrete decision in downstream documents.

- **Sailor-first.** Every design decision asks "what is best for the sailor at sea" before "what is best for the manufacturer."
- **Reliability over features.** A device that works for ten years beats a device with twenty features that fails after three.
- **Power awareness as architecture.** Boats are not connected to the grid. Power management is part of the protocol, not an afterthought.
- **Open all the way down.** Specification, reference implementations, test fixtures, documentation. No purchases required to participate.
- **Static and debuggable for v1.0.** Auto-negotiation, dynamic reconfiguration, and complex state machines are deferred. Static profiles, fixed bit rates, and simple state machines win.
- **Honest about tradeoffs.** Patent encumbrances, unresolved questions, and design limitations are documented openly in each specification document's Open Items section.

---

## 7. Status and Stability

The v0.1 specification is pre-release. Concrete decisions are locked (see [ARCHITECTURE.md](../ARCHITECTURE.md) §4) but document text is under active revision and field validation has not begun. Hardware prototypes do not yet exist.

Compatibility commitment for v1.0:

- Bit rate profile (250 kbit/s arbitration / 500 kbit/s data) is permanent for the v1.0 line
- Connector type and pinout (M12 A-coded, NMEA 2000 micro) are permanent
- Frame format (CAN FD per ISO 11898-1:2015, no Fast Packet) is permanent
- Power state model and selective wake-up behavior may refine before v1.0 is finalized
- PGN assignments and the signal catalog are open and will change before v1.0

Implementations targeting v0.x should expect to update before v1.0 ships.

---

## 8. Where to Go Next

| If you want to... | Read |
|---|---|
| Understand the hardware-level requirements | [02-physical-layer.md](./02-physical-layer.md) |
| Implement selective wake-up and power management | [04-power-management.md](./04-power-management.md) |
| See what is decided and why | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| See what is open and unresolved | Section 6 of [ARCHITECTURE.md](../ARCHITECTURE.md) and the [TODO.md](../TODO.md) |
| Track specification document status | [00-document-index.md](./00-document-index.md) |

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).
