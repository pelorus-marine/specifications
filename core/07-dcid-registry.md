# Pelorus Core — Data Contract Registry

A **Data Contract (DC)** is a Pelorus-owned, named definition of a message on Pelorus Core: priority, payload bit layout, instance model, and semantics. Each DC has a numeric **Data Contract ID (DC_ID)** allocated in this registry. The 18-bit DC_ID is carried in the Pelorus-native 29-bit identifier per [`03-data-link.md §2`](./03-data-link.md). A DC may additionally specify **bridges** to legacy identifiers (J1939 PGN, NMEA 2000, NMEA 0183) used by Core ↔ LMDE gateways.

The semantic counterpart of the registry is the `Vessel.*` catalog in [`../catalog/`](../catalog/); the bus-level identifier-to-catalog-index resolution lives in [`06-instance-binding.md`](./06-instance-binding.md). Dual-bus DCs (`Pelorus.BusHealth`, `Pelorus.TimeSync`) are defined in [`08-redundancy.md`](./08-redundancy.md). Wake-up and network-management payload layouts are normative in [`04-power.md`](./04-power.md). Multi-frame transport is normative in [`03-data-link.md §4`](./03-data-link.md). Firmware update is normative in [`12-firmware-update.md`](./12-firmware-update.md).

## 1. Pelorus Protocol Data Contracts

DC_ID range `0x00001`–`0x000FF` carries Pelorus-owned protocol traffic.

### 1.1 `Pelorus.WakeUp`

| Attribute | Value |
| --- | --- |
| DC_ID | `0x00001` |
| Priority | 0 (highest) |
| Type | Single frame |
| Length | 8 bytes |
| Transmission | Broadcast on selective wake events |
| Purpose | Triggers partial-network wake-up per ISO 11898-2:2016 |

| Byte(s) | Field |
| --- | --- |
| 0 | Functional-group bitmask ([`04-power.md §3`](./04-power.md)) |
| 1–7 | Reserved — transmit `0x00`, ignore on receive |

### 1.2 `Pelorus.NetworkManagement`

| Attribute | Value |
| --- | --- |
| DC_ID | `0x00002` |
| Priority | 6 |
| Type | Single frame (200 ms cadence when active) |
| Length | 8 bytes |
| Purpose | Coordinated cluster sleep / wake — CanNm-style behaviour |

| Byte | Field |
| --- | --- |
| 0 | NM state ([`04-power.md §6.2`](./04-power.md)) |
| 1 | Active functional groups — low byte |
| 2–7 | Reserved — transmit `0x00`, ignore on receive |

### 1.3 Dual-Bus Data Contracts

`Pelorus.BusHealth` (`DC_ID = 0x00003`) and `Pelorus.TimeSync` (`DC_ID = 0x00004`) are defined in [`08-redundancy.md`](./08-redundancy.md). `Pelorus.TimeSync` is **mandatory** in any dual-bus domain and optional in single-bus C2-only domains. Wire layouts, the `TimeStatus` byte encoding, Time Master election, and consumer obligations live with the dual-bus mechanism that consumes them.

### 1.4 Addressing Data Contracts

`Pelorus.AddressClaim` (`DC_ID = 0x00005`) and `Pelorus.AddressCommand` (`DC_ID = 0x00006`) carry J1939-81 NAME-based address-claim payloads. Procedures are normative in [`05-addressing.md`](./05-addressing.md). The address-claim *protocol* (NAME comparison, claim sequence, conflict resolution) is unchanged from J1939; only the wire identifier is Pelorus-native.

### 1.5 Request Mechanism

`Pelorus.Request` (`DC_ID = 0x00007`, priority 6) carries a requested `DC_ID` (3 bytes, little-endian) in its payload. Behaviour is normative in [`03-data-link.md §4.8`](./03-data-link.md).

### 1.6 Multi-Frame Transport

`Pelorus.MultiFrameControl` (`DC_ID = 0x00008`) and `Pelorus.MultiFrameData` (`DC_ID = 0x00009`) implement Pelorus-native multi-frame transport. Wire layout and state machine are normative in [`03-data-link.md §4`](./03-data-link.md).

### 1.7 Firmware Update

