# Pelorus Core — Criticality, Path Redundancy, and Dual-Bus Domains

The single source of truth for criticality classes, dual-bus operation, duplicate discard, and the Data Contracts (DCs) that support them. Single-bus segment limits and connector rules are in [`02-physical.md`](./02-physical.md). Identifier layout and frame format are in [`03-data-link.md`](./03-data-link.md). Address claiming is in [`05-addressing.md`](./05-addressing.md). Wake-up and NM behaviour are in [`04-power.md`](./04-power.md).

**Principle ordering.** Reliability and durability take precedence over ease of installation when the two conflict. Installers and manufacturers shall not omit path redundancy, separate routing, or declared degraded-mode behaviour for C0 or C1 solely to reduce install time or cable count, unless this document explicitly allows a single-bus exception.

## 1. Definitions

| Term | Definition |
|---|---|
| **Path redundancy** | Two electrically independent Pelorus Core CAN FD media (Bus A, Bus B) carrying the same logical application traffic active-active, with receivers accepting one copy per logical frame. |
| **Dual-bus domain** | A bounded installation region (functional zone, compartment group, or entire small vessel) where this document requires both Bus A and Bus B to be present and terminated per [`02-physical.md`](./02-physical.md). |
| **Critical zone map** | A written or machine-readable record, prepared at commissioning or product certification, listing each Pelorus-attached function with its criticality class (§2) and node class (§3). |
| **Segmentation** | Repeater-based electrical isolation between length-scaled segments per [`09-network.md`](./09-network.md). Orthogonal to path redundancy. |
| **Common-mode fault** | A fault that affects both Bus A and Bus B together (shared power loss, both cables in one bundle severed, identical firmware bug). Path redundancy does not eliminate common-mode risk; §10 mitigations apply. |

## 2. Criticality Classes

### 2.1 C0 — Safety-critical path

Functions whose loss or corruption can imminently compromise vessel control, collision avoidance, or crew safety.

Examples: autopilot demand/feedback loop on the same bus as the actuator interface; steering angle or rudder feedback used for closed-loop helm; engine/propulsion alarm and shutdown paths required by operational policy; bilge flood alarm where wired on Core.

Requirements:

- Shall be installed in a dual-bus domain with Class D nodes (or Class H serving Class S downstream) for every Core-attached device on that path, unless a documented single-bus exception is approved in the critical zone map with operator-visible continuous indication of degraded single-bus operation.
- Shall meet §10 minimum physical/electrical diversity where practical.
- Shall expose degraded-mode behaviour per §9 if only one bus remains serviceable.

### 2.2 C1 — Mission-critical

Functions whose loss degrades navigation or propulsion decision-making, but is not an immediate safety loss equivalent to C0 (e.g. loss of primary GNSS when a verified secondary position source exists off-bus).

Examples: primary heading, wind, and depth for primary helm display when no independent redundant sensor exists; primary gateway binding authority channel (Core side) when no secondary authority exists.

Requirements:

- Shall be installed in a dual-bus domain with Class D or Class H+Class S arrangements unless the critical zone map documents equivalent off-bus redundancy (e.g. duplicate sensor on Stream with qualified voting) and operator-visible indication when Core path is single-bus.
- Shall meet §9 degraded-mode rules when one bus fails.

### 2.3 C2 — Non-critical

Comfort, logging-only, or ancillary functions where loss does not materially affect safe navigation.

Examples: tank levels for non-safety tanks; saloon lighting state; non-primary displays.

Requirements:

- May use a single Pelorus Core bus (Class S) without path redundancy.
- Shall not be used to carry C0 or C1 traffic without upgrading the zone to C0/C1 rules.

### 2.4 Assignment Authority

The manufacturer (for a fixed product) or installer/integrator (for a vessel-specific fit-out) shall assign each Core-attached function to exactly one of C0, C1, or C2 in the critical zone map. Down-classifying C0 traffic to C2 to avoid dual-bus cost is non-conformant.

## 3. Node and Port Classes

