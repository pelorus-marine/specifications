# **Pelorus** Marine

**Open marine data network — CAN FD safety-critical core, high-bandwidth Ethernet stream, Rust-first references — built for reliability offshore.**

🚧 **Pre-specification.** Rapid change is normal; early participation matters.

---

## Why **Pelorus**

Marine electronics labeled "marine grade" are too often unreliable, expensive, and locked into closed proprietary ecosystems. The Legacy Marine Data Ecosystem — the dominant standard — is technically sound at its core but trapped by 20+ years of backward compatibility, closed specifications, and vendor incentives that prioritize differentiation over interoperability.

Sailors deserve better. **Pelorus** exists to provide it.

---

## What it is

Incumbent marine networking ([**LMDE**](./ARCHITECTURE.md#lmde) — *Legacy Marine Data Ecosystem*) works but sits behind certification gates, closed catalogs, and vendor-first incentives. **Pelorus** specifies an open stack sailors and builders can inspect, extend, and debug: power-aware **Pelorus Core** (CAN FD), **Pelorus Stream** (Ethernet, non-safety-critical media), and **Pelorus State** (coordination — planned), bridged sensibly to classical buses where needed.

---

## Read this next

**→ [`ARCHITECTURE.md`](./ARCHITECTURE.md)** — non-normative project record: why Pelorus exists, **LMDE** context and examples, Core / Stream / State roles, trademark and editorial rules.

**→ [`PELORUS_ALIGNMENT.md`](./PELORUS_ALIGNMENT.md)** — traceability and feedback-loop rituals across specs, `platform`, ECDIS, and reference implementations (when those repos are checked out beside this one).

Normative drafts live under **[`core/`](./core/)** and **[`stream/`](./stream/)** (each has a **`00-document-index.md`**). Cold start: [Core overview](./core/01-overview.md) · [Stream overview](./stream/01-overview.md). Track backlog and design threads on [**GitHub Issues**](https://github.com/pelorus-marine/specifications/issues).

---

## Highlights

- **Open** — CC-licensed specs; no fee to read or implement  
- **Sailor-first** — design choices favor offshore reality over brochure features  
- **Power-aware** — selective sleep and wake aligned with voyage context  
- **Debuggable** — transparent protocols and fixtures, not black-box gateways  
- **Split planes** — safety-critical **Pelorus Core** (CAN FD) vs bandwidth **Pelorus Stream** (Ethernet); Stream does not carry actuator authority  
- **Gateways, not same-wire myths** — classical [**LMDE**](./ARCHITECTURE.md#lmde) and Pelorus meet through gateways; shared-segment bit compatibility is not assumed  
- **Rust-first** — reference implementations aim to match the spec (`no_std`, `forbid(unsafe_code)` where applicable)  
- **Specified contracts** — DCIDs, addressing, and power behavior written for independent implementations  

**Site:** https://sevenseas.io/pelorus  
**Org:** https://github.com/pelorus-marine

---

## Releases

Tagged snapshots for citations and tooling: **[GitHub Releases](https://github.com/pelorus-marine/specifications/releases)** · **[CHANGELOG.md](./CHANGELOG.md)**

---

## Contribute · license

Questions and PRs: [GitHub Issues](https://github.com/pelorus-marine/specifications/issues). Guidelines: [`CONTRIBUTING.md`](./CONTRIBUTING.md).

Specifications: [CC BY 4.0](./LICENSE.md). Third-party names: [ARCHITECTURE §5](./ARCHITECTURE.md#5-trademarks-and-third-party-names). Pelorus branding on the website remains proprietary.
