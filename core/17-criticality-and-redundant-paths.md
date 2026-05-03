# Pelorus Core — Criticality, Path Redundancy, and Dual-Bus Domains

**Version:** 0.1 Draft  
**Last Updated:** May 3, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document is the **normative** single source of truth for:

- **Criticality classes** (C0 / C1 / C2) and what each **shall** require on the wire and in installation practice.
- **Path-redundant CAN FD** — two independent Pelorus Core buses (**Bus A** and **Bus B**) within a **dual-bus domain**, active-active replication, and receiver **duplicate discard** (specified in **[03-data-link-layer.md](./03-data-link-layer.md)**).
- **Node / port classes** (**Class S**, **Class D**, **Class H**) referenced from **[02-physical-layer.md](./02-physical-layer.md)** and **[10-repeater-specification.md](./10-repeater-specification.md)**.
- **Principle ordering:** **Reliability and durability** take precedence over **ease of installation and configuration** when the two conflict. Installers and manufacturers **shall not** omit path redundancy, separate routing, or declared degraded-mode behavior for C0 or C1 solely to reduce install time or cable count unless this document explicitly allows a single-bus exception.

Mechanisms (payload rules, duplicate discard algorithm, exemptions, DCID layouts) are normative in **03**, **05**, and **07**. Physical independence, segment limits per bus, and connector rules are normative in **02**. Topology, repeaters, and how dual-bus domains meet single-bus segments are normative in **08**. Conformance tests live in **[15-conformance-test-plan.md](./15-conformance-test-plan.md)**.

