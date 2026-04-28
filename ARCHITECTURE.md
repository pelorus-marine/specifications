# Pelorus — architecture record

**Last Updated:** April 27, 2026  
**Status:** Living (non-normative)

Locked requirements live in [01-overview.md](./core/01-overview.md) §9 and documents **02–16** in [`core/`](./core/). This file records **why** decisions were made, what was rejected, and what is still open.

---

## 1. Project

- **Mission:** Open marine data network; CAN FD core; Rust-first reference code; reliability offshore.
- **Terminology:** **Legacy Marine Data Ecosystem (LMDE)** — project code name for the incumbent certification-gated fieldbus and physical plant (see [01-overview.md](./core/01-overview.md) **Terminology**). Prefer **LMDE** over third-party trade names in running text; see [**§8**](#8-trademarks-and-third-party-names-editorial-not-legal-advice) for editorial policy on **NMEA®**, **OneNet®**, etc.
- **Presence:** Specification hub **https://sevenseas.io/pelorus** · org **https://github.com/pelorus-marine** · community face Seven Seas (`sevenseas.io`).

---

## 2. Problem Pelorus targets

Weaknesses of the **Legacy Marine Data Ecosystem** that Pelorus addresses: closed protocol and certification wall; **always-on power** (full suite energized even when passage or context makes much of it useless for days); classical CAN at 250 kbit/s locked in by install base; single-segment fragility; poor sailor-side debuggability; vendor-specific extensions. **Bandwidth is not the main issue** for typical navigation/engine DCIDs at 250 kbit/s — openness, power, reliability, and behavior matter more.

---

## 3. Stack shape

- **LMDE (legacy):** **Classical CAN** 250 kbit/s, 8-byte frames; **not** same-segment-interoperable with **Pelorus Core CAN FD** (see **01** §4).
- **Pelorus Core (CAN FD):** 250 kbit/s arbitration / 500 kbit/s data, 64-byte frames, M12 A-coded **LMDE micro** plant, linear bus + T-drops, ISO 11898-2:2016 partial networking / selective wake-up, segmentation via isolated repeaters.
- **Pelorus Stream (Ethernet):** High bandwidth, non-safety-critical. M12 D-coded 4-pin at 100 Mbit/s. v0.1 specification drafted across 28 documents in [`stream/`](./stream/); **draft** design targets in [stream/01-overview.md §9](./stream/01-overview.md#9-draft-design-targets-summary). PoE strategy and switch architecture remain open (§6.3).
- **Pelorus State (logical):** Event → snapshot → situation → policy/intents; specified under [`state/`](./state/00-document-index.md) (draft index only until documents are authored). Problem statement: [Issue #2](https://github.com/pelorus-marine/specifications/issues/2).

### 3.1 Pelorus Stream vs incumbent Ethernet marine practice (checklist)

Pelorus Stream is positioned as the **open** high-bandwidth complement to **Pelorus Core**, learning from what operators want from **Ethernet-based marine IP networks** without copying business models this project rejects.

| Theme | Take (Pelorus intent) | Leave (Pelorus rejects) |
|------|------------------------|-------------------------|
| Physical | IPv6-first; industrial M12 Ethernet plant; path to higher rates (X-coded) | Mandated single-vendor connector story where it implies exclusivity |
| Discovery | Zero-config **mDNS-SD** on the boat | Discovery that only works with paid tooling |
| Security | **Profiles**: open LAN baseline + optional hardening later | “Security” as gatekeeping or NDAs |
| Data model | **Explicit** link to **`Vessel.*`** + Core DCIDs where telemetry mirrors Core (Stream metadata `vss`) | Opaque proprietary envelopes as the only interop path |
| Safety | **Hard wall**: Stream never drives Core actuation | Any design where entertainment or telemetry can block safety traffic |
| Certification | Self-test vs reference implementations | Mandatory certification monopoly for hobbyist/small OEM participation |

This table compares **capabilities and governance**, not wire compatibility with any third-party standard. See also **§8** (trademark/editorial policy).

---

## 4. Locked Pelorus Core decisions (summary)

*(Detail and testable numbers: [02-physical-layer.md](./core/02-physical-layer.md), [03-data-link-layer.md](./core/03-data-link-layer.md), [04-power-management.md](./core/04-power-management.md), [01-overview.md](./core/01-overview.md) §9.)*

| Area | Decision |
|------|----------|
| Bit rate / frame | 250k arb / 500k data; CAN FD; **no** Fast Packet in core |
| Physical | M12 A-coded 5-pin, LMDE micro cable, split termination, 9–32 V, reverse-polarity protection; segment limits per 02/08 |
| Transceiver | ISO 11898-2:2016 partial networking + selective wake; CAN FD ≥1 Mbit/s; **no** SIC required at 500k data |
| Isolation | Tiered: mandatory above thresholds / high-power interfaces; optional for benign low-power sensors |
| Scaling | Repeaters: galvanic isolation, transparent CAN FD forward, ≤4 hops; star + central gateway recommended large vessels |
| Power | Four states; WUF/NM/PNC-style groups per **04**; selective wake **patents** in ISO 11898-2:2016 — RAND pledge; **commercial products need IP counsel** |
| Addressing / catalog | J1939-81 / ISO 11783-5 parity for SA/claiming; VSS + `Vessel.*`; extension DCID band per **07** (reconcile vs **03**) |

---

## 5. Rejected for v1.0 (do not re-propose without new evidence)

Higher data-phase rates; B-coded connectors to force cable churn; bit-rate auto-negotiation; universal galvanic isolation; Fast Packet in core; Signal K **as core** (app-level bridge OK); DIP-switch per-device profile selection; always-on bus as only mode; **sole** gateway as only profile authority (layered NV + gateway override instead).

---

## 6. Open issues

### 6.1 Specification

- Full DCID registry; **03** vs **07** on Pelorus extension range; **04** vs **07** on NM payload; ratify WUF/NM candidates (0x0FF80 / 0x0FF81).
- Validate **09** gateway and **10** repeater specs against hardware.
- Conformance fixtures (**15** stub).

### 6.2 Instance binding (blocking for clean semantics)

LMDE instance fields vs canonical `Vessel.*` paths: binding table ownership, drift, provisioning UX, failure modes. **Prerequisite:** captured **LMDE bus traffic** from a representative vessel (e.g. canboat-class tooling); document devices, DCIDs, instances, sailor-visible failures.

### 6.3 Pelorus Stream

v0.1 protocol specification drafted across 28 documents in [`stream/`](./stream/) (Issue [#1](https://github.com/pelorus-marine/specifications/issues/1)). **Draft targets** (not field-validated): UUIDv7 stream IDs, deterministic CBOR control plane, Opus 48 kHz audio, IPv6 link-local with mDNS-SD, UDP best-effort default with opt-in QUIC, strict non-safety-critical decoupling from Core. PoE strategy, switch sourcing, and conformance test plan remain open. Bench-validate before treating Stream §9 in `01-overview` as stable.

### 6.4 Data model

VSS + standalone `Vessel.*` decided; semantic overlay from DCID→canonical paths partial; custom Pelorus VSS attributes need formal definition in **06**.

### 6.5 Hardware / business

Prototype current and wake latency; EMC on cable plants; maritime IP review before commercial selective-wake products; corporate structure when/if commercial.

### 6.6 Pelorus State

Draft index only: [`state/00-document-index.md`](./state/00-document-index.md). Author `01`–`09` bodies per that index (supersedes overlapping split in [Issue #2](https://github.com/pelorus-marine/specifications/issues/2) task list).

---

## 7. Reading order (cold start)

1. [01-overview.md](./core/01-overview.md)  
2. [02-physical-layer.md](./core/02-physical-layer.md), [03-data-link-layer.md](./core/03-data-link-layer.md), [04-power-management.md](./core/04-power-management.md)  
3. [00-document-index.md](./core/00-document-index.md) for trust on **05–16**
4. [stream/01-overview.md](./stream/01-overview.md) for the non-safety-critical Stream subsystem; [stream/00-document-index.md](./stream/00-document-index.md) for the Stream document map  
5. [state/00-document-index.md](./state/00-document-index.md) for the planned Pelorus State subsystem ([Issue #2](https://github.com/pelorus-marine/specifications/issues/2))

---

Working rules: **Normative** requirements live in [01-overview.md](./core/01-overview.md) §9 and documents **02–16** under [`core/`](./core/); this file is background only. Do not relitigate **§5** rejections without maintainer direction. Prefer simplicity and static profiles for v1.0. Cite external claims. Update this file when decisions change.

---

## 8. Trademarks and third-party names (editorial; not legal advice)

Pelorus is an independent open specification. **This is not legal advice**; consult counsel before shipping product packaging or marketing that cites other organizations’ brands.

**Practical editorial rules for this repository:**

1. Prefer **project code names** already in use here — e.g. **LMDE** (*Legacy Marine Data Ecosystem*) — when referring generically to the incumbent marine CAN ecosystem, instead of repeating registered trade names in every sentence.
2. When **comparison** to a specific industry program is necessary, use **nominative fair use**: name the product or standard **once** to identify what is being compared, then use neutral phrases (“the incumbent Ethernet marine standard”, “the dominant certification-gated CAN stack”) for the rest of the section.
3. **Do not** imply affiliation, endorsement, or compatibility unless a normative document **tests** that claim. Say “**OneNet-style**” or “**N2K-class**” only when describing *categories* of behavior, not wire compliance.
4. **Registered marks** (examples often cited in marine electronics: **NMEA 2000®**, **NMEA OneNet®** — marks owned by the **National Marine Electronics Association**) should appear with **correct spelling** and, where your counsel advises, the **®** symbol on **first prominent use** in outward-facing materials. In **internal specification drafts**, lean on **LMDE** + “industry Ethernet marine networking” to reduce repetition of third-party marks while remaining clear.
5. **NMEA** and derivative names are **not** used as Pelorus product names; Pelorus is **not** “NMEA-compatible” unless a future conformance doc says so with test vectors.

Update this section if counsel provides a project trademark policy.
