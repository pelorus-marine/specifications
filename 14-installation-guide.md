# Pelorus Core — Installation Guide

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification (normative for v1.0)

---

## About This Document

This document provides practical instructions for installing a Pelorus Core network on a vessel. It covers planning, wiring, termination, power distribution, repeater placement, and basic commissioning.

It assumes you have read and understood the preceding specifications:
- `02-physical-layer.md` (cabling, connectors, termination, isolation)
- `08-network-architecture.md` (segment limits, repeaters, star topology)
- `10-repeater-specification.md` (repeater requirements)
- `12-hardware-design-guide.md` (hardware requirements)

**Design decisions (locked):**  
Installation must follow the physical and electrical rules in `02-physical-layer.md`. No deviations are permitted for v1.0.

---

## 1. Pre-Installation Planning

1. Measure the vessel and identify required segments (maximum 30 m per segment, 50 nodes per segment).
2. Decide on topology:
   - Small vessels: single linear bus
   - Large vessels: recommended star topology with central gateway
3. Plan repeater locations (if needed) to respect the 4-hop maximum.
4. Identify power injection points and ensure 9–32 V DC supply with reverse polarity protection.
5. Decide on isolation requirements (mandatory for high-power devices).

---

## 2. Cabling and Connectors

- Use only legacy-marine-compatible micro cable (M12 A-coded 5-pin).
- Backbone runs between terminators; drops use T-connectors.
- Maximum stub length: 6 m.
- Maximum backbone segment length: 30 m.
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
- Integration with existing legacy marine cabling (bridge notes)

---

*This document, together with documents 01–13, completes the minimum viable specification for Pelorus Core reference implementations, hardware prototyping, and vessel installation.*