| Class | Meaning |
|---|---|
| **Class S** | Single CAN FD transceiver; attaches to one of Bus A or Bus B only. Permitted for C2 and for C1 when §2.2 off-bus redundancy is documented. |
| **Class D** | Dual transceivers; attaches to both Bus A and Bus B. Target for new C0/C1 sensors, actuators, and displays in dual-bus domains. |
| **Class H** | Hub / RedBox; bridges Class S downstream segments onto both backbone buses with correct replication and sequence/bus-ID rules. Detail in [`09-network.md`](./09-network.md). |

## 4. Physical Requirements (Bus A and Bus B)

### 4.1 Electrical Independence

- Bus A and Bus B shall be separate CAN_H/CAN_L pairs, each with its own split termination per [`02-physical.md §5`](./02-physical.md).
- Neither bus shall share a single two-wire pair with the other; Class D devices shall use two independent transceivers (or an integrated dual-transceiver solution meeting the same isolation goals).

### 4.2 Segment Limits

- Each of Bus A and Bus B shall observe the same per-segment limits as a single Pelorus backbone ([`02-physical.md §4`](./02-physical.md)).
- Repeaters apply per bus; hop counts and lengths are not shared across Bus A and Bus B.

### 4.3 Connector Strategy (v1.0)

Class D nodes use **two M12 A-coded 5-pin connectors**, labeled **A** and **B** adjacent to each connector body. A future single-connector dual-bus pinout may be added without removing the two-port option; until ratified, single-connector Class D is not Pelorus Core conformant.

### 4.4 Bit-Rate and Length Scope

Pelorus Core v1.0 specifies one profile: 250 kbit/s arbitration / 500 kbit/s data, 30 m maximum backbone per segment, 6 m maximum stub, 50 nodes maximum per segment. These limits derive from the stub-loaded LMDE Micro topology and CAN FD signal-integrity headroom on that physical plant.

### 4.5 Patent / IP Notice

Active-active dual-CAN-FD redundancy and SYNC-based active/backup CAN-FD redundancy variants are an active patent area. The Pelorus Core design specified here is **active-active without a SYNC channel**, intended to avoid known SYNC-based constructs. This notice is informational; it does not constitute legal advice. Implementers planning commercial Pelorus-conformant hardware that implements path redundancy should perform their own freedom-to-operate review, in addition to the selective wake-up patent review required by [`04-power.md §1`](./04-power.md).

## 5. Address Claiming on Dual Buses

For Class D and Class H nodes:

1. **Simultaneous claim.** On power-up, reset, or join, a Class D node shall run the [`05-addressing.md §3`](./05-addressing.md) procedure on both buses in parallel (same preferred SA and same NAME on A and B).
2. **Data transmission gate.** The node shall not transmit application DCs (other than address-management traffic and `Pelorus.BusHealth` / `Pelorus.TimeSync` per §8) on either bus until it has successfully claimed the same SA on both buses, unless it enters degraded single-bus mode per §9 (operator-visible fault; continues on the surviving bus only).
3. **Conflict asymmetry.** If claiming succeeds on Bus A but fails on Bus B, the node shall either (a) select a new SA and re-claim on both buses from step 1, or (b) declare degraded single-bus on A and shall not transmit on B until B succeeds.
4. **Class H hubs** shall claim a unique SA on each bus segment they terminate; downstream Class S devices use normal claiming on their single attached segment — the hub performs replication onto both backbones.

Address-claim and Commanded Address frames shall not be subject to duplicate discard (§6.2).

## 6. Active-Active Transmission and Duplicate Discard

### 6.1 Normative Goals

- **Active-active.** Producers on Class D or Class H nodes shall transmit the same logical frame on both buses (same 29-bit identifier and same data field), subject to §7 PRH bus-ID bit where applicable.
- **Single logical delivery.** Receivers shall apply duplicate discard so each logical message is delivered to the application layer once per transmission instant.

### 6.2 Exemptions

The following shall not pass through duplicate discard (each bus is processed independently for network-management correctness):

- `Pelorus.AddressClaim` and all address-management traffic (including `Pelorus.AddressCommand` and any future address-management DCs).
- Multi-frame transport frames (`Pelorus.MultiFrameControl`, `Pelorus.MultiFrameData`) — reassembly per [`03-data-link.md §4`](./03-data-link.md) on each bus independently until a future revision defines multi-frame-level deduplication.
- `Pelorus.WakeUp` and `Pelorus.NetworkManagement` frames — processed independently on each bus.