**RFC alignment:** [GitHub Issue #6 — Dual Bus Redundancy, Duplicate Discard, and Related Reliability Improvements](https://github.com/pelorus-marine/specifications/issues/6) is the design RFC; normative text in **02–08**, **15–17** supersedes informal issue wording where they differ.

---

## 1. Definitions

| Term | Definition |
|------|------------|
| **Path redundancy** | Two **electrically independent** Pelorus Core CAN FD media (**Bus A**, **Bus B**) carrying the **same logical** application traffic (active-active), with receivers accepting one copy per logical frame per **[03](./03-data-link-layer.md)**. |
| **Dual-bus domain** | A bounded installation region (functional zone, compartment group, or entire small vessel) where **17** requires both Bus A and Bus B to be present and terminated per **02**. |
| **Critical zone map** | A written or machine-readable record, prepared at commissioning or product certification, listing each Pelorus-attached function and its **criticality class** (§2) and **node class** (§4). |
| **Segmentation** (repeaters) | Electrical isolation between **length-scaled** segments per **08** / **10** — **orthogonal** to path redundancy: repeaters address **30 m / hop** limits and **fault containment**; they do not replace a second parallel bus for **path** diversity. |
| **Common-mode fault** | A fault that affects both Bus A and Bus B together (shared power loss, both cables in one bundle severed, identical firmware bug on both transmit paths). Path redundancy **shall not** be claimed to eliminate common-mode risk; **§5** mitigations apply. |

---

## 2. Criticality classes (C0, C1, C2)

### 2.1 C0 — Safety-critical path

**Definition:** Functions whose loss or corruption can **imminently** compromise vessel control, collision avoidance, or crew safety in the operational context where the vessel is used.

**Non-exhaustive examples:** autopilot demand / feedback loop on the same bus as the actuator interface; steering angle or rudder feedback used for closed-loop helm; engine / propulsion **alarm** and **shutdown** paths required by operational policy; bilge flood alarm where wired on Core.

**Requirements:**

- **SHALL** be installed in a **dual-bus domain** with **Class D** nodes (or **Class H** serving **Class S** downstream per **10**) for every Core-attached device on that path unless a **documented single-bus exception** is approved in the critical zone map with **operator-visible** continuous indication of **degraded single-bus** operation.
- **SHALL** meet **§5** minimum physical / electrical diversity where practical.
- **SHALL** expose **degraded-mode** behavior per **§3** if only one bus remains serviceable.

### 2.2 C1 — Mission-critical

**Definition:** Functions whose loss degrades navigation or propulsion **decision-making** or violates voyage plan, but is not an immediate safety loss equivalent to C0 in all contexts (e.g. loss of primary GNSS when a verified secondary position source exists off-bus).

**Non-exhaustive examples:** primary heading, wind, and depth for primary helm display when no independent redundant sensor exists; primary gateway binding authority channel (Core side) when no secondary authority exists.

**Requirements:**

- **SHALL** be installed in a **dual-bus domain** with **Class D** or **Class H**+**Class S** arrangements **unless** the critical zone map documents an **equivalent off-bus redundancy** (e.g. duplicate sensor on Stream with qualified voting — outside Core scope) and **operator-visible** indication when Core path is single-bus.
- **SHALL** meet **§3** degraded-mode rules when one bus fails.

### 2.3 C2 — Non-critical

**Definition:** Comfort, logging-only, or ancillary functions where loss does not materially affect safe navigation or propulsion decisions.

**Non-exhaustive examples:** tank levels for non-safety tanks; saloon lighting state; non-primary displays.

**Requirements:**

- **MAY** use a **single** Pelorus Core bus (**Class S**) without path redundancy.
- **SHALL NOT** be used to carry C0 or C1 traffic without upgrading the zone to C0/C1 rules.

### 2.4 Assignment authority

The **manufacturer** (for a fixed product) or the **installer / integrator** (for a vessel-specific fit-out) **shall** assign each Core-attached function to **exactly one** of C0, C1, or C2 in the **critical zone map**. Down-classifying C0 traffic to C2 to avoid dual-bus cost is **non-conformant** with this document.

---

## 3. Degraded-mode behavior (dual-bus domain)

When **Bus A** or **Bus B** is lost, powered down, or in bus-off:

- **Class D** nodes **shall** continue transmitting and receiving on the **remaining** bus without requiring operator reset, subject to **05** address-claim rules on that bus.
- **SHALL** set **operator-visible** fault indication (display annunciator, alarm DCID, or gateway UI — minimum one path per **15**) within **5 s** of detecting sustained loss of the peer bus (per **07** Bus Health DCID thresholds or controller error-passive / bus-off on the failed path).
- **SHALL** continue to apply **duplicate discard** on the surviving bus so that when the failed bus returns, transient duplicates do not corrupt application state (**03**).

---

## 4. Node and port classes (summary)

Normative physical and hub rules: **[02](./02-physical-layer.md)**, **[10](./10-repeater-specification.md)**.

| Class | Meaning |
|-------|---------|
| **Class S** | **Single** CAN FD transceiver; attaches to **one** of Bus A or Bus B only. Permitted for **C2** and for **C1** only when **§2.2** off-bus redundancy is documented. |
| **Class D** | **Dual** transceivers; attaches to **both** Bus A and Bus B. **Target** for new C0/C1 sensors, actuators, and displays in dual-bus domains. |
| **Class H** | **Hub** / RedBox: bridges **Class S** downstream segments onto **both** backbone buses with correct replication and sequence / bus-ID rules per **03** / **10**. |

---

## 5. Common-mode mitigation (physical / electrical)

Path redundancy **alone** is insufficient for C0/C1. Where practical, installers and manufacturers **shall**:

- Route Bus A and Bus B along **physically separated** cable paths (different bundles, different penetrations where feasible); **shall not** claim full path redundancy if both buses share a **single** unprotected cable run through a single hazard zone without documenting the residual risk in the critical zone map.
- Prefer **independent** protected feeds for transceiver / node power on Bus A vs Bus B when the vessel electrical design supports it (see **02** / **14**).

---

## 6. Critical zone map and conformance

- For any product or installation **declared** conformant with **path-redundant Pelorus Core** per **[16-compliance-self-declaration.md](./16-compliance-self-declaration.md)**, a **critical zone map** **shall** be published (paper, PDF, or structured file) listing: zone name, C0/C1/C2 assignment per function, Bus A/B topology sketch, node classes (S/D/H), and reference to tests in **15** executed for that configuration.
- Vessels or products using **only** C2 single-bus Core **may** omit the dual-bus domain; the declaration **shall** state **“Pelorus Core, single-bus (C2-only)”** or equivalent so purchasers know the reliability tier.

---

## 7. Relationship to segmentation (repeaters)

- A **dual-bus domain** may contain **multiple** **02** segments per bus via **repeaters** (**08**, **10**), subject to hop limits **per bus**.
- **Repeaters** **shall not** be described as satisfying C0/C1 **path** redundancy unless the installation also implements **Bus A** and **Bus B** per this document.

---

## 8. Open Items (to be resolved before v1.0 promotion)

- Formal templates (JSON/YAML) for critical zone map interchange.
- Harmonization with flag-state rules if **[Pelorus State](../stream/)** or a future state subsystem assigns regulatory **equipment classes**.
- Minimum alarm DCID or UI channel for “degraded single-bus” (cross-reference **07** when assigned).
- Interaction with **LMDE** bridging when one Pelorus bus is failed (**09**).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
