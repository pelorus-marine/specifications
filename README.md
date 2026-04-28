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
- **Power-aware** — selective node sleep and wake so the network draws only what the current voyage context needs (not only at anchor — e.g. ocean passage may shed depth, radar, or other gear until coastal work again)
- **Interoperable** — bridges cleanly to the existing Legacy Marine Data Ecosystem (**Classical CAN**) from **Pelorus Core** (**CAN FD**) via gateways (not shared segments), and to older legacy serial marine networks where applicable
- **Modern** — Rust reference implementations, IPv6 link-local where applicable, mDNS service discovery

---

## What's Here

This is the umbrella organization for the **Pelorus** Marine project. Repositories will be added as the work progresses.

### Current documents

Normative text is split between **[`core/`](./core/)** (Pelorus Core, documents `00`–`16`) and **[`stream/`](./stream/)** (Pelorus Stream, `00`–`27`). Each subsystem has a single **authoritative index** (filenames, purpose, status, trust tier, completion tracking):

- **[`core/00-document-index.md`](./core/00-document-index.md)** — safety-critical CAN FD stack, gateway, conformance.
- **[`stream/00-document-index.md`](./stream/00-document-index.md)** — Ethernet media and telemetry; strictly non-safety-critical ([`stream/01-overview.md`](./stream/01-overview.md) §2–3).

**Cold start:** [Core overview](./core/01-overview.md) · [Stream overview](./stream/01-overview.md) · [ARCHITECTURE.md](./ARCHITECTURE.md) (non-normative repo-wide record).

### Recent Progress

- Overview document drafted as the entry point to the specification.
- Document index added to track specification completeness.
- Physical layer specification drafted to v0.1 (CAN FD profile, LMDE-compatible cabling and connectors, segmentation strategy, isolation tiers).
- Data link layer specification drafted to v0.1 (29-bit J1939-style identifiers, J1939 TP for multi-frame messages, no Fast Packet, no Remote Frames, reserved **Pelorus** identifier ranges).
- Power management specification completed to v0.4 (functional group bit allocations, four-state power model with state machine, NM cadence, FEC and bus biasing rules, implementation checklist). DCID allocations for WUF (0x0FF80) and NM (0x0FF81) are candidate values pending ratification in [core/07-dcid-registry.md](./core/07-dcid-registry.md).
- **Pelorus Stream specification** drafted to v0.1 across 28 sequential documents in [`stream/`](./stream/) (Issue [#1](https://github.com/pelorus-marine/specifications/issues/1)). Locked decisions: UUIDv7 stream IDs, deterministic CBOR control plane, Opus 48 kHz audio, IPv6 link-local with mDNS-SD discovery, UDP best-effort default with opt-in QUIC. Stream remains strictly non-safety-critical and decoupled from Core.

### Planned Repositories

- `spec` — The **Pelorus** protocol specification
- `dcid-rs` — Rust crate for parsing and decoding **Pelorus** DCIDs
- `pm` — Reference implementation of **Pelorus** power management (`pelorus-pm` crate)
- `gateway` — Reference firmware for the **Pelorus** / Legacy Marine Data Ecosystem / the older legacy serial marine protocol gateway node
- `signal-catalog` — VSS-syntax marine signal definitions

---

## Design Principles

**Sailor-first, not vendor-first.** The trade association behind the Legacy Marine Data Ecosystem represents equipment vendors; its incentives favor vendor revenue over sailor outcomes. **Pelorus** inverts that — every design decision asks "what's best for the sailor at sea" before "what's best for the manufacturer."

**Reliability over features.** A device that works for ten years is more valuable than a device with twenty features that fails after three. **Pelorus** targets longevity and field repairability as first-class requirements.

**Power awareness as architecture.** Boats are not on unlimited shore power. **Pelorus** treats power management as part of the protocol: nodes and functional groups sleep when the passage plan, watch, or conditions mean they are not needed — not only when lying at anchor.

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
