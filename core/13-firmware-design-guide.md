# Pelorus Core — Firmware Design Guide

**Version:** 0.1 Draft  
**Last Updated:** May 3, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document provides **non-normative** guidance for Pelorus Core firmware: architecture, state machines, diagnostics, and testing practice. **Normative** behavior for power, addressing, binding, path redundancy, and duplicate discard remains in [04-power-management.md](./04-power-management.md), [05-addressing.md](./05-addressing.md), [06-signal-catalog.md](./06-signal-catalog.md), [03-data-link-layer.md](./03-data-link-layer.md) §6, and [17-criticality-and-redundant-paths.md](./17-criticality-and-redundant-paths.md). Language and safety rules are summarized in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary); firmware-specific patterns are developed here.

---

## 1. Firmware Architecture Principles

- **Layered design:** Separate HAL, CAN driver, protocol stack, application logic, and power management.
- **Static allocation:** No heap allocation in real-time paths. Use static buffers and fixed-size structures.
- **State-machine first:** Every major behavior (power state, address claiming, binding cache) is implemented as a finite state machine.
- **Minimal dependencies:** Only crates that are `no_std` compatible and have been audited for safety.
- **Error handling:** `Result<T, PelorusError>` everywhere. No panics in production firmware.
- **Logging:** Lightweight, compile-time configurable logging (defmt or similar) for diagnostics.

---

## 2. Required State Machines

Firmware must implement the following state machines exactly as defined in the specifications:

- **Power Management State Machine** (`04-power-management.md`)
  - Four states: Active, Standby, Sleep, Deep Sleep
  - Selective wake-up handling via WUF (DCID 0x0FF80)
  - PNC mask processing and NM (DCID 0x0FF81) transmission

- **Address Claiming State Machine** (`05-addressing.md`)
  - Listen → Claim → Defend → Cannot Claim

- **Binding Table Cache** (`06-signal-catalog.md`)
  - Receive, validate, and cache the latest binding table
  - Fallback to raw DCID + instance mode when cache is invalid or absent

- **Repeater Forwarding** (`10-repeater-specification.md`)
  - Transparent CAN FD frame forwarding with fault isolation

- **Dual-bus receive path** (`03` §6, `17`, `07` §1.3) — **Class D** / **Class H** products
  - Duplicate Discard Table (DDT) keyed per **03**; single RX pipeline into application layer
  - Bus Health (**0x0FF82**) transmission and local error-counter sampling
  - Degraded single-bus annunciation when peer bus silent (**17** §3)

---

## 3. Dual-bus receive path (implementation patterns)

- **Peripheral layout:** Prefer two independent CAN controllers (or controller + validated secondary) with separate transceivers; align RX timestamps to a common monotonic clock for `DISCARD_WINDOW` (**03** §6.4.1).
- **DDT storage:** Fixed-size table indexed by SA (and DCID for PRH-capable messages); evict entries on **wake generation** change (**04** §13) or **NODE_FORGET_TIME** timeout.
- **Logging:** Rate-limit duplicate-discard and sequence-gap logs; surface saturating counters in Bus Health payload.

---

## 4. CAN FD Driver Requirements

- Use the MCU’s native CAN FD peripheral (or a validated external controller).
- Support 250 kbit/s arbitration phase and 500 kbit/s data phase.
- Hardware filtering where possible to reduce CPU load.
- Error counters and bus-off recovery per ISO 11898-2:2016.
- Split termination and bus biasing handled in hardware (per `02-physical-layer.md`).

---

## 5. Conformance and Testing

- All firmware must pass the official conformance test suite (see `15-conformance-test-plan.md`).
- Unit tests for every state machine transition.
- Integration tests using recorded bus traces.
- Hardware-in-the-loop testing on a real Pelorus network.
- Long-term stability testing (weeks of continuous operation with power cycling and fault injection).

---

## 6. Interaction with Other Pelorus Components

- **Physical Layer:** Firmware must respect all electrical and timing limits from `02-physical-layer.md`.
- **DCID Registry:** Use the exact field layouts and DCID numbers from `07-dcid-registry.md`.
- **Network Architecture:** Repeaters, hubs, and gateways must respect hop limits, star topology, and dual-bus domain rules from `08-network-architecture.md` and `17-criticality-and-redundant-paths.md`.
- **Gateway UI:** Gateways expose the binding table via the web UI defined in `09-gateway-specification.md`.
- **Hardware:** Firmware must work with the schematic patterns and isolation rules in `12-hardware-design-guide.md`.

---

## 7. Open Items (to be resolved before v1.0 promotion)

- Reference firmware repository structure and example projects
- Detailed `clippy` and coding style rules
- Defmt / logging configuration for production devices
- Firmware update mechanism (bootloader specification)
- Minimum test coverage requirements

---

*This document, together with documents 01–12, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*