| DC | DC_ID | Purpose |
| --- | --- | --- |
| `Pelorus.FirmwareUpdateQuery` | `0x0000A` | Query device for firmware version and update capabilities |
| `Pelorus.FirmwareUpdateResponse` | `0x0000B` | Response: version, slot model, signing model, supported manifests |
| `Pelorus.FirmwareUpdateBegin` | `0x0000C` | Begin update session — carries manifest |
| `Pelorus.FirmwareUpdateProgress` | `0x0000D` | Periodic progress + status from device under update |
| `Pelorus.FirmwareUpdateActivate` | `0x0000E` | Switch to new image (A/B slot atomic switch, or commit single-slot) |
| `Pelorus.FirmwareUpdateRollback` | `0x0000F` | Roll back to previous image |

All firmware-update DCs use priority 7. Wire layouts and protocol state machine are normative in [`12-firmware-update.md`](./12-firmware-update.md).

### 1.8 Reserved

DC_ID `0x00010`–`0x000FF` is reserved for future Pelorus protocol use. Allocation requires a pull request updating this document and the corresponding entry in [`../catalog/`](../catalog/) where applicable.

## 2. Compatibility Data Contracts

DC_ID range `0x00100`–`0x03FFF` carries Pelorus DCs whose payload bit layout corresponds to a legacy J1939, NMEA 2000, or NMEA 0183 message. Each such DC declares one or more **bridges** that gateways use to translate between Pelorus Core and LMDE.

### 2.1 Bridge Mechanism

A bridge entry on a DC declares a legacy identifier and the projection between the legacy payload and the Pelorus DC payload. Pelorus does not redefine the semantics of legacy fields it bridges; the source standard remains authoritative for field meaning, units, and scaling.

**Directional asymmetry.** Bridges are specified in two directions, with different normative weight:

- **Legacy → Pelorus (decomposition)** is **required**. A gateway bridging legacy traffic onto Pelorus Core shall parse the legacy payload, decode each declared field, and emit one or more Pelorus DC frames carrying the projected values. This is the direction that onboards legacy sensors during the transition.
- **Pelorus → Legacy (aggregation)** is **best-effort** and out of normative scope for v1.0. A gateway product that emits legacy frames from Pelorus DC traffic defines its own staleness policy for collecting the required source values, and may decline to bridge a given Pelorus DC if aggregation is impractical. Pelorus-native DC granularity is not constrained by the difficulty of round-trip bridging.

**Projection styles.** A bridge entry MAY declare its payload projection as:

- *bit-identical* — the Pelorus DC payload matches the legacy payload byte-for-byte. Gateways translate identifiers, reframe Classical CAN ↔ CAN FD, and forward payload unchanged. Recommended when a Pelorus DC corresponds 1:1 to a legacy message and grouping is appropriate.
- *repack* — the gateway decodes legacy fields and emits one or more Pelorus DC frames with re-arranged layout. Required when Pelorus uses finer-grained (per-quantity) DCs that bridge from multi-field legacy PGNs.

Registry entry shape (canonical machine-readable form lives in `catalog/contracts/*.yaml` when that artifact lands; this document carries human-readable equivalents):

```yaml
# Example A: per-quantity Pelorus DC, repack bridge from a multi-field PGN
- name: Pelorus.EngineSpeed
  dc_id: 0x00100
  priority: 5
  catalog_lane: Vessel.Propulsion.Engines[*].Speed
  instance:
    via: address_claim_name
  bridges:
    - protocol: J1939
      identifier_kind: PGN
      identifier: 61444            # EEC1
      projection: repack
      source_field: EngineSpeed    # SPN 190

# Example B: grouped Pelorus DC, bit-identical bridge
- name: Pelorus.AISClassAPosition
  dc_id: 0x00110
  priority: 4
  payload:
    layout_ref: J1939_DA::PGN129038
  catalog_lane: Vessel.Navigation.AIS.TargetClassA[*].Position
  instance:
    via: address_claim_name
  bridges:
    - protocol: J1939
      identifier_kind: PGN
      identifier: 129038
      projection: bit_identical
```

### 2.2 Initial Bridged Contracts

DC names describe the Pelorus payload contents; they are not derived from legacy message labels. Granularity follows the Pelorus design (per-quantity by default; grouped when atomicity or shared update cadence demands it). Bridges declare the legacy source field(s) per §2.1.

