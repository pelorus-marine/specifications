# Pelorus Core — Physical Layer

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Trusted

Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation. Dual-bus physical requirements live in [`08-redundancy.md`](./08-redundancy.md).

## 1. Bit Rate and Frame Format

| Phase | Rate |
|---|---|
| Arbitration | 250 kbit/s (identical to LMDE) |
| Data | 500 kbit/s |

Frames per ISO 11898-1:2015:

- 64-byte maximum payload
- 11-bit standard or 29-bit extended IDs (29-bit recommended for J1939-style addressing)
- BRS bit set
- ESI bit set per CAN FD spec

No Fast Packet. Multi-frame messages exceeding 64 bytes use J1939 Transport Protocol or are restructured into single-frame messages. v1.0 specifies 250k/500k as the single mandatory profile; higher data-phase rates are reserved for future named profiles.

## 2. Cable

LMDE micro is the standard cable: 24 AWG twisted pair for CAN, 22 AWG for power and ground, foil shield with drain wire, 120Ω ±10% characteristic impedance, ~12 pF/foot.

Optional variants for extended power requirements:

- **LMDE mid** — 20 AWG signal, 18 AWG power, 4A capacity
- **LMDE mini** — 18 AWG signal, 16 AWG power, 8A capacity

Mid and mini use larger connectors and require adapters at backbone-to-drop transitions.

| Total backbone current | Backbone length | Cable |
|---|---|---|
| <3A | <30m | Micro throughout |
| 3–4A | any | Mid backbone, micro drops |
| 4–8A or backbone >30m | any | Mini backbone, micro drops |

## 3. Connector

M12 A-coded 5-pin per IEC 61076-2-101, IP67/IP68 when mated and torqued, threaded coupling 14 mm pitch. Identical to LMDE micro.

| Pin | Signal | LMDE wire colour |
|---|---|---|
| 1 | Shield / Drain | Bare drain |
| 2 | NET-S (Power +) | Red |
| 3 | NET-C (Power −) | Black |
| 4 | CAN_H | White |
| 5 | CAN_L | Blue |

## 4. Topology

Linear bus with T-connector drops:

```
[Term] ─── Backbone ─── [T] ─── Backbone ─── [T] ─── Backbone ─── [Term]
                        │                     │
                       Drop                  Drop
                        │                     │
                     [Device]              [Device]
```

Single-segment limits:

| Parameter | Limit |
|---|---|
| Total backbone length | 30 m |
| Maximum drop length | 6 m |
| Total stub length | 78 m |
| Nodes per segment | 50 |
| Termination | 120Ω at each physical end |

Vessels exceeding these limits use repeater nodes per [`09-network.md`](./09-network.md). Maximum 4 repeater hops between any two endpoints.

## 5. Termination

Each segment terminated at both physical ends with split termination:

```
        CAN_H ───┬──── 60Ω ──┬──── 60Ω ──┬─── CAN_L
                              │
                            4.7nF (C0G/NP0)
                              │
                             GND
```

| Component | Value | Tolerance | Type |
|---|---|---|---|
| Termination resistors (×2) | 60Ω | ±1% | Metal film, 0.25W min |
| Midpoint capacitor | 4.7nF | ±10% | C0G or NP0 ceramic |

C0G/NP0 dielectric is mandatory; X7R/Y5V have temperature and voltage coefficients that compromise EMC.

A properly terminated segment presents 60Ω between CAN_H and CAN_L (the two 120Ω terminations in parallel) — verifiable with a multimeter, network powered off.

LMDE 120Ω simple terminator caps will function but do not provide split-termination EMC benefits.

## 6. Transceiver

Mandatory capabilities:

- ISO 11898-2:2016 partial networking
- Selective wake-up per ISO 11898-2:2016 §5.9.4
- CAN FD passive support at minimum 1 Mbit/s data phase
- ≤10 µA standby current in selective wake mode with bus monitoring active
- −40°C to +125°C operating range minimum
- ≥8 kV ESD tolerance (HBM) on bus pins

Compliant parts (non-exhaustive): NXP TJA1145, TJA1145/FD, TJA1146, NCA1145B; Microchip ATA6570; TI TCAN1145-Q1, TCAN1146-Q1.

Signal Improvement Capability (SIC) transceivers are not required at 500 kbit/s data phase on LMDE-style topologies.

Patent considerations for selective wake-up: see [`04-power.md`](./04-power.md).

