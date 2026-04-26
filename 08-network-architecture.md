# Pelorus Core — Network Architecture Specification

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification (normative for v1.0)

---

## About This Document

This document defines the network architecture for Pelorus Core, including segmentation, scaling to larger vessels, repeater behavior, and topology recommendations.

It builds directly on the locked physical-layer decisions in `02-physical-layer.md` and the addressing rules in `05-addressing.md`.

**Design decisions (locked):**
- Pelorus Core is a linear bus with T-drop topology on each segment (identical to legacy marine practice).
- A single segment is limited to 30 m and 50 nodes.
- Larger vessels must use repeater nodes to create multiple isolated segments.
- Repeaters provide galvanic isolation, transparent frame forwarding, optional filtering, and fault containment.
- Maximum 4 repeater hops between any two endpoints.
- Star topology with a central gateway is the recommended pattern for large vessels.

---

## 1. Single-Segment Limits

Every Pelorus Core segment shall observe these hard limits:

- **Maximum segment length:** 30 m (backbone cable between terminators)
- **Maximum nodes per segment:** 50
- **Maximum stub length:** 6 m (drop cable from T-connector to device)
- **Cable and connector:** M12 A-coded 5-pin, legacy marine micro standard (per `02-physical-layer.md`)
- **Termination:** Split termination at both ends of the segment
- **Power:** 9–32 V DC, reverse polarity protected

These limits ensure signal integrity at 250 kbit/s arbitration / 500 kbit/s data phase without requiring special transceivers.

---

## 2. Multi-Segment Networks and Repeaters

Vessels exceeding a single 30 m segment shall use **repeater nodes** to create multiple electrically isolated segments.

### Repeater Requirements
- Galvanic isolation between all connected segments (mandatory)
- Transparent forwarding of all valid CAN FD frames (no modification of identifier or data)
- Optional configurable filtering (to reduce unnecessary traffic between segments)
- Fault containment: a fault on one segment shall not propagate to others
- Optional power injection point (repeaters may supply or pass power)
- Low-power operation in sleep/deep-sleep states (per `04-power-management.md`)
- Must claim a valid source address (per `05-addressing.md`)

### Hop Limit
- Maximum **4 repeater hops** between any two endpoints on the network.
- This guarantees deterministic latency and prevents excessive propagation delay.

---

## 3. Recommended Topologies

### Small Vessels (< 30 m)
- Single segment, linear bus with T-drop topology (standard).

### Large Vessels
**Recommended:** Star topology with a central gateway node
- Central gateway acts as the hub
- Multiple repeater nodes connect directly to the gateway
- Each repeater creates one isolated segment
- All segments are bridged through the central gateway

This pattern:
- Minimizes hops (maximum 2 per path)
- Simplifies instance binding and power management
- Provides a natural location for the binding table authority and web UI
- Allows the gateway to act as the primary network management node

Alternative linear or tree topologies are permitted but not recommended for vessels requiring more than two segments.

---

## 4. Interaction with Other Pelorus Components

- **Addressing:** Repeaters and devices on all segments use the same 8-bit source address space and J1939-style address claiming (`05-addressing.md`).
- **Power Management:** Repeaters participate in selective wake-up using the same PNC and NM mechanisms (`04-power-management.md`).
- **Signal Catalog & Binding:** The binding table is network-wide and is published on every segment (`06-signal-catalog.md`).
- **PGNs:** All Pelorus PGNs (including WUF 0x0FF80 and NM 0x0FF81) are forwarded transparently by repeaters (`07-pgn-registry.md`).
- **Gateways:** The central gateway may also bridge to legacy marine networks; its behavior is defined in the future `09-gateway-specification.md`.

---

## 5. Open Items (to be resolved before v1.0 promotion)

- Exact repeater filtering rules and configuration interface
- Repeater address claiming and redundancy behavior
- Fault detection and reporting PGNs
- Conformance test procedures for multi-segment networks
- Maximum practical number of segments in a star topology

---

*This document, together with documents 01–07, completes the core architectural specification for Pelorus Core networks.*