| Pelorus DC | DC_ID | Priority | Bridge(s) | Projection | Catalog lane |
| --- | --- | --- | --- | --- | --- |
| `Pelorus.EngineSpeed` | `0x00100` | 5 | J1939 PGN 61444 (EEC1, field `EngineSpeed`) | repack | `Vessel.Propulsion.Engines[*].Speed` |
| `Pelorus.HeadingTrue` | `0x00101` | 2 | J1939 PGN 65256 (VD, field `Heading`) | repack | `Vessel.Navigation.HeadingTrue` |
| `Pelorus.EngineCoolantTemp` | `0x00102` | 5 | J1939 PGN 65262 (ET1, field `EngineCoolantTemperature`) | repack | `Vessel.Propulsion.Engines[*].CoolantTemp` |
| `Pelorus.Position` | `0x00103` | 2 | NMEA2000 PGN 129025 (lat/lon) + PGN 129029 (fix quality) | repack | `Vessel.Navigation.Position.{Latitude,Longitude,FixQuality}` |
| `Pelorus.DepthBelowTransducer` | `0x00104` | 5 | NMEA2000 PGN 128267 (field `Depth`) | repack | `Vessel.Navigation.Depth.BelowTransducer` |
| `Pelorus.DepthTransducerOffset` | `0x00105` | 6 | NMEA2000 PGN 128267 (field `Offset`) | repack | `Vessel.Navigation.Depth.TransducerOffset` |
| `Pelorus.AISClassAPosition` | `0x00110` | 4 | NMEA2000 PGN 129038 | bit_identical | `Vessel.Navigation.AIS.TargetClassA[*].Position` |
| `Pelorus.AISClassBPosition` | `0x00111` | 4 | NMEA2000 PGN 129039 | bit_identical | `Vessel.Navigation.AIS.TargetClassB[*].Position` |
| `Pelorus.AISClassBExtPosition` | `0x00112` | 4 | NMEA2000 PGN 129040 | bit_identical | `Vessel.Navigation.AIS.TargetClassB[*].PositionExt` |
| `Pelorus.AISUTCDate` | `0x00113` | 4 | NMEA2000 PGN 129793 | bit_identical | `Vessel.Navigation.AIS.UTCDate` |
| `Pelorus.AISClassAStatic` | `0x00114` | 4 | NMEA2000 PGN 129794 | bit_identical | `Vessel.Navigation.AIS.TargetClassA[*].Static` |
| `Pelorus.AISClassBStaticA` | `0x00115` | 4 | NMEA2000 PGN 129809 | bit_identical | `Vessel.Navigation.AIS.TargetClassB[*].StaticA` |
| `Pelorus.AISClassBStaticB` | `0x00116` | 4 | NMEA2000 PGN 129810 | bit_identical | `Vessel.Navigation.AIS.TargetClassB[*].StaticB` |

For bridged fields, the source standard (SAE J1939 Digital Annex; NMEA 2000 Appendix B) remains authoritative for unit, scaling, and range; the Pelorus DC inherits those semantics for the bridged field(s).

**Notes on multi-source bridges.** `Pelorus.Position` aggregates fast-updating lat/lon (PGN 129025, ~10 Hz) with slower-updating fix quality (PGN 129029, ~1 Hz). The gateway holds the last-known fix quality and attaches it to each emitted `Pelorus.Position` frame. This stateful aggregation is permitted in the legacy → Pelorus direction per §2.1; the reverse direction is not specified.

**Notes on enums.** `Pelorus.Position.FixQuality` is a Pelorus-native enum (NMEA 2000 PGN 129029 GNSS Method serves as the bridge source, with values remapped if needed):

| Value | Meaning |
| --- | --- |
| 0 | No fix |
| 1 | GNSS (uncorrected single-frequency) |
| 2 | DGNSS |
| 3 | Precision GNSS |
| 4 | RTK fixed |
| 5 | RTK float |
| 6 | Estimated / dead-reckoning |
| 7 | Manual input |
| 8 | Simulator |

**Notes on `Pelorus.DepthTransducerOffset`.** Carries the install-time signed offset (positive when transducer is mounted above the keel). Cadence is "on configuration change and periodic refresh" (≪ 1 Hz); priority 6 reflects its rare-transmission, low-criticality character. Consumers derive below-keel depth as `BelowTransducer − TransducerOffset`.

### 2.3 NAME Field

