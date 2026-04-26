# Pelorus Core — Installation Guide

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document provides **non-normative** installation guidance for Pelorus Core networks: planning, wiring, termination, power, repeaters, and commissioning. **Normative** physical and electrical rules are in [02-physical-layer.md](./02-physical-layer.md); the locked v1.0 installation policy is one line in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary).

Prerequisites: [02-physical-layer.md](./02-physical-layer.md), [08-network-architecture.md](./08-network-architecture.md), [10-repeater-specification.md](./10-repeater-specification.md), [12-hardware-design-guide.md](./12-hardware-design-guide.md).

---

## 1. Pre-Installation Planning

1. Measure the vessel and plan segments so backbone length, node count, and stub length stay within [08-network-architecture.md](./08-network-architecture.md) (summary in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary)).
2. Decide on topology:
   - Small vessels: single linear bus
   - Large vessels: recommended star topology with central gateway
3. Plan repeater locations (if needed) to respect the hop limit in [08-network-architecture.md](./08-network-architecture.md).
4. Identify power injection points and ensure 9–32 V DC supply with reverse polarity protection.
5. Decide on isolation requirements (mandatory for high-power devices).

---

## 2. Cabling and Connectors

- Use only cable and connectors specified in [02-physical-layer.md](./02-physical-layer.md) (typically LMDE-compatible micro cable, M12 A-coded 5-pin).
- Backbone runs between terminators; drops use T-connectors.
- Observe stub and backbone length limits in [08-network-architecture.md](./08-network-architecture.md).
- All connectors must be fully mated and sealed (IP67/IP68).
- Label every cable and drop clearly.

---

## 3. Termination and Bus Biasing

- Install split termination (two 60 Ω resistors + 4.7 nF C0G capacitor) at **both ends** of every segment.
- Do not use a single terminator or unterminated stubs.
- Verify termination resistance (≈120 Ω) between CAN_H and CAN_L with power off.

---

## 4. Power Distribution

- Power is supplied on the same cable as data (pins per `02-physical-layer.md`).
- Use fused or PTC-protected distribution blocks.
- Provide separate power injection points for long or heavily loaded segments.
- All devices must have reverse polarity protection.

---

## 5. Repeater and Gateway Installation

- Mount repeaters where they create clean isolated segments.
- In star topology, connect repeaters directly to the central gateway.
- The central gateway should be installed in an accessible, dry location with good ventilation and Wi-Fi/Ethernet access for the web UI.
- Ensure galvanic isolation is present on every repeater port.

---

## 6. Commissioning Steps

1. Install and terminate all segments before powering anything.
2. Power up one segment at a time and verify no bus errors.
3. Connect the gateway (if used) and confirm it claims an address.
4. Provision the binding table via the gateway web UI.
5. Test power management states (Active → Sleep → Wake).
6. Verify multi-segment forwarding and isolation.
7. Perform a full network test with all devices.

---

## 7. Troubleshooting Checklist

- Bus errors or no communication → check termination and stub lengths
- High standby current → verify isolation and transceiver sleep behavior
- Address conflicts → check NAME uniqueness
- Binding table not updating → verify gateway is powered and publishing NM messages
- Intermittent faults → inspect connectors and cable shielding

---

## 8. Open Items (to be resolved before v1.0 promotion)

- Detailed wiring diagrams and example layouts
- Recommended tools and test equipment list
- Step-by-step commissioning checklist with pass/fail criteria
- Common failure modes and sailor-level fixes
- Integration with existing LMDE cabling (bridge notes)

---

*This document, together with documents 01–13, completes the minimum viable specification for Pelorus Core reference implementations, hardware prototyping, and vessel installation.*