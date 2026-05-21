# Pelorus Core — Implementation Guide

Non-normative guidance for building Pelorus Core hardware, firmware, and installations, plus the inventory of official reference implementations. Normative requirements live in [`02-physical.md`](./02-physical.md), [`03-data-link.md`](./03-data-link.md), [`04-power.md`](./04-power.md), [`05-addressing.md`](./05-addressing.md), [`06-signal-catalog.md`](./06-signal-catalog.md), [`07-dcid-registry.md`](./07-dcid-registry.md), [`08-redundancy.md`](./08-redundancy.md), [`09-network.md`](./09-network.md), and [`12-firmware-update.md`](./12-firmware-update.md). Where this guide says "shall," it restates a normative rule from one of those.

## 1. Reference Implementations

The canonical Rust tree is `pelorus-marine/platform`. Logical components below may later become separate `pelorus-*` crates on crates.io, but they are developed in `platform` first.

| Component | Purpose | Home | Maturity |
| --- | --- | --- | --- |
| `pelorus-dc` | Data Contract encoding/decoding and validation (wire identifier + payload bit layouts) | `platform/pelorus-core/dc` | Evolving |
| `pelorus-pm` | Power management state machine and selective wake-up | `platform/pelorus-core/dc::wire` (full state machine TBD) | Partial |
| `pelorus-address` | Address claiming and NAME handling | `platform/pelorus-core/dc::protocol` (claim sequence TBD) | Partial |
| `pelorus-catalog` | VSS catalog parsing, binding table, runtime mapping | `platform/pelorus-core/correlation` + `semantics`; VSS editor in `platform/pelorus-inspector` | Partial |
| `pelorus-gateway` | Reference gateway firmware (bridge + web UI) | `reference-implementations/pelorus-gateway` (scaffold) | Scaffold |
| `pelorus-vdr` | Reference voyage data recorder (MDF4 on Linux / A55) | `reference-implementations/pelorus-vdr` (scaffold) | Scaffold |
| `pelorus-repeater` | Reference repeater firmware | TBD | Planned |

Optional standalone `pelorus-*` package splits and crates.io releases are TBD; until then, consumers depend on `pelorus-marine/platform` and the semantic versioning of `pelorus-core` as the release unit.

### 1.1 Implementation Principles

- **Language:** Rust, `no_std` where practical, `forbid(unsafe_code)` at the crate root
- **Determinism:** no heap allocation in real-time paths; static buffers where needed
- **Testing:** full unit and integration coverage against the conformance fixtures in [`11-conformance.md`](./11-conformance.md)
- **Licensing:** MIT or Apache 2.0 per crate
- **Versioning:** semantic versioning that tracks the specification version

A device or software component is conformant only if it passes the applicable tests in [`11-conformance.md`](./11-conformance.md) and publishes the self-declaration template. Reference implementations serve as the gold standard for "correct behaviour" during self-testing.

## 2. Hardware Design

### 2.1 Component Selection

**CAN FD transceiver:** ISO 11898-2:2016 partial networking with selective wake-up; ≥1 Mbit/s data-phase support; ≤10 µA standby in selective wake mode. Recommended: NXP TJA1145 / TJA1146, Microchip ATA6570, TI TCAN1145-Q1.

**Galvanic isolation:** mandatory for high-power or motor-interfacing devices; recommended for harsh electrical environments. Use digital isolators (ADuM1201, ISO774x) or isolated transceivers, with an isolated DC/DC for power (target standby <200 µA).

**Microcontroller:** Rust-first, `no_std` capable; at least one CAN FD peripheral (or external controller); low-power modes that allow selective wake-up; sufficient flash/RAM for binding-table cache and power state machine.

**Connectors:** all external connections M12 A-coded 5-pin (male on device, female on cable), IP67/IP68.

**Power protection:** mandatory reverse polarity; transient suppression (TVS on power and CAN lines); fuse or resettable PTC on power input.

### 2.2 Schematic Patterns

