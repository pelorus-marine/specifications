# Pelorus Core — Addressing

Source addressing, address claiming, conflict resolution, device identification. Dual-bus claiming for Class D / Class H nodes lives in [`08-redundancy.md`](./08-redundancy.md).

## 1. Address Space

8-bit Source Address (SA) field, bits 7–0 of the 29-bit identifier.

- Valid claimed addresses: 0x00–0xFD (253 addresses)
- 0xFE: Null address (used in some diagnostic messages)
- 0xFF: Global address (broadcast SA placeholder for nodes that have not claimed)

A node must successfully claim a unique SA before transmitting application data (DCs other than `Pelorus.AddressClaim` or `Pelorus.NetworkManagement`).

## 2. NAME Field

Every device is identified by a unique 64-bit NAME (J1939 format). The NAME is the primary identifier for address conflict resolution and is transmitted in every `Pelorus.AddressClaim` message.

NAME structure follows SAE J1939-81 exactly (8 bytes): Arbitrary Address Capable, Industry Group (Marine = 4), Device Class, Function, Function Instance, Device Class Instance, Manufacturer Code, Identity Number.

Pelorus does not specify alternate NAME encodings in v1.0.

### 2.1 NAME for Owner-Built Devices

Sailors building their own devices to install on their own vessels (using the Owner Private DC range per [`07-dcid-registry.md §3`](./07-dcid-registry.md)) shall transmit a valid `Pelorus.AddressClaim` — the address-claim mechanism does not exempt owner-built devices. The NAME field is not used to disambiguate Owner Private DCs (per [`07-dcid-registry.md §3`](./07-dcid-registry.md)), so any valid NAME is acceptable.

Recommended values for owner-built devices:

| NAME field | Recommended value | Rationale |
| --- | --- | --- |
| Arbitrary Address Capable | `1` | Allows the device to accept a `Pelorus.AddressCommand` (§4). |
| Industry Group | `4` (Marine) | Pelorus is Marine. |
| Device Class | per J1939-81, as appropriate (e.g. `60` = Navigation, `80` = Sensors) | Lets generic tools categorise the device. |
| Function | per J1939-81, or `255` if no listed function fits | Function code is informational; Pelorus does not route by it. |
| Function Instance | `0` for a single instance; sequential for multiples | |
| Device Class Instance | `0` for a single instance; sequential for multiples | |
| Manufacturer Code | `0` (J1939-conventional "unassigned") | Recommended convention for owner-built devices so tools and gateways can recognise them as such. Any value the owner chooses is acceptable. |
| Identity Number | any 21-bit value | Uniqueness is only required within the vessel; pick a stable per-device value (e.g. from MAC address, MCU serial). |

Owner-built devices using Manufacturer Code `0` shall not transmit on Vendor Proprietary DCs (`0x3F100–0x3FFFF`), which require a Manufacturer Code that receiving devices are configured to accept. They may freely transmit on Owner Private DCs (`0x3F000–0x3F0FF`) and on public general, compatibility, and Pelorus protocol DCs.

Small builders or open-source hardware projects that distribute the same device to multiple vessels — and therefore cannot use Owner Private DCs — may request a **free Pelorus-allocated Manufacturer Code** in the range `1900`–`2047` per [`../manufacturer-codes.md`](../manufacturer-codes.md). Allocation is by pull request; no fees, no membership, no commercial-status check. Vendors with an existing NMEA 2000 or SAE J1939 Manufacturer Code use that value directly without further registration — the same 11-bit code identifies a vendor across NMEA 2000, OneNet, and Pelorus Core.

## 3. Address Claim Procedure

The address-claim protocol follows SAE J1939-81 — only the wire identifier carrying the message is Pelorus-native (`Pelorus.AddressClaim`, `DC_ID = 0x00005`, priority 6). On power-up, reset, or when joining the network a node shall:

1. Listen for 250 ms for any existing `Pelorus.AddressClaim` messages.
2. Select a preferred address (or a dynamically chosen one if the preferred is taken).
3. Transmit a `Pelorus.AddressClaim` message containing its full 64-bit NAME and the desired SA.
4. Monitor the bus for conflicting claims.

**Conflict resolution:** the node with the numerically lower NAME (treated as a 64-bit unsigned integer) wins the address. The losing node selects a new address and re-claims with random back-off per J1939-81.

A node that cannot claim an address after repeated attempts enters a "cannot claim" state and may only transmit diagnostic or network-management messages.

## 4. Commanded Address (`Pelorus.AddressCommand`)

Support is required. `Pelorus.AddressCommand` (`DC_ID = 0x00006`, priority 6) carries the target NAME and the new SA. Payload semantics follow SAE J1939-81 Commanded Address. Allows gateways and provisioning UIs to assign a specific SA on a device.

## 5. Interaction with Power Management

- Address claiming occurs only after a node has woken (or is in Active state).
- A node in Standby/Sleep/Deep Sleep shall not transmit `Pelorus.AddressClaim` messages.
- On wake-up, a node shall re-verify/refresh its claimed address before transmitting application data. See [`04-power.md`](./04-power.md).

## 6. Relationship to Signal Catalog Instance Binding

Source Address alone does not carry semantic meaning. The mapping from SA + DC_ID + instance fields to semantic paths in the `Vessel.*` catalog (defined in [`../catalog/`](../catalog/)) is handled by the binding table in [`06-instance-binding.md`](./06-instance-binding.md). Address claiming itself remains purely about uniqueness on the bus.

## 7. Open Items

### 7.1 No authentication of NAME or Source Address

The address-claim mechanism inherited from SAE J1939-81 has no cryptographic authentication: any node can transmit a `Pelorus.AddressClaim` declaring any NAME and, if its NAME is numerically lower than the current holder's, win arbitration. A malicious or buggy node can therefore impersonate any device on the bus — including safety-critical talkers such as autopilots, GNSS, and alarm sources — and either silence the legitimate device (which loses arbitration) or inject spoofed payloads under its identity.

This is acceptable for v1.0 because:

- CAN-bus physical access is required to attack — there is no remote vector through the wire itself.
- The same exposure exists in NMEA 2000 / J1939 and the marine market has tolerated it for thirty years.
- Cryptographic message-level authentication on a 250 kbit/s arbitration / 500 kbit/s data-phase fieldbus has non-trivial frame-rate and latency costs.

It should not remain acceptable indefinitely because Pelorus Core explicitly carries safety-critical traffic (autopilot commands, MOB alarms, alarm assertions) where impersonation is a credible threat — for example, a compromised Pelorus Stream gateway with a Core-side transceiver, or a malicious USB-CAN diagnostic dongle plugged into a chandlery service port.

Action for a future profile: define an optional authenticated address-claim and per-DC message authentication scheme (candidate: CAN-Sec / SecOC-style truncated MAC with per-segment shared keys, or AUTOSAR Secure Onboard Communication adapted for CAN FD's 64-byte frame). Scope should cover (a) address-claim authentication so impersonation requires key compromise, and (b) per-DC freshness counters and MAC for the safety-critical priority bands (PRIO 0–2). Lower-priority bands may remain unauthenticated to preserve bus capacity.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
