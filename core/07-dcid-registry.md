# Pelorus Core — DCID Registry

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

Numeric DCID assignments and registry policy on Pelorus Core CAN FD. Identifier layout and DCID derivation are normative in [`03-data-link.md`](./03-data-link.md). Wake-up and NM payload layouts are normative in [`04-power.md`](./04-power.md). Dual-bus DCIDs (0x0FF82 Bus Health, 0x0FF83 Time Sync) live in [`08-redundancy.md`](./08-redundancy.md). The semantic counterpart is [`06-signal-catalog.md`](./06-signal-catalog.md).

## 1. Pelorus-Specific DCIDs

### 1.1 DCID 0x0FF80 — Wake-Up Frame (WUF)

| Attribute | Value |
|---|---|
| Priority | 0 (highest) |
| Type | Single |
| Length | 8 bytes |
| Transmission | Broadcast on selective wake events |
| Purpose | Triggers partial-network wake-up per ISO 11898-2:2016 |

| Byte(s) | Field |
|---|---|
| 0 | Functional-group bitmask ([`04-power.md §3`](./04-power.md)) |
| 1–7 | Reserved — transmit `0x00`, ignore on receive |

### 1.2 DCID 0x0FF81 — Network Management (NM)

| Attribute | Value |
|---|---|
| Priority | 6 |
| Type | Single (200 ms cadence when active) |
| Length | 8 bytes |
| Purpose | Coordinated cluster sleep / wake — CanNm-style behaviour |

| Byte | Field |
|---|---|
| 0 | NM state ([`04-power.md §6.2`](./04-power.md)) |
| 1 | Active functional groups — low byte |
| 2–7 | Reserved — transmit `0x00`, ignore on receive |

### 1.3 Dual-Bus DCIDs

DCID 0x0FF82 (Bus Health) and DCID 0x0FF83 (Time Sync, optional) are defined in [`08-redundancy.md`](./08-redundancy.md). They are listed in the reserved range here ([§3](#3-dcid-ranges-and-assignment-rules)) but their wire layouts and transmission rules live with the dual-bus mechanism that consumes them.

## 2. Compatibility DCIDs

Pelorus reuses selected DCID numbers from LMDE for seamless interoperability via gateways.

On LMDE, these messages appear in Classical CAN frames; on Pelorus Core, the same numeric DCID values and field layouts (where compatibility is claimed) are carried in CAN FD frames per [`03-data-link.md`](./03-data-link.md). This document registers Pelorus-side use; authoritative bit layouts for legacy messages remain in LMDE family standards (SAE J1939 Digital Annex).

The mapping from each DCID/field to the corresponding `Vessel.*` path is maintained in [`06-signal-catalog.md`](./06-signal-catalog.md) and the machine-readable `catalog/vessel.vspec`.

### 2.1 Initial Compatibility Assignments (J1939 heritage, `DP=0`, `R=0`)

Pelorus wire DCIDs reuse SAE J1939 PDU2 PGN numbers; derivation matches [`03-data-link.md §2.2`](./03-data-link.md). Bit layouts and scaling follow the J1939 Digital Annex; gateways bridging LMDE Classical CAN shall preserve field semantics.

| Pelorus wire DCID | J1939 PGN (dec) | Informative name | Primary catalog lane |
|---|---:|---|---|
| `0xF004` | 61444 | Electronic Engine Controller 1 | `EngineRpm` (and additional engine fields per DA) |
| `0xFEE8` | 65256 | Vehicle Heading | `HeadingTrue` |
| `0xFEC5` | 65253 | Engine Temperature 1 | `EngineCoolantTemp` |
| `0x1F812` | 129038 | AIS Class A Position Report | `Vessel.AIS.TargetClassA[*].Position` |
| `0x1F813` | 129039 | AIS Class B Position Report | `Vessel.AIS.TargetClassB[*].Position` |
| `0x1F814` | 129040 | AIS Class B Extended Position Report | `Vessel.AIS.TargetClassB[*].PositionExt` |
| `0x1FB81` | 129793 | AIS UTC and Date Report | `Vessel.AIS.UTCDate` |
| `0x1FB82` | 129794 | AIS Class A Static and Voyage Related | `Vessel.AIS.TargetClassA[*].Static` |
| `0x1FB91` | 129809 | AIS Class B Static, Part A | `Vessel.AIS.TargetClassB[*].StaticA` |
| `0x1FB92` | 129810 | AIS Class B Static, Part B | `Vessel.AIS.TargetClassB[*].StaticB` |

### 2.2 NAME Field

The NAME carried in Address Claimed traffic is defined only by SAE J1939-81 (with ISO 11783-5 where applicable). Pelorus does not specify alternate NAME bit allocations in v1.0. Procedures are normative in [`05-addressing.md`](./05-addressing.md); Address Claimed uses DCID 0x0EE00.

### 2.3 Commanded Address (DCID 0xFED8)

Support is required per [`05-addressing.md §4`](./05-addressing.md).

| Attribute | Value |
|---|---|
| Pelorus wire DCID | 0xFED8 |
| Purpose | Command a node to adopt a specific source address |
| Priority / PDU format / data field | Per SAE J1939 Digital Annex and [`03-data-link.md`](./03-data-link.md) framing rules |
| Pelorus-specific payload constraints | None in v1.0 |

## 3. DCID Ranges and Assignment Rules

Numeric DCIDs follow derivation in [`03-data-link.md §2.2`](./03-data-link.md). Sub-ranges:

| Range | Purpose |
|---|---|
| `0x00000`–`0x0FF7F` | Compatibility, standard marine, vendor proprietary, and protocol reservations. Subdivisions in [`03-data-link.md §3`](./03-data-link.md). This document registers which compatibility DCIDs Pelorus uses; legacy bit layouts remain in their respective standards. |
| `0x0FF80`–`0x0FFFF` | Pelorus extensions. Assigned: `0x0FF80` (WUF), `0x0FF81` (NM), `0x0FF82` (Bus Health, [`08-redundancy.md`](./08-redundancy.md)), `0x0FF83` (Time Sync, [`08-redundancy.md`](./08-redundancy.md)). `0x0FF84`–`0x0FF8F` reserved. |
| `0x10000`+ | Reserved for future manufacturer-specific or Pelorus v2+ allocation. Shall not collide with [`03-data-link.md`](./03-data-link.md) derivation rules. |

Assignment authority: Pelorus DCIDs are allocated in this registry. Additions require a pull request updating this document and the corresponding entries in the signal catalog.

## 4. Relationship to Signal Catalog and Binding

- Every DCID field carrying an instance value is resolved to a `Vessel.*` path via the binding table ([`06-signal-catalog.md §3–4`](./06-signal-catalog.md)).
- v1.0: binding-table contents are not carried on NM payload bytes. Distribution is out of band (gateway configuration, diagnostic session, Pelorus Stream).
- Low-power sensors only transmit raw DCIDs; semantic mapping is handled by binding-aware nodes.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
