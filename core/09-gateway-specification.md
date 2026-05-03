# Pelorus Core — Gateway Specification

**Version:** 0.1 Draft  
**Last Updated:** May 2, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **normative** functional specification for Pelorus Core gateway nodes: bridging, binding-table authority, and role in recommended topologies. A one-paragraph summary of gateway policy is in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary). Non-gateway-specific binding and instance rules remain in [06-signal-catalog.md](./06-signal-catalog.md) and [ARCHITECTURE.md](../ARCHITECTURE.md).

**Physical boundary:** A gateway connects at least one **Pelorus Core** segment (**CAN FD** per **02**/**03**) and at least one **LMDE** segment (**Classical CAN / CAN 2.0** per industry practice). Bridging **always** spans that physical and framing difference; **do not** attach classical-only J1939 devices to a Pelorus Core backbone as “just another node.”

---

## 1. Gateway Roles

A Pelorus gateway node performs three primary roles:

1. **Bridge** — Translates **Classical CAN (LMDE) ⟷ CAN FD (Pelorus)** frame formats and forwards mapped messages between networks.
2. **Binding Authority** — Provides the convenient UI for editing and publishing the vessel-specific binding table.
3. **Network Management Hub** — Acts as the recommended central point in star topologies and assists with power-management coordination.

A vessel may have zero, one, or multiple gateways. Multiple gateways are supported with authority priority rules.

---

## 2. Bridging Requirements

- Terminate **different physical layers** correctly on each side: **CAN FD** (Pelorus) vs **Classical CAN** (LMDE). Translate between **frame formats** and multi-frame rules (e.g. LMDE Fast Packet / classical constraints vs Pelorus CAN FD and J1939 TP per **03**) wherever a message is mapped.

- Forward all valid Pelorus DCIDs (including WUF 0x0FF80 and NM 0x0FF81) transparently.
- Map selected compatibility DCIDs from the Legacy Marine Data Ecosystem to the corresponding `Vessel.*` paths in the signal catalog (`06-signal-catalog.md` and `07-dcid-registry.md`).
- Perform instance mapping using the current binding table.
- Support both directions: legacy → Pelorus and Pelorus → legacy.
- Preserve priority and timing where possible.
- Do not introduce Fast Packet or other legacy-specific transport on the Pelorus side.

---

## 3. Binding Table Management

The gateway shall:

- Maintain the authoritative copy of the binding table in non-volatile memory.
- Distribute binding-table updates **out of band** for v1.0 (configuration export/import, diagnostic session, **[Pelorus Stream](../stream/01-overview.md)**, local UI/API — see **[07-dcid-registry.md](./07-dcid-registry.md)** §4 and **[06-signal-catalog.md](./06-signal-catalog.md)** §3–4). **Do not** rely on a Pelorus Core CAN DCID for binding publication until one is registered in **07** for a future revision.
- Provide a web-based provisioning UI for sailors to assign friendly labels and map devices.
- Detect and report instance drift or conflicts.
- Allow secondary gateways or diagnostic tools to take over as binding authority if the primary is absent.

See **[06-signal-catalog.md](./06-signal-catalog.md)** §3–4 for the full fault-tolerant binding model.

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
- Secondary gateways (if present) coordinate binding authority per **[06](./06-signal-catalog.md)** / multi-gateway rules (**Open Items** below until deterministic hand-off is specified).
- No hard dependency on any single physical node.

---

## 7. Open Items (to be resolved before v1.0 promotion)

- Optional **future** on-bus binding-table DCID or NM/WUF payload layout (v1.0 is **out of band** per **07** §4); delta encoding on Stream or diagnostic channel
- Web UI wireframes and minimum feature set
- Bridging conformance test plan for LMDE compatibility DCIDs
- Multi-gateway conflict resolution and hand-off rules
- Optional diagnostic logging interface

---

## 8. Core ↔ Stream coupling (gateway tiers)

Pelorus Stream (Ethernet) is **non-safety-critical** and orthogonal to Core **CAN FD**, but vessels integrate the two. **`ARCHITECTURE.md`** §3 and **`stream/01-overview.md`** §3.1 state the same coupling rules:

- **Standard gateway** — **Required** capability to expose Core toward Stream: bridge Core-sourced telemetry, identity, and catalog-aligned metadata onto the Stream substrate (Core→Stream). This is the normal productized path.

- **Capable bidirectional gateway** — **Includes** standard-gateway behavior **and** an explicitly designed, enumerated, and conformance-tested **Stream→Core** injection path. Ordinary Stream publishers **must not** originate frames on the Core fieldbus; reverse bridging **only** traverses this tier.

Wire formats and APIs on the Stream side of these bridges are specified under **`stream/`**; Core-side semantics remain DCID- and catalog-bound (**06**/**07**).

---

*This document — including **§8** (Core ↔ Stream coupling) — together with Core documents **01–08**, completes the minimum viable specification for Pelorus Core reference implementations and hardware prototyping.*