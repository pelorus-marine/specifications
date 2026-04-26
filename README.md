# Pelorus Marine

**An open marine data network standard. CAN FD-based, Rust-first, designed for reliability offshore.**

---

## Status

🚧 **Pre-specification — active early development.**

Pelorus is being designed in the open from day one. Expect rapid change, incomplete documentation, and unanswered questions. This is the right time to participate in shaping the standard.

---

## Why Pelorus

Marine electronics labeled "marine grade" are too often unreliable, expensive, and locked into closed proprietary ecosystems. NMEA 2000 — the dominant standard — is technically sound at its core but trapped by 20+ years of backward compatibility, closed specifications, and vendor incentives that prioritize differentiation over interoperability.

Sailors deserve better. Pelorus exists to provide it.

**Pelorus aims to be:**

- **Open** — full specification freely available, no NDAs, no licensing fees
- **Reliable** — built on CAN FD with deterministic real-time guarantees, designed for safety-critical use
- **Power-aware** — selective node sleep and wake, dramatically reducing overnight current draw at anchor
- **Interoperable** — bridges cleanly to existing NMEA 2000 and NMEA 0183 networks
- **Modern** — Rust reference implementations, IPv6 link-local where applicable, mDNS service discovery

---

## What's Here

This is the umbrella organization for the Pelorus Marine project. Repositories will be added as the work progresses.

### Current Documents

- **[power-management.md](./power-management.md)** — Developer reference for Pelorus power management on CAN FD networks (v0.3). Covers partial networking, selective wake-up, frame error counter, bus biasing, and marine operational modes. Fully cross-checked against ISO 11898-2:2016.

### Recent Progress
- Power management specification validated against the full ISO 11898-2:2016 standard (Sections 5.9–5.10). Ready for reference implementation work.

### Planned Repositories

- `spec` — The Pelorus protocol specification
- `pgn-rs` — Rust crate for parsing and decoding Pelorus PGNs
- `pm` — Reference implementation of Pelorus power management (`pelorus-pm` crate)
- `gateway` — Reference firmware for the Pelorus / NMEA 2000 / NMEA 0183 gateway node
- `signal-catalog` — VSS-syntax marine signal definitions

---

## Design Principles

**Sailor-first, not vendor-first.** NMEA is an industry association whose members are vendors. Their incentive structure protects vendor revenue. Pelorus inverts this — every design decision asks "what's best for the sailor at sea" before "what's best for the manufacturer."

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
- **Bridging:** Native bridges to NMEA 2000 (Classical CAN) and NMEA 0183 (RS-422)

---

## How to Follow Progress

- Watch this organization on GitHub for new repositories
- Read the documentation as it evolves (changes are tracked in git history)
- Open issues with questions, concerns, or contributions

---

## How to Contribute

Pelorus is at the stage where input on direction matters more than code. Contribution paths:

**Domain expertise.** Real-world experience with marine electronics, NMEA networks, racing or cruising — this informs design decisions that no amount of theoretical work can replace.

**Technical review.** The specifications are open for critique. If something is wrong, incomplete, or unclear, file an issue.

**Reference data.** Logged NMEA 2000 traffic from real vessels helps validate the bridge design and instance handling. If you can capture and share data from your boat (with appropriate privacy considerations), please do.

**Implementation testing.** As reference implementations come online, testing on real hardware in real marine environments is what separates a working specification from a theoretical one.

**Documentation.** Plain-English explanations of complex topics help adoption. If you can write clearly about CAN, NMEA, or marine networking, that's valuable.

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