# Pelorus — architecture record

**Last Updated:** April 26, 2026  
**Status:** Living (non-normative)

Locked requirements live in [01-overview.md](./01-overview.md) §9 and documents **02–16**. This file records **why** decisions were made, what was rejected, and what is still open.

---

## 1. Project

- **Mission:** Open marine data network; CAN FD core; Rust-first reference code; reliability offshore.
- **Terminology:** **Legacy Marine Data Ecosystem (LMDE)** — project code name for the incumbent certification-gated fieldbus and physical plant (see [01-overview.md](./01-overview.md) **Terminology**). No third-party trademarks for that ecosystem in this repo.
- **Presence:** Specification hub **https://sevenseas.io/pelorus** · org **https://github.com/pelorus-marine** · community face Seven Seas (`sevenseas.io`).

---

## 2. Problem Pelorus targets

Weaknesses of the **Legacy Marine Data Ecosystem** that Pelorus addresses: closed protocol and certification wall; always-on power; classical CAN at 250 kbit/s locked in by install base; single-segment fragility; poor sailor-side debuggability; vendor-specific extensions. **Bandwidth is not the main issue** for typical navigation/engine PGNs at 250 kbit/s — openness, power, reliability, and behavior matter more.

---

## 3. Stack shape

- **Pelorus Core (CAN FD):** 250 kbit/s arbitration / 500 kbit/s data, 64-byte frames, M12 A-coded **LMDE micro** plant, linear bus + T-drops, ISO 11898-2:2016 partial networking / selective wake-up, segmentation via isolated repeaters.
- **Pelorus Stream (Ethernet):** High bandwidth; connector direction set; full protocol stack **deferred**.

---

## 4. Locked Pelorus Core decisions (summary)

*(Detail and testable numbers: [02-physical-layer.md](./02-physical-layer.md), [03-data-link-layer.md](./03-data-link-layer.md), [04-power-management.md](./04-power-management.md), [01-overview.md](./01-overview.md) §9.)*

| Area | Decision |
|------|----------|
| Bit rate / frame | 250k arb / 500k data; CAN FD; **no** Fast Packet in core |
| Physical | M12 A-coded 5-pin, LMDE micro cable, split termination, 9–32 V, reverse-polarity protection; segment limits per 02/08 |
| Transceiver | ISO 11898-2:2016 partial networking + selective wake; CAN FD ≥1 Mbit/s; **no** SIC required at 500k data |
| Isolation | Tiered: mandatory above thresholds / high-power interfaces; optional for benign low-power sensors |
| Scaling | Repeaters: galvanic isolation, transparent CAN FD forward, ≤4 hops; star + central gateway recommended large vessels |
| Power | Four states; WUF/NM/PNC-style groups per **04**; selective wake **patents** in ISO 11898-2:2016 — RAND pledge; **commercial products need IP counsel** |
| Addressing / catalog | J1939-81 / ISO 11783-5 parity for SA/claiming; VSS + `Vessel.*`; extension PGN band per **07** (reconcile vs **03**) |

---

## 5. Rejected for v1.0 (do not re-propose without new evidence)

Higher data-phase rates; B-coded connectors to force cable churn; bit-rate auto-negotiation; universal galvanic isolation; Fast Packet in core; Signal K **as core** (app-level bridge OK); DIP-switch per-device profile selection; always-on bus as only mode; **sole** gateway as only profile authority (layered NV + gateway override instead).

---

## 6. Open issues

### 6.1 Specification

- Full PGN registry; **03** vs **07** on Pelorus extension range; **04** vs **07** on NM payload; ratify WUF/NM candidates (0x0FF80 / 0x0FF81).
- Validate **09** gateway and **10** repeater specs against hardware.
- Conformance fixtures (**15** stub).

### 6.2 Instance binding (blocking for clean semantics)

LMDE instance fields vs canonical `Vessel.*` paths: binding table ownership, drift, provisioning UX, failure modes. **Prerequisite:** captured **LMDE bus traffic** from a representative vessel (e.g. canboat-class tooling); document devices, PGNs, instances, sailor-visible failures.

### 6.3 Pelorus Stream

Stack, PoE, switching — mostly undecided.

### 6.4 Data model

VSS + standalone `Vessel.*` decided; semantic overlay from PGN→canonical paths partial; custom Pelorus VSS attributes need formal definition in **06**.

### 6.5 Hardware / business

Prototype current and wake latency; EMC on cable plants; maritime IP review before commercial selective-wake products; corporate structure when/if commercial.

---

## 7. Reading order (cold start)

1. [01-overview.md](./01-overview.md)  
2. [02-physical-layer.md](./02-physical-layer.md), [03-data-link-layer.md](./03-data-link-layer.md), [04-power-management.md](./04-power-management.md)  
3. [00-document-index.md](./00-document-index.md) for trust on **05–16**

---

Working rules: **Normative** requirements live in [01-overview.md](./01-overview.md) §9 and documents **02–16**; this file is background only. Do not relitigate **§5** rejections without maintainer direction. Prefer simplicity and static profiles for v1.0. Cite external claims. Update this file when decisions change.
