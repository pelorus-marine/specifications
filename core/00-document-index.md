# Pelorus Core — Specification Document Index

**Version:** Living  
**Last Updated:** April 26, 2026  
**Status:** Active  
**Trust:** Trusted

---

## About This Document

Numbered list of all documents that constitute a complete Pelorus Core specification. Numbers are stable references — once assigned, they do not change. New documents get new numbers; deprecated documents are marked but keep their numbers.

**Layout:** Tiers **1–4** (`00`–`16`) live in the repository’s **`core/`** directory (e.g. `specifications/core/01-overview.md`). **Tier 5** community files (`README.md`, `LICENSE.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`) live at the **repository root** next to `core/`.

This index is the authoritative list of Pelorus Core specification documents. Update when documents are added, drafted, or completed.

**Combined checkout:** When this repo sits beside `platform/`, `ecdis/`, and `reference-implementations/` in one tree, use **[`PELORUS_ALIGNMENT.md`](../PELORUS_ALIGNMENT.md)** (repository root here) for spec ↔ implementation traceability and feedback-loop checklists.

---

## 1. Trust Levels

Every document is annotated with a trust level so contributors know what to rely on:

- **Trusted** — written deliberately against external sources or published suite summaries; cited content is verified.
- **Unverified** — provisional draft of unknown provenance. Content has not been validated, may contradict trusted documents, may invent terms. Treat as a starting guess until reviewed.
- **Final** — frozen and not expected to change.

---

## 2. Document Index

### Tier 1 — Core Specification (Normative)

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 01 | `01-overview.md` | What Pelorus is, architecture summary, entry point | v0.1 draft | Trusted |
| 02 | `02-physical-layer.md` | Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation | v0.1 draft | Trusted |
| 03 | `03-data-link-layer.md` | CAN FD frame format usage, message addressing, error handling | v0.1 draft | Trusted |
| 04 | `04-power-management.md` | Selective wake-up, PNCs, power states, network management | v0.4 draft | Trusted (§1–5 ISO-validated; §6+ proposals subject to validation) |
| 05 | `05-addressing.md` | Source address claiming, conflict resolution, device identification | v0.1 draft | Unverified |
| 06 | `06-signal-catalog.md` | VSS-syntax catalog format, `Vessel.*` data model, instance handling | v0.1 draft | Unverified |
| 07 | `07-dcid-registry.md` | Specific DCID assignments and definitions | v0.1 draft | Unverified |

### Tier 2 — Architectural Specifications

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 08 | `08-network-architecture.md` | Segmentation, multi-segment networks, scaling | v0.1 draft | Unverified |
| 09 | `09-gateway-specification.md` | Classical CAN (LMDE) ↔ CAN FD (Pelorus Core) gateway | v0.1 draft | Unverified |
| 10 | `10-repeater-specification.md` | Pelorus Core repeater functional spec | v0.1 draft | Unverified |

### Tier 3 — Implementation Guidance

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 11 | `11-reference-implementations.md` | Pointers to canonical Rust crates, version compatibility | v0.1 draft | Unverified |
| 12 | `12-hardware-design-guide.md` | Schematic patterns, component selection, layout, EMC | v0.1 draft | Unverified |
| 13 | `13-firmware-design-guide.md` | State machines, embedded Rust patterns, testing | v0.1 draft | Unverified |
| 14 | `14-installation-guide.md` | Wiring guide, segment planning, troubleshooting | v0.1 draft | Unverified |

### Tier 4 — Compliance and Conformance

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 15 | `15-conformance-test-plan.md` | Conformance test plan (stub; procedures TBD) | v0.1 draft | Unverified |
| 16 | `16-compliance-self-declaration.md` | Manufacturer attestation template | v0.1 draft | Unverified |

### Tier 5 — Project Governance and Community

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 17 | `README.md` | GitHub org/repo landing page | v0.1 | Trusted |
| 18 | `LICENSE.md` | Creative Commons Attribution 4.0 | Final | Final |
| 19 | `ARCHITECTURE.md` | Architectural background and subsystem overview (non-normative) | v0.4 | Trusted |
| 20 | `CONTRIBUTING.md` | How to contribute to the specification | v0.1 draft | Unverified |
| 21 | `CODE_OF_CONDUCT.md` | Community behavior standards | v0.1 draft | Unverified |

---

## 3. Numbering Conventions

- Tier 1–4 documents use numeric prefixes (`01-`, `02-`, etc.) under **`core/`** so they sort in logical reading order
- Tier 5 documents use conventional names without prefixes at the **repo root** because GitHub and other tools expect them at known names
- Numbers are assigned at document creation and never reused
- Deprecated documents keep their number but get marked deprecated in this index

---

## 4. Completion Tracking

**Trusted or final:** 8 of 21

- `core/00-document-index.md`
- `core/01-overview.md` (v0.1)
- `core/02-physical-layer.md` (v0.1)
- `core/03-data-link-layer.md` (v0.1)
- `core/04-power-management.md` (v0.4)
- `ARCHITECTURE.md` (v0.4) (repo root)
- `LICENSE.md` (final) (repo root)
- `README.md` (v0.1) (repo root)

**Unverified — needs review:** 13 of 21

- `core/05-addressing.md`, `core/06-signal-catalog.md`, `core/07-dcid-registry.md`
- `core/08-network-architecture.md`, `core/09-gateway-specification.md`, `core/10-repeater-specification.md`
- `core/11-reference-implementations.md`, `core/12-hardware-design-guide.md`, `core/13-firmware-design-guide.md`, `core/14-installation-guide.md`
- `core/15-conformance-test-plan.md`, `core/16-compliance-self-declaration.md`
- `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` (repo root)

**Next priorities for v0.1 specification:**

1. Capture real LMDE bus traffic from a representative vessel (prerequisite for instance binding design — coordinate on [Pelorus Specifications — GitHub Issues](https://github.com/pelorus-marine/specifications/issues))
2. Review 05–07 against the trusted core (01–04) and either revalidate, rewrite, or delete each
3. Reconcile **instance binding** and catalog mechanics (**06**) against captured traffic and gateways (internal doc alignment: **03** / **04** / **07** DCID bands and **WUF** / **NM** wire layouts — **May 2026**)
