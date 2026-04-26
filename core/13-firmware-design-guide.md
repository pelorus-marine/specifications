# Pelorus Core — Firmware Design Guide

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document provides **non-normative** guidance for Pelorus Core firmware: architecture, state machines, diagnostics, and testing practice. **Normative** behavior for power, addressing, and binding remains in [04-power-management.md](./04-power-management.md), [05-addressing.md](./05-addressing.md), and [06-signal-catalog.md](./06-signal-catalog.md). Language and safety rules are summarized in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary); firmware-specific patterns are developed here.

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
  - Selective wake-up handling via WUF (PGN 0x0FF80)
  - PNC mask processing and NM (PGN 0x0FF81) transmission

- **Address Claiming State Machine** (`05-addressing.md`)
  - Listen → Claim → Defend → Cannot Claim

- **Binding Table Cache** (`06-signal-catalog.md`)
  - Receive, validate, and cache the latest binding table
  - Fallback to raw PGN + instance mode when cache is invalid or absent

- **Repeater Forwarding** (`10-repeater-specification.md`)
  - Transparent CAN FD frame forwarding with fault isolation

---

## 3. CAN FD Driver Requirements

- Use the MCU’s native CAN FD peripheral (or a validated external controller).
- Support 250 kbit/s arbitration phase and 500 kbit/s data phase.
- Hardware filtering where possible to reduce CPU load.
- Error counters and bus-off recovery per ISO 11898-2:2016.
- Split termination and bus biasing handled in hardware (per `02-physical-layer.md`).

---

## 4. Conformance and Testing

- All firmware must pass the official conformance test suite (see `15-conformance-test-plan.md`).
- Unit tests for every state machine transition.
- Integration tests using recorded bus traces.
- Hardware-in-the-loop testing on a real Pelorus network.
- Long-term stability testing (weeks of continuous operation with power cycling and fault injection).

---

## 5. Interaction with Other Pelorus Components

- **Physical Layer:** Firmware must respect all electrical and timing limits from `02-physical-layer.md`.
- **PGN Registry:** Use the exact field layouts and PGN numbers from `07-pgn-registry.md`.
- **Network Architecture:** Repeaters and gateways must respect hop limits and star topology rules from `08-network-architecture.md`.
- **Gateway UI:** Gateways expose the binding table via the web UI defined in `09-gateway-specification.md`.
- **Hardware:** Firmware must work with the schematic patterns and isolation rules in `12-hardware-design-guide.md`.

---

## 6. Open Items (to be resolved before v1.0 promotion)

- Reference firmware repository structure and example projects
- Detailed `clippy` and coding style rules
- Defmt / logging configuration for production devices
- Firmware update mechanism (bootloader specification)
- Minimum test coverage requirements

---

*This document, together with documents 01–12, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*