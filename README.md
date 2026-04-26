# **Pelorus** Marine

**An open marine data network standard. CAN FD-based, Rust-first, designed for reliability offshore.**

---

## Status

🚧 **Pre-specification — active early development.**

**Pelorus** is being designed in the open from day one. Expect rapid change, incomplete documentation, and unanswered questions. This is the right time to participate in shaping the standard.

**Website:** https://sevenseas.io/pelorus

---

## Why **Pelorus**

Marine electronics labeled "marine grade" are too often unreliable, expensive, and locked into closed proprietary ecosystems. The Legacy Marine Data Ecosystem — the dominant standard — is technically sound at its core but trapped by 20+ years of backward compatibility, closed specifications, and vendor incentives that prioritize differentiation over interoperability.

Sailors deserve better. **Pelorus** exists to provide it.

**Pelorus** aims to be:

- **Open** — full specification freely available, no NDAs, no licensing fees
- **Reliable** — built on CAN FD with deterministic real-time guarantees, designed for safety-critical use
- **Power-aware** — selective node sleep and wake, dramatically reducing overnight current draw at anchor
- **Interoperable** — bridges cleanly to the existing Legacy Marine Data Ecosystem and the older legacy serial marine protocol networks
- **Modern** — Rust reference implementations, IPv6 link-local where applicable, mDNS service discovery

---

## What's Here

This is the umbrella organization for the **Pelorus** Marine project. Repositories will be added as the work progresses.

### Current Documents

Start with the overview, then read by topic. The full document map is in [00-document-index.md](./00-document-index.md).

| # | Document | Description | Status |
|---|---|---|---|
| 01 | [01-overview.md](./01-overview.md) | What **Pelorus** is, two-layer architecture, v1.0 scope, reading guide. | v0.1 — trusted |
| 02 | [02-physical-layer.md](./02-physical-layer.md) | **Pelorus Core** physical layer: bit rates, cabling, connectors, topology, transceivers, power, termination, isolation. | v0.1 — trusted |
| 03 | [03-data-link-layer.md](./03-data-link-layer.md) | CAN FD frame format usage, 29-bit identifier and PGN structure, multi-frame transport, error handling. | v0.1 — trusted |
| 04 | [04-power-management.md](./04-power-management.md) | Partial networking, selective wake-up, marine functional groups, power states, NM behavior, frame error counter, and bus biasing. Cross-checked against ISO 11898-2:2016. | v0.4 — trusted (§1–5 ISO-validated; §6+ proposals subject to validation) |
| 05 | [05-addressing.md](./05-addressing.md) | Source address claiming, conflict resolution, device identification. | v0.1 — unverified draft |
| 06 | [06-signal-catalog.md](./06-signal-catalog.md) | VSS-syntax catalog format, `Vessel.*` data model, instance handling. | v0.1 — unverified draft |
| 07 | [07-pgn-registry.md](./07-pgn-registry.md) | Specific PGN assignments and definitions. | v0.1 — unverified draft |
| 08 | [08-network-architecture.md](./08-network-architecture.md) | Segmentation, multi-segment networks, scaling. | v0.1 — unverified draft |
| 09 | [09-gateway-specification.md](./09-gateway-specification.md) | LMDE-to-**Pelorus** gateway behavior. | v0.1 — unverified draft |
| 10 | [10-repeater-specification.md](./10-repeater-specification.md) | **Pelorus Core** repeater functional spec. | v0.1 — unverified draft |
| 11 | [11-reference-implementations.md](./11-reference-implementations.md) | Pointers to canonical Rust crates, version compatibility. | v0.1 — unverified draft |
| 12 | [12-hardware-design-guide.md](./12-hardware-design-guide.md) | Schematic patterns, component selection, layout, EMC. | v0.1 — unverified draft |
| 13 | [13-firmware-design-guide.md](./13-firmware-design-guide.md) | State machines, embedded Rust patterns, testing. | v0.1 — unverified draft |
| 14 | [14-installation-guide.md](./14-installation-guide.md) | Wiring guide, segment planning, troubleshooting. | v0.1 — unverified draft |
| 15 | [15-conformance-test-plan.md](./15-conformance-test-plan.md) | Conformance test plan (stub — procedures TBD). | v0.1 — unverified draft |
| 16 | [16-compliance-self-declaration.md](./16-compliance-self-declaration.md) | Manufacturer attestation template. | v0.1 — unverified draft |

