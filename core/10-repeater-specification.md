# Pelorus Core — Repeater Specification

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **normative** functional specification for Pelorus Core repeater nodes. Segment limits and hop-count rules are specified in [08-network-architecture.md](./08-network-architecture.md); physical port requirements in [02-physical-layer.md](./02-physical-layer.md). Locked repeater policy in brief: [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary).

---

## 1. Repeater Requirements

Every repeater node must meet the following:

- **Galvanic isolation:** Full isolation between every pair of connected segments.
- **Transparent forwarding:** Every valid CAN FD frame received on one port shall be retransmitted on all other ports without modification to identifier, data, or priority.
- **Fault containment:** A short, open, or excessive error state on one segment shall not propagate to other segments.
- **Power management:** Full compliance with selective wake-up, PNC masking, and the four power states (Active / Standby / Sleep / Deep Sleep) per `04-power-management.md`.
- **Addressing:** Must successfully claim a unique source address per `05-addressing.md` before forwarding application data.
- **Low-power operation:** Standby/sleep current targets identical to other Pelorus devices.
- **Connector:** M12 A-coded 5-pin on all ports (per `02-physical-layer.md`).

---

## 2. Optional Features

The following features are permitted but not required for basic repeater compliance:

- Configurable filtering (to reduce unnecessary traffic between segments)
- Power injection / pass-through capability
- Diagnostic LEDs or status reporting
- Web-based or local configuration interface (via gateway)

---

## 3. Topology and Hop Rules

- Repeaters shall be used to create isolated segments when a vessel exceeds 30 m or 50 nodes.
- In the recommended star topology, repeaters connect directly to a central gateway.
- Linear or tree repeater chains are allowed but must respect the **maximum 4-hop rule** between any two endpoints.
- Repeaters count as one hop each.

---

## 4. Interaction with Other Pelorus Components

- **Physical Layer:** All ports comply with `02-physical-layer.md` (250/500 kbit/s, split termination, M12 A-coded).
- **Addressing:** Repeaters claim addresses and participate in conflict resolution (`05-addressing.md`).
- **Power Management:** Repeaters forward WUF (0x0FF80) and NM (0x0FF81) messages and respect PNC masks (`04-power-management.md` and `07-dcid-registry.md`).
- **Signal Catalog & Binding:** Binding table and semantic mapping are network-wide; repeaters forward the binding table DCID transparently (`06-signal-catalog.md`).
- **DCID Registry:** All Pelorus DCIDs are forwarded without alteration (`07-dcid-registry.md`).
- **Network Architecture:** Repeaters enable the multi-segment and star topologies defined in `08-network-architecture.md`.
- **Gateway:** Repeaters work seamlessly with the central gateway defined in `09-gateway-specification.md`.

---

## 5. Open Items (to be resolved before v1.0 promotion)

- Exact filtering configuration format and rules
- Repeater-specific diagnostic and fault-reporting DCIDs
- Conformance test plan for isolation, fault containment, and hop-limit behavior
- Redundancy and failover behavior for multiple repeaters
- Minimum and maximum number of ports per repeater device

---

*This document, together with documents 01–09, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*