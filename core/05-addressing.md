# Pelorus Core — Addressing Specification

**Version:** 0.1 Draft  
**Last Updated:** May 3, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specifies source addressing, address claiming, conflict resolution, and device identification for Pelorus Core. The v1.0 addressing policy (J1939-81 / ISO 11783-5 parity) is summarized in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary). **Normative** procedures are defined here; **64-bit NAME** bit fields are cited in [07-dcid-registry.md](./07-dcid-registry.md#name-field-64-bit-device-identity). Commanded Address (**0xFED8**) is registered in [07-dcid-registry.md](./07-dcid-registry.md#commanded-address-dcid-0xfed8).

**Physical layer:** The procedures in this document apply to nodes on **Pelorus Core CAN FD** segments. **LMDE** segments use the **same J1939-81 rules on Classical CAN**; each electrical segment has its own address space. Correlation across segments (e.g. a device visible on both buses) is handled by **gateways** and the binding table, not by sharing one CAN segment between classical-only and CAN FD populations.

**Dual-bus domains:** Path-redundant installations (**Bus A** and **Bus B**) are defined in **[17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md)**. **§7** extends claiming for **Class D** / **Class H** nodes.

---

## 1. Address Space

Pelorus Core uses an 8-bit Source Address (SA) field (bits 7–0 of the 29-bit identifier).

- Valid claimed addresses: 0x00–0xFD (253 addresses)
- 0xFE: Null address (used in some diagnostic messages)
- 0xFF: Global address (broadcast destination in PDU2 messages)

Every node must successfully claim a unique SA before it may transmit application data (DCIDs other than Address Claimed or Network Management).

---

## 2. Device Identification — The NAME Field

Every device is identified by a unique 64-bit NAME (J1939 format). The NAME is the primary identifier for address conflict resolution and is transmitted in every Address Claimed message.

The NAME structure follows the Legacy Marine Data Ecosystem / **SAE J1939** exactly (8 bytes): Arbitrary Address Capable, Industry Group (Marine = 4), Device Class, Function, Function Instance, Device Class Instance, Manufacturer Code, Identity Number, and related subfields.

**Normative layout:** Bit-level NAME allocation is defined in **SAE J1939-81** (*Digital Annex* / manufacturer identification rules as cited there). Pelorus **does not** specify alternate NAME encodings in v1.0. Optional **preferred source-address ranges** per device class may be added later to **07** as informative guidance only — they do not change J1939-81 NAME semantics.

---

## 3. Address Claim Procedure

Nodes follow the same J1939-81 address-claim procedure as on the Legacy Marine Data Ecosystem:

On power-up, reset, or when joining the network a node shall:
1. Listen for 250 ms for any existing Address Claimed messages.
2. Select a preferred address (or a dynamically chosen one if the preferred is taken).
3. Transmit an "Address Claimed" message (DCID 0x0EE00, priority 6) containing its full 64-bit NAME and the desired SA.
4. Monitor the bus for conflicting claims.

**Conflict resolution rule (identical to the Legacy Marine Data Ecosystem):**
- The node with the **numerically lower NAME** (treated as a 64-bit unsigned integer) wins the address.
- The losing node must select a new address and re-claim (with random back-off timing per J1939-81 to avoid storms).

A node that cannot claim an address after repeated attempts shall enter a "cannot claim" state and may only transmit diagnostic or network-management messages.

---

## 4. Commanded Address

Support for the Commanded Address DCID (**0xFED8**) is **required**. Payload layout, timing, and arbitration priority follow **SAE J1939 Digital Annex** for the Commanded Address message unless **07** lists a Pelorus-specific constraint (none in v1.0 — see [07-dcid-registry.md](./07-dcid-registry.md#commanded-address-dcid-0xfed8)). This message allows gateways, configuration tools, and provisioning UIs to assign a specific SA on a device (instance binding and fleet workflows).

---

## 5. Interaction with Power Management

- Address claiming occurs only after a node has woken (or is in Active state).
- A node in Standby/Sleep/Deep Sleep shall not transmit Address Claimed messages.
- On wake-up, a node shall re-verify/refresh its claimed address before transmitting application data (see `04-power-management.md` for wake-up sequencing).

---

## 6. Relationship to Signal Catalog Instance Binding

Source Address alone does not carry semantic meaning (same limitation as the Legacy Marine Data Ecosystem). The mapping from SA + DCID + instance fields to semantic paths in the Vessel.* catalog is handled in `06-signal-catalog.md` (instance binding problem). Address claiming itself remains purely about uniqueness on the bus.

---

## 7. Dual-bus address claiming (Class D and Class H)

For nodes with two Pelorus Core ports attached to **Bus A** and **Bus B** in the same dual-bus domain (**[17](./17-criticality-and-redundant-paths.md)**, **[02](./02-physical-layer.md)**):

1. **Simultaneous claim:** On power-up, reset, or join, a **Class D** node **shall** run the **§3** procedure on **both** buses **in parallel** (same preferred SA and same NAME on A and B).
2. **Data transmission gate:** The node **shall not** transmit application DCIDs (other than address-management traffic and **0x0FF82** / **0x0FF83** per **07**) on **either** bus until it has successfully claimed the **same** SA on **both** buses, **unless** it enters **degraded single-bus** mode per **17** §3 (operator-visible fault; continues on the surviving bus only).
3. **Conflict asymmetry:** If claiming succeeds on Bus A but fails on Bus B, the node **shall** either (a) select a **new** SA and re-claim on **both** buses from step 1, or (b) declare **degraded single-bus** on A per **17** and **shall not** transmit on B until B succeeds.
4. **Class H hubs** **shall** claim a unique SA on each bus segment they terminate; downstream **Class S** devices use normal **§3** on their single attached segment — the hub performs replication onto both backbones per **10**.

Address-claim and commanded-address frames **shall not** be subject to duplicate discard (**03** §6.2).

---

## Open Items

- Informative preferred source-address ranges or device-class tables (optional future addition to **07** — does not replace **SAE J1939-81** NAME layout)
- Any Pelorus-specific NAME extensions (none planned for v1.0)
- Integration with repeater/gateway address spaces on multi-segment networks (see 08-network-architecture.md and 09-gateway-specification.md)

---

*This document, together with documents 01–04, **07**, **17**, and **03** §6, completes addressing for Pelorus Core including dual-bus domains.*