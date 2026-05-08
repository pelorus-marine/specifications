# Pelorus Core — Installation Guide

**Version:** 0.1 Draft  
**Last Updated:** May 4, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document provides **installation guidance** for Pelorus Core networks: planning, wiring, termination, power, repeaters, hubs, path redundancy, and commissioning. **Normative** physical and electrical rules are in [02-physical-layer.md](./02-physical-layer.md); **criticality and dual-bus** rules are normative in [17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md). Where this guide uses **SHALL**, it restates or points to those documents — it does not invent lighter requirements.

Prerequisites: [02-physical-layer.md](./02-physical-layer.md), [08-network-architecture.md](./08-network-architecture.md), [10-repeater-specification.md](./10-repeater-specification.md), [12-hardware-design-guide.md](./12-hardware-design-guide.md), [17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md).

---

## 1. Pre-Installation Planning

1. Measure the vessel and plan segments so backbone length, node count, and stub length stay within [08-network-architecture.md](./08-network-architecture.md) (summary in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary)).
2. Decide on topology:
   - Small vessels: single linear bus
   - Large vessels: recommended star topology with central gateway
3. Plan repeater locations (if needed) to respect the hop limit in [08-network-architecture.md](./08-network-architecture.md).
4. Identify power injection points and ensure 9–32 V DC supply with reverse polarity protection.
5. Decide on isolation requirements (mandatory for high-power devices).
6. Prepare a **critical zone map** per **[17](./17-criticality-and-redundant-paths.md)** §6: assign **C0 / C1 / C2** to each function and decide where **Bus A** and **Bus B** run.

---

## 2. Cabling and Connectors

- Use only cable and connectors specified in [02-physical-layer.md](./02-physical-layer.md) (typically LMDE-compatible micro cable, M12 A-coded 5-pin).
- Backbone runs between terminators; drops use T-connectors.
- Observe stub and backbone length limits in [08-network-architecture.md](./08-network-architecture.md).
- All connectors must be fully mated and sealed (IP67/IP68).
- Label every cable and drop clearly; for dual-bus domains, label **A** vs **B** at both ends and at every tee.

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
- Binding table not updating → verify gateway is powered; binding is **out of band** in v1.0 — use gateway UI, export/import, or diagnostic tool per **06** / **07** §4 (NM does **not** carry binding)
- Intermittent faults → inspect connectors and cable shielding
- One bus silent on dual-bus helm → check **Bus Health (0x0FF82)** on surviving bus; inspect failed backbone for opens, terminators, or transceiver damage (**17** §3 degraded mode)

---

## 8. Dual-bus (path redundancy) installation

For **C0** / **C1** zones per **17**:

- **SHALL** run **two** independent backbone pairs (Bus A, Bus B) per **02** §13; **SHALL NOT** route both through a single unprotected bundle through a single hazard zone without documenting residual risk on the critical zone map (**17** §5).
- **SHOULD** use separated cable trays / penetrations where feasible; crossing is acceptable only with mechanical protection documented in the map.
- **SHOULD** use independent fused feeds to Bus A and Bus B power injection points when the vessel DC distribution supports it.
- Commission Bus Health (**07** §1.3) on a test display before declaring the dual-bus domain complete.
- **`DISCARD_WINDOW` sizing:** Verify the receiver `DISCARD_WINDOW` against **[03 §6.4.3](./03-data-link-layer.md#643-discard_window-lower-bound-formula)**: `DISCARD_WINDOW >= 2 * H * L_hop + 2 * D_clk + safety_margin`. Floor is **50 ms**. If the dual-bus domain has more than two hops on either bus, or no Time Master (**[07 §1.4](./07-dcid-registry.md#14-time-sync-dcid-0x0ff83-optional)**), use a larger value and record it in the **critical zone map** (**[17 §6](./17-criticality-and-redundant-paths.md#6-critical-zone-map-and-conformance)**).

---

## 9. Open Items (to be resolved before v1.0 promotion)

- Detailed wiring diagrams and example layouts
- Recommended tools and test equipment list
- Step-by-step commissioning checklist with pass/fail criteria
- Common failure modes and sailor-level fixes
- Integration with existing LMDE cabling (bridge notes)

---

*This document, together with documents 01–13, completes the minimum viable specification for Pelorus Core reference implementations, hardware prototyping, and vessel installation.*