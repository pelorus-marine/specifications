# Pelorus Core — Data Contract Registry

**Version:** 0.3 Draft
**Last Updated:** May 19, 2026
**Trust:** Unverified

A **Data Contract (DC)** is a Pelorus-owned, named definition of a message on Pelorus Core: priority, payload bit layout, instance model, and semantics. Each DC has a numeric **Data Contract ID (DC_ID)** allocated in this registry. The 18-bit DC_ID is carried in the Pelorus-native 29-bit identifier per [`03-data-link.md §2`](./03-data-link.md). A DC may additionally specify **bridges** to legacy identifiers (J1939 PGN, NMEA 2000, NMEA 0183) used by Core ↔ LMDE gateways.

The semantic counterpart of the registry is [`06-signal-catalog.md`](./06-signal-catalog.md). Dual-bus DCs (`PelorusDC.BusHealth`, `PelorusDC.TimeSync`) are defined in [`08-redundancy.md`](./08-redundancy.md). Wake-up and network-management payload layouts are normative in [`04-power.md`](./04-power.md). Multi-frame transport is normative in [`03-data-link.md §4`](./03-data-link.md). Firmware update is normative in [`12-firmware-update.md`](./12-firmware-update.md).

## 1. Pelorus Protocol Data Contracts

DC_ID range `0x00001`–`0x000FF` carries Pelorus-owned protocol traffic.

### 1.1 `PelorusDC.WakeUp`

| Attribute | Value |
|---|---|
| DC_ID | `0x00001` |
| Priority | 0 (highest) |
| Type | Single frame |
| Length | 8 bytes |
| Transmission | Broadcast on selective wake events |
| Purpose | Triggers partial-network wake-up per ISO 11898-2:2016 |

| Byte(s) | Field |
|---|---|
| 0 | Functional-group bitmask ([`04-power.md §3`](./04-power.md)) |
| 1–7 | Reserved — transmit `0x00`, ignore on receive |

### 1.2 `PelorusDC.NetworkManagement`

| Attribute | Value |
|---|---|
| DC_ID | `0x00002` |
| Priority | 6 |
| Type | Single frame (200 ms cadence when active) |
| Length | 8 bytes |
| Purpose | Coordinated cluster sleep / wake — CanNm-style behaviour |

| Byte | Field |
|---|---|
| 0 | NM state ([`04-power.md §6.2`](./04-power.md)) |
| 1 | Active functional groups — low byte |
| 2–7 | Reserved — transmit `0x00`, ignore on receive |

### 1.3 Dual-Bus Data Contracts

`PelorusDC.BusHealth` (`DC_ID = 0x00003`) and `PelorusDC.TimeSync` (`DC_ID = 0x00004`, optional) are defined in [`08-redundancy.md`](./08-redundancy.md). Wire layouts and transmission rules live with the dual-bus mechanism that consumes them.

### 1.4 Addressing Data Contracts

`PelorusDC.AddressClaim` (`DC_ID = 0x00005`) and `PelorusDC.AddressCommand` (`DC_ID = 0x00006`) carry J1939-81 NAME-based address-claim payloads. Procedures are normative in [`05-addressing.md`](./05-addressing.md). The address-claim *protocol* (NAME comparison, claim sequence, conflict resolution) is unchanged from J1939; only the wire identifier is Pelorus-native.

### 1.5 Request Mechanism

`PelorusDC.Request` (`DC_ID = 0x00007`, priority 6) carries a requested `DC_ID` (3 bytes, little-endian) in its payload. Behaviour is normative in [`03-data-link.md §4.8`](./03-data-link.md).

### 1.6 Multi-Frame Transport

`PelorusDC.MultiFrameControl` (`DC_ID = 0x00008`) and `PelorusDC.MultiFrameData` (`DC_ID = 0x00009`) implement Pelorus-native multi-frame transport. Wire layout and state machine are normative in [`03-data-link.md §4`](./03-data-link.md).

### 1.7 Firmware Update

| DC | DC_ID | Purpose |
|---|---|---|
| `PelorusDC.FirmwareUpdateQuery` | `0x0000A` | Query device for firmware version and update capabilities |
| `PelorusDC.FirmwareUpdateResponse` | `0x0000B` | Response: version, slot model, signing model, supported manifests |
| `PelorusDC.FirmwareUpdateBegin` | `0x0000C` | Begin update session — carries manifest |
| `PelorusDC.FirmwareUpdateProgress` | `0x0000D` | Periodic progress + status from device under update |
| `PelorusDC.FirmwareUpdateActivate` | `0x0000E` | Switch to new image (A/B slot atomic switch, or commit single-slot) |
| `PelorusDC.FirmwareUpdateRollback` | `0x0000F` | Roll back to previous image |

All firmware-update DCs use priority 7. Wire layouts and protocol state machine are normative in [`12-firmware-update.md`](./12-firmware-update.md).

### 1.8 Reserved

DC_ID `0x00010`–`0x000FF` is reserved for future Pelorus protocol use. Allocation requires a pull request updating this document and the corresponding entry in [`06-signal-catalog.md`](./06-signal-catalog.md) where applicable.

## 2. Compatibility Data Contracts

DC_ID range `0x00100`–`0x03FFF` carries Pelorus DCs whose payload bit layout corresponds to a legacy J1939, NMEA 2000, or NMEA 0183 message. Each such DC declares one or more **bridges** that gateways use to translate between Pelorus Core and LMDE.

