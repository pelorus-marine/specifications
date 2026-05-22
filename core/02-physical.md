# Pelorus Core — Physical Layer

Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation. Dual-bus physical requirements live in [`08-redundancy.md`](./08-redundancy.md).

## 1. Bit Rate and Frame Format

| Phase | Rate |
| --- | --- |
| Arbitration | 250 kbit/s (identical to LMDE) |
| Data | 500 kbit/s |

Frames per ISO 11898-1:2015:

- 64-byte maximum payload
- 11-bit standard or 29-bit extended IDs (29-bit recommended for J1939-style addressing)
- BRS bit set
- ESI bit set per CAN FD spec

Multi-frame messages exceeding 64 bytes use J1939 Transport Protocol or are restructured into single-frame messages. v1.0 specifies 250k/500k as the single mandatory profile; higher data-phase rates are reserved for future named profiles.

## 2. Cable

LMDE micro is the standard cable: 24 AWG twisted pair for CAN, 22 AWG for power and ground, foil shield with drain wire, 120Ω ±10% characteristic impedance, ~12 pF/foot.

Optional variants for extended power requirements:

- **LMDE mid** — 20 AWG signal, 18 AWG power, 4A capacity
- **LMDE mini** — 18 AWG signal, 16 AWG power, 8A capacity

Mid and mini use larger connectors and require adapters at backbone-to-drop transitions.

| Total backbone current | Backbone length | Cable |
| --- | --- | --- |
| <3A | <30m | Micro throughout |
| 3–4A | any | Mid backbone, micro drops |
| 4–8A or backbone >30m | any | Mini backbone, micro drops |

## 3. Connector

M12 A-coded 5-pin per IEC 61076-2-101, IP67/IP68 when mated and torqued, threaded coupling 14 mm pitch. Identical to LMDE micro.

| Pin | Signal | LMDE wire colour |
| --- | --- | --- |
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
| --- | --- |
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
| --- | --- | --- | --- |
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

## 7. Bus Power

### 7.1 Architecture

Pelorus Core uses **centrally conditioned bus power**. Vessel battery power does not reach the bus directly. A **Bus Power Injector (BPI)** sits between vessel power and the bus and is the only entity permitted to convert dirty vessel power into bus power. Devices on the bus consume clean, regulated, isolated 24 V DC and are not required to tolerate vessel-side transients, ripple, reverse polarity, brownout, load dump, or alternator artefacts.

This is a deliberate departure from NMEA 2000 / J1939 practice (battery-direct to every device). The rationale:

- **Reliability before installation convenience** ([`01-overview.md §6`](./01-overview.md#6-design-principles)). Vessel power is among the most hostile electrical environments in any consumer/industrial domain; a centrally conditioned bus is the standard architecture for every other fieldbus that takes itself seriously (DeviceNet, PROFIBUS DP, PoE, MIL-STD-704, MIL-STD-1275).
- **Power is foundational, not optional.** Concentrating conditioning at one node lets every other node be simpler, lower-cost-per-unit-reliability, and less defectively designed in the long tail. Per-device load-dump and brownout handling is what fails first in cheap marine devices.
- **BPI single-point-of-failure is mitigated by Core redundancy.** Class C0 / C1 critical sections use dual-bus topology per [`08-redundancy.md`](./08-redundancy.md); each bus carries its own independent BPI. Single-bus installs accept the BPI as a critical component and instrument it accordingly (see §7.7).

A BPI may be a standalone product or integrated into another device (gateway, hub, autopilot controller, central console). Compliance is functional, not packaged.

### 7.2 BPI Requirements

#### 7.2.1 Input (vessel side)

| Parameter | Value |
| --- | --- |
| Input voltage, nominal operating | 9–32 V DC |
| Input voltage, continuous tolerance | up to 36 V |
| Input voltage, transient tolerance | up to 40 V for 400 ms (ISO 7637-2 load dump) |
| Input minimum operating | 9 V |
| Input reverse polarity | survives −36 V indefinite, no damage |
| Input brownout | continues regulated output for ≥50 ms after input loss (cranking ride-through) |
| Input EMI filtering | per CISPR 25 Class 5, IEC 60945 |

Covers 12 V and 24 V vessel electrical systems natively. A 24 V vessel still requires a BPI — the BPI is about conditioning and isolation, not just voltage transformation.

#### 7.2.2 Output (bus side)

| Parameter | Value |
| --- | --- |
| Output voltage, nominal | 24 V DC |
| Output voltage, regulation | ±5% under all rated load and input conditions (22.8 V – 25.2 V at the BPI output terminals) |
| Output ripple | ≤100 mVpp at full rated load, 20 Hz – 20 MHz |
| Output current tiers | 3 A (micro), 4 A (mid), 8 A (mini) — matches cable capacity in §2 |
| Output short-circuit | auto-recovering current limit, no latch-off without explicit command |
| Output overvoltage | hard clamp at 27 V; output disconnects if exceeded |
| Output transient under input disturbance | ≤±10% deviation, recovery to ±5% within 10 ms |
| Hold-up at full load | ≥50 ms after input loss at minimum input voltage |

#### 7.2.3 Galvanic Isolation

| Parameter | Value |
| --- | --- |
| Input-to-output isolation | 1500 V DC minimum, 60 V working |
| Isolation method | transformer-coupled DC/DC; optocoupler or digital isolator for any feedback path |
| Output ground reference | floating; not bonded to vessel DC negative |

The BPI isolates the entire bus from vessel ground. Per §9, the bus shield is referenced to vessel DC negative at exactly one point (typically at the BPI); the bus power conductors remain floating.

#### 7.2.4 Fault Behaviour

The BPI continues operating through:

- Single line-to-line short between NET-S and NET-C on its output (current-limited, recovers when fault clears)
- Single line-to-shield short on its output (current-limited)
- Input voltage outside operating range (output disconnects cleanly, no overshoot on recovery)
- Output overcurrent at any tier limit (current-limited, fault reported)

The BPI does not latch off without explicit `Pelorus.PowerInjector` command except in the overvoltage case (§7.2.2).

#### 7.2.5 Pelorus Participation

The BPI is a Pelorus Core node. It claims an address per [`05-addressing.md`](./05-addressing.md) and transmits the `Pelorus.PowerInjector` Data Contract (DC_ID allocated in [`07-dcid-registry.md`](./07-dcid-registry.md)) at minimum every 5 seconds reporting:

- Input voltage (V)
- Output voltage (V)
- Output current (A)
- BPI temperature (°C)
- Cumulative output energy since last reset (Wh)
- Fault flags: overcurrent, overvoltage, overtemperature, input out-of-range, output sense fail, isolation degraded
- Time since last fault clear (s)

The BPI reports `Pelorus.BusHealth` on its bus port like any other node.

#### 7.2.6 Multiple BPIs on a Segment

v1.0 permits at most one BPI per single-bus segment. Parallel BPIs with active current sharing are deferred (see §11.3). A dual-bus segment carries one BPI per bus (one on Bus A, one on Bus B); the two BPIs are electrically independent and report independently.

### 7.3 Device Power Input (bus side)

Pelorus Core devices accept clean bus power only. Required device-side input specification:

| Parameter | Value |
| --- | --- |
| Input voltage, nominal | 24 V DC |
| Input voltage, operating range at device connector | 14 V – 30 V (accounts for backbone voltage drop and BPI regulation) |
| Input voltage, transient survival | up to 36 V for 400 ms, no damage |
| Input reverse polarity | not required — BPI guarantees polarity; defensive protection is permitted but optional |
| Input EMC | per §10.1 |

Device PSUs may assume the bus is clean, isolated 24 V. They are not required to implement load-dump suppression, wide-range buck-boost, or input transient absorption beyond the 36 V brief transient — the BPI handles all of these.

Devices may include defensive reverse polarity protection (single series Schottky or P-MOSFET) as a robustness measure against field wiring errors. This is permitted but does not relax any BPI requirement.

### 7.4 Power Consumption Declaration

Each device declares current draw at 24 V in:

- **Active** — typical and maximum during normal operation
- **Standby** — responding to wake-up requests
- **Sleep** — selective wake-up with bus monitoring active
- **Deep Sleep** — bus monitoring disabled (where supported)

State machine and transitions in [`04-power.md`](./04-power.md).

### 7.5 LEN

Pelorus redefines LEN (Load Equivalency Number) for the 24 V bus: **1 LEN = 25 mA at 24 V** (≡ 600 mW), so a Pelorus LEN represents the same delivered power as one LMDE LEN (50 mA × 12 V = 600 mW). Devices declare LEN in their device description; LEN reflects typical active current, not peak.

This preserves installer arithmetic compatibility with LMDE LEN budgets at the *power* level while reflecting the actual bus voltage. Bridges to LMDE handle the unit difference transparently.

### 7.6 Segment Power Budgets

| Cable | Single power injection | Center power injection |
| --- | --- | --- |
| Micro | 60 LEN (1.5 A @ 24 V) | 80 LEN (2.0 A @ 24 V) |
| Mid | 80 LEN (2.0 A @ 24 V) | 100 LEN (2.5 A @ 24 V) |
| Mini | 160 LEN (4.0 A @ 24 V) | 200 LEN (5.0 A @ 24 V) |

Currents are half the LMDE equivalent at the same LEN because bus voltage doubled. Lower current → lower I²R loss → tighter voltage at the far node. The BPI is the injection point; no separate "power tee within 3 m of battery" rule applies because the BPI replaces that boundary. Inline fuse on the BPI input rated to cable capacity is required.

### 7.7 Voltage Drop

Verify the 14 V device minimum is met at the most distant device under maximum load:

```text
V_node_min = V_BPI_min − (I_total × R_cable × L_eff)
```

where `V_BPI_min = 22.8 V` (BPI output at −5% regulation), `R_cable` is the round-trip cable resistance from §2, and `L_eff` is the effective distance from the BPI to the far node (full backbone length for single-end injection with distributed load; backbone/2 for center injection with distributed load, integrated for uniform load distribution).

Round-trip cable resistance:

- Micro: 0.21 Ω/m
- Mid: 0.13 Ω/m
- Mini: 0.06 Ω/m

Worked example — 60 LEN (1.5 A) distributed across a 30 m micro backbone, single-end BPI:

```text
V_drop_distributed = 1.5 A × 0.21 Ω/m × 30 m / 2 = 4.73 V
V_node_min = 22.8 V − 4.73 V = 18.07 V
```

Well above the 14 V device minimum.

### 7.8 BPI Fault Tolerance and Redundancy

A single-bus segment carries one BPI; BPI failure brings the segment down. Pelorus Core treats this explicitly:

- **Class C0 and C1 (safety-critical, dual-bus) installs** mitigate BPI failure by carrying an independent BPI on each of Bus A and Bus B per [`08-redundancy.md`](./08-redundancy.md). Loss of one BPI degrades the affected bus only.
- **Class C2 / C3 (single-bus) installs** accept BPI failure as a bus outage. Detection is via `Pelorus.PowerInjector` reporting and via the absence of all bus traffic after a configured timeout.
- **BPI failure modes that do not cleanly disconnect** (e.g., output voltage drifts low, ripple rises, hold-up degrades) shall be detected by the BPI's self-monitoring and reported via `Pelorus.PowerInjector` fault flags before bus traffic is affected, where physically possible.

The BPI is the single most important reliability component on the bus. Conformance ([`11-conformance.md`](./11-conformance.md)) shall enforce BPI requirements at least as strictly as any other device class.

## 8. Galvanic Isolation

The bus is already isolated from vessel DC negative at the BPI (§7.2.3). Bus-to-vessel ground loops do not exist by construction. Per-device isolation requirements are therefore narrower than under battery-direct architectures: they exist only where a device introduces a *second* ground reference distinct from the bus and from vessel ground.

### 8.1 Mandatory

For devices that:

- Interface to high-power systems with their own ground reference (autopilots, motor control, solenoid drivers, thruster controllers)
- Monitor or control engine systems (ignition, fuel pumps, alternator field, starter circuits)
- Drive any inductive load returned through vessel ground (relays, valves, motors)
- Connect to a separate high-voltage subsystem (house DC, shore power, inverter DC link)
- Connect to any non-Pelorus communication bus that is itself grounded (LMDE / NMEA 2000, J1939, RS-485 with shielded ground)

The triggering condition is the presence of a second ground reference, not active current draw. The BPI removes "draws >100 mA active" as an isolation trigger because the bus side is already isolated.

### 8.2 Strongly Recommended

- Devices in harsh electrical environments (engine compartment, lazarette, mast) regardless of secondary ground reference
- Sensor-only devices in cross-vessel installations spanning multiple bonded zones
- Safety-critical devices regardless of architecture

### 8.3 Optional

- Pure bus-only devices (no secondary ground reference, no external interface): isolation is not required because the bus is already isolated at the BPI
- Low-power sensor-only devices (<50 mA active) with bus-only connectivity

### 8.4 Implementation

| Requirement | Spec |
| --- | --- |
| Isolation rating (device-internal) | 1500 V minimum, 60 V working |
| Bus-side supply | Powered from the bus 24 V; no second DC/DC required for isolation purposes if the bus side and the secondary side are kept separate |
| Signal isolation | Digital isolators (capacitive, magnetic, or optical) on the boundary between bus-side and secondary-ground-side circuits |
| Grounding | Bus-side return = bus NET-C (isolated by BPI); secondary-side ground = whatever vessel reference the secondary circuit uses; the two connect only through the controlled isolation barrier |

The BPI satisfies the bus-side isolation requirement for the whole segment; device-internal isolation barriers exist to protect the bus from the device's secondary circuits, not from vessel ground itself.

### 8.5 Sleep Current

| Class | Sleep current target at 24 V |
| --- | --- |
| Bus-only device (no secondary ground) | ≤50 µA |
| Device with internal isolation barrier | ≤100 µA |

Targets tightened relative to the battery-direct architecture because the BPI absorbs the worst standby-power offenders (input filtering, reverse polarity FET leakage, wide-range buck quiescent) at one point instead of replicating them per device.

## 9. Shield and Ground

- Shield (Pin 1) carried through every T-connector and inline coupler
- Shield connected to the bus shield/drain pin on every device
- Shield grounded to vessel DC negative at exactly one point per segment — at the BPI per §7. No other device shall bond shield to vessel ground.
- The bus power conductors (NET-S, NET-C) remain floating relative to vessel ground; isolation between bus power and vessel ground is provided by the BPI's input-to-output barrier (§7.2.3)
- For devices with a secondary ground reference (§8.1), bus-side shield connects to the bus shield system; the secondary-side ground connects to the secondary reference; the two connect only through the controlled isolation barrier inside the device

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

## 11. Open Items

### 11.1 Backbone length beyond 30 m (deferred to v2)

The 30 m single-segment backbone limit is set by CAN FD data-phase timing at 500 kbit/s — round-trip cable propagation plus transceiver loop delay must remain a small fraction of the 2000 ns bit time. The limit is conservative; with a documented sample-point requirement and accounting for typical transceiver loop delay (~250 ns), 40 m is electrically achievable without further hardware constraints. Beyond ~40 m, CiA 601-3 Signal Improvement Capability (SIC) transceivers actively suppress reflections and support 60+ m at the same data rate — at the cost of requiring **every node on the segment** to be SIC.

For v1.0, the single named profile remains 30 m / 6 m / 50 nodes (per [`08-redundancy.md §4.4`](./08-redundancy.md#44-bit-rate-and-length-scope)); vessels exceeding 30 m use repeaters per [`09-network.md`](./09-network.md). For v2.0, a tiered topology profile is proposed:

- **Standard** — 30 m backbone, no transceiver constraints (current §4).
- **Extended** — 40 m backbone with a published sample-point requirement and tightened transceiver loop-delay budget.
- **Long-bus** — 60 m backbone, all-SIC segment required, CiA 601-3 compliance mandatory.

Action: characterise reference-transceiver loop-delay distribution and publish a measured propagation-budget table before promoting Extended or Long-bus tiers to normative.

### 11.2 Zoned ingress protection requirements (needs empirical evaluation)

The current §10.2 specifies a single floor — IP67 minimum, IP68 recommended for exposed locations — inherited from NMEA 2000 practice. This is inadequate for the realistic failure modes Pelorus targets. Two anchoring facts:

- **IPX8 has no standard depth/duration.** IEC 60529 defines IPX8 as "continuous immersion, depth and duration specified by the manufacturer," so a single "IP68" label on a datasheet is not a comparable specification. Pelorus must declare numeric values, not just a digit.
- **NMEA 2000's IP67 fails real bilge scenarios.** Field experience (project author's vessel, two incidents) records the bus submerged for ~1 hour in salty bilge water following pump failure. IPX7 guarantees only 30 minutes at 1 m in fresh tap water. The bus survived the underlying physics by margin, not by specification.

A zoned approach is proposed for v2 along the lines of IEC 60945's protected/exposed/submerged taxonomy, with marine-realistic durations and seawater test fluid:

| Installation zone | Minimum rating | Test conditions | Behaviour |
| --- | --- | --- | --- |
| Dry interior — nav station, helm, cabin overhead | IP65 + IP67 | Standard IPX7, fresh water | Survive |
| Exposed deck and cockpit — pedestal, mast base, deck pods | IP66 + IP67 | IPX6 wash-down + IPX7 | Survive |
| Engine room, lazarette, above expected bilge level | IP67 + oil/diesel resistance | 1 hour at 1 m in seawater | Survive |
| Bilge, sump, below expected high-water mark | IP68 declared | 24 h at 1 m in seawater, **after** 1000 h IEC 60068-2-52 salt-fog conditioning | Function during for bilge-alarm and pump-status DCs; survive-and-recover otherwise |
| Permanent submersion — transducer wells, stern-tube sensors | IP68 declared to vessel max draft × 2 | Continuous, vessel service life | Function during |
| Masthead, spreader | IP66 + IP67 + UV | IPX6/IPX7 + ISO 4892-2 UV 1000 h | Survive |

Open questions that need testing before this becomes normative:

1. **Seawater vs fresh water testing.** Standard IP tests use tap water (~10 kΩ·m). Seawater is ~0.2 Ω·m and finds gaps fresh water does not. Define a Pelorus-specific test fluid (3.5% NaCl per ASTM D1141 substitute ocean water?) and validate that reference connectors actually pass.
2. **Conditioning sequence.** Whether salt-fog → vibration → immersion or vibration → salt-fog → immersion better predicts field behaviour. Both sequences should be run on candidate connectors and the worse-case adopted.
3. **"Function during immersion" budget.** For bilge-alarm and bilge-pump-status DCs to transmit while submerged requires hermetic housing AND a PCB tolerant of any seepage AND a bus-side transceiver path that doesn't short under salt water bridging CAN_H to CAN_L. Quantify which transceiver families pass and what circuit topology is required (conformal coating sufficient? potting required? series PTCs on bus pins?).
4. **Fail-open under seal failure.** A bilge or submerged device whose seal eventually fails must not short the bus. A salt-water short between CAN_H and CAN_L is ~kΩ — sufficient to drag arbitration below threshold and bring down the segment. Candidate mitigations: series Schottky on each bus pin (adds drop), resettable PTC (adds resistance), per-node bus disconnect on detected internal water (requires a sensor). Test which actually works without compromising signal integrity in normal operation.
5. **Recovery test.** Define a numeric recovery requirement — e.g., a Bilge-rated device must rejoin the bus and pass full DC transmission within 60 seconds of removal from immersion and external drying.
6. **Mated-connector vs unmated-port rating.** IP rating applies only when M12 connectors are mated and torqued. Unmated ports require sealing caps; the cap's rating must be declared and tested independently. Determine whether field-installable connectors can ever reach Bilge/Submerged ratings or must be prohibited in those zones.
7. **Vessel-level worst-case rule.** Because any single leaking node can short the segment, the segment's effective IP rating is set by its weakest installed device. The conformance regime needs an explicit rule: a vessel installation declares the worst zone any device touches, and all devices on that segment meet that zone's rating — or the segment is split and bridged.

Action: stand up a connector-and-housing test fixture; run candidate M12 A-coded connectors, seal cap variants, and representative reference-device housings through the proposed test matrix; publish measured data before promoting any of the zone rows to normative. Until then, §10.2 remains "IP67 minimum, IP68 recommended for exposed locations" but installers should treat it as a floor, not a ceiling, and zone their installations accordingly.

### 11.3 Parallel BPIs with active current sharing (deferred to v2)

v1.0 permits at most one BPI per single-bus segment (§7.2.6). Larger vessels with high aggregate LEN, long backbones, or distributed-injection topologies could benefit from two or more BPIs feeding the same bus with active current sharing — analogous to N+1 redundant power supplies in datacentre racks.

Active current sharing across regulated 24 V sources is non-trivial. Two BPIs with matched output voltages will not share current equally unless they coordinate explicitly; the one with slightly higher set-point hogs the load and the other contributes nothing until it droops. Established approaches:

- **Droop sharing.** Each BPI lowers its output voltage proportional to its output current. Self-balancing, no communication required, but bus voltage varies with total load. Used in basic redundant supplies.
- **Active sharing (CAN-Bus-style).** BPIs exchange current measurements over a private signalling channel and adjust output to match. Most accurate, requires inter-BPI communication.
- **Master-follower.** One BPI sets the voltage, others follow with current feedback. Simpler than active sharing but the master is a single point of failure.

Open questions before this becomes normative:

1. **Which sharing scheme suits Pelorus.** Droop is simplest and stateless; active sharing is tighter but adds an inter-BPI protocol (over the Pelorus bus itself? over a dedicated signal?).
2. **Failure semantics.** If one BPI in a pair fails, does the other take full load instantly, or after a controlled handoff?
3. **Bus voltage regulation tightening.** Multiple BPIs sharing load may need a wider regulation window than the single-BPI ±5%.
4. **Interaction with [`08-redundancy.md`](./08-redundancy.md).** Class C0 / C1 already use one BPI per bus on a dual-bus topology — that's redundancy across buses, not across BPIs on one bus. Parallel-BPI is a *third* axis of redundancy and the conformance story needs to cover both.

Action: characterise droop and active-sharing behaviour on a reference BPI; specify which schemes are permitted, prohibited, or required in v2.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
