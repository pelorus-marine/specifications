# Pelorus Core — Specification Document Index

**Purpose:** Numbered list of all documents that constitute a complete Pelorus Core specification. Numbers are stable references — once assigned, they do not change. New documents get new numbers; deprecated documents are marked but keep their numbers.

**Last Updated:** April 26, 2026

---

## Trust Levels

Every document is annotated with a trust level so contributors know what to rely on:

- **Trusted** — written deliberately against external sources or locked decisions; cited content is verified.
- **Unverified** — provisional draft of unknown provenance. Content has not been validated, may contradict locked decisions, may invent terms. Treat as a starting guess until reviewed.
- **Final** — frozen and not expected to change.

---

## Document Index

### Tier 1 — Core Specification (Normative)

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 01 | `01-overview.md` | What Pelorus is, architecture summary, entry point | v0.1 draft | Trusted |
| 02 | `02-physical-layer.md` | Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation | v0.1 draft | Trusted |
| 03 | `03-data-link-layer.md` | CAN FD frame format usage, message addressing, error handling | v0.1 draft | Trusted |
| 04 | `04-power-management.md` | Selective wake-up, PNCs, power states, network management | v0.4 draft | Trusted (§1–5 ISO-validated; §6+ proposals subject to validation) |
| 05 | `05-addressing.md` | Source address claiming, conflict resolution, device identification | v0.1 draft | Unverified |
| 06 | `06-signal-catalog.md` | VSS-syntax catalog format, `Vessel.*` data model, instance handling | v0.1 draft | Unverified |
| 07 | `07-pgn-registry.md` | Specific PGN assignments and definitions | v0.1 draft | Unverified |

### Tier 2 — Architectural Specifications

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 08 | `08-network-architecture.md` | Segmentation, multi-segment networks, scaling | v0.1 draft | Unverified |
| 09 | `09-gateway-specification.md` | Legacy-marine-to-Pelorus gateway behavior | v0.1 draft | Unverified |
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
| 15 | `15-conformance-test-plan.md` | Verification procedures, expected results, edge cases | v0.1 draft | Unverified |
| 16 | `16-compliance-self-declaration.md` | Manufacturer attestation template | v0.1 draft | Unverified |

### Tier 5 — Project Governance and Community

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 17 | `README.md` | GitHub org/repo landing page | v0.1 | Trusted |
| 18 | `LICENSE.md` | Creative Commons Attribution 4.0 | Final | Final |
| 19 | `ARCHITECTURE.md` | Durable record of architectural decisions and rationale (non-normative) | v0.2 | Trusted |
| 20 | `CONTRIBUTING.md` | How to contribute to the specification | v0.1 draft | Unverified |
| 21 | `CODE_OF_CONDUCT.md` | Community behavior standards | v0.1 draft | Unverified |

---

## Numbering Conventions

- Tier 1-4 documents use numeric prefixes (`01-`, `02-`, etc.) so they sort in logical reading order in any file browser
- Tier 5 documents use conventional names without prefixes (README, LICENSE, etc.) because GitHub and other tools expect them at known names
- Numbers are assigned at document creation and never reused
- Deprecated documents keep their number but get marked deprecated in this index

## Completion Tracking

**Trusted or final:** 8 of 21

- 00-document-index.md
- 01-overview.md (v0.1)
- 02-physical-layer.md (v0.1)
- 03-data-link-layer.md (v0.1)
- 04-power-management.md (v0.4)
- ARCHITECTURE.md (v0.2)
- LICENSE.md (final)
- README.md (v0.1)

**Unverified — needs review:** 13 of 21

- 05-addressing.md, 06-signal-catalog.md, 07-pgn-registry.md
- 08-network-architecture.md, 09-gateway-specification.md, 10-repeater-specification.md
- 11-reference-implementations.md, 12-hardware-design-guide.md, 13-firmware-design-guide.md, 14-installation-guide.md
- 15-conformance-test-plan.md, 16-compliance-self-declaration.md
- CONTRIBUTING.md, CODE_OF_CONDUCT.md

**Next priorities for v0.1 specification:**

1. Capture real legacy marine bus traffic from a representative vessel (prerequisite for instance binding design — see ARCHITECTURE.md §6.2)
2. Review 05–07 against the trusted core (01–04) and either revalidate, rewrite, or delete each
3. Reconcile cross-document conflicts: PGN range (03 vs 07), NM payload (04 vs 07), instance binding (06)

---

*This index is the authoritative list of Pelorus Core specification documents. Update when documents are added, drafted, or completed.*
