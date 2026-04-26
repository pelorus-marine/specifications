# Pelorus Marine

**An open marine data network standard. CAN FD-based, Rust-first, designed for reliability offshore.**

---

## Status

🚧 **Pre-specification — active early development.**

Pelorus is being designed in the open from day one. Expect rapid change, incomplete documentation, and unanswered questions. This is the right time to participate in shaping the standard.

---

## Why Pelorus

Marine electronics labeled "marine grade" are too often unreliable, expensive, and locked into closed proprietary ecosystems. The legacy marine data ecosystem — the dominant standard — is technically sound at its core but trapped by 20+ years of backward compatibility, closed specifications, and vendor incentives that prioritize differentiation over interoperability.

Sailors deserve better. Pelorus exists to provide it.

**Pelorus aims to be:**

- **Open** — full specification freely available, no NDAs, no licensing fees
- **Reliable** — built on CAN FD with deterministic real-time guarantees, designed for safety-critical use
- **Power-aware** — selective node sleep and wake, dramatically reducing overnight current draw at anchor
- **Interoperable** — bridges cleanly to existing legacy marine data ecosystem and the older legacy serial marine protocol networks
- **Modern** — Rust reference implementations, IPv6 link-local where applicable, mDNS service discovery

---

## What's Here

This is the umbrella organization for the Pelorus Marine project. Repositories will be added as the work progresses.

### Current Documents

Start with the overview, then read by topic. The full document map is in [00-document-index.md](./00-document-index.md).

- **[01-overview.md](./01-overview.md)** — What Pelorus is, two-layer architecture, v1.0 scope, reading guide (v0.1).
- **[02-physical-layer.md](./02-physical-layer.md)** — Pelorus Core physical layer: bit rates, cabling, connectors, topology, transceivers, power, termination, isolation (v0.1 draft).
- **[03-data-link-layer.md](./03-data-link-layer.md)** — CAN FD frame format usage, 29-bit identifier and PGN structure, multi-frame transport, error handling (v0.1 draft).
- **[04-power-management.md](./04-power-management.md)** — Developer reference for Pelorus power management on CAN FD networks. Covers partial networking, selective wake-up, marine functional groups, power states, NM behavior, frame error counter, and bus biasing. Cross-checked against ISO 11898-2:2016 (v0.4 draft).

### Recent Progress

- Overview document drafted as the entry point to the specification.
- Document index added to track specification completeness.
- Physical layer specification drafted to v0.1 (CAN FD profile, legacy-marine-compatible cabling and connectors, segmentation strategy, isolation tiers).
- Data link layer specification drafted to v0.1 (29-bit J1939-style identifiers, J1939 TP for multi-frame messages, no Fast Packet, no Remote Frames, reserved Pelorus identifier ranges).
- Power management specification completed to v0.4 (functional group bit allocations, four-state power model with state machine, NM cadence, FEC and bus biasing rules, implementation checklist). PGN allocations for WUF (0x0FF80) and NM (0x0FF81) are candidate values pending ratification in 07-pgn-registry.md.

### Planned Repositories

- `spec` — The Pelorus protocol specification
- `pgn-rs` — Rust crate for parsing and decoding Pelorus PGNs
- `pm` — Reference implementation of Pelorus power management (`pelorus-pm` crate)
- `gateway` — Reference firmware for the Pelorus / legacy marine data ecosystem / the older legacy serial marine protocol gateway node
- `signal-catalog` — VSS-syntax marine signal definitions

---

## Design Principles

**Sailor-first, not vendor-first.** The legacy marine standards body is an industry association whose members are vendors. Their incentive structure protects vendor revenue. Pelorus inverts this — every design decision asks "what's best for the sailor at sea" before "what's best for the manufacturer."

**Reliability over features.** A device that works for ten years is more valuable than a device with twenty features that fails after three. Pelorus targets longevity and field repairability as first-class requirements.

**Power awareness as architecture.** Boats are not connected to the grid. Every milliamp matters at anchor. Pelorus treats power management as part of the protocol, not an afterthought.

**Open all the way down.** From the wire format to the reference implementation to the documentation to the test fixtures, everything is open source. The specification is written from freely accessible reference materials so contributors never need to purchase standards documents to participate.

**Honest about tradeoffs.** Where Pelorus has limitations, patent encumbrances, or unresolved questions, they are documented openly. See the Open Questions section in any specification document.

---

## Technical Foundations

- **Physical layer:** CAN FD for safety-critical data, Ethernet (M12 connectors) for high-bandwidth non-critical data
- **Connectors:** M12 throughout — IP67/IP68, industrial proven, multi-source
- **Network management:** ISO 11898-2:2016 partial networking adapted for marine operational modes
- **Reference implementations:** Rust, `no_std` first, `forbid(unsafe_code)`
- **Bridging:** Native bridges to the legacy marine data ecosystem (Classical CAN) and the older legacy serial marine protocol (RS-422)

---

## How to Follow Progress

- Watch this organization on GitHub for new repositories
- Read the documentation as it evolves (changes are tracked in git history)
- Open issues with questions, concerns, or contributions

---

## How to Contribute

Pelorus is at the stage where input on direction matters more than code. Contribution paths:

**Domain expertise.** Real-world experience with marine electronics, legacy marine networks, racing or cruising — this informs design decisions that no amount of theoretical work can replace.

**Technical review.** The specifications are open for critique. If something is wrong, incomplete, or unclear, file an issue.

**Reference data.** Logged legacy marine traffic from real vessels helps validate the bridge design and instance binding. If you can capture and share data from your boat (with appropriate privacy considerations), please do.

**Implementation testing.** As reference implementations come online, testing on real hardware in real marine environments is what separates a working specification from a theoretical one.

**Documentation.** Plain-English explanations of complex topics help adoption. If you can write clearly about CAN, legacy marine protocols, or marine networking in general, that's valuable.

---

## License

Documentation and specifications: [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).

Source code in subsidiary repositories will typically be licensed under MIT or Apache 2.0 — see individual repositories for specifics.

---

## Contact

GitHub issues are the preferred channel for technical discussion.

---

*"Navigation instruments used to be reliable, precise, and built to last. Then we forgot what that meant. Pelorus is bringing that back."*

---

**GitHub:** https://github.com/pelorus-marine