The NAME carried in `Pelorus.AddressClaim` payload is defined by SAE J1939-81 (with ISO 11783-5 where applicable). Pelorus does not specify alternate NAME bit allocations in v1.0.

## 3. DC_ID Namespace

The 18-bit DC_ID space (`0x00000`–`0x3FFFF`) is partitioned as follows:

| Range | Purpose | Slot count |
| --- | --- | --- |
| `0x00000` | Reserved — shall not be assigned | 1 |
| `0x00001`–`0x000FF` | Pelorus protocol (network management, transport, addressing, diagnostics, firmware update) | 255 |
| `0x00100`–`0x03FFF` | Compatibility — bridged contracts (J1939 / NMEA 2000 / NMEA 0183 origin) | ~16K |
| `0x04000`–`0x2FFFF` | General Pelorus contracts (sensors, controls, services) | ~180K |
| `0x30000`–`0x3EFFF` | Reserved for v2+ Pelorus expansion | ~61K |
| `0x3F000`–`0x3F0FF` | Owner Private — per-vessel, no registration | 256 |
| `0x3F100`–`0x3FFFF` | Vendor Proprietary — NAME Manufacturer Code disambiguation | 3840 |

The DC_ID namespace is Pelorus-owned in its entirety; Pelorus does not carve from any third-party identifier space.

**Assignment authority.** Pelorus DCs are allocated in this registry. Additions require a pull request updating this document, the corresponding entries in [`../catalog/`](../catalog/), and any machine-readable contract artifact (per [`CONTRIBUTING.md`](../CONTRIBUTING.md)).

**Owner Private range (`0x3F000–0x3F0FF`).** Reserved for sailor-built or owner-built devices installed on a specific vessel. No external registration is required — not with Pelorus, SAE, NMEA, or any vendor. Slot assignments within this range are recorded in the vessel's critical zone map ([`08-redundancy.md §12`](./08-redundancy.md)) and have no defined meaning outside that vessel.

Frames on DCs in this range:

- Shall not be used in firmware shipped to multiple vessels. Products distributed to more than one vessel shall use the general Pelorus contract range (PR-allocated) or Vendor Proprietary (registered Manufacturer Code).
- Are not disambiguated by NAME Manufacturer Code — receivers gate on per-vessel configuration recorded in the critical zone map.
- Shall not declare `bridges[*]` to legacy NMEA / J1939 / NMEA-0183 identifiers, and shall not be translated by gateways. The range is Pelorus-Core-only by construction.
- May use any NAME value (see [`05-addressing.md §2.1`](./05-addressing.md) for recommended NAME values for owner-built devices).

**Vendor Proprietary range (`0x3F100–0x3FFFF`).** Reserved for vendor-specific contracts in commercial products. Pelorus does not register Vendor Proprietary DCs.

Frames on DCs in this range:

- Are identified by the tuple `(NAME Manufacturer Code, DC_ID)`, where the Manufacturer Code is resolved from the source SA via the address-claim cache populated by [`05-addressing.md`](./05-addressing.md). The same `DC_ID` slot used by two different Manufacturer Codes denotes two distinct contracts. Code-space partitioning and the free Pelorus-allocated range (`1900`–`2047`) for small builders without an NMEA / SAE code are defined in [`../manufacturer-codes.md`](../manufacturer-codes.md).
- Shall be ignored by receivers from sources whose Manufacturer Code the receiver is not configured to process, and from sources whose address claim has not yet succeeded.
- Shall not declare `bridges[*]` to legacy proprietary PGNs in v1.0. The Pelorus and NMEA / J1939 disambiguation mechanisms differ (NAME-based vs payload-prefix), and v1.0 does not specify a translation rule.

Vendors using this range are responsible for avoiding collisions within their own product lines (the same Manufacturer Code is shared across all of a vendor's products). Cross-vendor collisions are precluded by the NAME-based disambiguation rule above.

## 4. Relationship to Signal Catalog and Binding

- Every DC field carrying an instance value is resolved to a `Vessel.*` path (defined in [`../catalog/`](../catalog/)) via the binding table in [`06-instance-binding.md`](./06-instance-binding.md).
- v1.0: binding-table contents are not carried on `Pelorus.NetworkManagement` payload bytes. Distribution is out of band (gateway configuration, diagnostic session, Pelorus Stream).
- Low-power sensors only transmit raw DCs; semantic mapping is handled by binding-aware nodes.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
