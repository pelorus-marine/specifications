# Pelorus Architecture and Decisions

**Purpose:** This document is the durable record of architectural decisions made for the Pelorus Marine network standard. It exists so that any future contributor can read it and pick up the work without rederiving the reasoning.

**Last Updated:** April 26, 2026
**Status:** Living document — update as decisions evolve.

This document is **not** part of the normative Pelorus specification. It is the project's working memory. Locked technical decisions (Section 4) are mirrored in the relevant numbered specification documents; this file records *why* each decision was made and which alternatives were rejected.

---

## How To Use This Document

If you are a new contributor, **read this document first** before making any architectural recommendations. The decisions documented here are the result of substantial deliberation. Do not relitigate them without explicit direction from the project maintainer.

If a decision needs to change, update this document with the new decision and the reasoning that motivated the change.

---

## 1. Project Context

**Founder:** René Herrero

**Project mission:** An open marine data network standard. CAN FD-based, Rust-first, designed for reliability offshore. A genuine alternative to the closed, vendor-controlled legacy marine data ecosystem.

**Strategic philosophy:**

- Everything is given away for free (open source)
- Quality over speed; depth over breadth
- Reputation-based community building over marketing
- Sailor wellbeing prioritized over vendor convenience

**Brand architecture:**

- **SevenSeas (sevenseas.io)** — community face for the project
- **Pelorus** — the open standard, hosted at sevenseas.io/pelorus
- **github.com/pelorus-marine** — the technical home of the standard
- Org name: pelorus-marine (more descriptive than just "pelorus", which is already taken on GitHub)
- The protocol is "Pelorus" in conversation; "pelorus-marine" in URLs and identifiers

---

## 2. The Problem Pelorus Solves

The legacy marine data ecosystem has known weaknesses that Pelorus addresses:

- **Closed protocol** — proprietary PGNs, expensive certification, NDA requirements
- **Always-on power model** — no selective device sleep, wastes battery overnight
- **Aging infrastructure** — locked to classical CAN at 250 kbit/s, can't migrate due to backward compatibility
- **Single-bus reliability** — backbone failure takes down everything
- **No diagnostic transparency** — sailors can't debug their own networks
- **Vendor lock-in tactics** — proprietary extensions break interoperability
- **Bandwidth limitations** — for radar, sonar, video (though instrumentation is fine at 250 kbit/s)

Notably, **bandwidth is NOT the headline problem** for typical instrumentation use. The legacy marine bus at 250 kbit/s is actually adequate for GPS, depth, wind, heading, AIS, and engine data. The real problems are reliability, openness, power consumption, and vendor behavior. Pelorus's value proposition centers on those, not on raw speed.

---

## 3. Two-Layer Architecture

Pelorus has two distinct physical layers serving different use cases.

### Pelorus Core (CAN FD)

Safety-critical instrumentation backbone. Real-time, deterministic, reliable. Replaces legacy marine instrumentation functionality.

### Pelorus Stream (Ethernet)

High-bandwidth non-critical layer. Radar, sonar, video, cloud connectivity. Uses M12 D-coded for compatibility with established industrial marine Ethernet cabling (without protocol-level interoperability with any legacy marine Ethernet protocol).

**These layers are bridged by gateway nodes but operate as separate physical buses.** This is intentional and correct — each layer is optimized for what it does well.

This document focuses primarily on Pelorus Core because that's where most decisions have been made. Pelorus Stream design is largely deferred.

---

## 4. Pelorus Core Decisions (Locked)

### 4.1 Bit Rates

- Arbitration phase: **250 kbit/s** (identical to the legacy marine data ecosystem)
- Data phase: **500 kbit/s**
- Frame size: up to **64 bytes** (CAN FD)
- **No Fast Packet** — all current legacy marine PGNs fit in one or two CAN FD frames

### 4.2 Physical Layer

- **Connector:** M12 A-coded 5-pin, identical pinout to legacy marine micro
- **Cable:** legacy marine micro standard (mid and mini optional for special cases)
- **Topology:** Linear bus with T-connector drops, legacy-marine-style
- **Maximum stub:** 6 m (preserves legacy marine installation practice)
- **Maximum segment:** 30 m
- **Maximum nodes per segment:** 50
- **Termination:** Split termination (two 60Ω resistors with 4.7 nF C0G/NP0 capacitor at midpoint)
- **Power input:** 9–32 V (covers 12 V and 24 V boat systems natively)
- **Reverse polarity protection:** Mandatory

### 4.3 Transceiver Requirements