### 6.3 Duplicate Discard Algorithm

Receivers in a dual-bus domain shall maintain a Duplicate Discard Table (DDT) with at most one entry per source address `S` that has been heard on either bus.

#### 6.3.1 Entries Keyed with PRH (DCs with §7 header)

For each received frame from source `S` with PRH sequence `N` received on bus `B`:

1. If no DDT entry for `S` for this DC_ID: **accept**; create entry `(S, DC_ID, N, B, now)`.
2. If entry exists:
   - If `N == entry.last_sequence` and `(now − entry.last_seen_time) < DISCARD_WINDOW`: **discard**.
   - Else: **accept**; update entry to `(S, DC_ID, N, B, now)`.

| Parameter | Value | Notes |
|---|---|---|
| `DISCARD_WINDOW` | 50 ms | Minimum; per-installation value shall satisfy §6.3.3. |
| `NODE_FORGET_TIME` | 60 s | Remove entry if no frame from `(S, DC_ID)` on either bus. |

#### 6.3.2 Compatibility and Other Application DCs (no PRH)

For frames without a PRH (including all §2 compatibility layouts in [`07-dcid-registry.md`](./07-dcid-registry.md) where Digital Annex byte positions are preserved), receivers shall use **payload-and-ID duplicate discard**:

- Key: `(S, DC_ID, DLC, data[0..DLC-1])`.
- On receive on bus `B`: if an entry exists for the same key from the other bus within `DISCARD_WINDOW`, **discard**; else **accept** and record `(key, B, now)`.

If an application requires identical back-to-back payloads faster than `DISCARD_WINDOW`, it shall use a DC that carries an explicit sequence (Pelorus-native DC with PRH).

#### 6.3.3 `DISCARD_WINDOW` Lower Bound

For an installation with maximum `H` repeater/hub hops between any producer and any consumer on either bus, declared per-hop maximum forwarding latency `L_hop`, and bounded inter-node clock drift `D_clk`:

```
DISCARD_WINDOW >= 2 * H * L_hop  +  2 * D_clk  +  safety_margin
```

with default `safety_margin = 10 ms`. The 50 ms value above is the absolute floor; deeper or higher-drift installations shall use a larger value and document it in the critical zone map.

`D_clk` is derived from the most recent `Pelorus.TimeSync` observed on either bus (§8.2):

| `TimeSync` state observed | `D_clk` to use |
|---|---|
| `SourceClass ∈ {1,2,3,4}` **and** `AccBucket ≤ 4` | 10 ms — recommended floor `DISCARD_WINDOW = 50 ms` is sufficient. |
| `SourceClass ∈ {1,2,3,4}` **and** `AccBucket = 5` | 100 ms — apply formula. |
| `SourceClass ∈ {1,2,3,4}` **and** `AccBucket ∈ {6, 7}` | declared worst-case (record in critical zone map) — apply formula. |
| `SourceClass ∈ {0, 5, 6}`, or no `Pelorus.TimeSync` observed within the last 5 s | declared worst-case — apply formula. |

The 50 ms value above remains the absolute floor for `DISCARD_WINDOW`; installations shall not go below it even when `AccBucket = 0`.

### 6.4 Multi-Frame Transport and Duplicate Discard

Until a future revision specifies multi-frame-level deduplication, receivers shall run [`03-data-link.md §4`](./03-data-link.md) reassembly independently per bus for multi-frame traffic. Application consumers should merge only after complete reassembly and may treat identical completed messages from A and B within `DISCARD_WINDOW` as one logical delivery.

## 7. Pelorus Redundancy Header (PRH)

A fixed 3-byte preamble used at the start of the CAN FD data field for Pelorus-native broadcast DCs that participate in path redundancy.

| Byte(s) | Field |
|---|---|
| 0–1 | **Sequence** — `uint16` little-endian; rolling counter per `(SA, DC_ID)`. |
| 2 | **BusId_WakeGen** — bit 0: Bus ID (0 = Bus A, 1 = Bus B); bits 4–1: Wake generation (0–15, see §9); bits 7–5: reserved — transmit `0`, ignore on receive. |

