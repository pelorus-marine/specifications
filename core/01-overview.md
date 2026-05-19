# Pelorus Core — Overview

**Version:** 0.3 Draft
**Last Updated:** May 19, 2026
**Trust:** Trusted

Entry point to the Pelorus Core specification. Normative requirements live in [`02`](./02-physical.md) onward.

## 1. What Pelorus Core Is

The safety-critical CAN FD subsystem of the Pelorus marine data network. CAN FD on M12 A-coded 5-pin connectors and LMDE-style cabling. Open: no NDA, no certification fee, no third-party gatekeeper.

The high-bandwidth Ethernet counterpart is Pelorus Stream ([`stream/01-overview.md`](../stream/01-overview.md)).

**LMDE** = Legacy Marine Data Ecosystem. Project code name for the incumbent J1939-on-Classical-CAN (CAN 2.0) marine instrumentation fieldbus and its physical plant. Pelorus Core uses CAN FD; it is not electrically interoperable with LMDE on the same wire. Pelorus reuses the J1939 physical layer connector standard, address-claim protocol, and 64-bit NAME field, but defines its own Pelorus-native Data Contract ID namespace and multi-frame transport. Compatibility with legacy J1939 / NMEA 2000 messages is provided via gateway-mediated bridges declared in [`07-dcid-registry.md`](./07-dcid-registry.md).

## 2. Two-Layer Architecture

Pelorus has two physical layers serving different traffic classes:

| Layer | Transport | Role |
|---|---|---|
| **Pelorus Core** | CAN FD | Safety-critical instrumentation, deterministic |
| **Pelorus Stream** | Ethernet | High-bandwidth data — radar, charts, AIS, audio |

They are independent buses, bridged by gateway nodes where required. A failed Stream subsystem must leave Core fully functional.

## 3. Wire Profile

- CAN FD per ISO 11898-1:2015
- 250 kbit/s arbitration, 500 kbit/s data phase
- 64-byte payloads; no Fast Packet
- M12 A-coded 5-pin connectors, LMDE micro cable
- Linear bus per segment; repeaters for vessels exceeding 30 m
- ISO 11898-2:2016 partial networking with selective wake-up

Detailed wire requirements: [`02-physical.md`](./02-physical.md), [`03-data-link.md`](./03-data-link.md).

## 4. Coexistence with LMDE

Same connectors and cable, different frame format. Classical CAN nodes do not correctly receive CAN FD frames; cross-connecting cables produces a non-functional bus but does not damage equipment.

A vessel typically runs both networks during transition, bridged by a gateway that translates identifiers via the bridge table in [`07-dcid-registry.md`](./07-dcid-registry.md), reframes between CAN FD and Classical CAN, handles instance binding, and adapts rates. See [`09-network.md`](./09-network.md).

## 5. Boundary with Pelorus Stream

Stream is non–hard-real-time-control: it carries safety-relevant but loss-tolerant data (radar video, charts, AIS, nav). Hard-real-time control authority — helm, autopilot, throttle, thruster — stays on Core. Ordinary Stream endpoints shall not transmit on Core; Stream→Core injection is permitted only through a capable bidirectional gateway.

## 6. Design Principles

- **Sailor-first.** Every decision asks "what is best for the sailor at sea" before "what is best for the manufacturer."
- **Reliability over features.** A device that works for ten years beats one with twenty features that fails after three.
- **Reliability over installation convenience.** When fault tolerance trades against installer time or cable count, take the reliable outcome unless [`08-redundancy.md`](./08-redundancy.md) explicitly allows a lighter option.
- **Power awareness as architecture.** Power management matches operational context, not a single "anchor vs underway" caricature.
- **Open all the way down.** Specification, reference implementations, test fixtures. No purchases required.
- **Static and debuggable for v1.0.** Auto-negotiation and dynamic reconfiguration are deferred. Fixed bit rates and simple state machines.
- **Honest about tradeoffs.** Patent encumbrances and unresolved questions are documented in each doc's Open Items.

## 7. v1.0 Scope and Compatibility

v1.0 covers Pelorus Core only. Permanent for v1.0: bit rate profile, connector type and pinout, frame format, wire identifier layout, Data Contract namespace structure. Refining before v1.0 ships: power state model, signal catalog.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
