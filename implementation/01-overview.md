# Pelorus Implementation — Overview

Pragmatic guidance for engineers and integrators building Pelorus devices, firmware, and vessel installations. **Non-normative** — every "shall" in this directory restates a requirement from one of the subsystem specifications.

## 1. Scope

Cross-subsystem implementation patterns:

- **Hardware design** ([`02-hardware.md`](./02-hardware.md)) — schematic templates, PCB layout, MCU and component selection, mechanical and validation.
- **Installation** ([`03-installation.md`](./03-installation.md)) — pre-install planning, commissioning sequence, dual-bus install ergonomics, troubleshooting.
- **Software / reference implementations** ([`04-software.md`](./04-software.md)) — pointer to the Pelorus Platform repository, where reference crates and firmware live. Software specifics intentionally not duplicated here.

## 2. What this guide is NOT

This guide does not restate normative requirements. The specs are authoritative for:

| Domain | Normative home |
| --- | --- |
| Pelorus Core wire, physical layer, BPI, addressing, redundancy | [`../core/`](../core/) |
| Pelorus Stream wire, physical layer, transport, services | [`../stream/`](../stream/) |
| Pelorus State pipeline | [`../state/`](../state/) |
| `Vessel.*` semantic catalog | [`../catalog/`](../catalog/) |
| Pelorus Core conformance | [`../core/11-conformance.md`](../core/11-conformance.md) |

When this guide and a spec disagree, the spec wins.

## 3. Subsystem Coverage at a Glance

| Subsystem | Hardware concerns | Installation concerns | Reference software |
| --- | --- | --- | --- |
| **Core** (CAN FD) | M12 A-coded 5-pin, CAN FD transceivers, BPI, galvanic isolation per criticality | Segment topology, dual-bus zone planning, BPI placement, commissioning | Platform crates per [`04-software.md`](./04-software.md) |
| **Stream** (Ethernet) | M12 X-coded 8-pin, 802.3bt PoE, dual-fabric | Fabric A/B segregation, PoE budget, switch placement | Platform crates per [`04-software.md`](./04-software.md) |
| **State** (software) | — | — | Platform crates per [`04-software.md`](./04-software.md) |
| **Catalog** (vocabulary) | — | — | `vss-tools` profile per [`04-software.md`](./04-software.md) |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
