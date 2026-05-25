# **Pelorus** Marine

**Open marine data network — CAN FD safety-critical core, high-bandwidth Ethernet stream, Rust-first references — built for reliability offshore.**

🚧 **Pre-specification.** Everything here is draft until v1.0 — rapid change is normal; early participation matters.

---

## Why **Pelorus**

Marine electronics labeled "marine grade" are too often unreliable, expensive, and locked into closed proprietary ecosystems. The Legacy Marine Data Ecosystem — the dominant standard — is technically sound at its core but trapped by 20+ years of backward compatibility, closed specifications, and vendor incentives that prioritize differentiation over interoperability.

The technologies needed to fix this — CAN FD frames, partial networking with selective wake-up, split termination, free hobbyist device IDs — have been industry practice elsewhere for a decade or more. They have not landed in marine networking because the governance model requires consensus among large manufacturers with no commercial incentive to obsolete their own installed base. The bill comes due as 8-byte frames, no managed sleep, certification fees that exclude hobbyists, and a members-only message catalog. Pelorus' technical decisions catch up to current practice; the open governance — CC BY 4.0 spec, free manufacturer codes, no certification gate — is what keeps it from falling behind again.

Closed specs and vendor-encrypted proprietary PGNs have a second-order cost: vessel data stays trapped per vendor. There is no shared format, no community pool, no foundation for marine machine learning to build on. Open governance unlocks the substrate too — not just the protocol but the data that flows over it.

Marine compute operates on bounded power that fails routinely — cloudy week, dead alternator, damaged hydrogenerator. Pelorus treats power as a first-class design constraint: services declare tier, the system gracefully sheds non-critical features when generation falters, and the safety path keeps running on what's left.

Sailors deserve better. **Pelorus** exists to provide it.

---

## What it is

Incumbent marine networking ([**LMDE**](./ARCHITECTURE.md#lmde) — *Legacy Marine Data Ecosystem*) works but sits behind certification gates, closed catalogs, and vendor-first incentives. **Pelorus** specifies an open stack sailors and builders can inspect, extend, and debug: power-aware **Pelorus Core** (CAN FD), **Pelorus Stream** (Ethernet, non-safety-critical media), and **Pelorus State** (fused world-state — early draft), bridged sensibly to classical buses where needed.

---

## Read this next

**→ [`ARCHITECTURE.md`](./ARCHITECTURE.md)** — non-normative project record: why Pelorus exists, **LMDE** context and examples, Core / Stream / State roles, trademark and editorial rules.

Normative drafts live under **[`core/`](./core/)**, **[`stream/`](./stream/)**, and **[`state/`](./state/)** (each has a **`00-document-index.md`**; State is the least mature). The shared `Vessel.*` semantic vocabulary consumed by all three lives in **[`catalog/`](./catalog/)**. Non-normative hardware, installation, and reference-software guidance lives in **[`implementation/`](./implementation/)**. Cold start: [Catalog overview](./catalog/01-overview.md) · [Core overview](./core/01-overview.md) · [Stream overview](./stream/01-overview.md) · [State overview](./state/01-overview.md) · [Implementation overview](./implementation/01-overview.md). Track backlog and design threads on [**GitHub Issues**](https://github.com/pelorus-marine/specifications/issues).

---

## Highlights

- **Open** — CC-licensed specs; no fee to read or implement  
- **Sailor-first** — design choices favor offshore reality over brochure features  
- **Power-aware** — selective sleep, wake, and graceful tiered degradation when energy is scarce  
- **Open data substrate** — CC-licensed catalog and open trace format (ASAM MDF4) so vessel data isn't trapped behind vendor encryption  
- **Debuggable** — transparent protocols and fixtures, not black-box gateways  
- **Split planes** — safety-critical **Pelorus Core** (CAN FD) vs bandwidth **Pelorus Stream** (Ethernet); Stream does not carry actuator authority  
- **Gateways, not same-wire myths** — classical [**LMDE**](./ARCHITECTURE.md#lmde) and Pelorus meet through gateways; shared-segment bit compatibility is not assumed  
- **Rust-first** — reference implementations aim to match the spec (`no_std`, `forbid(unsafe_code)` where applicable)  
- **Specified contracts** — Data Contracts (DC_IDs) in a Pelorus-native namespace, addressing, and power behavior written for independent implementations  
- **Open firmware update** — vendor-neutral protocol so any compliant tool can update any compliant device  

**Site:** <https://sevenseas.io/pelorus>  
**Org:** <https://github.com/pelorus-marine>

---

## Releases

Tagged snapshots for citations and tooling: **[GitHub Releases](https://github.com/pelorus-marine/specifications/releases)**

---

## Contribute · license

Questions and PRs: [GitHub Issues](https://github.com/pelorus-marine/specifications/issues). Guidelines: [`CONTRIBUTING.md`](./CONTRIBUTING.md).

Specifications: [CC BY 4.0](./LICENSE.md). Third-party names: [ARCHITECTURE §5](./ARCHITECTURE.md#5-trademarks-and-third-party-names). Pelorus branding on the website remains proprietary.
