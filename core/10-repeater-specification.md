# Pelorus Core — Repeater Specification

**Version:** 0.1 Draft  
**Last Updated:** May 4, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **normative** functional specification for Pelorus Core **repeater** nodes and **hub (Class H)** devices used for **path redundancy** (**[17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md)**). Segment limits and hop-count rules are specified in [08-network-architecture.md](./08-network-architecture.md); physical port requirements in [02-physical-layer.md](./02-physical-layer.md). Repeater policy in brief: [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary).

---

## 1. Repeater Requirements

Every repeater node must meet the following:

- **Galvanic isolation:** Full isolation between every pair of connected segments.
- **Transparent forwarding:** Every valid CAN FD frame received on one port shall be retransmitted on all other ports without modification to identifier, data, or priority — **except** where **§3** explicitly permits a **Class H** hub to originate paired copies onto Bus A and Bus B for downstream **Class S** traffic (same identifier and data on both buses).
- **Fault containment:** A short, open, or excessive error state on one segment shall not propagate to other segments.
- **Power management:** Full compliance with selective wake-up, PNC masking, and the four power states (Active / Standby / Sleep / Deep Sleep) per `04-power-management.md`.
- **Addressing:** Must successfully claim a unique source address per `05-addressing.md` before forwarding application data.
- **Low-power operation:** Standby/sleep current targets identical to other Pelorus devices.
- **Connector:** M12 A-coded 5-pin on all ports (per `02-physical-layer.md`).

---

## 2. Optional Features

The following features are permitted but not required for basic repeater compliance:

- Configurable filtering (to reduce unnecessary traffic between segments)
- Power injection / pass-through capability
- Diagnostic LEDs or status reporting
- Web-based or local configuration interface (via gateway)

---

## 3. Hub (Class H) — RedBox-equivalent behavior

A **hub** is a repeater-class device that provides **at least two backbone ports** (**Bus A**, **Bus B**) and **one or more downstream** Pelorus Core segment ports. It satisfies **§1** on every port pair.

### 3.1 Downstream Class S attachment

- A **Class S** device on a downstream segment has a **single** transceiver; the hub **shall** receive its frames and **retransmit identical** CAN FD frames (same 29-bit identifier, same data field) on **both** Bus A and Bus B unless one backbone is declared failed (then **one** bus only, with operator-visible degraded mode per **17** §3).
- The hub **shall not** change the **source address** of replicated downstream traffic — duplicate discard (**03** §6) uses the **originator’s** SA.

### 3.2 Hub-generated management traffic

- The hub **shall** transmit **DCID 0x0FF82** (Bus Health) on each backbone port it serves, per **[07 §1.3](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)**.
- The hub **may** implement **DCID 0x0FF83** (Time Sync) on one or both buses.

### 3.3 Fault containment

- A short or stuck-dominant fault on a **downstream** segment **shall not** propagate to **either** backbone bus beyond what ISO 11898 fault confinement already imposes on the hub’s ports.

### 3.4 Hub bidirectional duplicate discard

A hub sees the **same logical frame** on **both** Bus A and Bus B whenever an upstream **Class D** producer (or another hub) is replicating active-active. To avoid double-injection onto downstream segments, a hub **shall** apply duplicate discard on its **backbone ingress** path before forwarding to downstream ports:

- Maintain a **DDT** keyed per **[03 §6.4](./03-data-link-layer.md#64-duplicate-discard-algorithm)**, indexed across **both** Bus A and Bus B inputs.
- For a downstream forwarding decision, **deliver one copy** of each logical message to each downstream segment within `DISCARD_WINDOW`; the second copy received from the peer backbone **shall be discarded** for downstream forwarding (the duplicate counter on the hub’s **0x0FF82** for that ingress bus is incremented per **[07 §1.3](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)**).
- **Backbone-to-backbone** forwarding is **not** required (each backbone already carries its own copy from the producer); a hub **shall not** re-inject a Bus A frame onto Bus B or vice versa for **upstream** replication unless the originator is a **Class S** device on a downstream port (in which case **§3.1** applies).

This rule applies to all DCIDs **subject to** duplicate discard (**§6.2** of **03**); exempt DCIDs (address claim, TP, WUF, NM) are forwarded independently per port as today.

### 3.5 Hub bus-off and degraded backbone

When one backbone port enters **bus-off** or sustained error-passive, the hub **shall**:

- Continue forwarding between the **surviving backbone** and downstream segments so that downstream **Class S** devices remain reachable; this is a **degraded single-bus** path and **shall** be reflected in **0x0FF82** Bus Health on the surviving bus with `Bus state = Degraded-Single` (**[07 §1.3](./07-dcid-registry.md#13-bus-health-dcid-0x0ff82)**).
- For frames sourced **from** a downstream **Class S** device that the hub would normally replicate to **both** backbones: continue replicating onto the **surviving** backbone; the missed-frame counter for the **failed** backbone in **0x0FF82** **shall** be incremented for each frame the hub could not forward there.
- **Not** buffer cross-forwarded frames longer than `DISCARD_WINDOW` while waiting for the failed backbone to recover; queued frames older than `DISCARD_WINDOW` **shall** be dropped (counter in **0x0FF82** for the failed bus).
- Resume normal active-active replication on **bus return** without manual intervention, consistent with **[03 §6.6](./03-data-link-layer.md#66-interaction-with-power-management-and-bus-return)**. Duplicate discard handles transient duplicates during recovery.

---

## 4. Topology and Hop Rules

- Repeaters shall be used to create isolated segments when a vessel exceeds 30 m or 50 nodes.
- In the recommended star topology, repeaters connect directly to a central gateway.
- Linear or tree repeater chains are allowed but must respect the **maximum 4-hop rule** between any two endpoints **on a given bus** (Bus A or Bus B).
- Repeaters count as one hop each.

---

## 5. Interaction with Other Pelorus Components

- **Physical Layer:** All ports comply with `02-physical-layer.md` (250/500 kbit/s, split termination, M12 A-coded); **Class H** backbone ports per **02** §13.
- **Addressing:** Repeaters and hubs claim addresses and participate in conflict resolution (`05-addressing.md`); **§7** applies to hubs on dual backbones.
- **Power Management:** Repeaters forward WUF (0x0FF80) and NM (0x0FF81) messages and respect PNC masks (`04-power-management.md` and `07-dcid-registry.md`).
- **Signal Catalog & Binding:** Binding authority is network-wide; **v1.0** distribution is **out of band** per **06** / **07** — repeaters and hubs **shall not** assume an on-bus binding-table DCID.
- **DCID Registry:** All Pelorus DCIDs are forwarded without alteration except **Class H** replication semantics in **§3** (`07-dcid-registry.md`).
- **Network Architecture:** Repeaters and hubs enable the multi-segment, star, and dual-bus topologies in `08-network-architecture.md`.
- **Gateway:** Repeaters and hubs work with the gateway per `09-gateway-specification.md`.

---

## 6. Open Items (to be resolved before v1.0 promotion)

- Exact filtering configuration format and rules
- Hub port count limits and **failover** timing when one backbone bus is removed
- Conformance test plan for isolation, fault containment, hop-limit behavior, and **Class H** replication
- Minimum and maximum number of ports per repeater or hub device

---

*This document, together with documents 01–09, **17**, and **08**, specifies Pelorus Core repeaters and path-redundancy hubs.*