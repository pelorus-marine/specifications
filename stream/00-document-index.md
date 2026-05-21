# Pelorus Stream — Specification Document Index

Authoritative list of Pelorus Stream specification documents. Pelorus Stream is the IPv6/Ethernet high-bandwidth subsystem of the Pelorus marine data network.

Stream is **non–hard-real-time-control**. It carries safety-relevant data that tolerates loss or brief delay (radar video, S-100 charts, high-rate nav). It shall not carry hard-real-time control authority — helm, autopilot, throttle, thruster. Those remain on Pelorus Core ([`core/01-overview.md`](../core/01-overview.md)).

## Trust Levels

- **Trusted** — written deliberately against external sources; cited content verified.
- **Unverified** — provisional draft; not validated.
- **Final** — frozen.

## Documents

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 00 | [`00-document-index.md`](./00-document-index.md) | This index | Living | Trusted |
| 01 | [`01-overview.md`](./01-overview.md) | What Stream is, boundary with Core, design principles | Draft | Unverified |
| 02 | [`02-data-model.md`](./02-data-model.md) | Stream identifier, type, priority, payload, metadata | Draft | Unverified |
| 03 | [`03-physical.md`](./03-physical.md) | M12 X-coded, 802.3bt PoE, dual-fabric installation | Draft | Unverified |
| 04 | [`04-transport.md`](./04-transport.md) | QUIC over IPv6 link-local; datagram header; reliable streams vs datagrams | Draft | Unverified |
| 05 | [`05-control-protocol.md`](./05-control-protocol.md) | Control message taxonomy, envelope, deterministic CBOR, versioning | Draft | Unverified |
| 06 | [`06-session-and-state.md`](./06-session-and-state.md) | Session lifecycle, observable state, subscription | Draft | Unverified |
| 07 | [`07-redundancy.md`](./07-redundancy.md) | Dual-fabric state machine, datagram dedup table, Class S vs Class D, RedBox | Draft | Unverified |
| 08 | [`08-discovery-and-registry.md`](./08-discovery-and-registry.md) | mDNS-SD, Stream service catalog, distributed registry | Draft | Unverified |
| 09 | [`09-time-sync.md`](./09-time-sync.md) | IEEE 802.1AS (gPTP), grandmaster selection per fabric | Draft | Unverified |
| 10 | [`10-services-nav.md`](./10-services-nav.md) | Radar video, radar control, S-100 charts, high-rate nav, replication, health | Draft | Unverified |
| 11 | [`11-events-and-errors.md`](./11-events-and-errors.md) | Stream events, updates, application + transport error taxonomies | Draft | Unverified |
| 12 | [`12-lib.md`](./12-lib.md) | Reference Rust library entry point: public API surface | Draft | Unverified |

## Numbering

Numbers are assigned at document creation and not reused. This index renumbers from a prior 28-doc draft; the absorbed-doc mapping is recorded in version-control history.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
