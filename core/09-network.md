# Pelorus Core — Network

Network architecture, repeaters and hubs, and LMDE gateway behaviour. Single-segment limits and physical requirements are in [`02-physical.md`](./02-physical.md). Dual-bus topology and Class H rules live in [`08-redundancy.md`](./08-redundancy.md).

## 1. Single-Segment Limits

| Parameter | Limit |
|---|---|
| Backbone length | 30 m |
| Stub length | 6 m |
| Nodes per segment | 50 |
| Termination | Split termination at both ends per [`02-physical.md §5`](./02-physical.md) |
| Power | 9–32 V DC, reverse polarity protected |

These limits ensure signal integrity at 250 kbit/s arbitration / 500 kbit/s data phase without special transceivers.

## 2. Multi-Segment Networks and Repeaters

Vessels exceeding a single 30 m segment use repeater nodes to create multiple electrically isolated segments. Maximum **4 repeater hops** between any two endpoints on one Pelorus Core bus (Bus A or Bus B in a dual-bus domain counts as a separate bus for hop accounting).

### 2.1 Repeater Requirements

- **Galvanic isolation** between every pair of connected segments
- **Transparent forwarding:** every valid CAN FD frame received on one port is retransmitted on all other ports without modification — except where §3 explicitly permits a Class H hub to originate paired copies onto Bus A and Bus B for downstream Class S traffic
- **Fault containment:** a short, open, or excessive error state on one segment shall not propagate to other segments
- **Power management:** full compliance with selective wake-up and the four power states per [`04-power.md`](./04-power.md)
- **Addressing:** must successfully claim a unique source address per [`05-addressing.md`](./05-addressing.md) before forwarding application data
- **Connector:** M12 A-coded 5-pin on all ports

### 2.2 Optional Features

- Configurable filtering to reduce inter-segment traffic
- Power injection / pass-through capability
- Diagnostic LEDs or status reporting
- Web-based or local configuration interface (via gateway)

## 3. Hub (Class H) — RedBox-equivalent Behaviour

A hub provides at least two backbone ports (Bus A, Bus B) and one or more downstream Pelorus Core segment ports. It satisfies §2.1 on every port pair.

### 3.1 Downstream Class S Attachment

A Class S device on a downstream segment has a single transceiver. The hub shall receive its frames and retransmit identical CAN FD frames (same 29-bit identifier, same data field) on both Bus A and Bus B unless one backbone is declared failed (then one bus only, with operator-visible degraded mode per [`08-redundancy.md §10`](./08-redundancy.md)).

The hub shall not change the source address of replicated downstream traffic — duplicate discard uses the originator's SA.

### 3.2 Hub-Generated Management Traffic

- The hub shall transmit `Pelorus.BusHealth` on each backbone port it serves.
- The hub may implement `Pelorus.TimeSync` on one or both buses.

### 3.3 Hub Bidirectional Duplicate Discard

A hub sees the same logical frame on both Bus A and Bus B whenever an upstream Class D producer is replicating active-active. To avoid double-injection onto downstream segments, a hub shall apply duplicate discard on its backbone ingress before forwarding to downstream ports:

- Maintain a DDT keyed per [`08-redundancy.md §6.3`](./08-redundancy.md), indexed across both Bus A and Bus B inputs.
- Deliver one copy of each logical message to each downstream segment within `DISCARD_WINDOW`; the second copy received from the peer backbone shall be discarded for downstream forwarding (the duplicate counter on the hub's `Pelorus.BusHealth` for that ingress bus is incremented).
- Backbone-to-backbone forwarding is not required (each backbone already carries its own copy from the producer); a hub shall not re-inject a Bus A frame onto Bus B or vice versa for upstream replication unless the originator is a Class S device on a downstream port (in which case §3.1 applies).

This rule applies to all DCs subject to duplicate discard. Exempt DCs (address claim, multi-frame transport, `Pelorus.WakeUp`, `Pelorus.NetworkManagement`) are forwarded independently per port.

### 3.4 Hub Bus-Off and Degraded Backbone

When one backbone port enters bus-off or sustained error-passive, the hub shall:

- Continue forwarding between the surviving backbone and downstream segments so that downstream Class S devices remain reachable. This is reflected in `Pelorus.BusHealth` on the surviving bus with `Bus state = 3` (Degraded single-bus).
- For frames sourced from a downstream Class S device that the hub would normally replicate to both backbones: continue replicating onto the surviving backbone; the missed-frame counter for the failed backbone in `Pelorus.BusHealth` shall be incremented for each frame the hub could not forward there.
- Not buffer cross-forwarded frames longer than `DISCARD_WINDOW` while waiting for the failed backbone to recover; older queued frames shall be dropped.
- Resume normal active-active replication on bus return without manual intervention; duplicate discard handles transient duplicates during recovery.

## 4. Recommended Topologies

### 4.1 Small Vessels (< 30 m)

Single segment, linear bus with T-drop topology.

### 4.2 Large Vessels — Star with Central Gateway

Recommended pattern:

- Central gateway acts as the hub
- Multiple repeater nodes connect directly to the gateway
- Each repeater creates one isolated segment
- All segments are bridged through the central gateway

This minimises hops (max 2 per path), simplifies instance binding and power management, provides a natural location for the binding-table authority and web UI, and lets the gateway act as the primary network management node.

Alternative linear or tree topologies are permitted but not recommended for vessels requiring more than two segments.

## 5. Gateway

A gateway connects at least one Pelorus Core segment (CAN FD per [`02-physical.md`](./02-physical.md) / [`03-data-link.md`](./03-data-link.md)) and at least one LMDE segment (Classical CAN per industry practice). Bridging always spans that physical and framing difference; classical-only J1939 devices shall not be attached to a Pelorus Core backbone as "just another node."

In a dual-bus domain, a gateway should attach to both Bus A and Bus B with Class D-equivalent behaviour so that LMDE↔Pelorus bridging and binding distribution remain available if one Pelorus bus fails. If the gateway attaches to only one Pelorus bus, the installation shall document the residual single-bus risk for C0/C1 in the critical zone map ([`08-redundancy.md §12`](./08-redundancy.md)).

### 5.1 Gateway Roles

1. **Bridge** — translates Classical CAN (LMDE) ⟷ CAN FD (Pelorus) frame formats and forwards mapped messages between networks
2. **Binding authority** — provides the convenient UI for editing and publishing the vessel-specific binding table
3. **Network management hub** — acts as the recommended central point in star topologies and assists with power-management coordination

A vessel may have zero, one, or multiple gateways. Multiple gateways are supported with authority priority rules.

### 5.2 Bridging Requirements

- Terminate different physical layers correctly on each side (CAN FD vs Classical CAN). Translate between frame formats and multi-frame rules (LMDE Fast Packet vs Pelorus-native multi-frame transport per [`03-data-link.md §4`](./03-data-link.md)).
- For each LMDE message bridged to Core, look up the corresponding Pelorus DC via the `bridges[*]` entry in [`07-dcid-registry.md`](./07-dcid-registry.md). Emit the Pelorus CAN FD frame using the DC's Pelorus-native wire identifier and the same payload bytes; the bit layout is preserved by registry constraint so payload passes through without parsing.
- For each Pelorus DC bridged to LMDE, perform the inverse: emit the legacy CAN frame using the bridged identifier and the same payload bytes.
- Forward Pelorus protocol DCs (`Pelorus.WakeUp`, `Pelorus.NetworkManagement`, etc.) only between Pelorus segments; they have no LMDE counterpart and shall not be emitted on the LMDE side.
- Perform instance mapping using the current binding table.
- Support both directions: legacy → Pelorus and Pelorus → legacy.
- Preserve priority and timing where possible.
- Do not introduce Fast Packet or other legacy-specific transport on the Pelorus side.

### 5.3 Binding Table Management

The gateway shall:

- Maintain the authoritative copy of the binding table in non-volatile memory.
- Distribute binding-table updates **out of band** for v1.0 (configuration export/import, diagnostic session, Pelorus Stream, local UI/API).
- Provide a web-based provisioning UI for sailors to assign friendly labels and map devices.
- Detect and report instance drift or conflicts.
- Allow secondary gateways or diagnostic tools to take over as binding authority if the primary is absent.

Full fault-tolerant binding model: [`06-signal-catalog.md §3–4`](./06-signal-catalog.md).

### 5.4 Power Management Role

- Participate fully in the Pelorus power-management state machine ([`04-power.md`](./04-power.md)).
- Act as the convenient source of the current marine functional group profile.
- Re-publish the last-known profile on boot or when requested.
- Support manual override via the web UI.

The gateway is not required for power management to function — nodes fall back to their NV-stored last-known profile.

### 5.5 User Interface

- A simple web UI accessible over Wi-Fi or Ethernet
- Pages for: viewing current binding table, assigning friendly labels, editing functional group profiles, network diagnostics (node list, power states, bus statistics)
- Secure local access; no cloud dependency required

### 5.6 Fault Tolerance

- The network continues to operate fully if the gateway is powered off, failed, or disconnected.
- Nodes that need semantics cache the latest binding table and power profile.
- Secondary gateways coordinate binding authority per [`06-signal-catalog.md`](./06-signal-catalog.md).
- No hard dependency on any single physical node.

## 6. Core ↔ Stream Coupling

Pelorus Stream is non–hard-real-time-control and orthogonal to Core. Vessels integrate the two via two gateway tiers:

- **Standard gateway** — required capability to expose Core toward Stream: bridge Core-sourced telemetry, identity, and catalog-aligned metadata onto the Stream substrate (Core→Stream). The normal productised path.
- **Capable bidirectional gateway** — includes standard-gateway behaviour plus an explicitly designed, enumerated, and conformance-tested Stream→Core injection path. Ordinary Stream publishers must not originate frames on the Core fieldbus; reverse bridging only traverses this tier.

Wire formats and APIs on the Stream side of these bridges are specified under [`stream/`](../stream/); Core-side semantics remain DC- and catalog-bound.

### 6.1 Stream→Core Injection on a Dual-Bus Target

When the target Core zone is a dual-bus domain per [`08-redundancy.md`](./08-redundancy.md), a capable bidirectional gateway shall inject frames so path redundancy is preserved:

- **Pelorus-native broadcast DCs that carry a PRH:** inject the same logical frame on both Bus A and Bus B with identical SA, DC_ID, payload, and PRH sequence; the gateway maintains the rolling sequence per `(SA, DC_ID)` like any Class D producer.
- **Compatibility DCs and other application DCs without a PRH:** inject the same SA, DC_ID, DLC, and data field on both Bus A and Bus B; receivers apply payload-and-ID duplicate discard.
- **Exempt DCs** (address claim, multi-frame transport, `Pelorus.WakeUp`, `Pelorus.NetworkManagement`): each bus is treated independently; no replication coordination beyond standard gateway behaviour.
- If the gateway is attached to only one Pelorus bus in the target dual-bus domain, it shall not advertise full Stream→Core dual-bus capability; the limitation shall be documented in the critical zone map.

Stream-layer redundancy is out of scope for Core. Stream uses dual-fabric QUIC over Ethernet with IEEE 802.1AS time sync; Core only requires that Stream→Core injection respects §6.1 above.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
