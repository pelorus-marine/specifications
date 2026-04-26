# Pelorus Core — Addressing Specification

**Version:** 0.1 Draft
**Last Updated:** April 26, 2026
**Status:** Pre-specification

---

## About This Document

This document specifies source addressing, address claiming, conflict resolution, and device identification for Pelorus Core.  

**Design decision (locked):** Pelorus Core addressing is **identical** to the legacy marine data ecosystem (SAE J1939-81 / ISO 11783-5). No deviations are introduced in v1.0. This makes gateway translation to legacy marine networks trivial and preserves all existing diagnostic and configuration tool behavior.

---

## 1. Address Space

Pelorus Core uses an 8-bit Source Address (SA) field (bits 7–0 of the 29-bit identifier).

- Valid claimed addresses: 0x00–0xFD (253 addresses)
- 0xFE: Null address (used in some diagnostic messages)
- 0xFF: Global address (broadcast destination in PDU2 messages)

Every node must successfully claim a unique SA before it may transmit application data (PGNs other than Address Claimed or Network Management).

---

## 2. Device Identification — The NAME Field

Every device is identified by a unique 64-bit NAME (J1939 format). The NAME is the primary identifier for address conflict resolution and is transmitted in every Address Claimed message.

The NAME structure follows legacy marine data ecosystem exactly (8 bytes):

- Byte 0–7: Arbitrary Address Capable (1 bit), Industry Group (3 bits, Marine = 4), Device Class, Function, Function Instance, Device Class Instance, Manufacturer Code, Unique Number, etc.

Exact bit field allocations and preferred address ranges per device class/function will be defined in `07-pgn-registry.md`.

---

## 3. Address Claim Procedure

Nodes follow the exact J1939-81 / legacy marine data ecosystem address claim procedure:

On power-up, reset, or when joining the network a node shall:
1. Listen for 250 ms for any existing Address Claimed messages.
2. Select a preferred address (or a dynamically chosen one if the preferred is taken).
3. Transmit an "Address Claimed" message (PGN 0x0EE00, priority 6) containing its full 64-bit NAME and the desired SA.
4. Monitor the bus for conflicting claims.

**Conflict resolution rule (identical to the legacy marine data ecosystem):**
- The node with the **numerically lower NAME** (treated as a 64-bit unsigned integer) wins the address.
- The losing node must select a new address and re-claim (with random back-off timing per J1939-81 to avoid storms).

A node that cannot claim an address after repeated attempts shall enter a "cannot claim" state and may only transmit diagnostic or network-management messages.

---

## 4. Commanded Address

Support for the Commanded Address PGN (0xFED8) is **required**. This allows gateways, configuration tools, and the central gateway UI to force a specific address on a device (useful for instance binding and provisioning).

---

## 5. Interaction with Power Management

- Address claiming occurs only after a node has woken (or is in Active state).
- A node in Standby/Sleep/Deep Sleep shall not transmit Address Claimed messages.
- On wake-up, a node shall re-verify/refresh its claimed address before transmitting application data (see `04-power-management.md` for wake-up sequencing).

---

## 6. Relationship to Signal Catalog Instance Binding

Source Address alone does not carry semantic meaning (same limitation as legacy marine data ecosystem). The mapping from SA + PGN + instance fields to semantic paths in the Vessel.* catalog is handled in `06-signal-catalog.md` (instance binding problem). Address claiming itself remains purely about uniqueness on the bus.

---

## Open Items

- Exact preferred address ranges or device-class tables (to be added in 07-pgn-registry.md)
- Any Pelorus-specific NAME extensions (none planned for v1.0)
- Integration with repeater/gateway address spaces on multi-segment networks (see 08-network-architecture.md and 09-gateway-specification.md)

---

*This document completes the minimum viable specification together with documents 01–04.*