**Basic node:** power input → reverse polarity + TVS → isolated DC/DC (if required) → LDO/regulators; CAN bus: transceiver → optional isolator → MCU CAN FD peripheral; split termination (2 × 60 Ω + 4.7 nF C0G) at each end of every segment; bus biasing per [`02-physical.md`](./02-physical.md).

**Repeater:** multiple isolated CAN ports (each transceiver + isolator); common power domain or per-port isolation as needed; MCU coordinates forwarding and power management.

**High-power device** (autopilot, windlass interface): mandatory galvanic isolation on both power and CAN; separate isolated supply for high-current sections.

### 2.3 PCB Layout

- Keep CAN traces short and differential
- Separate digital and power planes where possible
- Guard rings and stitching vias around CAN traces
- Place termination components close to connectors
- Conformal coating mandatory on finished boards (acrylic or polyurethane)
- Component placement for field repairability (no glued-down parts; standard footprints)

### 2.4 EMC and Environmental

- IEC 60945 or equivalent marine EMC; conducted and radiated emissions/immunity testing
- Enclosures: IP68, machined aluminum or marine-grade plastic
- Operating temperature: −25 °C to +70 °C
- Vibration and shock per MIL-STD-810 or equivalent marine practice

### 2.5 Mechanical and Repairability

- Field-serviceable with basic tools
- Modular construction preferred (separate PCB, enclosure, connectors)
- Clear silkscreen for troubleshooting
- Spare parts and repair documentation publicly available

### 2.6 Validation

- Prototype boards tested on a real Pelorus network
- Real-world current measurements in all power states
- Wake-up latency characterisation
- EMC pre-compliance on actual marine cabling
- Long-term liveaboard testing on a real vessel

## 3. Firmware Design

### 3.1 Architecture Principles

- **Layered:** separate HAL, CAN driver, protocol stack, application logic, power management
- **Static allocation:** no heap in real-time paths; static buffers and fixed-size structures
- **State-machine first:** every major behaviour as a finite state machine
- **Minimal dependencies:** only `no_std`-compatible audited crates
- **Error handling:** `Result<T, PelorusError>` everywhere; no panics in production firmware
- **Logging:** lightweight, compile-time configurable (defmt or similar)

### 3.2 Required State Machines

- **Power management** ([`04-power.md`](./04-power.md)): Active / Standby / Sleep / Deep Sleep; selective wake-up via `Pelorus.WakeUp`; PNC mask processing; `Pelorus.NetworkManagement` transmission
- **Address claiming** ([`05-addressing.md`](./05-addressing.md)): Listen → Claim → Defend → Cannot Claim
- **Binding table cache** ([`06-signal-catalog.md`](./06-signal-catalog.md)): receive, validate, cache the latest table; fallback to raw DC_ID + instance mode when cache invalid or absent
- **Repeater forwarding** ([`09-network.md`](./09-network.md)): transparent CAN FD frame forwarding with fault isolation
- **Dual-bus receive** ([`08-redundancy.md`](./08-redundancy.md)) for Class D / Class H products: Duplicate Discard Table; single RX pipeline into application layer; `Pelorus.BusHealth` transmission and local error-counter sampling; degraded single-bus annunciation when peer bus silent

### 3.3 Dual-Bus RX Implementation Patterns

- **Peripheral layout:** prefer two independent CAN controllers (or controller + validated secondary) with separate transceivers; align RX timestamps to a common monotonic clock for `DISCARD_WINDOW`
- **DDT storage:** fixed-size table indexed by SA (and DC_ID for PRH-capable messages); evict on wake-generation change or `NODE_FORGET_TIME` timeout
- **Logging:** rate-limit duplicate-discard and sequence-gap logs; surface saturating counters in Bus Health payload

### 3.4 CAN FD Driver

- Use the MCU's native CAN FD peripheral (or a validated external controller)
- 250 kbit/s arbitration, 500 kbit/s data
- Hardware filtering where possible to reduce CPU load
- Error counters and bus-off recovery per ISO 11898-2:2016

### 3.5 Testing

