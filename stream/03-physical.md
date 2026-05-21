# Pelorus Stream — Physical Layer

Connector, cabling, PoE, and dual-fabric installation. Dual-fabric runtime behaviour (state machine, RedBox, deduplication) is in [`07-redundancy.md`](./07-redundancy.md).

## 1. Connector

**M12 X-coded 8-pin per IEC 61076-2-109.** Mandatory.

- Only M12 variant supporting 4-pair Ethernet and PoE simultaneously
- Rated 10 Gbit/s
- IP67, briefly submersible
- Screw-locking — vibration-resistant
- 4 A per contact at 60 V — adequate for 802.3bt Type 3 power delivery

M12 D-coded (4-pin, 100 Mbit/s, no PoE) is rejected — it cannot carry PoE.

## 2. Power over Ethernet

**Switch (PSE):** IEEE 802.3bt Type 3 (60 W per port).
**Node (PD):** IEEE 802.3at Type 2 (30 W) minimum.

### 2.1 Switch (PSE) Requirements

| Parameter | Value |
| --- | --- |
| Input voltage (12 V systems) | 10–16 V DC |
| Input voltage (24 V systems) | 20–32 V DC |
| Input voltage (48 V systems) | 40–60 V DC |
| Reverse polarity protection | Mandatory on input |
| Transient protection | ISO 7637-2 (automotive/marine transients, load dumps, alternator spikes) |
| Brownout behaviour | Graceful port shedding in priority order (§3) |
| Operating temperature | −25 °C to +70 °C (IEC 60945) |
| Conformal coating on PCB | Mandatory |
| Power input connector | M12 (consistent with Pelorus connector family) |

### 2.2 Node (PD) Requirements

Every Class D (dual-fabric) Stream node implements **dual PoE PD** with internal power arbitration:

```
Fabric A port → PoE PD controller A → DC/DC → internal rail ─┐
                                                              ├→ Power arbiter → Node logic
Fabric B port → PoE PD controller B → DC/DC → internal rail ─┘
                (ideal diode OR / active OR controller)
```

- Port A and Port B power the node independently.
- Internal arbiter (ideal diode OR controller) selects whichever fabric is powered.
- If both fabrics are powered, both PD controllers operate; power is not doubled (diode OR prevents back-feed).
- Loss of either fabric's power does not affect node operation.
- **Startup sequencing.** Port B PoE negotiation shall be staggered ≥500 ms after Port A to prevent simultaneous inrush across all dual-PD nodes at power-on.

Class S (single-fabric) nodes implement a single PoE PD on their one connected port.

## 3. PoE Power Priority

Each switch port is assigned a priority class, configured by the installer:

| Class | Examples | Brownout behaviour |
| --- | --- | --- |
| **Critical** | ECDIS, radar processor, GPS/GNSS node | Last to shed — maintained until switch input is below minimum |
| **Standard** | Chart plotter, autopilot display, hub | Shed after Non-essential |
| **Non-essential** | Cabin displays, crew devices | First to shed |

A Class D node's Fabric A port and Fabric B port shall both be configured with the same priority class. Mismatched priority classes on redundant ports of the same device is a configuration error and shall be flagged by conformance tooling.

## 4. Dual-Fabric Installation Requirements

These are **normative installation requirements**, not recommendations.

- Fabric A switch and Fabric B switch shall be **separate physical devices**.
- Fabric A and Fabric B cabling shall follow **physically separate routes** — not the same conduit, not the same cable tray.
- Fabric A and Fabric B switches shall be powered from **separate power feeds** with separate breakers.
- PoE budget per §5, sized independently per fabric.
- Fabric A cables: **blue** sheath or labelling.
- Fabric B cables: **yellow** sheath or labelling.
- Where vessel size permits, Fabric A and Fabric B switches should be located in physically separate compartments.
- Where practical, Fabric A and Fabric B cable routes should traverse opposite sides of the vessel.

## 5. PoE Budget Sizing

### 5.1 Per-Port Allocation

Each populated port is allocated its PD's class maximum (Type 2 = 30 W; Type 3 = 60 W). Unpopulated ports = 0 W.

```
P_alloc = Σ PD_class_max(port_i)
```

### 5.2 Headroom

Switch PoE budget shall be ≥ 1.2 × `P_alloc` to absorb 802.3bt transient class events and inrush.

### 5.3 Input Current

Switch DC input shall be rated for continuous draw:

```
I_input = (P_alloc × 1.2) / (V_input_min × η)
```

`V_input_min` is the lower bound of the configured rail (10 V / 20 V / 40 V per §2.1); `η` is PSU efficiency (use 0.85 if unspecified by the switch vendor).

Worked example — 8-port switch, all Type 3 PDs, 12 V rail: `P_alloc = 480 W`; `I_input ≈ 68 A` worst case. Size input wiring and breaker accordingly. 12 V plant becomes impractical above ~4 fully-loaded Type 3 ports; 24 V or 48 V is preferred for higher port counts.

### 5.4 Critical-Class Reserve

The sum of Critical-class allocations (§3) shall fit within the switch's continuous input rating without relying on Standard or Non-essential shedding.

### 5.5 Per-Fabric Independence

A dual-PD node's Fabric A and Fabric B ports each contribute their full class allocation to their own fabric's `P_alloc`. Dual-PD does not halve per-fabric allocation (§2.2).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