**Scope:**

- `Pelorus.BusHealth` and `Pelorus.TimeSync` shall use this PRH.
- Any future Pelorus-native broadcast DC with payload ≥ 4 bytes shall include this PRH at bytes 0–2 before its application fields. Pelorus-native DCs with payload < 4 bytes may omit the PRH (such DCs use §6.3.2 payload-and-ID dedup).
- Compatibility DCs (J1939 / NMEA-2000 heritage) shall not carry a PRH; they are deduplicated via §6.3.2.
- Protocol DCs `Pelorus.WakeUp`, `Pelorus.NetworkManagement`, `Pelorus.AddressClaim`, `Pelorus.AddressCommand`, `Pelorus.Request`, `Pelorus.MultiFrameControl`, `Pelorus.MultiFrameData` shall not carry a PRH; they are exempt from duplicate discard (§6.2).

## 8. Dual-Bus Data Contracts

### 8.1 `Pelorus.BusHealth`

| Attribute | Value |
|---|---|
| DC_ID | `0x00003` |
| Priority | 6 (NM/diagnostics band) |
| Length | 12 bytes |
| Transmission | Every Class D or Class H node in a dual-bus domain shall transmit on each bus independently at 2 s nominal (±500 ms) while Active. Class S may transmit on its attached bus only. In degraded single-bus state, transmission continues on the surviving bus with `Bus state = 3`; transmission on the failed bus stops until that bus returns. |

| Byte(s) | Field |
|---|---|
| 0–1 | Sequence — `uint16` LE; rolling counter per `(SA, Pelorus.BusHealth)` for duplicate discard |
| 2 | BusId_WakeGen — see §7 |
| 3 | TX error counter (CAN controller; saturates at 255) |
| 4 | RX error counter (saturates at 255) |
| 5 | Bus-off event count since power-on (saturates at 255) |
| 6–7 | Duplicate frames discarded since power-on (`uint16` LE, saturates at 65535) |
| 8–9 | Missed-frame / sequence-gap count (`uint16` LE, saturates at 65535) — informative |
| 10 | Node class: 0 = Class S, 1 = Class D, 2 = Class H |
| 11 | Bus state: 0 = Active/Error-active; 1 = Error-passive; 2 = Bus-off; 3 = Degraded single-bus |

### 8.2 `Pelorus.TimeSync`

`Pelorus.TimeSync` carries the vessel-wide time reference and the machine-readable trust level of its current source. It bounds inter-node clock drift `D_clk` for §6.3.3 duplicate discard, provides UTC for AIS, voyage data recording, anchor watch, and alarm logs, and lets safety-critical consumers gate their behaviour on the present quality of the clock (locked / authenticated / holdover / spoof-suspected).

| Attribute | Value |
|---|---|
| DC_ID | `0x00004` |
| Priority | 6 |
| Length | 8 bytes |
| Transmission | The Time Master shall transmit on each bus at 1 s nominal (±100 ms) while Active. |
| Scope | **Mandatory** in any dual-bus domain. **Optional** in single-bus C2-only domains. |

Stream-layer time sync remains IEEE 802.1AS where Ethernet is present — `Pelorus.TimeSync` is Core-only. A Stream/Core gateway disciplining the Core side from an 802.1AS grandmaster shall reflect that origin in `TimeStatus.SourceClass = 6` (bridged).

#### 8.2.1 Time Master Election

Exactly one node in a dual-bus domain shall act as Time Master at any time. Candidates are Class D or Class H nodes whose application includes a clock source (GNSS receiver, terrestrial timing receiver, Stream/802.1AS slave port, or operator entry).

Each candidate computes the tuple `(SourceClass, AccBucket, SA)` from its current state (§8.2.3). The candidate with the lexicographically lowest tuple wins; ties are broken by lowest SA.

A candidate becomes Time Master after observing no better candidate transmitting `Pelorus.TimeSync` for 3 s, and shall yield within 1 s of observing a better candidate transmitting on either bus. Candidates with no usable clock source (`SourceClass = 0` after their 3 s power-up grace per §8.2.4) shall not transmit `Pelorus.TimeSync`.