## 7. Power

### 7.1 Voltage Range

| Parameter | Value |
|---|---|
| Nominal operating | 9–32 V |
| Continuous tolerance | up to 36 V |
| Transient tolerance | up to 40 V (alternator load dump) |
| Reverse polarity | −36 V minimum without damage |
| Minimum operating | 9 V |

Covers 12 V and 24 V boat electrical systems natively.

### 7.2 Reverse Polarity Protection

Mandatory on all devices. Acceptable: series Schottky diode, P-channel MOSFET protection, or bridge rectifier.

### 7.3 Power Consumption Declaration

Each device declares current draw at 12 V in:

- **Active** — typical and maximum during normal operation
- **Standby** — responding to wake-up requests
- **Sleep** — selective wake-up with bus monitoring active
- **Deep Sleep** — bus monitoring disabled (where supported)

State machine and transitions in [`04-power.md`](./04-power.md).

### 7.4 LEN

Pelorus uses LMDE LEN (Load Equivalency Number) for installer compatibility: 1 LEN = 50 mA at 12V. Devices declare LEN in their device description; LEN reflects typical active current, not peak.

### 7.5 Segment Power Budgets

| Cable | Single power injection | Center power injection |
|---|---|---|
| Micro | 60 LEN (3 A) | 80 LEN (4 A) |
| Mid | 80 LEN (4 A) | 100 LEN (5 A) |
| Mini | 160 LEN (8 A) | 200 LEN (10 A) |

Conservative limits accounting for backbone voltage drop. Power injected via power tee; one tee per segment minimum, located near segment center, within 3 m of battery; inline fuse rated to cable capacity required.

### 7.6 Voltage Drop

Verify the 9 V minimum is met at the most distant device under maximum load:

```
V_drop = I_total × R_cable × L_cable
```

Round-trip cable resistance:

- Micro: 0.21 Ω/m
- Mid: 0.13 Ω/m
- Mini: 0.06 Ω/m

## 8. Galvanic Isolation

### 8.1 Mandatory

For devices that:

- Draw >100 mA active
- Interface to high-power systems (autopilots, motor control, solenoid drivers)
- Monitor or control engine systems (ignition, fuel pumps, alternator field)
- Drive any inductive load (relays, valves, motors)
- Connect to a separate high-voltage subsystem

### 8.2 Strongly Recommended

- Devices in harsh electrical environments (engine compartment, lazarette, mast)
- Sensor-only devices in cross-vessel installations
- Safety-critical devices regardless of power level

### 8.3 Optional

- Low-power sensor-only devices (<50 mA active) in benign environments
- Devices co-located with the power injection point and gateway

### 8.4 Implementation

| Requirement | Spec |
|---|---|
| Isolation rating | 1500 V minimum, 60 V working |
| Bus-side supply | Isolated DC/DC or transformer-coupled |
| Signal isolation | Digital isolators (capacitive, magnetic, or optical) for CAN_RX, CAN_TX, STBY |
| Grounding | Bus-side ground = shield/drain; local-side ground = vessel ground |

### 8.5 Sleep Current

Isolation typically adds 50–200 µA standby vs non-isolated:

| Isolation | Sleep current target |
|---|---|
| Non-isolated | ≤100 µA |
| Isolated | ≤200 µA (best effort) |

## 9. Shield and Ground

- Shield (Pin 1) carried through every T-connector and inline coupler
- Shield connected to the bus shield/drain pin on every device
- Shield grounded to vessel DC negative at exactly one point per segment (typically the power injection point)
- For galvanically isolated devices, bus-side shield to bus shield system; local-side ground to device local ground; the two connect only through the controlled isolation barrier

## 10. EMC and Environmental

### 10.1 EMC Compliance

Devices should be tested to:

- CISPR 25 (vehicles, boats, ICE — radio disturbance)
- IEC 60945 (maritime navigation and radiocommunication)
- FCC Part 15 Class B (US)
- CE marking per applicable EU directives

### 10.2 Environmental

- IP67 minimum ingress protection (IP68 recommended for exposed locations)
- Operating temperature −20°C to +70°C minimum
- Salt fog per IEC 60068-2-52 or MIL-STD-810 Method 509
- Vibration per IEC 60068-2-6

### 10.3 Conformal Coating

All Pelorus Core PCBs should be conformally coated. Devices without conformal coating should not be marketed as suitable for marine use.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
