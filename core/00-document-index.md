# Pelorus Core — Specification Document Index

Authoritative list of Pelorus Core specification documents. Pelorus Core is the safety-critical CAN FD subsystem of the Pelorus marine data network.

## Trust Levels

- **Trusted** — written deliberately against external sources; cited content verified.
- **Unverified** — provisional draft; not validated.
- **Final** — frozen.

The protocol-agnostic `Vessel.*` semantic catalog, consumed by Core / Stream / State, is normative in [`../catalog/`](../catalog/) and not duplicated here.

## Documents

| # | Filename | Purpose | Status | Trust |
| --- | --- | --- | --- | --- |
| 00 | [`00-document-index.md`](./00-document-index.md) | This index | Living | Trusted |
| 01 | [`01-overview.md`](./01-overview.md) | What Pelorus Core is, architecture summary, entry point | Draft | Trusted |
| 02 | [`02-physical.md`](./02-physical.md) | Bit rates, cabling, connectors, topology, transceivers, termination, isolation | Draft | Trusted |
| 03 | [`03-data-link.md`](./03-data-link.md) | CAN FD frame format, message addressing, error handling | Draft | Trusted |
| 04 | [`04-power.md`](./04-power.md) | Selective wake-up, partial networking, power states | Draft | Trusted |
| 05 | [`05-addressing.md`](./05-addressing.md) | Source address claiming, NAME, conflict resolution | Draft | Unverified |
| 06 | [`06-instance-binding.md`](./06-instance-binding.md) | Bus identifier triple → catalog index; binding table; out-of-band distribution; cache requirements | Draft | Unverified |
| 07 | [`07-dcid-registry.md`](./07-dcid-registry.md) | Data Contract registry — DC names, DC_ID assignments, bridges to legacy identifiers | Draft | Unverified |
| 08 | [`08-redundancy.md`](./08-redundancy.md) | Criticality classes, dual-bus, duplicate discard, Bus Health | Draft | Unverified |
| 09 | [`09-network.md`](./09-network.md) | Segmentation, scaling, LMDE gateway, repeater spec | Draft | Unverified |
| 10 | [`10-implementation.md`](./10-implementation.md) | Reference crates, hardware guidance, firmware patterns, installation | Draft | Unverified |
| 11 | [`11-conformance.md`](./11-conformance.md) | Conformance test plan + self-declaration template | Draft | Unverified |
| 12 | [`12-firmware-update.md`](./12-firmware-update.md) | Open, vendor-neutral firmware update protocol | Draft | Unverified |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
