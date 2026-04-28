# Pelorus — architecture record

**Last Updated:** April 28, 2026  
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
- **Classical CAN ceiling**: Install base is stuck at ~250 kbit/s Classical CAN and **8-byte** frames; Pelorus Core uses **CAN FD** with **up to 64-byte** frames. Typical navigation/engine DCIDs still don’t need “more Mbps” as much as openness, power discipline, and behavior.
- **Backbone fault domain**: A linear segment is one electrical island—opens, shorts, or bad terminators can blind **everything** on that backbone.
- **Opaque diagnostics**: Little vendor-neutral tooling to capture and decode live traffic as **your** ship’s contract.
- **Vendor islands**: Optional-field gaps and proprietary extensions → cross-brand surprises and **gateway-heavy** rigs despite compatible cabling.
- **Parallel Ethernet silos**: Vendor marine Ethernet fabrics add another incompatible layer beside CAN—more cables, boxes, and translation—not one unified media plane.
- **Cadence drag**: Cert-gated evolution is slow next to automotive or IT stack velocity—features queue behind programs and committee cycles.
- **Tooling capture**: Firmware updates and deep diagnostics often depend on OEM apps, dongles, or dealer chains—not something you fully own at anchor.


---

## 3. Subsystems

### [Core](./core/01-overview.md)

**CAN FD fieldbus** for safety-critical instrumentation and controls. Application traffic is defined by Pelorus [**DCIDs**](./core/07-dcid-registry.md) — the wire contracts naming payloads and semantics on the bus—with selective wake groups and **M12 DeviceNet** physical plant.

The rest of Pelorus stacks around **Core** as the authoritative source of wired device contracts; Stream and higher layers must not degrade Core when they fail.

[**LMDE**](#lmde) and Pelorus Core are [**not** same-segment‑interoperable](./core/01-overview.md#4-coexistence-with-the-legacy-marine-data-ecosystem).

### [Stream](./stream/01-overview.md)

Ethernet **non-safety-critical** layer for bandwidth-heavy traffic: **M12** D-coded 100 Mbit/s, IPv6, discovery—a **transport** substrate, **not** an actuator or safety plane. Example use cases include:

- Bridge monitoring
- Engine diagnostics
- Voyage data recording
- Radar integration
- ECDIS connectivity
- AIS data sharing
- Remote vessel monitoring
- Smart sensor networks
- Autonomous navigation support
- Cybersecure marine networking
- Cloud data synchronization
- Fleet management analytics
- Real-time weather data integration
- Port communication systems
- Onboard IoT device integration
- CCTV and video surveillance streaming
- Docking and situational awareness cameras
- Passenger infotainment systems
- Distributed audio/music systems
- Crew communication and video conferencing

**Core stays authoritative** for safety-critical semantics.

**Core → Stream:** Telemetry and identity/metadata aligned with DCIDs are bridged onto Stream through the **standard gateway**.

**Stream → Core:** Reverse injection onto the Core fieldbus—anything originating on the Ethernet side toward CAN—is permitted **only** through a **capable bidirectional gateway**. Arbitrary Stream publishers **must not** originate traffic as Core talkers.

### [State](./state/01-overview.md)

**Pelorus State** (when specified) coordinates priorities among publishers. Stream transports payloads; it does **not** replace Core as nautical truth.

Logical **event → snapshot → situation → policy/intent** pipeline **above** Core and Stream, with **no** fieldbus or Ethernet I/O of its own.

---

## 4. Trademarks and third-party names

Pelorus is an independent open specification. **This is not legal advice**; consult counsel before shipping product packaging, marketing, or certifications that cite other organizations’ brands.

The commercial networks named as examples under [**Legacy Marine Data Ecosystem (LMDE)**](#lmde) — **NMEA 2000®**, **SeaTalk NG**, **SimNet**, **SmartCraft**, **Volvo Penta EVC**, **Yamaha Command Link**, **RayNet**, **Navico Ethernet**, **Garmin Marine Network**, **Furuno NavNet**, **NMEA OneNet®**, and the vendor / house marks referenced in those lines (including **Raymarine**, **Navico**, **Simrad**, **Lowrance**, **B&G**, **Garmin**, **Furuno**, **Volvo Penta**, **Yamaha**, **Mercury**, **MerCruiser**) — are **third-party** names cited **nominatively** to identify real-world buses and fabrics; **rights belong to their respective owners**, not Pelorus.

**Practical editorial rules for this repository:**

1. Default to [**LMDE**](#lmde) and neutral wording for the incumbent ecosystem; name specific products only when it helps the reader.
2. **Compare:** name the program **once**, then use neutral phrases for the rest of the section (**nominative fair use**).
3. **No** implied endorsement or wire-level compatibility unless a normative doc proves it. Use **“OneNet-style”** / **“N2K-class”** only for general categories of behavior.
4. **NMEA 2000®** and **NMEA OneNet®** are **NMEA** marks—spell correctly; **®** on first prominent use where counsel advises. Other names above are **third-party marks**—treat likewise.
5. **Pelorus** is **not** an **NMEA** product name and is **not** “NMEA-compatible” unless a conformance document establishes that with tests.

