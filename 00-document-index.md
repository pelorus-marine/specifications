# Pelorus Core — Specification Document Index

**Purpose:** Numbered list of all documents that constitute a complete Pelorus Core specification. Numbers are stable references — once assigned, they do not change. New documents get new numbers; deprecated documents are marked but keep their numbers.

**Last Updated:** April 26, 2026

---

## Document Index

### Tier 1 — Core Specification (Normative)

| # | Filename | Purpose | Status |
|---|---|---|---|
| 01 | `01-overview.md` | What Pelorus is, architecture summary, entry point | v0.1 draft |
| 02 | `02-physical-layer.md` | Bit rates, cabling, connectors, topology, transceivers, power, termination, isolation | v0.1 draft |
| 03 | `03-data-link-layer.md` | CAN FD frame format usage, message addressing, error handling | v0.1 draft |
| 04 | `04-power-management.md` | Selective wake-up, PNCs, power states, network management | v0.4 draft |
| 05 | `05-addressing.md` | Source address claiming, conflict resolution, device identification | Not started |
| 06 | `06-signal-catalog.md` | VSS-syntax catalog format, `Vessel.*` data model, instance handling | Not started |
| 07 | `07-pgn-registry.md` | Specific PGN assignments and definitions | Not started |

### Tier 2 — Architectural Specifications

| # | Filename | Purpose | Status |
|---|---|---|---|
| 08 | `08-network-architecture.md` | Segmentation, multi-segment networks, scaling | Not started |
| 09 | `09-gateway-specification.md` | NMEA 2000 to Pelorus gateway behavior | Not started |
| 10 | `10-repeater-specification.md` | Pelorus Core repeater functional spec | Not started |

### Tier 3 — Implementation Guidance

| # | Filename | Purpose | Status |
|---|---|---|---|
| 11 | `11-reference-implementations.md` | Pointers to canonical Rust crates, version compatibility | Not started |
| 12 | `12-hardware-design-guide.md` | Schematic patterns, component selection, layout, EMC | Not started |
| 13 | `13-firmware-design-guide.md` | State machines, embedded Rust patterns, testing | Not started |
| 14 | `14-installation-guide.md` | Wiring guide, segment planning, troubleshooting | Not started |

### Tier 4 — Compliance and Conformance

| # | Filename | Purpose | Status |
|---|---|---|---|
| 15 | `15-conformance-test-plan.md` | Verification procedures, expected results, edge cases | Not started |
| 16 | `16-compliance-self-declaration.md` | Manufacturer attestation template | Not started |

### Tier 5 — Project Governance and Community

| # | Filename | Purpose | Status |
|---|---|---|---|
| 17 | `README.md` | GitHub org/repo landing page | v0.1 |
| 18 | `LICENSE.md` | Creative Commons Attribution 4.0 | Final |
| 19 | `ARCHITECTURE.md` | Internal decision record (not part of normative spec) | v0.1 |
| 20 | `CONTRIBUTING.md` | How to contribute to the specification | Not started |
| 21 | `CODE_OF_CONDUCT.md` | Community behavior standards | Not started |
| 22 | `CHANGELOG.md` | Specification version history | Not started |
| 23 | `patent-considerations.md` | IP landscape, selective wake-up patents, NMEA trademark, commercial guidance | Not started |

---

## Numbering Conventions

- Tier 1-4 documents use numeric prefixes (`01-`, `02-`, etc.) so they sort in logical reading order in any file browser
- Tier 5 documents use conventional names without prefixes (README, LICENSE, etc.) because GitHub and other tools expect them at known names
- Numbers are assigned at document creation and never reused
- Deprecated documents keep their number but get marked deprecated in this index

## Completion Tracking

**Currently complete or drafted:** 6 of 23 (26%)

- 01-overview.md (v0.1)
- 02-physical-layer.md (v0.1)
- 03-data-link-layer.md (v0.1)
- 04-power-management.md (v0.4)
- LICENSE.md (final)
- README.md (v0.1)

**Next priorities for v0.1 specification:**

1. `05-addressing.md` — source address claiming, conflict resolution
2. `06-signal-catalog.md` — the data model
3. `07-pgn-registry.md` — initial PGN set, ratifies WUF/NM PGNs proposed in 04

These three plus existing documents constitute the minimum viable specification for prototyping work to begin.

---

*This index is the authoritative list of Pelorus Core specification documents. Update when documents are added, drafted, or completed.*