### 2.1 Bridge Mechanism

A bridge entry on a DC declares a legacy identifier the DC corresponds to. Bridge bit layout MUST be bit-identical to the Pelorus DC payload — gateways translate identifiers and reframe (Classical CAN ↔ CAN FD) but never parse-and-repack payloads. CI enforcement of bit identity is repo tooling (out of v1.0 spec scope; tracked in [`CONTRIBUTING.md`](../CONTRIBUTING.md)).

Registry entry shape (canonical machine-readable form lives in `catalog/contracts/*.yaml` when that artifact lands; this document carries human-readable equivalents):

```yaml
- name: PelorusDC.AISClassAPosition
  dc_id: 0x00110
  priority: 4
  payload:
    layout_ref: J1939_DA::PGN129038   # bit-identical to bridged source
  catalog_lane: Vessel.AIS.TargetClassA[*].Position
  instance:
    via: address_claim_name
  bridges:
    - protocol: J1939
      identifier_kind: PGN
      identifier: 129038
```

The bit layout content of bridged messages is preserved verbatim from the J1939 Digital Annex (or other source standard); Pelorus does not redefine semantics for bridged contracts.

### 2.2 Initial Bridged Contracts (J1939 heritage)

| Pelorus DC | DC_ID | Priority | J1939 PGN bridge | Catalog lane |
|---|---|---|---|---|
| `PelorusDC.EngineController1` | `0x00100` | 5 | 61444 | `Vessel.Powertrain.Engine[*].Rpm` (and additional engine fields per DA) |
| `PelorusDC.VehicleHeading` | `0x00101` | 2 | 65256 | `Vessel.HeadingTrue` |
| `PelorusDC.EngineTemp1` | `0x00102` | 5 | 65253 | `Vessel.Powertrain.Engine[*].CoolantTemp` |
| `PelorusDC.AISClassAPosition` | `0x00110` | 4 | 129038 | `Vessel.AIS.TargetClassA[*].Position` |
| `PelorusDC.AISClassBPosition` | `0x00111` | 4 | 129039 | `Vessel.AIS.TargetClassB[*].Position` |
| `PelorusDC.AISClassBExtPosition` | `0x00112` | 4 | 129040 | `Vessel.AIS.TargetClassB[*].PositionExt` |
| `PelorusDC.AISUTCDate` | `0x00113` | 4 | 129793 | `Vessel.AIS.UTCDate` |
| `PelorusDC.AISClassAStatic` | `0x00114` | 4 | 129794 | `Vessel.AIS.TargetClassA[*].Static` |
| `PelorusDC.AISClassBStaticA` | `0x00115` | 4 | 129809 | `Vessel.AIS.TargetClassB[*].StaticA` |
| `PelorusDC.AISClassBStaticB` | `0x00116` | 4 | 129810 | `Vessel.AIS.TargetClassB[*].StaticB` |

Authoritative bit layouts remain in the SAE J1939 Digital Annex; gateways bridging LMDE Classical CAN shall preserve field semantics.

### 2.3 NAME Field

The NAME carried in `PelorusDC.AddressClaim` payload is defined by SAE J1939-81 (with ISO 11783-5 where applicable). Pelorus does not specify alternate NAME bit allocations in v1.0.

## 3. DC_ID Namespace

The 18-bit DC_ID space (`0x00000`–`0x3FFFF`) is partitioned as follows:

| Range | Purpose | Slot count |
|---|---|---|
| `0x00000` | Reserved — shall not be assigned | 1 |
| `0x00001`–`0x000FF` | Pelorus protocol (network management, transport, addressing, diagnostics, firmware update) | 255 |
| `0x00100`–`0x03FFF` | Compatibility — bridged contracts (J1939 / NMEA 2000 / NMEA 0183 origin) | ~16K |
| `0x04000`–`0x2FFFF` | General Pelorus contracts (sensors, controls, services) | ~180K |
| `0x30000`–`0x3EFFF` | Reserved for v2+ Pelorus expansion | ~61K |
| `0x3F000`–`0x3FFFF` | Vendor proprietary | 4096 |

The DC_ID namespace is Pelorus-owned in its entirety; Pelorus does not carve from any third-party identifier space.

**Assignment authority.** Pelorus DCs are allocated in this registry. Additions require a pull request updating this document, the corresponding entries in [`06-signal-catalog.md`](./06-signal-catalog.md), and any machine-readable contract artifact (per [`CONTRIBUTING.md`](../CONTRIBUTING.md)).

**Vendor proprietary range.** Values `0x3F000`–`0x3FFFF` are available for vendor-specific contracts that do not need a registry entry. Vendors are responsible for avoiding collisions within their own product lines; Pelorus does not register vendor proprietary DCs.

## 4. Relationship to Signal Catalog and Binding

- Every DC field carrying an instance value is resolved to a `Vessel.*` path via the binding table ([`06-signal-catalog.md §3–4`](./06-signal-catalog.md)).
- v1.0: binding-table contents are not carried on `PelorusDC.NetworkManagement` payload bytes. Distribution is out of band (gateway configuration, diagnostic session, Pelorus Stream).
- Low-power sensors only transmit raw DCs; semantic mapping is handled by binding-aware nodes.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