- All firmware passes the conformance suite in [`11-conformance.md`](./11-conformance.md)
- Unit tests for every state machine transition
- Integration tests using recorded bus traces
- Hardware-in-the-loop testing on a real Pelorus network
- Long-term stability testing (weeks of continuous operation with power cycling and fault injection)

## 4. Installation

### 4.1 Pre-Installation Planning

1. Measure the vessel and plan segments so backbone length, node count, and stub length stay within [`02-physical.md`](./02-physical.md) and [`09-network.md §1`](./09-network.md).
2. Decide on topology: small vessels — single linear bus; large vessels — recommended star with central gateway.
3. Plan repeater locations to respect the hop limit.
4. Identify power injection points; ensure 9–32 V DC supply with reverse polarity protection.
5. Decide on isolation requirements (mandatory for high-power devices).
6. Prepare a critical zone map ([`08-redundancy.md §12`](./08-redundancy.md)): assign C0 / C1 / C2 to each function and decide where Bus A and Bus B run.

### 4.2 Cabling and Connectors

- Use only cable and connectors specified in [`02-physical.md`](./02-physical.md) (typically LMDE-compatible micro cable, M12 A-coded 5-pin)
- Backbone runs between terminators; drops use T-connectors
- All connectors fully mated and sealed (IP67/IP68)
- Label every cable and drop; for dual-bus, label A vs B at both ends and at every tee

### 4.3 Termination

- Split termination (two 60 Ω + 4.7 nF C0G) at both ends of every segment
- Do not use a single terminator or unterminated stubs
- Verify termination resistance (~60 Ω between CAN_H and CAN_L) with power off

### 4.4 Power

- Power on the same cable as data (pins per [`02-physical.md`](./02-physical.md))
- Fused or PTC-protected distribution blocks
- Separate power injection points for long or heavily loaded segments

### 4.5 Repeater and Gateway

- Mount repeaters where they create clean isolated segments
- In star topology, connect repeaters directly to the central gateway
- Install the central gateway in an accessible, dry location with good ventilation and Wi-Fi/Ethernet access for the web UI
- Galvanic isolation present on every repeater port

### 4.6 Commissioning

1. Install and terminate all segments before powering anything.
2. Power up one segment at a time and verify no bus errors.
3. Connect the gateway and confirm it claims an address.
4. Provision the binding table via the gateway web UI.
5. Test power management states (Active → Sleep → Wake).
6. Verify multi-segment forwarding and isolation.
7. Perform a full network test with all devices.

### 4.7 Dual-Bus Path Redundancy

For C0 / C1 zones per [`08-redundancy.md`](./08-redundancy.md):

- Run two independent backbone pairs (Bus A, Bus B) per [`08-redundancy.md §4`](./08-redundancy.md). Do not route both through a single unprotected bundle through a single hazard zone without documenting residual risk on the critical zone map.
- Use separated cable trays / penetrations where feasible; crossing is acceptable only with mechanical protection documented in the map.
- Use independent fused feeds to Bus A and Bus B power injection points when vessel DC distribution supports it.
- Commission `Pelorus.BusHealth` on a test display before declaring the dual-bus domain complete.
- **`DISCARD_WINDOW` sizing:** verify against the formula in [`08-redundancy.md §6.3.3`](./08-redundancy.md). Floor is 50 ms. If the domain has more than two hops on either bus, or no Time Master (`Pelorus.TimeSync`), use a larger value and record it in the critical zone map.

### 4.8 Troubleshooting

| Symptom | Check |
| --- | --- |
| Bus errors or no communication | Termination and stub lengths |
| High standby current | Isolation and transceiver sleep behaviour |
| Address conflicts | NAME uniqueness |
| Binding table not updating | Gateway powered? Binding is out of band in v1.0 — use gateway UI, export/import, or diagnostic tool |
| Intermittent faults | Connectors and cable shielding |
| One bus silent on dual-bus helm | Bus Health on surviving bus; inspect failed backbone for opens, terminators, or transceiver damage |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
