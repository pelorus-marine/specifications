# Pelorus Core — DCID Registry

**Version:** 0.1 Draft  
**Last Updated:** May 4, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the Pelorus DCID (Data Contract ID) registry — numeric assignments and registry policy on the Pelorus Core CAN FD bus. It is the transport counterpart to [06-signal-catalog.md](./06-signal-catalog.md). Identifier layout and DCID derivation are normative in **[03-data-link-layer.md](./03-data-link-layer.md)**. **Wake-up and NM payload layouts** for Pelorus DCIDs **0x0FF80** / **0x0FF81** are normative in **[04-power-management.md §7](./04-power-management.md#7-reserved-identifiers-and-data-conventions)** — this document duplicates **only** summary tables aligned with **04**; on conflict, **04** wins.

Stack-level decisions (J1939-style identifiers, no Fast Packet, Pelorus extension range) are summarized in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary). Address claiming and NAME handling are in **[05-addressing.md](./05-addressing.md)**.

---

## 1. Pelorus-Specific DCIDs

These DCIDs are defined exclusively for Pelorus and ratify the candidates from **[04-power-management.md](./04-power-management.md)**.

### DCID 0x0FF80 — Wake-Up Frame (WUF)

- **Priority:** 0 (highest)
- **Type:** Single
- **Length:** 8 bytes
- **Transmission:** Broadcast on selective wake events
- **Purpose:** Triggers partial-network wake-up per ISO 11898-2:2016

