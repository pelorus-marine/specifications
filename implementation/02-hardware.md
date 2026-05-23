# Pelorus Implementation — Hardware Design

Schematic templates, PCB layout, MCU selection, mechanical, and validation patterns for Pelorus Core and Pelorus Stream devices. **Non-normative.** Component, connector, termination, isolation, EMC, and IP requirements are normative in the subsystem physical-layer specs.

## 1. Reference Spec Anchors

Before designing hardware, treat the following as authoritative:

| Topic | Normative location |
| --- | --- |
| Core transceivers, connectors, cabling, termination, isolation, EMC, IP, conformal coating | [`../core/02-physical.md`](../core/02-physical.md) |
| Core Bus Power Injector (BPI) | [`../core/02-physical.md §7`](../core/02-physical.md) |
| Stream M12 X-coded, 802.3bt PoE, dual-fabric | [`../stream/03-physical.md`](../stream/03-physical.md) |

This document does not repeat any of those requirements.

## 2. MCU Selection

**Core nodes:**

- Rust-capable, `no_std` runtime
- At least one native CAN FD peripheral (or capacity for an external CAN FD controller)
- Low-power modes compatible with the four power states in [`../core/04-power.md`](../core/04-power.md)
- Sufficient non-volatile storage for the binding-table cache ([`../core/06-instance-binding.md`](../core/06-instance-binding.md)) and the power state machine

**Stream nodes:**

- Network stack capable of QUIC over IPv6 link-local (typically Linux on Cortex-A class, or an RTOS with a QUIC library)
- 802.3bt PD support (controller IC such as TI TPS2378 or equivalent)

## 3. Schematic Patterns

### 3.1 Basic Core node

Bus 24 V from BPI → device PSU → MCU rails. CAN transceiver → optional digital isolator → MCU CAN FD peripheral. Split termination components placed close to the connector at each physical segment end per [`../core/02-physical.md §5`](../core/02-physical.md).

### 3.2 Repeater

Multiple isolated CAN ports (transceiver + isolator per port); MCU coordinates forwarding and power management. Normative repeater requirements in [`../core/09-network.md §2`](../core/09-network.md).

### 3.3 High-power Core device (autopilot, windlass interface, thruster controller)

Galvanic isolation between the bus side and the high-power side is required per the categories in [`../core/02-physical.md §8`](../core/02-physical.md). Separate isolated supply for the high-current sections.

### 3.4 Stream node

802.3bt PD front-end, dual-port for Class D (per [`../stream/07-redundancy.md`](../stream/07-redundancy.md)), Ethernet PHY, network MCU/SoC.

## 4. PCB Layout

- Keep CAN traces short, differential pair, equal-length
- Separate digital and power planes where possible
- Guard rings and stitching vias around CAN traces
- Place split-termination components close to the connector
- Conformal coating mandatory per [`../core/02-physical.md §10.3`](../core/02-physical.md)
- Component placement for field repairability — no glued-down parts, standard footprints, clear silkscreen for troubleshooting

## 5. Mechanical and Repairability

- Field-serviceable with basic tools
- Modular construction preferred (separate PCB, enclosure, connectors)
- Spare parts and repair documentation publicly available

## 6. Validation Checklist

Before declaring a design ready for production:

- Prototype boards tested on a real Pelorus segment (Core) or fabric (Stream)
- Real-world current measurements in all four power states per [`../core/04-power.md`](../core/04-power.md)
- Wake-up latency characterised against the requirements in [`../core/04-power.md`](../core/04-power.md)
- EMC pre-compliance on actual marine cabling
- Long-term liveaboard testing on a real vessel
- Conformance fixtures pass per [`../core/11-conformance.md`](../core/11-conformance.md)

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
