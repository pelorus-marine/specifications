# Pelorus Core — Specification Document Index

Authoritative list of Pelorus Core specification documents. Pelorus Core is the safety-critical CAN FD subsystem of the Pelorus marine data network.

The protocol-agnostic `Vessel.*` semantic catalog, consumed by Core / Stream / State, is normative in [`../catalog/`](../catalog/) and not duplicated here. Non-normative implementation, installation, and reference-software guidance lives in [`../implementation/`](../implementation/).

## Documents

| # | Filename | Purpose |
| --- | --- | --- |
| 00 | [`00-document-index.md`](./00-document-index.md) | This index |
| 01 | [`01-overview.md`](./01-overview.md) | What Pelorus Core is, architecture summary, entry point |
| 02 | [`02-physical.md`](./02-physical.md) | Bit rates, cabling, connectors, topology, transceivers, termination, isolation |
| 03 | [`03-data-link.md`](./03-data-link.md) | CAN FD frame format, message addressing, error handling |
| 04 | [`04-power.md`](./04-power.md) | Selective wake-up, partial networking, power states |
| 05 | [`05-addressing.md`](./05-addressing.md) | Source address claiming, NAME, conflict resolution |
| 06 | [`06-instance-binding.md`](./06-instance-binding.md) | Bus identifier triple → catalog index; binding table; out-of-band distribution; cache requirements |
| 07 | [`07-dcid-registry.md`](./07-dcid-registry.md) | Data Contract registry — DC names, DC_ID assignments, bridges to legacy identifiers |
| 08 | [`08-redundancy.md`](./08-redundancy.md) | Criticality classes, dual-bus, duplicate discard, Bus Health |
| 09 | [`09-network.md`](./09-network.md) | Segmentation, scaling, LMDE gateway, repeater spec |
| 10 | [`10-alerts.md`](./10-alerts.md) | Alert categories, lifecycle, wire encoding, ack semantics, NMEA 2000 alert bridge |
| 11 | [`11-conformance.md`](./11-conformance.md) | Conformance test plan + self-declaration template |
| 12 | [`12-firmware-update.md`](./12-firmware-update.md) | Open, vendor-neutral firmware update protocol |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