**Wire layout:** Normative — **[04 §7.2](./04-power-management.md#72-wuf-data-field)** (functional groups in byte **0** per **[04 §6](./04-power-management.md#6-pelorus-marine-functional-groups-pncs)**; bytes **1–7** reserved in **v1.0**).

| Byte(s) | Field |
|---------|--------|
| 0 | Functional-group bitmask (**04** §6) |
| 1–7 | Reserved — transmit **`0x00`**, ignore on receive (**04** §7.2) |

### DCID 0x0FF81 — Network Management (NM)

- **Priority:** 6
- **Type:** Single (200 ms cadence when active per **[04 §9.1](./04-power-management.md#91-nm-cadence)**)
- **Length:** 8 bytes
- **Purpose:** Coordinated cluster sleep / wake — CanNm-style behavior (**[04 §9](./04-power-management.md#9-network-management-behavior)**)

**Wire layout:** Normative — **[04 §7.4](./04-power-management.md#74-nm-data-field)** (byte **0** NM state **§9.2**; byte **1** active-groups low byte; bytes **2–7** reserved in **v1.0**).

| Byte | Field |
|------|--------|
| 0 | NM state (**04** §9.2) |
| 1 | Active functional groups — low byte (**04** §7.4) |
| 2–7 | Reserved — transmit **`0x00`**, ignore on receive (**04** §7.4) |

### 1.3 Bus health (DCID 0x0FF82)

- **Priority:** 6 (same band as NM / diagnostics — see **[03 §3.3](./03-data-link-layer.md#33-priority-allocation)**)
- **Type:** PDU2 broadcast
- **Length:** 12 bytes (CAN FD single frame)
- **Transmission:** Every **Class D** or **Class H** node in a **dual-bus domain** **shall** transmit this DCID on **each** bus independently at **2 s** nominal interval (tolerance **± 500 ms**) while **Active** (per **04**). **Class S** nodes **may** transmit on their attached bus only. In **degraded single-bus** state (peer bus silent), transmission **shall** continue on the **surviving** bus with `Bus state = 3 (Degraded-Single)`; transmission on the failed bus **shall** stop until that bus returns to a usable state.
- **Purpose:** Operator-visible transceiver/controller health, duplicate-discard statistics, and wake-generation for **path redundancy** (**[17](./17-criticality-and-redundant-paths.md)**, **[03 §6](./03-data-link-layer.md#6-path-redundancy-dual-bus)**).

**Wire layout (normative):**

| Byte(s) | Field |
|---------|--------|
| 0–1 | **Sequence** — `uint16` little-endian; rolling counter per `(SA, DCID 0x0FF82)` for duplicate discard (**03** §6.4.1) |
| 2 | **BusId_WakeGen** — bits **0**: Bus ID (**0** = Bus A, **1** = Bus B); bits **4–1**: Wake generation (**0–15**), incremented on each exit from Sleep/Deep Sleep to Active (**04** §13); bits **7–5**: reserved — transmit **`0`**, ignore on receive |
| 3 | TX error counter (CAN controller; saturates at **255**) |
| 4 | RX error counter (saturates at **255**) |
| 5 | Bus-off event count since power-on (saturates at **255**) |
| 6–7 | Duplicate frames discarded since power-on (`uint16` LE, saturates at **65535**) |
| 8–9 | Missed-frame / sequence-gap count (`uint16` LE, saturates at **65535**) — informative |
| 10 | Node class: **0** = Class S, **1** = Class D, **2** = Class H (**17** / **02**) |
| 11 | Bus state: **0** = Active / Error-active; **1** = Error-passive; **2** = Bus-off; **3** = Degraded single-bus (peer bus silent per **17** §3) |

### 1.4 Time sync (DCID 0x0FF83, optional)

- **Priority:** 6  
- **Type:** PDU2 broadcast  
- **Length:** 8 bytes  
- **Transmission:** If implemented, a designated **Time Master** node (gateway, hub, or GNSS-equipped device) **shall** transmit at **1 s** nominal while Active. Receivers **may** use this to tighten `DISCARD_WINDOW` uncertainty (**03** §6.4.1 / §6.4.3). **Stream**-layer time sync remains **[IEEE 802.1AS](https://standards.ieee.org/standard/802_1AS-2020.html)** where Ethernet is present — this DCID is **Core-only**.
- **C0 recommendation:** Dual-bus domains carrying **C0** traffic per **[17 §2.1](./17-criticality-and-redundant-paths.md#21-c0--safety-critical-path)** **should** include at least one **Time Master** so that the steady-state **inter-node clock drift** `D_clk` per **[03 §6.4.3](./03-data-link-layer.md#643-discard_window-lower-bound-formula)** can be bounded at **`<= 10 ms`**, allowing the recommended `50 ms` `DISCARD_WINDOW` to be used. If no Time Master is present, the install **shall** widen `DISCARD_WINDOW` per the formula in **03 §6.4.3** and document the chosen value in the critical zone map (**[17 §6](./17-criticality-and-redundant-paths.md#6-critical-zone-map-and-conformance)**).

**Wire layout (normative):**

| Byte(s) | Field |
|---------|--------|
| 0–1 | **Sequence** — `uint16` LE per `(SA, DCID 0x0FF83)` |
| 2 | **BusId_WakeGen** — same encoding as **§1.3** byte 2 |
| 3–6 | **CoreTime** — `uint32` LE: milliseconds since UTC midnight, or monotonic millisecond counter if UTC unavailable (implementation-defined; **shall** be documented in product literature) |
| 7 | Reserved — transmit **`0x00`**, ignore on receive |

---

## 2. Compatibility DCIDs

Pelorus reuses selected DCID numbers from the **Legacy Marine Data Ecosystem** to enable seamless interoperability with existing LMDE instrumentation via gateways.

**Wire encoding:** On **LMDE**, those messages appear in **Classical CAN (CAN 2.0)** frames (8-byte data field per frame unless combined with LMDE multi-frame rules). On **Pelorus Core**, the **same numeric DCID values and field layouts** (where compatibility is claimed) are carried in **CAN FD** frames per **03**. This document registers Pelorus-side use; authoritative bit layouts for legacy messages remain in LMDE family standards.

The mapping from each DCID/field to the corresponding `Vessel.*` path in the signal catalog is maintained in `06-signal-catalog.md` and the machine-readable `catalog/vessel.vspec` file.

### Initial compatibility assignments (J1939 heritage, `DP = 0`, `R = 0`)

Pelorus wire DCIDs below reuse **SAE J1939** PDU2 PGN numbers — derivation matches **[03-data-link-layer.md §3.2](./03-data-link-layer.md#32-dcid-derivation)** (same numeric DCID on Pelorus Core CAN FD as on a classical J1939 broadcast). Bit layouts and scaling follow **SAE J1939 Digital Annex** for the cited PGNs; gateways bridging **LMDE** classical CAN SHALL preserve field semantics.

Multi-field PGNs carry several measurements in one frame; the **`Dcid`** column names the **Pelorus semantic lane** primarily associated with that PGN for catalog binding — precise signal extraction remains **DBC** / binding-table work (**06**).

| Pelorus wire DCID | J1939 PGN (dec) | Informative name | Primary Pelorus `Dcid` lane |
|---|---:|---|---|
| **0xF004** | 61444 | Electronic Engine Controller 1 | `EngineRpm` (and additional engine fields per DA) |
| **0xFEE8** | 65256 | Vehicle Heading | `HeadingTrue` |
| **0xFEC5** | 65253 | Engine Temperature 1 | `EngineCoolantTemp` (coolant among temperature fields per DA) |

### NAME field (64-bit device identity)

The **NAME** carried in Address Claimed traffic is defined **only** by **SAE J1939-81** (with **ISO 11783-5** where applicable). Pelorus **does not** specify alternate NAME bit allocations in v1.0. Procedures are normative in **[05-addressing.md](./05-addressing.md)**; Address Claimed uses DCID **0x0EE00** per **03** / **05**.

### Commanded Address (DCID 0xFED8)

Support for **Commanded Address** on Pelorus Core is **required** per **[05 §4](./05-addressing.md#4-commanded-address)**.

| Attribute | Value |
|-----------|--------|
| **Pelorus wire DCID** | **0xFED8** |
| **Purpose** | Command a node to adopt a specific source address (provisioning, fleet tools, gateway-directed binding workflows). |
| **Priority / PDU format / data field** | Per **SAE J1939 Digital Annex** for the Commanded Address message and **[03-data-link-layer.md](./03-data-link-layer.md)** framing rules. |
| **Pelorus-specific payload constraints** | **None** in v1.0 — behavior matches industry J1939 Commanded Address unless a future revision registers exceptions here. |

---

## 3. DCID Ranges and Assignment Rules

Numeric DCIDs follow derivation in **[03 §3.2](./03-data-link-layer.md#32-dcid-derivation)**. Sub-ranges inside the overall marine numeric space are allocated as follows:

1. **0x00000–0x0FF7F** — Compatibility, standard marine, vendor proprietary bands, and Pelorus protocol reservations **except** the Pelorus extension block below — subdivisions and reserved slots (address claim, transport protocol, proprietary **A** / **B** windows, etc.) are normative in **[03 §4](./03-data-link-layer.md#4-reserved-identifier-ranges)**. This document registers **which** compatibility DCIDs Pelorus uses for interoperability; bit layouts for legacy families remain in their respective standards (**§2** above).

2. **0x0FF80–0x0FFFF** — **Pelorus extensions** — assignments in **§1** of this document (**0x0FF82**, **0x0FF83** per **§1.3** / **§1.4**); **0x0FF84–0x0FF8F** reserved per **[03 §4](./03-data-link-layer.md#4-reserved-identifier-ranges)**.

3. **0x10000 and above** — Reserved for future manufacturer-specific or Pelorus **v2+** numeric allocation policy (document here when used). Shall not collide with **[03](./03-data-link-layer.md)** derivation rules.

Assignment authority: Pelorus DCIDs are allocated in this registry. Future additions require a pull request that updates this document and the corresponding entries in the signal catalog.

---

## 4. Relationship to Signal Catalog & Binding

- Every DCID field that carries an instance value is resolved to a `Vessel.*` path via the binding table (see **[06-signal-catalog.md](./06-signal-catalog.md)** §3–4).
- **v1.0:** Binding-table contents and versioning are **not** carried on **0x0FF81** NM payload bytes (**[04 §7.4](./04-power-management.md#74-nm-data-field)**). Distribution is **out of band** (gateway configuration, diagnostic session, **[Pelorus Stream](../stream/01-overview.md)**, or future NM reserved-byte allocation).
- Low-power sensors only transmit raw DCIDs; semantic mapping is handled by any binding-aware node.

---

## 5. Open Items (to be resolved before v1.0 promotion)

- Future Pelorus-native broadcast DCIDs in **`0x0FF84`–`0x0FFFF`** with payload **>= 4 bytes** **shall** include the PRH defined in **[03 §6.3](./03-data-link-layer.md#63-prh--pelorus-redundancy-header-pelorus-native-dcids)** at bytes 0–2 of the data field; assignments here will state PRH usage explicitly.
- Unify **CoreTime** (**§1.4**) with a future Pelorus-wide time scale (GNSS, **802.1AS** bridge, or `Vessel.*` signal)
- Expand compatibility DCIDs beyond **§2** initial J1939 assignments (e.g. additional propulsion, navigation, environment PGNs; NMEA2000-specific mappings via gateway profiles)
- Optional informative tables: preferred SA ranges per device class (non-normative supplement to **SAE J1939-81** NAME rules — does not replace **05** / **§2** NAME citations above)
- Whether **future** WUF / NM payloads use reserved bytes **1–7** / **2–7** for extended masks, binding hints, or authority — today reserved (**04** §7)
- Transmission rates and repetition rules for each DCID (NM cadence ratified in **04** §9.1)
- Conformance test fixtures
- Integration with the machine-readable `catalog/vessel.vspec` file

---

*This registry, together with documents 01–06, **03** §6, **17**, and **15**, supports the minimum viable Core wire and management contract.*