The standard gateway is the default Time Master holder when no GNSS-equipped Class D node is present; it sources its clock from the LMDE side, an external NTP/PTP feed on Stream, an internal disciplined oscillator, or operator entry, and declares the resulting `SourceClass` honestly.

#### 8.2.2 Wire Layout

| Byte(s) | Field |
|---|---|
| 0–1 | Sequence — `uint16` LE per `(SA, Pelorus.TimeSync)` |
| 2 | BusId_WakeGen — see §7 |
| 3–6 | CoreTime — `uint32` LE; interpretation depends on `TimeStatus.SourceClass` (§8.2.3) |
| 7 | TimeStatus — see §8.2.3 |

#### 8.2.3 `CoreTime` and `TimeStatus` Encoding

`CoreTime` is interpreted according to `TimeStatus.SourceClass`:

- `SourceClass ∈ {1, 2, 3, 4, 6}` — milliseconds since UTC midnight. Range `0`–`86_399_999`, extended to `86_400_999` when `LeapPending = 1` and a positive leap second is in progress at 23:59:60 UTC.
- `SourceClass ∈ {0, 5}` — monotonic millisecond counter (epoch implementation-defined). Consumers shall not interpret as wall-clock UTC.

`TimeStatus` (byte 7) packs four fields. Bit 0 is the LSB:

```
 bit │  7  │  6  │  5    4    3  │  2    1    0
     │  L  │  S  │   AccBucket   │   SourceClass
```

| Field | Bits | Definition |
|---|---|---|
| `SourceClass` | 0–2 | Trust class of the current time source — enum below. |
| `AccBucket` | 3–5 | Coarse current UTC offset bound — enum below. |
| `SpoofSuspect` (S) | 6 | `0` = receiver reports no anomaly; `1` = receiver-level spoofing/jamming indication, peer cross-check disagreement, or Master cannot self-assess. |
| `LeapPending` (L) | 7 | `0` = no leap second announced for the current UTC day; `1` = leap second announced. |

**`SourceClass` (3 bits, 8 slots):**

| Value | Meaning |
|---:|---|
| `0` | Free-running — no UTC ever acquired. `CoreTime` is monotonic. |
| `1` | GNSS-disciplined, currently locked. |
| `2` | GNSS-disciplined **and** cryptographically authenticated (Galileo OSNMA, GPS M-code, GPS Chimera, or equivalent). |
| `3` | Terrestrial-disciplined (eLoran, R-Mode, terrestrial DGNSS time reference). |
| `4` | Holdover — previously disciplined, currently flywheeling on local oscillator. |
| `5` | Operator-set — manually entered, or set from a non-disciplined source. |
| `6` | Bridged — clock acquired via a gateway from another Pelorus domain (Stream/802.1AS, peer Core domain). Trust is inherited from the upstream domain. |
| `7` | Reserved. |

**`AccBucket` (3 bits, 8 slots)** — current estimated UTC offset of the Time Master, **not** nameplate accuracy:

| Value | Bound |
|---:|---|
| `0` | ≤ 1 μs |
| `1` | ≤ 10 μs |
| `2` | ≤ 100 μs |
| `3` | ≤ 1 ms |
| `4` | ≤ 10 ms — **threshold for the recommended `DISCARD_WINDOW = 50 ms`** (§6.3.3). |
| `5` | ≤ 100 ms |
| `6` | > 100 ms, or unbounded |
| `7` | Unknown / not asserted |

#### 8.2.4 Producer Obligations

The Time Master:

1. Shall set `SourceClass` to its **current** state, not its nameplate capability. A node disciplined by a GNSS receiver that has lost lock shall transition `1 → 4` (holdover) on lock loss, not continue to claim `1`.
2. Shall set `AccBucket` to its current estimated offset, widening the bucket as holdover elapses per the oscillator class declared in product literature.
3. Shall set `SpoofSuspect = 0` only if its receiver supports signal-integrity monitoring **and** reports no current anomaly. A Time Master whose receiver lacks anomaly reporting shall set `SpoofSuspect = 1` and shall not advertise `SourceClass = 2`.
4. May assert `SpoofSuspect = 1` based on peer cross-check — comparing its own `CoreTime` against another GNSS-equipped Class D node's `Pelorus.TimeSync` on the bus and detecting divergence beyond the declared `AccBucket`.
5. Shall set `LeapPending = 1` for the entire UTC day during which a positive or negative leap second occurs at 23:59:60 UTC, when the leap announcement is known to the Master through its upstream source.
6. During its first 3 s after power-up, before acquiring discipline, shall transmit `SourceClass = 0`, `AccBucket = 7` so receivers immediately recognise an unready Master.

