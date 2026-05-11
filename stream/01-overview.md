# Pelorus Stream — Overview

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

Entry point to the Pelorus Stream specification. Normative requirements live in [`02-data-model.md`](./02-data-model.md) onward.

## 1. What Pelorus Stream Is

The IPv6/Ethernet high-bandwidth subsystem of the Pelorus marine data network, complementary to Pelorus Core (CAN FD). Stream carries data that is too bandwidth-intensive or architecturally inappropriate for CAN FD: raw radar video, S-100 chart files, high-rate navigation data, and radar control.

Concretely, Stream provides:

- IPv6 link-local Ethernet over a dual-fabric M12 X-coded plant ([`03-physical.md`](./03-physical.md), [`07-redundancy.md`](./07-redundancy.md))
- QUIC as the universal transport — datagrams for unreliable, reliable streams for everything else ([`04-transport.md`](./04-transport.md))
- mDNS-SD service discovery and a distributed registry ([`08-discovery-and-registry.md`](./08-discovery-and-registry.md))
- IEEE 802.1AS time synchronisation ([`09-time-sync.md`](./09-time-sync.md))
- Concrete service profiles for navigation-relevant traffic: radar video, radar control, S-100 charts, high-rate nav, stream health ([`10-services-nav.md`](./10-services-nav.md))

Cabin audio, intercom, and entertainment are **out of scope**. AIS targets are low-rate instrument data and live on Pelorus Core ([`core/07-dcid-registry.md`](../core/07-dcid-registry.md)).

## 2. Boundary with Pelorus Core

Stream is **non–hard-real-time-control**. It may carry safety-relevant data that tolerates loss or brief delay (radar video, S-100 charts, high-rate nav). It shall not carry hard-real-time control authority — helm, autopilot, throttle, thruster, or any actuator command. Those remain on Pelorus Core ([`core/01-overview.md`](../core/01-overview.md)).

- A failed Stream subsystem must leave Core fully functional. A misbehaving Stream node must not be able to degrade Core.
- Ordinary Stream endpoints shall not transmit on Core, originate Core frames, or hold authoritative Core resources.
- Stream→Core injection is permitted only through a capable bidirectional gateway that explicitly implements and validates the Stream→Core policy surface (see [`core/09-network.md §6`](../core/09-network.md)).
- Stream may read Core via the standard Core→Stream gateway path (gateway-published identity, mirrored telemetry).
- Soft control on Stream (radar range/gain) is permitted; hard actuator control is not.

A node that participates in both Core and Stream runs them as separate stacks with no shared safety-critical state.

## 3. Three-Layer Architecture

```
                 ┌───────────────────────────┐
                 │   Pelorus State           │  decisions, coordination
                 │                           │  prioritisation, suppression
                 └─────────┬─────────────────┘
                           │ observes / commands
              ┌────────────┴────────────┐
              ▼                         ▼
   ┌──────────────────────┐   ┌──────────────────────┐
   │     Pelorus Core     │   │    Pelorus Stream    │
   │       (CAN FD)       │   │      (Ethernet)      │
   │ hard-real-time       │   │ best-effort, high    │
   │ control authority    │   │ bandwidth, dual      │
   │                      │   │ fabric               │
   └──────────────────────┘   └──────────────────────┘
```

Stream emits events ([`11-events-and-errors.md`](./11-events-and-errors.md)) and exposes per-stream observable state ([`06-session-and-state.md`](./06-session-and-state.md)). The State subsystem subscribes, aggregates, and derives intents. Stream code shall not link or import State APIs; State imports Stream.

## 4. Design Principles

- **Non–hard-real-time-control.** Stream carries safety-relevant data; it does not carry actuator authority. See §2.
- **State decides, Stream transports.** Prioritisation, suppression, and coordination are State concerns. Stream documents specify mechanism, not policy.
- **One transport.** QUIC for everything. No bare UDP control plane, no multicast fan-out — fan-out is done by replication nodes ([`10-services-nav.md`](./10-services-nav.md)).
- **Bounded latency over guaranteed delivery for media.** For radar video and high-rate nav, fresh data beats complete data.
- **Discoverable, not configured.** Stream nodes appear via mDNS-SD with all the metadata a subscriber needs. Static configuration is permitted but never required.
- **Dual fabric is the default.** Class D is the standard node profile; Class S is for non-safety auxiliary nodes only.
- **Open all the way down.** Specification, reference implementations, test fixtures. No NDA, no certification fees, no vendor program.

## 5. Status

v0.2 is pre-release. No wire encoding, port, or codec choice is an interoperability commitment until at least one reference triple (publisher, subscriber, display/listener) passes the smoke tests. Implementations targeting v0.x should expect frequent revision until the first bench-validated drop.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
