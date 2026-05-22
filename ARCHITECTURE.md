# Pelorus — architecture record

**Status:** Living (non-normative)

## 1. Project

### Mission

Open marine data network; CAN FD core; Rust-first reference code; reliability offshore.

<h3 id="lmde">Legacy Marine Data Ecosystem (LMDE)</h3>

[**LMDE**](#lmde) is this project’s umbrella term for the certification-gated, mostly **proprietary** marine connectivity landscape—CAN fieldbuses, OEM Ethernet fabrics, and connector families that dominate recreational installs. Compared to open, sailor-owned buses, it optimizes for **vendor control** and **program revenue**. Typical drawbacks:

- **Specifications and certification** are **very expensive**—or **unobtainable outright**; where shared at all, access is often gated by **mandatory NDAs** and pay-to-play programs.
- **Interoperability** commonly means **bridges and gateway SKUs**, not one open wire format from sensor to laptop.
- **Similar-looking cable plants** can carry **incompatible framing**—looks like “marine network,” behaves like silos.
- **Helm-side debuggability** is weak: the interesting signal-level truth is frequently **contractually or practically closed**.

**Pelorus** does **not** assert bit-level compatibility with any incumbent stack.

The bulleted names below are **examples** of real-world commercial buses and networks that fall under the [**LMDE**](#lmde) umbrella—**non-exhaustive**, **nominative**, and **not** normative targets for Pelorus conformance.

- **NMEA 2000®** — Multidrop CAN instrument network; certification-gated ecosystem for certified recreational marine electronics.
- **SeaTalk NG** — Raymarine NMEA-2000-class cabling and device network for integrated displays and sensors.
- **SimNet** — Simrad / Lowrance / B&G (Navico) CAN network for MFDs, sonar, and radar interfaces.
- **SmartCraft** — Mercury / MerCruiser engine and vessel digital diagnostics and data bus.
- **Volvo Penta EVC** — Volvo propulsion and vessel control digital network.
- **Yamaha Command Link** — Yamaha outboard digital instrumentation and control bus.
- **RayNet** — Raymarine high-speed Ethernet fabric (e.g. radar / plotter backhaul).
- **Navico Ethernet** — Navico-family marine Ethernet for displays, imaging, and accessories.
- **Garmin Marine Network** — Garmin plotter, sonar, and sensor integration fabric.
- **Furuno NavNet** — Furuno integrated navigation, radar, and sensor network.
- **NMEA OneNet®** — NMEA IPv6 / Ethernet application-layer family for high-bandwidth marine IP (parallel path to 2000-class CAN for many vendors).

### Presence

- [Pelorus project site](https://sevenseas.io/pelorus) — Landing page for the Pelorus open marine data network.
- [Specifications repository](https://github.com/pelorus-marine/specifications) — Source of truth for this document set; issues and changes live here.
- [Seven Seas community](https://sevenseas.io/) — Project-facing brand and wider community entry point (not limited to Pelorus).

---

## 2. Problem Pelorus targets

Weaknesses of [**Legacy Marine Data Ecosystem**](#lmde) that Pelorus addresses:

- **Specification gate**: Proprietary message catalogs; costly certification; NDAs—hard for sailors and small builders to inspect, extend, or verify independently.
- **Lab-first qualification**: Conformance is proven on the bench, not under years of salt spray, vibration, wet connectors, and RF-rich passages.
- **Always-on drain**: Without selective sleep as the norm, the suite draws continuous aggregate current even when voyage context makes much of it useless for days.
- **Classical CAN ceiling**: Install base is stuck at ~250 kbit/s Classical CAN and **8-byte** frames; Pelorus Core uses **CAN FD** with **up to 64-byte** frames. Typical navigation/engine DC_IDs still don’t need “more Mbps” as much as openness, power discipline, and behavior.
- **Backbone fault domain**: A linear segment is one electrical island—opens, shorts, or bad terminators can blind **everything** on that backbone.
- **Opaque diagnostics**: Little vendor-neutral tooling to capture and decode live traffic as **your** ship’s contract.
- **Vendor islands**: Optional-field gaps and proprietary extensions → cross-brand surprises and **gateway-heavy** rigs despite compatible cabling.
- **Parallel Ethernet silos**: Vendor marine Ethernet fabrics add another incompatible layer beside CAN—more cables, boxes, and translation—not one unified media plane.
- **Cadence drag**: Cert-gated evolution is slow next to automotive or IT stack velocity—features queue behind programs and committee cycles.
- **Tooling capture**: Firmware updates and deep diagnostics often depend on OEM apps, dongles, or dealer chains—not something you fully own at anchor.

### Root cause and durable answer

The bullets above are not independent technical failures; they are downstream effects of a single root cause. Each fix — CAN FD frames, managed sleep, split termination with EMC capacitor, free manufacturer codes, open conformance — has been industry practice in adjacent fields (automotive, industrial) for a decade or more. None have landed in marine networking because the governance model requires consensus among the large manufacturers who already shipped product against the older specification and have no commercial incentive to obsolete their own installed base.

Pelorus' technical decisions catch up to current practice. The **governance model** — CC BY 4.0 specification, free hobbyist manufacturer codes, no certification gate, public reference implementations and test fixtures — is what keeps the stack from falling behind again the next time the underlying technology moves. Closed governance is the upstream defect; the technical symptoms above are what it costs.

---

## 3. Subsystems

### [Core](./core/01-overview.md)

**CAN FD fieldbus** for safety-critical instrumentation and controls. Application traffic is defined by Pelorus [**Data Contracts (DC_IDs)**](./core/07-dcid-registry.md) — Pelorus-owned, named definitions with their own numeric DC_ID, payload bit layout, priority, and optional bridges to legacy J1939 / NMEA 2000 identifiers. Selective wake groups and **M12 A-coded 5-pin** (per IEC 61076-2-101, identical to LMDE micro) physical plant.

**Path redundancy:** Where **[criticality class](./core/08-redundancy.md)** **C0** or **C1** requires it, Pelorus Core uses **dual** independent CAN FD buses (**Bus A** / **Bus B**) with active-active replication and receiver duplicate discard (**[08 §6](./core/08-redundancy.md#6-active-active-transmission-and-duplicate-discard)**). That is **orthogonal** to **repeater segmentation** (length and fault containment). **Reliability and durability** are ordered ahead of install convenience when they conflict (**[01 §6](./core/01-overview.md#6-design-principles)**).

The rest of Pelorus stacks around **Core** as the authoritative source of wired device contracts; Stream and higher layers must not degrade Core when they fail.

[**LMDE**](#lmde) and Pelorus Core are [**not** same-segment‑interoperable](./core/01-overview.md#4-coexistence-with-lmde).

### [Stream](./stream/01-overview.md)

Ethernet **non-safety-critical** layer for bandwidth-heavy traffic: **M12 X-coded 8-pin** (per IEC 61076-2-109 with 802.3bt PoE), IPv6, discovery—a **transport** substrate, **not** an actuator or safety plane. Example use cases include:

- Bridge monitoring
- Engine diagnostics
- Voyage data recording
- Radar video and control
- ECDIS connectivity
- S-100 chart distribution
- High-rate navigation data
- Stream health monitoring
- CCTV and video surveillance streaming
- Docking and situational awareness cameras

AIS targets are low-rate instrument data and live on Pelorus Core, not Stream (see [`stream/01-overview.md §1`](./stream/01-overview.md) and [`core/07-dcid-registry.md`](./core/07-dcid-registry.md)). Cabin audio, intercom, and entertainment are out of scope ([`stream/01-overview.md §1`](./stream/01-overview.md)).

**Core stays authoritative** for safety-critical semantics.

**Core → Stream:** Telemetry and identity/metadata aligned with DC_IDs are bridged onto Stream through the **standard gateway**.

**Stream → Core:** Reverse injection onto the Core fieldbus—anything originating on the Ethernet side toward CAN—is permitted **only** through a **capable bidirectional gateway**. Arbitrary Stream publishers **must not** originate traffic as Core talkers.

### [State](./state/01-overview.md)

**Pelorus State** (when specified) coordinates priorities among publishers. Stream transports payloads; it does **not** replace Core as nautical truth.

Logical **ingest → snapshot → situation → policy/intent** pipeline **above** Core and Stream, with **no** fieldbus or Ethernet I/O of its own.

### [Catalog](./catalog/01-overview.md)

**Pelorus Catalog** is shared, protocol-agnostic vocabulary — the `Vessel.*` COVESA-VSS tree consumed by Core, Stream, and State. Not a subsystem in the transport sense; **infrastructure used by all three**. Core's Data Contracts project to catalog leaves via the `data_contract` overlay; Stream telemetry keys CBOR maps by `Vessel.*` paths; State imports the same paths as its semantic input. The catalog defines **meaning** (names, types, units, instance handling); it does **not** define wire encoding, transport, or arbitration.

For a worked example of how Core and Stream sit together on a real vessel, see [§4 Sample combined network](#4-sample-combined-network--40-ft-sailing-yacht).

---

## 4. Sample combined network — 40 ft sailing yacht

This section is **non-normative**. It illustrates how the pieces in §3 fit together on a representative cruising sailing yacht (~40 ft / ~12 m). Numbers — counts of sensors, exact bus lengths, gateway placement — are illustrative; a real install is captured in a **critical zone map** per [`core/08 §12`](./core/08-redundancy.md#12-critical-zone-map-and-conformance).

### 4.1 Vessel context and choices

- **Legacy NMEA 2000 instruments and autopilot stay where they are.** Wind, depth, AIS, heading, and the autopilot all arrived with the boat on a single classical CAN backbone; replacing them would mean re-rigging the masthead, re-pulling cable, and re-installing the rudder feedback loop, so they live on as the [**LMDE**](#lmde) side of the install. The classical heading-sensor-to-autopilot loop runs entirely on the LMDE bus and does **not** depend on Pelorus Core.
- **Modern navigation runs on Pelorus Core.** The chart-and-plot station is **dual-bus C0** end-to-end ([`core/08 §2.1`](./core/08-redundancy.md#21-c0--safety-critical-path)): two ECDIS plotters and the primary GNSS all drop onto Bus A and Bus B as Class D nodes, so the helm survives a single-bus failure without operator action.
- **The Pelorus Gateway is the only crossing point.** It is a **standard** gateway tier ([`core/01 §4`](./core/01-overview.md#4-coexistence-with-lmde)) that reads NMEA 2000 PGNs from the LMDE backbone and republishes them as Pelorus Data Contracts on Bus A and Bus B. It is **Class D**, so it remains reachable on whichever Pelorus Core bus survives. Pelorus Core publishers do **not** write back onto LMDE — in this MVP the autopilot follows the LMDE heading sensor, not ECDIS route guidance.
- **No Pelorus Stream on this snapshot.** Cameras, NAS, bridge tablet, and chart distribution are out of scope here — the focus is the LMDE → Core bridge and the Core redundancy story. A real install typically pairs Pelorus Core with a Pelorus Stream switch ([`stream/01-overview.md`](./stream/01-overview.md)).

### 4.2 Topology diagram

![Sample combined network topology — LMDE bridged into Pelorus Core](./diagrams/topology.drawio.svg)

> Diagram source: [`diagrams/topology.drawio.svg`](./diagrams/topology.drawio.svg). The file follows the `.drawio.svg` convention — a regular SVG image that is also editable in [draw.io](https://app.diagrams.net) (File → Open from device, or drag-drop). Plain text, lives alongside the spec, edits diff cleanly in git. The drawio source is embedded in the SVG's `content` attribute for full round-trip fidelity.
>
> **Legend.** The horizontal bar at the top is the single **NMEA 2000** (classical CAN) backbone of the **LMDE** group; five drops hang off it — Wind, Depth, AIS above, Autopilot and Heading below. The two vertical bars at the bottom are **Bus A** and **Bus B** of the **Pelorus Core** redundant CAN FD network; ECDIS 1, ECDIS 2, and Position (GNSS) sit between them, each with a drop onto **both** buses. The **Pelorus Gateway** drops onto the NMEA 2000 backbone and onto both Pelorus Core buses simultaneously, and is the only crossing point between LMDE and Pelorus Core. Reverse injection from Pelorus Core toward LMDE is out of scope for a standard gateway tier — see LMDE coexistence in [`core/01 §4`](./core/01-overview.md#4-coexistence-with-lmde).

### 4.3 Walk-through — LMDE NMEA 2000 side

The LMDE backbone is a single classical CAN bus carrying three instrument drops. None of these talkers are Pelorus-native; they originate as NMEA 2000 PGNs and only enter Pelorus Data Contracts via the **Pelorus Gateway**.

| Drop | What it does |
| --- | --- |
| **Wind** | Apparent / true wind angle and speed from a masthead anemometer. Typical N2K cadence ~1–4 Hz. |
| **Depth** | Depth below transducer (and optionally water temperature) from a hull-mounted sounder. ~1 Hz. |
| **AIS** | AIS receiver / transponder publishing target reports onto the bus. AIS belongs on Pelorus Core in a Pelorus-native install ([`core/07`](./core/07-dcid-registry.md)); here it sits on LMDE because the installed transponder is N2K-only and is bridged. |
| **Heading** | Heading reference (fluxgate or rate-compensated) feeding the LMDE autopilot directly via the bus, and bridged into Pelorus Core so the ECDIS plotters can render it. ~10 Hz. |
| **Autopilot** | Closes the rudder loop on the LMDE heading sensor. The classical autopilot loop is **wholly contained on the NMEA 2000 backbone** — it does not depend on Pelorus Core. ECDIS-driven route following would require a capable bidirectional gateway and is out of scope for this MVP. |

The backbone follows whatever cable plant the installed vendor prescribes (micro-C is most common). Pelorus does **not** assert bit-level compatibility with NMEA 2000 — LMDE and Pelorus Core are [not same-segment-interoperable](./core/01-overview.md#4-coexistence-with-lmde); the gateway is the only crossing point.

### 4.4 Walk-through — Pelorus Core side

Pelorus Core runs as **dual independent CAN FD** buses (**Bus A** and **Bus B**) with active-active replication and receiver duplicate discard ([`core/08 §6`](./core/08-redundancy.md#6-active-active-transmission-and-duplicate-discard)). Every device on this side is **Class D** (dual transceiver) and drops onto **both** buses.

| Drop | Class | Criticality | What it does |
| --- | --- | --- | --- |
| **ECDIS 1** | **D** | **C0** | Primary electronic chart and route display. Active chartplotter under normal conditions. |
| **ECDIS 2** | **D** | **C0** | Hot-standby chartplotter; consumes the same DC_IDs from Bus A and Bus B and is one helm action away from becoming primary. |
| **Position (GNSS)** | **D** | **C0** | Primary nav source; broadcasts position, COG/SOG, and heading active-active on Bus A and Bus B with **PRH** on Pelorus-native broadcast DC_IDs ([`core/08 §6`](./core/08-redundancy.md#6-active-active-transmission-and-duplicate-discard)). |
| **Pelorus Gateway** | **D** | spans C0 + LMDE | Bridges the LMDE NMEA 2000 backbone into Pelorus Core; drops onto Bus A and Bus B so it remains reachable through a single-bus failure ([`core/01 §4`](./core/01-overview.md#4-coexistence-with-lmde)). |

**Selected DC_IDs in play** ([`core/07`](./core/07-dcid-registry.md)):

- **0x00003** — Bus Health, transmitted by every Class D node on each bus at 2 s ± 500 ms.
- **0x00004** — Time Sync (optional). Recommended here because the whole Pelorus Core fabric is C0; the gateway acts as Time Master so `D_clk ≤ 10 ms` and the receiver `DISCARD_WINDOW = 50 ms` is sufficient ([`core/08 §6.3.3`](./core/08-redundancy.md#633-discard_window-lower-bound)).
- **Bridged DC_IDs** for wind, depth, AIS, and heading — the gateway maps incoming NMEA 2000 PGNs to Pelorus-native or compatibility DC_IDs and rebroadcasts them active-active on Bus A and Bus B ([`core/07 §2`](./core/07-dcid-registry.md)). The autopilot is **not** bridged onto Pelorus Core — it is a closed loop on the LMDE side.

### 4.5 What happens during a single-bus failure on Pelorus Core

1. A connector on **Bus B** opens between the gateway and the ECDIS pair.
2. Class D nodes continue transmitting active-active; receivers downstream of the break now see only **Bus A** copies.
3. The **Duplicate Discard Table** on each receiver was already accepting one copy per logical frame, so application delivery is uninterrupted — no message gap larger than `DISCARD_WINDOW + max(producer_period)` ([`core/08 §10.1`](./core/08-redundancy.md#101-failover-convergence-c0--c1)).
4. Bus Health on Bus A reports `Bus state = Degraded-Single`; both ECDIS plotters surface an operator-visible annunciator within 5 s ([`core/08 §10`](./core/08-redundancy.md#10-degraded-mode-behaviour)).
5. The LMDE side is **unaffected** — its NMEA 2000 backbone is electrically separate. Bridged wind, depth, AIS, and heading traffic still arrives via the surviving Pelorus Core bus through the gateway, and the **classical LMDE heading-to-autopilot loop continues uninterrupted** because it never depended on Pelorus Core in the first place.
6. When the connector is repaired, Bus B returns. Receivers accept frames on the returning bus and apply normal duplicate discard; no re-sync handshake ([`core/08 §10.2`](./core/08-redundancy.md#102-bus-return-after-failure)).

### 4.6 What this example deliberately does **not** show

- **Pelorus Stream** — no Ethernet substrate is depicted; cameras, NAS, bridge tablet, and chart distribution are out of scope for this snapshot. Real installs typically pair Pelorus Core with a Pelorus Stream switch ([`stream/01-overview.md`](./stream/01-overview.md)).
- **Engine, tanks, battery, lighting** — a real cruiser carries more talkers; this example focuses on the navigation cluster to keep the LMDE bridge and Core redundancy story visible.
- **Capable bidirectional gateway** — the gateway here is the **standard** tier: LMDE → Pelorus Core for bridged instrument data, authoritative metadata on the Core side. Pelorus Core publishers do **not** write back onto the LMDE bus, so **ECDIS-driven route following on the LMDE autopilot is not available in this MVP** — the autopilot operates in heading-hold (or manual follow-up) from the LMDE heading sensor. Adding a capable bidirectional gateway is a future option, gated by [`core/01 §4`](./core/01-overview.md#4-coexistence-with-lmde).
- **Higher data rates** — v1.0 is a single named profile (250 / 500 kbit/s, 30 m / 6 m / 50 nodes per bus) per [`core/08 §4.4`](./core/08-redundancy.md#44-bit-rate-and-length-scope). Higher rates will arrive as additional named profiles, not via a generic length-vs-rate scaling.

---

## 5. Trademarks and third-party names

Pelorus is an independent open specification. **This is not legal advice**; consult counsel before shipping product packaging, marketing, or certifications that cite other organizations’ brands.

The commercial networks named as examples under [**Legacy Marine Data Ecosystem (LMDE)**](#lmde) — **NMEA 2000®**, **SeaTalk NG**, **SimNet**, **SmartCraft**, **Volvo Penta EVC**, **Yamaha Command Link**, **RayNet**, **Navico Ethernet**, **Garmin Marine Network**, **Furuno NavNet**, **NMEA OneNet®**, and the vendor / house marks referenced in those lines (including **Raymarine**, **Navico**, **Simrad**, **Lowrance**, **B&G**, **Garmin**, **Furuno**, **Volvo Penta**, **Yamaha**, **Mercury**, **MerCruiser**) — are **third-party** names cited **nominatively** to identify real-world buses and fabrics; **rights belong to their respective owners**, not Pelorus.

**Practical editorial rules for this repository:**

1. Default to [**LMDE**](#lmde) and neutral wording for the incumbent ecosystem; name specific products only when it helps the reader.
2. **Compare:** name the program **once**, then use neutral phrases for the rest of the section (**nominative fair use**).
3. **No** implied endorsement or wire-level compatibility unless a normative doc proves it. Use **“OneNet-style”** / **“N2K-class”** only for general categories of behavior.
4. **NMEA 2000®** and **NMEA OneNet®** are **NMEA** marks—spell correctly; **®** on first prominent use where counsel advises. Other names above are **third-party marks**—treat likewise.
5. **Pelorus** is **not** an **NMEA** product name and is **not** “NMEA-compatible” unless a conformance document establishes that with tests.