#### 8.2.5 Consumer Obligations

| Consumer | Rule |
|---|---|
| Duplicate Discard (§6.3.3) | Use the recommended `DISCARD_WINDOW = 50 ms` only when the most recent `Pelorus.TimeSync` has `SourceClass ∈ {1, 2, 3, 4}` **and** `AccBucket ≤ 4`. Otherwise apply the formula in §6.3.3. |
| AIS forwarder, voyage data recorder, alarm log | Shall not stamp records with `CoreTime` when `SourceClass ∈ {0, 5}` or `SpoofSuspect = 1`. Records shall be marked "time not trusted" or stamped from a higher-trust local source. |
| ECDIS / helm display | Should surface an operator-visible annunciator within 5 s of a transition to `SourceClass = 4`, `SourceClass = 5`, or `SpoofSuspect = 1`. |
| Any consumer reading `CoreTime` as UTC | Shall check `SourceClass` **before** interpreting `CoreTime` (§8.2.3 — `CoreTime` is monotonic, not UTC, when `SourceClass ∈ {0, 5}`). |

#### 8.2.6 Forward Compatibility

Receivers shall treat any `SourceClass` value or `AccBucket` value not defined above as the worst-case interpretation:

- Unknown `SourceClass` shall be treated as `SourceClass = 0` for the purposes of §8.2.5, and the frame shall not be used to tighten `DISCARD_WINDOW`.
- Unknown `AccBucket` shall be treated as `AccBucket = 7` (unknown).
- Receivers shall not raise errors or refuse subsequent frames on encountering unknown values.

This allows future revisions to assign currently-reserved slots without breaking deployed receivers.

## 9. Wake Generation and DDT Invalidation

### 9.1 Wake Generation Counter

Each node that participates in path redundancy (Class D or Class H) shall maintain a 4-bit wake generation counter in retained or non-volatile storage, incremented modulo 16 on every transition from Sleep or Deep Sleep to Active (first NM Normal-Operation participation counts as Active for this purpose). The current value shall appear in BusId_WakeGen bits 4–1 of Bus Health (§8.1) and Time Sync (§8.2 when implemented).

### 9.2 DDT Invalidation on Wake

Receivers shall treat a change in wake generation for source `S` (observed via Bus Health, or on first post-wake application frame if Bus Health not yet received) as a signal to delete all DDT entries for `S`. Until Bus Health is transmitted, Class D nodes shall still increment wake generation on wake so that the first `Pelorus.BusHealth` frame reflects the new value.

### 9.3 Sleep and Duplicate Discard

Nodes entering Sleep or Deep Sleep cease application transmissions per [`04-power.md §5`](./04-power.md); duplicate discard state on peer nodes is governed by `NODE_FORGET_TIME` (§6.3.1).

## 10. Degraded-Mode Behaviour

When Bus A or Bus B is lost, powered down, or in bus-off:

- Class D nodes shall continue transmitting and receiving on the remaining bus without requiring operator reset, subject to address-claim rules on that bus.
- Shall set operator-visible fault indication (display annunciator, alarm DC, or gateway UI) within 5 s of detecting sustained loss of the peer bus.
- Shall continue to apply duplicate discard on the surviving bus so that when the failed bus returns, transient duplicates do not corrupt application state.

### 10.1 Failover Convergence (C0 / C1)

For C0 and C1 producers transmitting active-active on both buses at their declared steady-state cadence:

- Shall not introduce an application-layer message gap, on a sustained single-bus failure, larger than `DISCARD_WINDOW + max(producer_period)` for any logical message that was being delivered immediately before the failure.
- Shall not require an operator action, restart, or re-binding to keep delivering messages on the surviving bus.
- On bus return, the receiver shall resume normal duplicate discard without producing false duplicates for messages already delivered from the surviving bus during the outage.

