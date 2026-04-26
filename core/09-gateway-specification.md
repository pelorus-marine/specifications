# Pelorus Core — Gateway Specification

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **normative** functional specification for Pelorus Core gateway nodes: bridging, binding-table authority, and role in recommended topologies. A one-paragraph summary of locked gateway policy is in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary). Non-gateway-specific binding and instance rules remain in [06-signal-catalog.md](./06-signal-catalog.md) and [ARCHITECTURE.md](../ARCHITECTURE.md).

---

## 1. Gateway Roles

A Pelorus gateway node performs three primary roles:

1. **Bridge** — Translates and forwards messages between Pelorus Core and LMDE networks.
2. **Binding Authority** — Provides the convenient UI for editing and publishing the vessel-specific binding table.
3. **Network Management Hub** — Acts as the recommended central point in star topologies and assists with power-management coordination.

A vessel may have zero, one, or multiple gateways. Multiple gateways are supported with authority priority rules.

---

## 2. Bridging Requirements

- Forward all valid Pelorus PGNs (including WUF 0x0FF80 and NM 0x0FF81) transparently.
- Map selected compatibility PGNs from the Legacy Marine Data Ecosystem to the corresponding `Vessel.*` paths in the signal catalog (`06-signal-catalog.md` and `07-pgn-registry.md`).
- Perform instance mapping using the current binding table.
- Support both directions: legacy → Pelorus and Pelorus → legacy.
- Preserve priority and timing where possible.
- Do not introduce Fast Packet or other legacy-specific transport on the Pelorus side.

---

## 3. Binding Table Management

The gateway shall:

- Maintain the authoritative copy of the binding table in non-volatile memory.
- Publish the binding table (or delta) via a Pelorus PGN (defined in `07-pgn-registry.md`) on every connected segment.
- Provide a web-based provisioning UI for sailors to assign friendly labels and map devices.
- Detect and report instance drift or conflicts.
- Allow secondary gateways or diagnostic tools to take over as binding authority if the primary is absent.

See `06-signal-catalog.md` §3–4 for the full fault-tolerant binding model.

---

## 4. Power Management Role

- Participate fully in the Pelorus power-management state machine (`04-power-management.md`).
- Act as the convenient source of the current marine functional group profile.
- Re-publish the last-known profile on boot or when requested.
- Support manual override via the web UI.

The gateway is **not** required for power management to function — nodes fall back to their NV-stored last-known profile.

---

## 5. User Interface and Provisioning

The gateway shall expose:

- A simple web UI (accessible over Wi-Fi or Ethernet when available).
- Pages for:
  - Viewing current binding table
  - Assigning friendly labels to devices
  - Editing marine functional group profiles
  - Network diagnostics (node list, power states, bus statistics)
- Secure local access (no cloud dependency required).

---

## 6. Fault Tolerance and Redundancy

- The network continues to operate fully if the gateway is powered off, failed, or disconnected.
- All nodes that need semantics cache the latest binding table and power profile.
- Secondary gateways (if present) automatically assume authority based on the published Authority Priority field in NM messages.
- No hard dependency on any single physical node.

---

## 7. Open Items (to be resolved before v1.0 promotion)

- Exact PGN format for binding table publication and delta updates
- Web UI wireframes and minimum feature set
- Bridging conformance test plan for LMDE compatibility PGNs
- Multi-gateway conflict resolution and hand-off rules
- Optional diagnostic logging interface

---

*This document, together with documents 01–08, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*