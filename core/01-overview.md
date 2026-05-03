# Pelorus Core — Overview

**Version:** 0.1 Draft  
**Last Updated:** May 2, 2026  
**Status:** Pre-specification  
**Trust:** Trusted

---

## About This Document

This is the entry point to the Pelorus specification. It states what Pelorus is, what it replaces, how the protocol stack is divided, and which documents to read next. **Normative** requirements live in downstream documents (02 onward). [§9](#9-cross-cutting-decisions-authoritative-summary) collects **cross-cutting decisions** in one place so other documents can cross-reference instead of repeating them; for bit-level and testable requirements, always use the numbered specification for that topic.

Read this first. Then go to [02-physical-layer.md](./02-physical-layer.md) for hardware-level requirements or [04-power-management.md](./04-power-management.md) for selective wake-up and power state behavior.

---

## Terminology: Legacy Marine Data Ecosystem (LMDE)

**Legacy Marine Data Ecosystem** is a **project code name** for the incumbent, certification-gated marine instrumentation fieldbus and its physical plant (connectors, cable families, and de facto in-use DCID set) that dominates new recreational-vessel installs. This specification **does not** use third-party trademarks for that ecosystem; **LMDE** is the abbreviated form in technical text after the first mention in a document.

**Physical layer (read this with “J1939”):** In practice, LMDE stacks use **J1939-family** semantics on **Classical CAN (CAN 2.0)** — extended 29-bit IDs, **8-byte payloads**, a single 250 kbit/s bit rate on the wire, and no CAN FD framing **for application data**. **Pelorus Core** uses **CAN FD** (ISO 11898-1:2015) with the arbitration / data-phase profile specified in **02**/**03**. Pelorus deliberately adopts **J1939-derived** identifier layout, DCID numbering, transport, and addressing rules **on the CAN FD bus**; that alignment is **not** the same as running arbitrary classical-only J1939 nodes on the same electrical segment as Pelorus (see §4).

---

## 1. What Pelorus Is

Pelorus is an open marine data network standard. It is CAN FD-based for safety-critical instrumentation and Ethernet-based for high-bandwidth media. It is designed to install on existing LMDE cabling and connectors, to operate cleanly without an internet connection, and to minimize aggregate network current whenever large parts of the suite are legitimately unused — not only when lying at anchor, but whenever voyage context makes that gear irrelevant (e.g. mid-ocean passage with no need for a depth sounder until landfall).

Pelorus is a specification, not a product. Reference implementations exist in Rust. Commercial products that comply with the specification can use the "Pelorus Core Compatible" branding without a license fee, an NDA, or third-party certification.

The specification, the reference implementations, the test fixtures, and the documentation are all released under permissive open licenses. Documents are CC BY 4.0; code is MIT or Apache 2.0.

---

## 2. The Problem

The Legacy Marine Data Ecosystem is technically sound at its core but trapped by twenty years of backward compatibility and closed-ecosystem incentives. The specific weaknesses Pelorus addresses:

- **Closed protocol** — proprietary DCIDs, paid certification, NDA requirements to obtain the specification
- **Always-on power** — no selective device sleep; the suite draws 2–3 A continuously even when much of it is unused for hours or days (overnight at the dock, long passages with irrelevant sensors still powered), consuming 24–36 Ah overnight on a typical vessel
- **Single-segment topology** — backbone failure takes down the entire network
- **Aging physical layer** — fixed at classical CAN at 250 kbit/s; cannot migrate without breaking installed equipment
- **No diagnostic transparency** — sailors cannot debug their own networks
- **Vendor lock-in tactics** — proprietary extensions break interoperability between brands

Bandwidth is **not** the headline problem. The Legacy Marine Data Ecosystem at 250 kbit/s is adequate for GPS, depth, wind, heading, AIS, and engine data. Pelorus does not exist to make instrument data faster. It exists to make it open, reliable, power-aware, and debuggable. Higher bandwidth is delivered by a separate Ethernet layer for the use cases that need it.

---

## 3. Two-Layer Architecture

Pelorus has two distinct physical layers serving different traffic classes. They are bridged by gateway nodes but operate as independent physical buses.

### 3.1 Pelorus Core (CAN FD)

Safety-critical instrumentation backbone. Real-time, deterministic, reliable. Replaces LMDE functionality on a vessel.

- CAN FD at 250 kbit/s arbitration / 500 kbit/s data phase
- 64-byte frames eliminate Fast Packet for current LMDE message types
- ISO 11898-2:2016 partial networking with selective wake-up
- M12 A-coded 5-pin connectors, LMDE micro cable, LMDE-style installation practice
- Linear bus per segment, repeater nodes for vessels exceeding 30 m
- Segmented architecture for fault containment

### 3.2 Pelorus Stream (Ethernet)

High-bandwidth, non-safety-critical layer. Radar, sonar, video, cloud connectivity.

- M12 D-coded 4-pin (100 Mbit/s) recommended, X-coded reserved for future Gigabit profiles
- Connector compatibility with established industrial M12 Ethernet cabling (no protocol-level interoperability with any incumbent proprietary marine Ethernet protocols)
- Protocol stack, PoE strategy, and switch architecture are largely undecided as of v0.1

This document and the rest of the v0.1 specification focus on Pelorus Core. Pelorus Stream design is deferred until Core is stable and validated on real hardware.

---

## 4. Coexistence with the Legacy Marine Data Ecosystem

Pelorus Core uses identical connectors and cable to LMDE micro. The two networks are **not** electrically interoperable on the same wire: LMDE application traffic is **Classical CAN**; Pelorus application traffic is **CAN FD**. Classical nodes do not correctly receive CAN FD data frames; bit rates and framing differ. Cross-connecting cables between the two networks results in a non-functional bus but does not damage equipment.

A vessel typically runs both networks during transition: the Legacy Marine Data Ecosystem for legacy devices, Pelorus Core for new equipment, with a gateway node bridging selected messages between them. The gateway performs **frame-format translation** (Classical CAN ⟷ CAN FD) where required, preserves **DCID / payload semantics** for mapped messages, handles **instance binding**, and adapts rates. See [09-gateway-specification.md](./09-gateway-specification.md).

Visual differentiation (Pelorus marine blue cable jackets, port labeling, distinctive terminator caps) is recommended but not mandatory.

---

## 5. v1.0 Scope

The v1.0 specification covers Pelorus Core only. The minimum viable specification consists of:

| # | Document | Purpose |
|---|---|---|
| 01 | [01-overview.md](./01-overview.md) | This document |
| 02 | [02-physical-layer.md](./02-physical-layer.md) | Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation |
| 03 | [03-data-link-layer.md](./03-data-link-layer.md) | CAN FD frame format usage, message addressing, error handling |
| 04 | [04-power-management.md](./04-power-management.md) | Selective wake-up, partial network clusters, power states, network management |
| 05 | [05-addressing.md](./05-addressing.md) | Source address claiming, conflict resolution, device identification |
| 06 | [06-signal-catalog.md](./06-signal-catalog.md) | VSS-syntax catalog format, `Vessel.*` data model, instance handling |
| 07 | [07-dcid-registry.md](./07-dcid-registry.md) | Specific DCID assignments and definitions |

Tier 2 (network architecture, gateway, repeater) and Tier 3 (implementation guidance) documents extend the core but are not required for an interoperable v1.0 device. See [00-document-index.md](./00-document-index.md) for the full document list.

### Explicitly Deferred From v1.0

The following were considered and held for later versions.

- Higher data phase rates (1 Mbit/s, 2 Mbit/s) — held for v2.0+
- Auto-negotiation of bit rates — held indefinitely; static profile is correct for v1.0
- Fast Packet support — not adopted; 64-byte CAN FD frames cover existing LMDE DCIDs
- Mandated universal galvanic isolation — replaced by tiered requirement (mandatory for high-power, optional for low-power sensors)
- Signal K as core component — treated as one possible app-level consumer, not part of the core stack
- Pelorus Stream protocol stack — design deferred until Core is validated

---

## 6. Design Principles

These guide every concrete decision in downstream documents.

- **Sailor-first.** Every design decision asks "what is best for the sailor at sea" before "what is best for the manufacturer."
- **Reliability over features.** A device that works for ten years beats a device with twenty features that fails after three.
- **Power awareness as architecture.** Boats are not on unlimited shore power. Power management matches **operational context** — which devices matter for this leg, this watch, this weather — not a single "anchor vs underway" caricature.
- **Open all the way down.** Specification, reference implementations, test fixtures, documentation. No purchases required to participate.
- **Static and debuggable for v1.0.** Auto-negotiation, dynamic reconfiguration, and complex state machines are deferred. Static profiles, fixed bit rates, and simple state machines win.
- **Honest about tradeoffs.** Patent encumbrances, unresolved questions, and design limitations are documented openly in each specification document's Open Items section.

---

## 7. Status and Stability

The v0.1 specification is pre-release. **Normative** requirements are summarized in [§9](#9-cross-cutting-decisions-authoritative-summary) below and stated in **02–04**; [ARCHITECTURE.md](../ARCHITECTURE.md) is non-normative background (problem framing and subsystem overview). Document text is still under revision and field validation has not begun. Hardware prototypes do not yet exist.

Compatibility commitment for v1.0:

- Bit rate profile (250 kbit/s arbitration / 500 kbit/s data) is permanent for the v1.0 line
- Connector type and pinout (M12 A-coded, LMDE micro) are permanent
- Frame format (CAN FD per ISO 11898-1:2015, no Fast Packet) is permanent
- Power state model and selective wake-up behavior may refine before v1.0 is finalized
- DCID assignments and the signal catalog are open and will change before v1.0

Implementations targeting v0.x should expect to update before v1.0 ships.

---

## 8. Where to Go Next

| If you want to... | Read |
|---|---|
| Understand the hardware-level requirements | [02-physical-layer.md](./02-physical-layer.md) |
| Implement selective wake-up and power management | [04-power-management.md](./04-power-management.md) |
| See what is decided and why | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| Track open work | [GitHub Issues](https://github.com/pelorus-marine/specifications/issues) |
| Track specification document status | [00-document-index.md](./00-document-index.md) |

---

## 9. Cross-cutting decisions (authoritative summary)

Downstream documents (02–16) state **normative requirements** and rationale. This section collects cross-cutting decisions in one narrative so other documents can reference a single summary. If text elsewhere repeats this material for context, treat this section as the summary; do not maintain duplicate prose.

- **Physical profile (02–04):** Pelorus Core is CAN FD on LMDE-style cabling and M12 A-coded 5-pin connectors; arbitration 250 kbit/s, data phase 500 kbit/s; linear bus per segment with split termination and 9–32 V DC supply. LMDE segments are **Classical CAN (CAN 2.0)**. Pelorus Core and LMDE are **not** electrically interoperable on the same segment; they coexist on a vessel via gateways ([§4](#4-coexistence-with-the-legacy-marine-data-ecosystem)).

- **Data link (03):** **On Pelorus:** J1939-style 29-bit identifiers and DCID encoding in **CAN FD** frames; 64-byte payloads; **no** Fast Packet; multi-frame payloads use J1939 Transport Protocol; application data is push or request-via-DCID — **no** Remote Transmission Request frames. **On LMDE:** same family of IDs and semantics in **Classical CAN** frames (8-byte payloads, Fast Packet where used).

- **Power management (04):** Partial networking and selective wake-up per ISO 11898-2:2016 (with the patent considerations documented there). Functional groups / PNC-style behavior let the network shed nodes that are **not needed for the current voyage context** (anchoring, coastal, offshore, storm, etc.), not only for "at anchor" scenarios. Detail is in 04.

- **Addressing (05):** Source addressing, address claiming, and the NAME field follow **SAE J1939-81 / ISO 11783-5** with **no** Pelorus-specific deviations in v1.0. **Pelorus** nodes run these procedures on **CAN FD**; **LMDE** nodes run the same procedures on **Classical CAN** — one segment, one frame format; gateways join segments.

- **Signal catalog (06):** Canonical semantics use **COVESA VSS** syntax under a **`Vessel.*`** root; no custom catalog syntax in v1.0.

- **DCID registry (07):** Pelorus-specific DCIDs use the high range **`0x0FF80`–`0x0FFFF`** (exact assignments in **07** §1). Compatibility DCIDs and addressing-related registrations (**NAME**, Commanded Address **0xFED8**) live in **07** §2; **SAE J1939 Digital Annex** remains normative for cited legacy payload bit layouts unless **07** explicitly overrides.

- **Network architecture (08):** Per segment: max **30 m** backbone, max **50** nodes, max **6 m** stubs; multi-segment networks use **repeaters** with **galvanic isolation** between segments; max **4** repeater hops between any two endpoints; **star topology with a central gateway** is the recommended pattern for large vessels.

- **Gateway (09):** The gateway is a **convenient, not mandatory** authority (no single point of failure for the network). It bridges **CAN FD (Pelorus Core)** and **Classical CAN (LMDE)** segments, coordinates binding-table provisioning (**out-of-band** distribution for v1.0 per **07** §4 / **06** §3–4), and may offer a web UI — **operation without** that UI must remain possible.

- **Repeater (10):** Repeaters **shall** isolate segments electrically, **transparently** forward valid CAN FD frames, and **fully** participate in power management and addressing.

- **Reference implementations (11):** Official reference code is **Rust**, **`no_std`** where practical, and **`forbid(unsafe_code)`**.

- **Hardware (12):** Designs **shall** be repairable and field-serviceable; **conformal coating** is mandatory; **galvanic isolation** follows the tiers and thresholds in 02 and 12. Early hardware acceptance for the project includes **liveaboard validation** as described in 12.

- **Firmware (13):** Same language and safety rules as 11; power, addressing, and binding behavior **shall** match 04–06.

- **Installation (14):** Installation **shall** comply with 02; **no** deviations for v1.0.

- **Conformance (15–16):** Conformance is established by **self-testing** against reference implementations; **no** third-party certification body is required for v1.0 (see 16).

---

## 10. License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