### 10.2 Bus Return After Failure

When a previously failed bus recovers and resumes carrying valid CAN FD traffic, receivers shall:

1. Accept frames on the returning bus immediately and apply normal §6.3 duplicate-discard rules — there is no re-sync handshake or replay protocol in v1.0.
2. Not treat sequence numbers or payloads observed on the returning bus as duplicates of frames already delivered from the surviving bus during the outage unless they fall inside the active `DISCARD_WINDOW` for the same `(S, DC_ID)`.
3. Continue existing DDT entries for sources whose wake generation has not changed; only invalidate when generation changes per §9.2.

## 11. Common-Mode Mitigation

Path redundancy alone is insufficient for C0/C1. Where practical, installers and manufacturers shall:

- Route Bus A and Bus B along physically separated cable paths (different bundles, different penetrations where feasible). Shall not claim full path redundancy if both buses share a single unprotected cable run through a single hazard zone without documenting the residual risk in the critical zone map.
- Prefer independent protected feeds for transceiver/node power on Bus A vs Bus B when the vessel electrical design supports it.

Identical firmware on both transmit paths can produce identical incorrect data on both buses; path redundancy does not mitigate this. v1.0 does not require dissimilar firmware as a conformance gate. A future C0 / SOLAS-aligned profile may add such requirements.

## 12. Critical Zone Map and Conformance

For any product or installation declared conformant with path-redundant Pelorus Core per [`11-conformance.md`](./11-conformance.md), a critical zone map shall be published (paper, PDF, or structured file) listing: zone name, C0/C1/C2 assignment per function, Bus A/B topology sketch, node classes (S/D/H), and reference to executed conformance tests.

The critical zone map shall additionally record, where applicable to the installation:

- **Time Master assignment** for each dual-bus domain (§8.2.1) — which node is the elected Time Master under normal conditions, and its declared `AccBucket` widening schedule when in holdover.
- **Declared `D_clk`** (§6.3.3) when no Time Master is present, when the Time Master's expected `AccBucket` exceeds 4, or when the installation chooses a `DISCARD_WINDOW` larger than the 50 ms floor.
- **Owner Private DC slot assignments.** Every DC slot in the `0x3F000–0x3F0FF` Owner Private range ([`07-dcid-registry.md §3`](./07-dcid-registry.md)) that the vessel uses, with its semantic meaning (e.g. "`0x3F005` = bilge pressure aft, `uint16` mbar, 1 Hz"), the source device, and the consumers configured to process it. Owner Private slots have no meaning outside the vessel; the critical zone map is the only place they are documented.

Vessels or products using only C2 single-bus Core may omit the dual-bus domain; the declaration shall state "Pelorus Core, single-bus (C2-only)" or equivalent so purchasers know the reliability tier.

A vessel using only Owner Private DCs and no path-redundancy claim is not required to publish a critical zone map. Owners are encouraged to maintain one regardless, as it is the only place Owner Private slot assignments are recorded and the only protection against later collisions with newly-installed commercial gear.

## 13. Relationship to Segmentation

- A dual-bus domain may contain multiple segments per bus via repeaters ([`09-network.md`](./09-network.md)), subject to hop limits per bus.
- Repeaters shall not be described as satisfying C0/C1 path redundancy unless the installation also implements Bus A and Bus B per this document.

## 14. Phased Deployment (informative)

Vessels and product lines may roll out path redundancy incrementally:

- **Stage 0 — single-bus C2 only.** Existing single-bus Pelorus Core install. Declared as "Pelorus Core, single-bus (C2-only)". No dual-bus or C0/C1 claims.
- **Stage 1 — dual bus in critical zones.** Add Bus B in helm, autopilot, and propulsion-alarm zones; upgrade nodes there to Class D or add a Class H hub for Class S sensors. Other zones remain single-bus C2. Critical zone map shows the boundary and class of every Core-attached function.
- **Stage 2 — vessel-wide dual bus.** All Core-attached devices on dual-bus, including comfort and logging signals upgraded to C2-on-dual-bus (allowed; never required).

At any intermediate stage, the conformance declaration shall reflect the actual configuration; do not advertise dual-bus conformance for zones that have not yet been physically dual-routed.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
