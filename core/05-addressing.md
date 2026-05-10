# Pelorus Core — Addressing

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

Source addressing, address claiming, conflict resolution, device identification. Dual-bus claiming for Class D / Class H nodes lives in [`08-redundancy.md`](./08-redundancy.md).

## 1. Address Space

8-bit Source Address (SA) field, bits 7–0 of the 29-bit identifier.

- Valid claimed addresses: 0x00–0xFD (253 addresses)
- 0xFE: Null address (used in some diagnostic messages)
- 0xFF: Global address (broadcast destination in PDU2 messages)

A node must successfully claim a unique SA before transmitting application data (DCIDs other than Address Claimed or Network Management).

## 2. NAME Field

Every device is identified by a unique 64-bit NAME (J1939 format). The NAME is the primary identifier for address conflict resolution and is transmitted in every Address Claimed message.

NAME structure follows SAE J1939-81 exactly (8 bytes): Arbitrary Address Capable, Industry Group (Marine = 4), Device Class, Function, Function Instance, Device Class Instance, Manufacturer Code, Identity Number.

Pelorus does not specify alternate NAME encodings in v1.0.

## 3. Address Claim Procedure

Per SAE J1939-81. On power-up, reset, or when joining the network a node shall:

1. Listen for 250 ms for any existing Address Claimed messages.
2. Select a preferred address (or a dynamically chosen one if the preferred is taken).
3. Transmit an Address Claimed message (DCID 0x0EE00, priority 6) containing its full 64-bit NAME and the desired SA.
4. Monitor the bus for conflicting claims.

**Conflict resolution:** the node with the numerically lower NAME (treated as a 64-bit unsigned integer) wins the address. The losing node selects a new address and re-claims with random back-off per J1939-81.

A node that cannot claim an address after repeated attempts enters a "cannot claim" state and may only transmit diagnostic or network-management messages.

## 4. Commanded Address (DCID 0xFED8)

Support is required. Payload, timing, and arbitration priority follow SAE J1939 Digital Annex for the Commanded Address message. Allows gateways and provisioning UIs to assign a specific SA on a device.

## 5. Interaction with Power Management

- Address claiming occurs only after a node has woken (or is in Active state).
- A node in Standby/Sleep/Deep Sleep shall not transmit Address Claimed messages.
- On wake-up, a node shall re-verify/refresh its claimed address before transmitting application data. See [`04-power.md`](./04-power.md).

## 6. Relationship to Signal Catalog Instance Binding

Source Address alone does not carry semantic meaning. The mapping from SA + DCID + instance fields to semantic paths in the `Vessel.*` catalog is handled in [`06-signal-catalog.md`](./06-signal-catalog.md) (instance binding). Address claiming itself remains purely about uniqueness on the bus.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