Documents 01–04 are the trusted core; 05–16 are unverified provisional drafts pending review against the core. See [00-document-index.md](./00-document-index.md) for trust definitions and tier groupings.

### Recent Progress

- Overview document drafted as the entry point to the specification.
- Document index added to track specification completeness.
- Physical layer specification drafted to v0.1 (CAN FD profile, LMDE-compatible cabling and connectors, segmentation strategy, isolation tiers).
- Data link layer specification drafted to v0.1 (29-bit J1939-style identifiers, J1939 TP for multi-frame messages, no Fast Packet, no Remote Frames, reserved **Pelorus** identifier ranges).
- Power management specification completed to v0.4 (functional group bit allocations, four-state power model with state machine, NM cadence, FEC and bus biasing rules, implementation checklist). PGN allocations for WUF (0x0FF80) and NM (0x0FF81) are candidate values pending ratification in 07-pgn-registry.md.

### Planned Repositories

- `spec` — The **Pelorus** protocol specification
- `pgn-rs` — Rust crate for parsing and decoding **Pelorus** PGNs
- `pm` — Reference implementation of **Pelorus** power management (`pelorus-pm` crate)
- `gateway` — Reference firmware for the **Pelorus** / Legacy Marine Data Ecosystem / the older legacy serial marine protocol gateway node
- `signal-catalog` — VSS-syntax marine signal definitions

---

## Design Principles

**Sailor-first, not vendor-first.** The trade association behind the Legacy Marine Data Ecosystem represents equipment vendors; its incentives favor vendor revenue over sailor outcomes. **Pelorus** inverts that — every design decision asks "what's best for the sailor at sea" before "what's best for the manufacturer."

**Reliability over features.** A device that works for ten years is more valuable than a device with twenty features that fails after three. **Pelorus** targets longevity and field repairability as first-class requirements.

**Power awareness as architecture.** Boats are not connected to the grid. Every milliamp matters at anchor. **Pelorus** treats power management as part of the protocol, not an afterthought.

**Open all the way down.** From the wire format to the reference implementation to the documentation to the test fixtures, everything is open source. The specification is written from freely accessible reference materials so contributors never need to purchase standards documents to participate.

**Honest about tradeoffs.** Where **Pelorus** has limitations, patent encumbrances, or unresolved questions, they are documented openly. See the Open Questions section in any specification document.

---

## Technical Foundations

- **Physical layer:** CAN FD for safety-critical data, Ethernet (M12 connectors) for high-bandwidth non-critical data
- **Connectors:** M12 throughout — IP67/IP68, industrial proven, multi-source
- **Network management:** ISO 11898-2:2016 partial networking adapted for marine operational modes
- **Reference implementations:** Rust, `no_std` first, `forbid(unsafe_code)`
- **Bridging:** Native bridges to the Legacy Marine Data Ecosystem (Classical CAN) and the older legacy serial marine protocol (RS-422)

---

## How to Follow Progress

- Watch this organization on GitHub for new repositories
- Read the documentation as it evolves (changes are tracked in git history)
- Open issues with questions, concerns, or contributions

---

## How to Contribute

**Pelorus** is at the stage where input on direction matters more than code. Contribution paths:

**Domain expertise.** Real-world experience with marine electronics, LMDE networks, racing or cruising — this informs design decisions that no amount of theoretical work can replace.

**Technical review.** The specifications are open for critique. If something is wrong, incomplete, or unclear, file an issue.

**Reference data.** Logged LMDE bus traffic from real vessels helps validate the bridge design and instance binding. If you can capture and share data from your boat (with appropriate privacy considerations), please do.

**Implementation testing.** As reference implementations come online, testing on real hardware in real marine environments is what separates a working specification from a theoretical one.

**Documentation.** Plain-English explanations of complex topics help adoption. If you can write clearly about CAN, LMDE protocols, or marine networking in general, that's valuable.

---

## License

Documentation and specifications: [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).

Source code in subsidiary repositories will typically be licensed under MIT or Apache 2.0 — see individual repositories for specifics.

Website source: MIT or Apache 2.0, at your option.  
**Pelorus** name, logos, artwork on https://sevenseas.io/pelorus, and related branding are proprietary; all rights reserved.

---

## Contact

Technical discussion: GitHub issues on the relevant repository.

Project site: https://sevenseas.io/pelorus · **GitHub:** https://github.com/pelorus-marine

---

*"Navigation instruments used to be reliable, precise, and built to last. Then we forgot what that meant. **Pelorus** is bringing that back."*