- ISO 11898-2:2016 partial networking with selective wake-up
- CAN FD passive support at minimum 1 Mbit/s
- Standby current ≤10 µA in selective wake mode
- **No SIC required** — 500 kbit/s data phase doesn't push signal integrity limits
- Compliant parts include: NXP TJA1145, TJA1146, NCA1145B; Microchip ATA6570; TI TCAN1145-Q1

### 4.4 Galvanic Isolation

- **Mandatory** for devices >100 mA active or interfacing high-power systems (autopilots, motors, engine systems, solenoids)
- **Strongly recommended** for sensor-only low-power devices in harsh electrical environments
- **Optional** for low-power sensor-only devices in benign environments

### 4.5 Segmentation for Larger Vessels

- Single segment limit: 30 m
- Vessels exceeding this use **repeater nodes** to create multiple segments
- Repeaters: galvanic isolation between segments, transparent CAN FD frame forwarding, optional filtering, fault containment, may serve as power injection points
- Maximum 4 repeater hops between any two endpoints
- Star topology with central gateway is the recommended pattern for large vessels

### 4.6 Power Management

- ISO 11898-2:2016 partial networking with selective wake-up
- Four power states defined: Active, Standby, Sleep, Deep Sleep
- Sleep state target: ≤100 µA (non-isolated), ≤200 µA (isolated)
- Deep Sleep target: ≤10 µA
- Marine functional groups: anchor_watch, underway, engine, comms, domestic, storm
- See `04-power-management.md` for full specification

### 4.7 Patent Considerations

- ISO 11898-2:2016 selective wake-up function involves patents
- Patent holders include Audi, BMW, Continental, DENSO, Elmos, NXP, Renesas, Bosch, ST, VW
- RAND licensing committed by patent holders
- **Open source reference implementations and personal use are lower risk**
- **Commercial Pelorus products implementing selective wake-up require maritime IP attorney review**
- Basic wake-up (Section 5.9.2) and WUP wake-up (Section 5.9.3) do not have the same patent disclosure

---

## 5. Decisions Considered and Rejected

These decisions were considered seriously and rejected. Do not propose them again without strong new reasoning.

### 5.1 Higher Data Phase Rates (1 Mbit/s, 2 Mbit/s)

**Rejected** because:

- Pushes signal integrity limits with 6 m stubs
- Requires SIC transceivers (more expensive, fewer suppliers)
- Bandwidth wasn't the headline problem anyway
- 500 kbit/s gives 4–5× effective throughput improvement over the legacy marine data ecosystem with no signal integrity risk

### 5.2 B-coded or Other Differentiated M12 Connectors

**Rejected** because:

- Forces sailors to buy entirely new cable inventory
- Limited supplier base for B-coded compared to A-coded
- Cross-connection failure is non-destructive (just doesn't work)
- Adoption friction outweighs safety benefit
- Visual differentiation (color, labeling) is sufficient

### 5.3 Auto-Negotiation of Bit Rates

**Rejected for v1.0** because:

- Bootstrap problem requires bit-rate-independent signaling that CAN doesn't have
- Synchronization complexity during rate transitions
- Late joiner handling
- Heterogeneous network state machines
- Multi-vendor interoperability bugs (Ethernet auto-negotiation has decades of these)
- Combinatorial testing burden
- Diagnostic complexity for sailors

A single fixed profile (250k/500k) is correct for v1.0. Auto-negotiation could be added in v2.0+ if the ecosystem matures.

### 5.4 Mandated Universal Galvanic Isolation

**Rejected** because:

- Conflicts with deep sleep current targets (isolated DC/DC adds 50–200 µA standby)
- Significant BOM cost for low-power sensor devices
- Engineering reality: not all devices need isolation equally

Tiered approach (mandatory for high-power, optional for sensors) is the correct compromise.

### 5.5 Fast Packet Implementation

**Rejected** because:

- 64-byte CAN FD frames eliminate the need for the vast majority of legacy marine messages
- Fast Packet adds complex stateful firmware (sequence counters, reassembly buffers, source tracking)
- Reception state can corrupt under bus errors
- Single-frame messages have cleaner failure modes

### 5.6 Signal K as Core Component

**Rejected** because:

- JSON over TCP is unsuitable for safety-critical data (non-deterministic, OS-dependent)
- Foundation is wrong for what Pelorus is trying to be
- May be useful as one of many app-level consumers, but not as core infrastructure

### 5.7 DIP Switches for Profile Selection

**Rejected** because:

- Per-device configuration requires touching every device when network changes
- Silent failure when one device has wrong setting
- Hard to document for vessel-specific configurations
- Profile selection should be network-level, not device-level

### 5.8 Always-On Bus (Legacy Marine Power Model)

**Rejected** because:

- Wastes battery overnight (24–36 Ah typical)
- Modern automotive solved this with partial networking
- Power management is a first-class Pelorus feature, not an afterthought

### 5.9 Single Gateway as Profile Authority

**Rejected as sole mechanism** because:

- Creates single point of failure
- Resolution: layered approach with NV-stored last-known profile, bootstrap fallback, and gateway override (see `04-power-management.md`)

---

## 6. Open Questions

These questions are explicitly NOT yet decided. They remain open for future work.

### 6.1 Specification-Level

Items marked **Proposed** have a candidate design in a v0.x draft and are subject to revision after prototype validation. Items marked **Open** have no design yet.

- **Open** — Specific PGN number assignments for Pelorus (full registry). Candidate WUF and NM allocations exist (PGN 0x0FF80 / 0x0FF81 in `04-power-management.md` §7) but the broader registry is unwritten. The unverified `07-pgn-registry.md` conflicts with `03-data-link-layer.md` §4 on Pelorus PGN range.
- **Open** — Legacy-marine-to-Pelorus bridge gateway functional specification. The unverified `09-gateway-specification.md` is in the repository but has not been validated.
- **Open** — Pelorus Core repeater functional specification (filtering rules, fault handling). The unverified `10-repeater-specification.md` is in the repository but has not been validated.
- **Proposed** — NM message format and transmission cadence (200 ms, AUTOSAR CanNm-style state machine, four NM states). Specified in `04-power-management.md` §9. The unverified `07-pgn-registry.md` proposes a different NM payload layout that conflicts with `04`.
- **Proposed** — Reserved CAN identifier range for WUF transmissions (PGN 0x0FF80, priority 0). Specified in `04-power-management.md` §7.1. The Pelorus extension PGN range (0x0FF80–0x0FF8F per `03` vs 0x0FF80–0x0FFFF per `07`) needs reconciliation before final ratification.
- **Proposed** — PNC numbering scheme: bits 0–5 of WUF data byte 0 assigned to the six standard marine groups (anchor_watch, underway, engine, comms, domestic, storm); bits 6–63 reserved. Specified in `04-power-management.md` §6. Vendor-specific cluster registration process is still open.

### 6.2 Instance Binding Problem (CRITICAL)

**Status:** Identified as critical, not designed yet.

Instance numbers in legacy marine PGNs (e.g., engine 0 vs 1) have no inherent semantic meaning. The Pelorus Signal Catalog uses semantic paths (`Vessel.Propulsion.Engines[0]`) but the binding from a device's instance values to those paths is fragile.

Open sub-questions:

- Who owns the binding table and where does it live?
- How does Pelorus detect and handle instance drift?
- How does the catalog handle multi-instance signals — named semantics, typed arrays, or hybrid?
- What is the provisioning interface in the gateway web UI?
- What are acceptable failure modes and recovery paths?

**Required before designing:** Capture real legacy marine bus data from a representative liveaboard vessel using the canboat `analyzer` tool. Document every device, every PGN, every instance number. Write down real-world failure scenarios from the sailor's perspective.

### 6.3 Pelorus Stream (Ethernet Layer)

- Connector: M12 D-coded 4-pin (100 Mbit/s) recommended; X-coded 8-pin (Gigabit) reserved for future
- Protocol stack mostly undecided
- Power-over-Ethernet strategy undecided
- Switch architecture undecided

### 6.4 Data Model

- **Decided** — VSS (COVESA Vehicle Signal Specification) syntax for the Pelorus Signal Catalog.
- **Decided** — Standalone catalog under `Vessel.*` root, not contributed upstream to COVESA initially.
- **Speculative** — Mapping breakdown (~65% direct from the legacy marine data ecosystem, ~25% marine-specific extensions, ~10% filtered out). Numbers were guessed; will be replaced with measured data once the canboat capture from a representative vessel is available.
- **Partial** — `pelorus-xml-to-vss/` exists and generates `specifications/catalog/vessel.vspec` from the canboat PGN database (110 PGNs, 976 sensor leaves under `Vessel.PGN.*`). The semantic overlay layer that maps PGNs to canonical paths (`Vessel.Propulsion.Engines[0].Speed`, etc.) is not yet built. `vss-tools` validation is not yet wired up.
- **Open** — Custom Pelorus VSS attributes (`pgn`, `instance-field`, `pelorus-priority`) are referenced in the unverified `06-signal-catalog.md` but not formally defined.

### 6.5 Signal K Status

- Treated as one of many possible app-level consumers, not core
- A Signal K bridge module may be a Pelorus product, but Signal K is not part of core architecture
- Direction is to "forget about Signal K" for core decisions

### 6.6 Hardware Validation

- Real-world current measurements with prototype nodes
- Wake-up latency characterization
- EMC behavior on marine cabling
- Bus biasing performance on legacy marine standard cables

### 6.7 Business and Legal

- Maritime IP counsel review of selective wake-up patent landscape (deferred until commercial product near)
- Corporate structure (deferred until commercial activity begins)
- Designation of the founder's liveaboard vessel as a research platform / business asset

---

## 7. Strategic Decisions

### 7.1 Project Sequencing

- **Phase 1 (now):** Specifications, reference materials, community foundation
- **Phase 2:** Reference implementation in Rust (`pelorus-pm`, `pelorus-pgn`, etc.)
- **Phase 3:** Hardware prototyping on a real-world liveaboard vessel
- **Phase 4:** Community building, early adopter engagement
- **Phase 5:** Commercial considerations (only if traction justifies)

### 7.2 Target Markets

- **Primary visibility:** Racing community (highest standards, most vocal, drives reputation)
- **Primary volume:** Bluewater cruising community (values reliability over marginal performance)
- **Secondary:** Recreational sailors who care about open, repairable equipment
- These are the same sailors at different life stages, not separate markets

### 7.3 Differentiation From Existing Solutions

Pelorus is NOT competing on:

- Raw bandwidth (covered by Pelorus Stream / industrial Ethernet equivalents)
- Brand recognition (will be earned, not bought)
- Lowest price (Pelorus targets premium reliability)

Pelorus IS competing on:

- Reliability (segmented architecture, fault containment, repairability)
- Openness (full spec public, no NDAs, no certification fees)
- Power management (dramatic overnight current reduction)
- Diagnostic transparency (sailor-debuggable)
- Long-term repairability (modular, replaceable components, open documentation)

### 7.4 Competitor Reference Points

- **Victron Energy** — best existing model. Open-ish, trusted, premium-priced, present on both budget and luxury vessels. The brand position Pelorus aspires to.
- **B&G Hercules WTP** — what Pelorus should NOT be. Closed proprietary system at $7,999 for racing.
- **Signal K + OpenPlotter** — adjacent open-source community. Relevant for community building, but Signal K's foundation is wrong for Pelorus core.

### 7.5 Hardware Quality Targets

- **Five times the price of competitors, five times the warranty.** Warranty as performance bond.
- MIL-STD-810 environmental testing
- IP68 enclosures, machined aluminum, M12 connectors throughout
- Conformal coating mandatory
- Repairable by design (modular, field-serviceable, globally supportable)
- Real-world-tested — genuine liveaboard validation, not just lab testing

---

## 8. Working Style Notes

When working on Pelorus:

- **Push back.** Not looking for motivation; the devil hides in the details. Don't be encouraging. Be honest. Surface caveats. Identify failure modes. Propose alternatives.
- **Do not relitigate decided architecture.** The Section 4 decisions are locked. Do not propose alternatives unless explicitly invited to reconsider.
- **Cite sources.** When making claims about external standards, products, or behaviors, search and cite.
- **Prefer simplicity for v1.0.** Auto-negotiation, dynamic reconfiguration, complex state machines are deferred. Static, simple, debuggable wins for v1.0.
- **Direct, technical, no marketing fluff.** Markdown only unless otherwise requested. Documents should be useful artifacts, not impressive-looking ones.
- **Acknowledge what you don't know.** Honest uncertainty beats confident wrongness.
- **Time is the scarce resource.** Get to the point. Don't pad.

---

## 9. Continuation Instructions

If you are picking up Pelorus work from a cold start:

1. Read this document first.
2. Read `01-overview.md` for the high-level shape of the protocol.
3. Read `02-physical-layer.md`, `03-data-link-layer.md`, and `04-power-management.md` for the trusted technical core.
4. Consult `00-document-index.md` for per-document trust levels before treating any of `05`–`16` or the community docs as authoritative.
5. Confirm understanding of the current state by summarizing it back to the maintainer.
6. Ask what specific area to work on.
7. Do not propose architectural changes without explicit direction.

If proposing new work, prefer:

- Filling open questions in Section 6 (instance binding, PGN assignments, gateway specification)
- Implementing reference code (Rust crates, firmware, tooling)
- Improving existing documents based on review and validation
- Generating community-facing artifacts (README improvements, contribution guides, getting-started docs)

If you find errors in this document or the linked specifications, propose corrections. The project values accurate documents, not flattering ones.

---

*This document is the durable record. Update it when decisions evolve.*
