# Pelorus Core — Physical Layer Specification

**Version:** 0.1 Draft  
**Last Updated:** May 4, 2026  
**Status:** Pre-specification  
**Trust:** Trusted

---

## About This Document

This document specifies the physical layer for Pelorus Core: bit rates, cabling, connectors, topology, transceiver requirements, power distribution, termination, and isolation. For stack context, coexistence with LMDE buses, and cross-document summaries, see [01-overview.md](./01-overview.md) ([§3 Two-layer architecture](./01-overview.md#3-two-layer-architecture), [§4 Coexistence](./01-overview.md#4-coexistence-with-the-legacy-marine-data-ecosystem), [§6 Design principles](./01-overview.md#6-design-principles), [§9 Cross-cutting decisions](./01-overview.md#9-cross-cutting-decisions-authoritative-summary)).

---

## 1. Scope and Compatibility Statement

This document defines the **normative** physical requirements for Pelorus Core segments. **Pelorus Core** segments use **CAN FD** (see §2). **LMDE** legacy segments discussed for coexistence use **Classical CAN (CAN 2.0)** application traffic on the same connector/cable plant but **must not** be merged on one segment with Pelorus CAN FD populations as specified. High-level scope and legacy coexistence: [01-overview.md §3–4](./01-overview.md#3-two-layer-architecture).

Pelorus Core segments shall:

- Use LMDE-compatible cabling, connectors, and installation practices where this document specifies them
- Operate as a **separate** electrical bus from classical-CAN LMDE networks (see [01-overview §4](./01-overview.md#4-coexistence-with-the-legacy-marine-data-ecosystem))
- Support scaling via repeater-isolated segments per [08-network-architecture.md](./08-network-architecture.md)

---

## 2. Bit Rate and Frame Format

### 2.1 Operating Bit Rates

| Phase | Rate | Notes |
|---|---|---|
| Arbitration phase | 250 kbit/s | Identical to the Legacy Marine Data Ecosystem |
| Data phase | 500 kbit/s | CAN FD speed boost |

### 2.2 Frame Format

Pelorus Core uses CAN FD frames per ISO 11898-1:2015:

- Maximum frame payload: 64 bytes
- 11-bit standard or 29-bit extended identifiers (29-bit recommended for J1939-style addressing)
- BRS (Bit Rate Switch) bit set to enable data phase speed boost
- ESI (Error State Indicator) bit set per CAN FD specification

### 2.3 No Fast Packet

Pelorus Core does not implement the Legacy Marine Data Ecosystem's Fast Packet protocol. Multi-frame messages exceeding 64 bytes use the J1939 Transport Protocol (TP) or are restructured into single-frame messages where possible. The vast majority of LMDE DCIDs (including all signals up to 100 bytes) fit in a single CAN FD frame, eliminating Fast Packet's complexity.

### 2.4 Rationale

The 250k/500k profile was chosen for the following reasons:

- Arbitration phase identical to the Legacy Marine Data Ecosystem preserves all LMDE physical layer characteristics including 6 m stub tolerance
- 500 kbit/s data phase provides ~4-5× effective throughput improvement over the Legacy Marine Data Ecosystem for substantial messages
- Signal integrity is well-margined at 500 kbit/s data phase, eliminating the need for Signal Improvement Capability (SIC) transceivers
- Standard CAN FD transceivers from multiple vendors support this profile
- 64-byte frames eliminate Fast Packet for all current LMDE message types

Higher data phase rates (1 Mbit/s, 2 Mbit/s) are reserved for future Pelorus Core profiles. v1.0 specifies 250k/500k as the single mandatory profile.

---

## 3. Cable Specification

### 3.1 Standard Cable

Pelorus Core uses LMDE micro cable as the standard cable specification. This cable is:

- Specified per DeviceNet thin cable / LMDE micro
- 24 AWG twisted pair for CAN signals
- 22 AWG conductors for power and ground
- Foil shield with drain wire
- Characteristic impedance 120Ω ±10%
- Capacitance approximately 12 pF/foot
- Available from multiple marine suppliers worldwide

### 3.2 Optional Cable Variants

For installations with extended power requirements, the following cable variants are permitted:

- **LMDE mid cable** — 20 AWG signal pair, 18 AWG power, 4A current capacity
- **LMDE mini cable** — 18 AWG signal pair, 16 AWG power, 8A current capacity

Mid and mini cable use different connector sizes than micro and require appropriate adapters at backbone-to-drop transitions. Implementers should default to micro cable unless specific power distribution requirements justify the larger sizes.

### 3.3 Cable Selection Guidance

For typical recreational vessel installations, micro cable is sufficient throughout. Mini backbone with micro drops is recommended for vessels exceeding 30m or installations with many high-power devices. Decision criteria:

- Total backbone current under 3A and segment length under 30m: micro throughout
- Total backbone current 3-4A: mid cable backbone with micro drops
- Total backbone current 4-8A or backbone over 30m: mini cable backbone with micro drops

---

## 4. Connector Specification

### 4.1 Connector Type

Pelorus Core uses M12 A-coded 5-pin connectors throughout, identical to LMDE micro:

- IEC 61076-2-101 compliant
- A-coded keying
- 5 contacts arranged per LMDE micro standard
- IP67/IP68 rated when mated and properly torqued
- Threaded coupling, 14mm thread pitch

### 4.2 Pinout

The pinout matches LMDE micro exactly:

| Pin | Signal | Wire color (per LMDE field practice) |
|---|---|---|
| 1 | Shield / Drain | Bare drain wire |
| 2 | NET-S (Power positive) | Red |
| 3 | NET-C (Power negative) | Black |
| 4 | CAN_H | White |
| 5 | CAN_L | Blue |

### 4.3 Cross-Connection Considerations

Because Pelorus Core uses identical connectors and pinout to the Legacy Marine Data Ecosystem, cables physically interchange between the two networks. Cross-connection between Pelorus Core and Legacy Marine Data Ecosystem buses results in a non-functional network (different bit rates, CAN FD frames not recognized by classical CAN transceivers) but does not damage equipment.

### 4.4 Recommended Visual Differentiation

To reduce cross-connection errors, Pelorus Core devices and cables should be visually distinguishable from the Legacy Marine Data Ecosystem:

- Pelorus Core ports on devices should be labeled "Pelorus Core" or "Pelorus" adjacent to each connector
- Pelorus-branded cables and accessories should use distinctive coloring (Pelorus marine blue recommended) on connector bodies, cable jackets, or strain relief boots
- Pelorus terminator caps should be visually distinct from LMDE terminator caps

These are recommendations rather than mandates. Sailors using generic Legacy Marine Data Ecosystem cables on Pelorus networks will see no visual differentiation, which is an accepted tradeoff for cable inventory compatibility.

---

## 5. Topology

### 5.1 Standard Topology

Pelorus Core uses linear bus topology with T-connector drops, identical to LMDE micro:

```
[Term] ─── Backbone ─── [T] ─── Backbone ─── [T] ─── Backbone ─── [Term]
                        │                     │
                       Drop                  Drop
                        │                     │
                     [Device]              [Device]
```

### 5.2 Single Segment Limits

A single Pelorus Core segment shall not exceed:

| Parameter | Limit |
|---|---|
| Total backbone length | 30 m |
| Maximum drop (stub) length | 6 m |
| Total stub length (sum of all drops) | 78 m |
| Maximum nodes per segment | 50 |
| Termination | 120Ω at each physical end |

These limits match LMDE micro for backbone length above 100m would require mini cable, which is permitted but not standard for Pelorus.

### 5.3 Segmentation for Larger Vessels

Vessels requiring more than a single segment use repeater nodes to create multiple segments. Repeater nodes:

- Connect two electrically isolated CAN FD segments
- Forward valid CAN FD frames between segments transparently
- Provide galvanic isolation between segments
- Optionally filter messages by DCID for traffic management
- May serve as power injection points for one or both segments
- Detect and contain faults on a single segment without affecting the network

A typical large-vessel installation uses a star topology with the gateway/repeater node at the center and multiple segments radiating out (engine room, nav station, mast, and similar functional zones).

### 5.4 Repeater Limits

A network shall not have more than four repeater hops between any two endpoints. This limit is driven by cumulative latency in safety-critical message paths. Most practical installations require no more than two hops.

---

## 6. Termination

### 6.1 Required Termination

Each Pelorus Core segment shall be terminated at both physical ends with split termination:

```
        CAN_H ───┬──── 60Ω ──┬──── 60Ω ──┬─── CAN_L
                                │
                              4.7nF (C0G/NP0)
                                │
                               GND
```

### 6.2 Component Specifications

| Component | Value | Tolerance | Type |
|---|---|---|---|
| Termination resistors (×2) | 60Ω | ±1% | Metal film, 0.25W minimum |
| Midpoint capacitor | 4.7nF | ±10% | C0G or NP0 ceramic |

The C0G/NP0 capacitor specification is mandatory. Lower-grade dielectrics (X7R, Y5V) have temperature and voltage coefficients that compromise EMC performance.

### 6.3 Implementation

Termination is provided by:

- Dedicated terminator caps that screw onto T-connectors at the bus ends
- Built into devices that occupy bus ends (less common, generally discouraged because it requires that specific device to remain connected)

Simple 120Ω terminator caps from the Legacy Marine Data Ecosystem will work on Pelorus Core networks but do not provide the EMC benefits of split termination. Sailors using existing terminator inventory will have functional but suboptimal networks. New Pelorus installations should use split-termination caps.

### 6.4 Total Bus Resistance

A properly terminated Pelorus Core segment presents 60Ω resistance between CAN_H and CAN_L (the two 120Ω total terminations in parallel). Installers can verify termination by measuring resistance with the network powered off — a reading near 60Ω indicates correct termination.

---

## 7. Transceiver Requirements

### 7.1 Mandatory Capabilities

Pelorus Core nodes shall use a CAN FD transceiver with the following capabilities:

- Compliant with ISO 11898-2:2016 partial networking specification
- Selective wake-up function per ISO 11898-2:2016 Section 5.9.4
- CAN FD passive support at minimum 1 Mbit/s data phase (to ensure reliable filtering of Pelorus Core traffic at 500 kbit/s)
- Standby current ≤10 µA in selective wake mode with bus monitoring active
- Operating temperature range −40°C to +125°C minimum
- Bus pin ESD tolerance ≥8 kV (HBM) minimum

### 7.2 Compliant Transceiver Examples

The following parts are known to meet Pelorus Core requirements (non-exhaustive):

- NXP TJA1145, TJA1145/FD, TJA1146
- NXP NCA1145B
- Microchip ATA6570
- TI TCAN1145-Q1, TCAN1146-Q1

### 7.3 SIC Not Required

Pelorus Core does not require Signal Improvement Capability (SIC) transceivers. The 500 kbit/s data phase provides sufficient signal integrity headroom on LMDE-style topologies including 6 m drops without active ringing suppression. SIC may be specified in future Pelorus Core profiles if higher data phase rates are introduced.

### 7.4 Patent Considerations

The selective wake-up function involves patents disclosed in ISO 11898-2:2016. Commercial Pelorus Core implementations should review the patent landscape and licensing requirements before product release. See [04-power-management.md](./04-power-management.md) §3 for the patent notice and the disclosed patent holders.

---

## 8. Power Distribution

### 8.1 Voltage Range

Pelorus Core devices shall operate over the following input voltage range:

| Parameter | Value |
|---|---|
| Nominal operating range | 9-32 V |
| Continuous tolerance | up to 36 V |
| Transient tolerance | up to 40 V (alternator load dump) |
| Reverse polarity tolerance | −36 V minimum without damage |
| Minimum operating voltage | 9 V (device may shut down below this) |

This range covers 12 V and 24 V boat electrical systems natively without requiring external converters or configuration.

### 8.2 Reverse Polarity Protection

All Pelorus Core devices shall implement reverse polarity protection. Acceptable implementations include:

- Series diode (Schottky preferred for low forward drop)
- P-channel MOSFET protection circuit
- Bridge rectifier (for devices that should operate regardless of polarity)

### 8.3 Power Consumption Declaration

Each Pelorus Core device shall declare its power consumption in each operating state it supports:

- **Active** — typical and maximum current draw during normal operation, in mA at 12 V
- **Standby** — current draw when device is responding to wake-up requests but not actively processing
- **Sleep** — current draw when device is in selective wake-up sleep with bus monitoring active
- **Deep Sleep** — current draw when device is in deepest power state with bus monitoring disabled (where supported)

These declarations enable installers to calculate segment power budgets accurately. The formal state machine, transitions, and wake-up behavior are specified in [04-power-management.md](./04-power-management.md).

### 8.4 LEN Compatibility

Pelorus Core adopts the Legacy Marine Data Ecosystem LEN (Load Equivalency Number) system for compatibility with existing installer practice:

- 1 LEN = 50 mA at 12V
- Devices declare their LEN value in their device description
- Network total LEN must not exceed segment power budget

The LEN value should reflect typical active-state current draw, not worst-case or peak.

### 8.5 Segment Power Budgets

Maximum allowable total LEN per segment depends on cable type and power injection topology:

| Cable | Single Power Injection | Center Power Injection |
|---|---|---|
| Micro | 60 LEN (3 A) | 80 LEN (4 A) |
| Mid | 80 LEN (4 A) | 100 LEN (5 A) |
| Mini | 160 LEN (8 A) | 200 LEN (10 A) |

These are conservative limits accounting for voltage drop along the backbone. Actual capacity depends on installation specifics.

### 8.6 Power Injection Points

Power is injected into the bus via power tee connectors. Recommendations:

- One power tee per segment minimum
- Power tee located near segment center for best voltage distribution
- Power tee within 3m of battery or main bus for minimum supply line voltage drop
- Inline fuse rated to cable capacity required at power injection point

### 8.7 Voltage Drop Calculations

Installers shall verify that the minimum operating voltage (9V) is met at the most distant device under maximum load conditions. The voltage drop calculation:

```
V_drop = I_total × R_cable × L_cable
```

Where:
- V_drop is voltage drop along the backbone
- I_total is total current at the calculation point
- R_cable is cable resistance per meter (round-trip)
- L_cable is cable length to the calculation point

Approximate cable resistance values (round-trip, both power and ground):
- Micro: 0.21 Ω/m
- Mid: 0.13 Ω/m
- Mini: 0.06 Ω/m

---

## 9. Galvanic Isolation

### 9.1 Mandatory Isolation

Galvanic isolation between the CAN bus and device local power supply is mandatory for devices meeting any of the following criteria:

- Active-state current draw exceeds 100 mA
- Device interfaces to high-power systems (autopilots, motor control, solenoid drivers)
- Device monitors or controls engine systems (ignition, fuel pumps, alternator field)
- Device drives any inductive load (relays, valves, motors)
- Device connects to a separate high-voltage subsystem (24V for instrument bus, separate inverter, etc.)

### 9.2 Strongly Recommended Isolation

Galvanic isolation is strongly recommended (but not mandated) for:

- Any device installed in a harsh electrical environment (engine compartment, lazarette, mast)
- Sensor-only devices in cross-vessel installations where ground potential differences are likely
- Devices critical to safety regardless of power level

### 9.3 Optional Isolation

Galvanic isolation is optional for:

- Low-power sensor-only devices (under 50 mA active) installed in benign electrical environments
- Devices physically co-located with the power injection point and gateway

### 9.4 Implementation Requirements

Where isolation is required or implemented, the following minimum specifications apply:

- Isolation voltage rating: 1500V minimum, working voltage to 60V
- Isolation barrier between bus side and local power side
- Isolated DC/DC converter or transformer-coupled supply for transceiver power
- Digital isolators (capacitive, magnetic, or optical) for CAN_RX, CAN_TX, and STBY signals between transceiver and microcontroller
- Bus-side ground connected to bus shield/drain
- Local-side ground connected to vessel ground per local installation

### 9.5 Sleep Current Considerations

Galvanic isolation typically increases minimum standby current by 50-200 µA compared to non-isolated designs due to isolated DC/DC converter overhead. Pelorus Core devices implementing isolation should target the highest practical standby current optimization but may exceed the non-isolated targets:

| Isolation State | Sleep State Current Target |
|---|---|
| Non-isolated | ≤ 100 µA |
| Isolated | ≤ 200 µA (best effort) |

This is an acceptable tradeoff for the reliability and safety benefits of isolation.

---

## 10. Shield and Ground

### 10.1 Cable Shield

The drain wire (Pin 1) provides a continuous shield path along the backbone. Shield handling requirements:

- Shield connected through every T-connector and inline coupler
- Shield connected to bus shield/drain pin on every device
- Shield grounded at exactly one point per segment (typically the power injection point)

### 10.2 Single-Point Ground

The shield shall be connected to vessel ground (DC negative) at exactly one point per segment to prevent ground loops. Multiple grounding points create circulating currents that compromise EMC performance and signal integrity.

### 10.3 Galvanic Isolation Boundary

For galvanically isolated devices, the bus-side shield connects to the bus shield system, and the local-side ground connects to the device's local power ground. The two sides are connected only through the controlled isolation barrier.

---

## 11. EMC and Environmental Requirements

### 11.1 EMC Compliance

Pelorus Core devices should be tested to:

- CISPR 25 (Vehicles, boats and internal combustion engines — radio disturbance characteristics)
- IEC 60945 (Maritime navigation and radiocommunication equipment — general requirements)
- FCC Part 15 Class B (United States)
- CE marking per applicable EU directives

### 11.2 Environmental

Pelorus Core devices intended for marine installation should meet:

- IP67 minimum ingress protection (IP68 recommended for exposed locations)
- Operating temperature −20°C to +70°C minimum (extended ranges recommended)
- Salt fog resistance per IEC 60068-2-52 or MIL-STD-810 Method 509
- Vibration resistance per IEC 60068-2-6

### 11.3 Conformal Coating

All Pelorus Core device PCBs should be conformally coated for marine environments. This is a strong recommendation rather than a mandate, but devices without conformal coating should not be marketed as suitable for marine use.

---

## 12. Compliance Verification

### 12.1 Self-Declaration

Pelorus Core implementations are not subject to mandatory third-party certification. Manufacturers self-declare compliance with this specification. The Pelorus project will publish compliance test procedures that manufacturers can use for self-verification.

### 12.2 Conformance Test Coverage

Conformance verification should cover:

- Bit timing and frame format at 250k arbitration / 500k data phase
- Selective wake-up functionality with reference WUF patterns
- Power consumption in all defined states
- Voltage range operation including transients and reverse polarity
- Termination resistance and EMC behavior
- Galvanic isolation testing where applicable
- Environmental compliance per Section 11

### 12.3 Interoperability Testing

The Pelorus project will maintain a reference implementation and compatibility test bed. Manufacturers may submit devices for interoperability testing against the reference implementation. Successful interoperability testing entitles the device to use the "Pelorus Core Compatible" branding.

---

## 13. Path redundancy — Bus A and Bus B (dual-bus domain)

Normative policy for **when** dual buses are required: **[17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md)**. This section defines **physical** requirements for Bus A and Bus B on Pelorus Core.

### 13.1 Electrical independence

- **Bus A** and **Bus B** **shall** be separate **CAN_H / CAN_L** pairs (separate segment topology per bus), each with its own **split termination** per **§6**.
- Neither bus **shall** share a single two-wire pair with the other; **Class D** devices **shall** use two independent transceivers (or an integrated dual-transceiver solution meeting the same isolation goals).

### 13.2 Segment limits

- **Each** of Bus A and Bus B **shall** observe the same per-segment limits as a single Pelorus backbone (**§5.2**, **[08-network-architecture.md](./08-network-architecture.md)**).
- **Repeaters** apply **per bus**; hop counts and lengths are **not** shared across Bus A and Bus B.

### 13.3 Node and port classes

| Class | Physical requirement |
|-------|----------------------|
| **Class S** | One M12 A-coded Pelorus Core port (or one approved segment attachment) to **either** Bus A **or** Bus B. |
| **Class D** | **Two** ports: one to Bus A, one to Bus B. **SHALL** use **two** M12 A-coded connectors **or** one integrated dual-port assembly until a future single-connector pinout is registered in this document. |
| **Class H** | At minimum **two** backbone ports (Bus A, Bus B) plus one or more downstream segment ports per **[10-repeater-specification.md](./10-repeater-specification.md)**; galvanic isolation between downstream segments and between each backbone port as specified in **10**. |

### 13.4 Connector strategy (v0.1)

- **Interim mandatory profile for Class D:** **two** **M12 A-coded 5-pin** connectors, labeled **A** and **B** adjacent to each connector body.
- A **future** single-connector dual-bus pinout **may** be added without removing the two-port option; until ratified, single-connector Class D is **not** Pelorus Core conformant.

### 13.5 Power diversity (informative)

- For **C0** / **C1** zones, **[17](./17-criticality-and-redundant-paths.md)** encourages independent protected feeds for the two transceiver supplies where the vessel design permits — see **14** for installation notes.

### 13.6 Bit rate and segment-length scope (v1.0)

Pelorus Core v1.0 specifies **exactly one** profile: **250 kbit/s arbitration / 500 kbit/s data**, **30 m** maximum backbone per segment, **6 m** maximum stub, **50** nodes maximum per segment (per **§2** and **§5**). These limits derive from the **stub-loaded LMDE Micro** topology and CAN FD signal-integrity headroom on that physical plant; they are **not** a generic CAN FD length-vs-rate calculation.

Tables that map a single **bit rate** to a single **maximum length** without modeling **stub count, stub length, node count, transceiver class, and termination quality** (such as the indicative table in [GitHub Issue #6 §2](https://github.com/pelorus-marine/specifications/issues/6)) **shall not** be used to override **§5** for v1.0. Higher data-phase rates (1 Mbit/s, 2 Mbit/s) — when introduced — will be defined as **named profiles** in a future revision, each with their own complete topology and timing budget.

### 13.7 Patent / IP notice for active-active dual CAN FD (informative)

Active-active dual-CAN-FD redundancy and SYNC-based active/backup CAN-FD redundancy variants are an active patent area. **GitHub Issue #6** cites **US Patent 12,567,994** as describing a SYNC-based active/backup variant. The Pelorus Core design specified in **§13** and **[03 §6](./03-data-link-layer.md#6-path-redundancy-dual-bus)** is **active-active without a SYNC channel**, and is intended to avoid that specific construct.

This notice is informational; it does **not** constitute legal advice. Implementers planning **commercial** Pelorus-conformant hardware that implements path redundancy **should** perform their own patent landscape and freedom-to-operate review before product release, in addition to the **selective wake-up** patent review already required by **[04 §3](./04-power-management.md#3-patent-notice-read-first)**.

---

## 14. Open Items

The following items are not yet specified and will be addressed in future revisions:

- Specific DCID number assignments for Pelorus Core (currently under design)
- Legacy Marine Data Ecosystem bridge gateway functional specification
- Pelorus Core repeater functional specification (filtering rules, fault handling)
- Specific power tee implementations (single, center, dual injection)
- Mechanical specifications for Pelorus Core devices (mounting, form factors)
- Detailed marking and labeling requirements
- Backward compatibility profile for LMDE classical CAN wiring

---

## Appendix A: Comparison with the Legacy Marine Data Ecosystem

| Parameter | Legacy Marine Data Ecosystem | Pelorus Core |
|---|---|---|
| Arbitration bit rate | 250 kbit/s | 250 kbit/s (identical) |
| Data bit rate | 250 kbit/s | 500 kbit/s |
| Maximum frame payload | 8 bytes (Fast Packet to 223) | 64 bytes |
| Multi-frame protocol | Fast Packet | None required (or J1939 TP) |
| Connector | M12 A-coded 5-pin | M12 A-coded 5-pin (identical) |
| Cable | DeviceNet thin/mid/thick | LMDE micro/mid/mini (identical) |
| Maximum stub | 6 m | 6 m (identical) |
| Maximum backbone (micro) | 100 m | 30 m per segment |
| Termination | 120Ω simple | 120Ω split termination |
| Power input range | 9-16 V | 9-32 V |
| Galvanic isolation | Not required | Mandatory for high-power |
| Power management | None | Selective wake-up |
| Topology scaling | Single bus | Segmented via repeaters |
| Specification | Closed, paid, NDA | Open, free, no NDA |

---

## Appendix B: Glossary

| Term | Definition |
|---|---|
| **CAN FD** | CAN with Flexible Data-Rate, ISO 11898-1:2015; **Pelorus Core** application frames use this format |
| **Classical CAN** | CAN 2.0 / ISO 11898-1 base format (no flexible data rate); **8-byte** payload maximum per data frame; typical **LMDE / J1939 marine** application traffic at 250 kbit/s |
| **DLC** | Data Length Code, 4-bit field indicating frame data size |
| **ESD** | Electrostatic Discharge |
| **HBM** | Human Body Model, ESD test methodology |
| **LEN** | Load Equivalency Number, LMDE power unit (1 LEN = 50 mA) |
| **DCID** | Data Contract ID — Pelorus name for the historic **Parameter Group Number** role in J1939-family stacks; numeric values align with LMDE where compatibility is claimed. **LMDE:** carried in **Classical CAN** frames. **Pelorus:** carried in **CAN FD** frames (this document set). |
| **SIC** | Signal Improvement Capability, CAN ringing suppression per CiA 601-4 |
| **WUF** | Wake-Up Frame, CAN frame triggering selective node wake-up |

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
