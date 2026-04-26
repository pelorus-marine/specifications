# Pelorus Core — Hardware Design Guide

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document provides **non-normative** guidance for Pelorus Core hardware: component selection, schematic patterns, PCB layout, EMC, and mechanical practice. **Normative** electrical and isolation requirements remain in [02-physical-layer.md](./02-physical-layer.md). Locked hardware posture (repairability, coating, isolation tiers) is summarized in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary); acceptance criteria and detailed patterns are in this document.

Prerequisites: [02-physical-layer.md](./02-physical-layer.md), [04-power-management.md](./04-power-management.md), [08-network-architecture.md](./08-network-architecture.md), [09-gateway-specification.md](./09-gateway-specification.md), [10-repeater-specification.md](./10-repeater-specification.md).

---

## 1. Component Selection

### Required / Recommended Parts

**CAN FD Transceiver**
- Must support ISO 11898-2:2016 partial networking with selective wake-up
- Minimum data-phase support: 1 Mbit/s (500 kbit/s is used)
- Standby current ≤ 10 µA in selective wake mode
- Recommended: NXP TJA1145/TJA1146, Microchip ATA6570, TI TCAN1145-Q1

**Galvanic Isolation**
- Mandatory for high-power or motor-interfacing devices
- Recommended for all devices in harsh electrical environments
- Use digital isolators (e.g. ADuM1201, ISO774x) or isolated transceivers
- Isolated DC/DC converter for power (target standby < 200 µA)

**Microcontroller**
- Rust-first, `no_std` capable
- At least one CAN FD peripheral (or external controller)
- Low-power modes that allow selective wake-up
- Sufficient flash/RAM for binding table cache and power state machine

**Connectors**
- All external connections: M12 A-coded 5-pin (male on device, female on cable)
- IP67/IP68 rated, metal or high-quality plastic

**Power Protection**
- Reverse polarity protection (mandatory)
- Transient suppression (TVS diodes on power and CAN lines)
- Fuse or resettable PTC on power input

---

## 2. Schematic Patterns

### Basic Node Schematic
- Power input → reverse polarity + TVS → isolated DC/DC (if required) → LDO/regulators
- CAN bus: transceiver → optional isolator → MCU CAN FD peripheral
- Split termination (2 × 60 Ω + 4.7 nF C0G) at each end of every segment
- Bus biasing network per `02-physical-layer.md`

### Repeater Schematic
- Multiple isolated CAN ports (each with its own transceiver + isolator)
- Common power domain or per-port isolation as needed
- MCU coordinates forwarding and power management

### High-Power Device (e.g. autopilot, windlass interface)
- Mandatory galvanic isolation on both power and CAN
- Separate isolated supply for high-current sections

---

## 3. PCB Layout Guidelines

- Keep CAN bus traces short and differential
- Separate digital and power planes where possible
- Guard rings and stitching vias around CAN traces
- Place termination components close to connectors
- Conformal coating mandatory on finished boards (acrylic or polyurethane)
- Component placement for field repairability (no glued-down parts, standard footprints)

---

## 4. EMC and Environmental Requirements

- Conform to marine EMC standards (IEC 60945 or equivalent)
- Conducted and radiated emissions/immunity testing required
- Conformal coating on all boards
- Enclosures: IP68, machined aluminum or high-quality marine-grade plastic
- Temperature range: –25 °C to +70 °C operating
- Vibration and shock per MIL-STD-810 or equivalent marine practice

---

## 5. Mechanical and Repairability

- All devices must be field-serviceable with basic tools
- Modular construction preferred (separate PCB, enclosure, connectors)
- Clear labeling and silkscreen for troubleshooting
- Spare parts and repair documentation shall be publicly available

---

## 6. Validation and Testing

- Prototype boards shall be tested on a real Pelorus network
- Real-world current measurements in all power states
- Wake-up latency characterization
- EMC pre-compliance on actual marine cabling
- Long-term liveaboard testing on a real vessel

---

## 7. Open Items (to be resolved before v1.0 promotion)

- Reference schematic and PCB examples (KiCad)
- Bill-of-materials templates for common node types
- Detailed EMC test plan
- Conformal coating application guidelines
- Repairability checklist and minimum warranty targets

---

*This document, together with documents 01–11, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*