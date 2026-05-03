# Pelorus Core — Network Architecture Specification

**Version:** 0.1 Draft  
**Last Updated:** May 3, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **normative** network architecture for Pelorus Core: segment limits, multi-segment scaling, repeater usage, topology recommendations, and **dual-bus domains** (path redundancy). It builds on [02-physical-layer.md](./02-physical-layer.md), [05-addressing.md](./05-addressing.md), and [17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md). A concise summary appears in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary).

---

## 1. Single-Segment Limits

Every Pelorus Core segment shall observe these hard limits:

- **Maximum segment length:** 30 m (backbone cable between terminators)
- **Maximum nodes per segment:** 50
- **Maximum stub length:** 6 m (drop cable from T-connector to device)
- **Cable and connector:** M12 A-coded 5-pin, LMDE micro standard (per `02-physical-layer.md`)
- **Termination:** Split termination at both ends of the segment
- **Power:** 9–32 V DC, reverse polarity protected

These limits ensure signal integrity at 250 kbit/s arbitration / 500 kbit/s data phase without requiring special transceivers.

---

## 2. Multi-Segment Networks and Repeaters

Vessels exceeding a single 30 m segment shall use **repeater nodes** to create multiple electrically isolated segments.

### Repeater Requirements
- Galvanic isolation between all connected segments (mandatory)
- Transparent forwarding of all valid CAN FD frames (no modification of identifier or data)
- Optional configurable filtering (to reduce unnecessary traffic between segments)
- Fault containment: a fault on one segment shall not propagate to others
- Optional power injection point (repeaters may supply or pass power)
- Low-power operation in sleep/deep-sleep states (per `04-power-management.md`)
- Must claim a valid source address (per `05-addressing.md`)

### Hop Limit
- Maximum **4 repeater hops** between any two endpoints on **one** Pelorus Core bus (Bus A or Bus B in a dual-bus domain counts as a **separate** bus for hop accounting).
- This guarantees deterministic latency and prevents excessive propagation delay.

---

## 3. Recommended Topologies

### Small Vessels (< 30 m)
- Single segment, linear bus with T-drop topology (standard).

### Large Vessels
**Recommended:** Star topology with a central gateway node
- Central gateway acts as the hub
- Multiple repeater nodes connect directly to the gateway
- Each repeater creates one isolated segment
- All segments are bridged through the central gateway

This pattern:
- Minimizes hops (maximum 2 per path)
- Simplifies instance binding and power management
- Provides a natural location for the binding table authority and web UI
- Allows the gateway to act as the primary network management node

Alternative linear or tree topologies are permitted but not recommended for vessels requiring more than two segments.

---

## 4. Interaction with Other Pelorus Components

- **Addressing:** Repeaters and devices on all segments use the same 8-bit source address space and J1939-style address claiming (`05-addressing.md`).
- **Power Management:** Repeaters participate in selective wake-up using the same PNC and NM mechanisms (`04-power-management.md`).
- **Signal Catalog & Binding:** The binding table is **network-wide** in authority — all segments (and both buses in a dual-bus domain) **shall** use a consistent binding view for a given vessel revision. **v1.0:** binding-table **distribution** is **out of band** per **[06-signal-catalog.md](./06-signal-catalog.md)** §3–4 and **[07-dcid-registry.md](./07-dcid-registry.md)** §4 — repeaters **shall not** be assumed to “publish” binding on CAN; on-bus binding sync remains **future** reserved NM/WUF bytes.
- **DCIDs:** All Pelorus DCIDs (including WUF 0x0FF80 and NM 0x0FF81) are forwarded transparently by repeaters (`07-dcid-registry.md`).
- **Gateways:** The central gateway may also bridge to LMDE networks; its behavior is defined in [09-gateway-specification.md](./09-gateway-specification.md).

---

## 5. Critical zones and dual-bus domains (path redundancy)

**Path redundancy** (two parallel CAN FD media — **Bus A** and **Bus B**) is specified in **[17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md)** and **[03-data-link-layer.md](./03-data-link-layer.md)** §6. **Repeaters** (§2 above) address **electrical segment length** and **fault containment**; they **do not** replace a second backbone for **path** diversity.

### 5.1 Topology pattern

- A **dual-bus domain** contains **two** complete Pelorus Core fieldbuses (A and B) between the same logical endpoints (active-active replication, receiver duplicate discard).
- **Class D** nodes attach to **both** buses; **Class S** nodes attach to one bus unless served by a **Class H** hub (**[10-repeater-specification.md](./10-repeater-specification.md)**).
- A vessel **may** mix **dual-bus domains** (e.g. helm / autopilot region for **C0**/**C1**) with **single-bus** segments elsewhere (**C2** only) per **17**.

### 5.2 Coexistence with repeaters

- Each of Bus A and Bus B **may** be extended with repeaters independently; **hop limits** apply **per bus**, not summed across A+B.
- A **Class H** device at the boundary of a dual-bus domain **shall** present fault containment between downstream **Class S** segments and **both** backbones per **10**.

### 5.3 Fault and health visibility

- **Fault detection** on each bus uses **DCID 0x0FF82** (Bus Health) per **[07-dcid-registry.md](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)**; repeaters and gateways **should** forward Bus Health unless filtered by explicit policy (documented in the critical zone map).

---

## 6. Open Items (to be resolved before v1.0 promotion)

- Exact repeater filtering rules and configuration interface
- Repeater address claiming and **hub (Class H)** failover behavior when one backbone bus is lost
- Conformance test procedures for multi-segment **and** dual-bus networks
- Maximum practical number of segments in a star topology **per bus**

---

*This document, together with documents 01–07, **17**, and **10**, completes the architectural specification for Pelorus Core scaling and path redundancy.*