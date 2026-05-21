# Pelorus Core — Power Management

Selective wake-up, partial networking, functional groups, power states, network management. Wake-generation interaction with dual-bus duplicate discard lives in [`08-redundancy.md`](./08-redundancy.md).

## 1. Patent Notice

ISO 11898-2:2016 §5.9.4 (selective wake-up) discloses involved patents. Disclosed holders: Audi, BMW, Continental Teves, DENSO, Elmos Semiconductor, Freescale (now NXP), General Motors, NXP, Renesas, Robert Bosch, STMicroelectronics, Volkswagen. Holders have committed to RAND licensing through ISO. Commercial Pelorus hardware implementing selective wake-up should review patent landscape and licensing before product release. Basic wake-up and wake-up pattern (WUP) — ISO 11898-2 §5.9.2/§5.9.3 — do not have the same patent disclosure and are lower-functionality alternatives.

## 2. Wake-Up Frame Format

A WUF is a Classical CAN frame (per ISO 11898-1:2015):

- 11-bit or 29-bit identifier (selectable per node)
- DLC 0–8
- 0–8 bytes of data
- Valid only if free of CRC, stuff, and form errors through the CRC delimiter

CAN FD frames are not recognized as valid WUFs by basic ISO 11898-2 implementations. CAN FD-tolerant transceivers (TJA1145/FD, ATA6570, TCAN1145-Q1) detect FDF=recessive then res=dominant and wait `nBits_idle` (6–10) recessive bits before considering a new SOF. **Pelorus mandates CAN FD-tolerant transceivers** so CAN FD data traffic does not interfere with WUF detection. WUFs themselves are transmitted as Classical CAN frames.

### 2.1 Identifier Matching

Each node stores a target ID and an ID mask. A `1` in the mask means "don't care"; a `0` means the bit must match. The IDE bit (11-bit vs 29-bit) is not part of the mask — it must match exactly.

### 2.2 DLC Matching

The received DLC must equal the configured DLC. Special case: configured DLC = 0 means the data field is not evaluated and the data mask is ignored.

### 2.3 Data Field Group Matching

When DLC ≥ 1, the data field implements group addressing. Up to 8 bytes × 8 bits = **64 distinct groups**.

A wake-up occurs if **at least one bit position** has `1` in both the received frame's data field and the node's data mask. Multiple matching bits are fine — only one is required. Group membership is additive: a node wakes for any group it belongs to, and a single WUF can wake multiple cooperating groups at once.

## 3. Functional Groups (PNCs)

Pelorus reserves the lowest six bits of WUF data byte 0 for standard marine functional groups. Bits 6–63 are reserved for future Pelorus assignment and shall not be used by v1.0 implementations.

| Bit | Group | Typical members | Wake trigger |
| --- | --- | --- | --- |
| 0 | `anchor_watch` | GNSS, depth, anchor alarm | At anchor; periodic or on drift |
| 1 | `underway` | GNSS, heading, wind, AIS, autopilot, log | Vessel moving |
| 2 | `engine` | Engine ECU, fuel, alternator, exhaust temp | Ignition on or engine running |
| 3 | `comms` | VHF, AIS transmit, satellite, LMDE bridge | DSC inbound, scheduled poll, user request |
| 4 | `domestic` | Tank levels, battery monitors, refrigeration | Periodic housekeeping or user request |
| 5 | `storm` | Wind, AIS receive, GNSS, barometer | Severe weather mode; reduced bandwidth |
| 6–63 | Reserved | — | Shall not be used in v1.0 |

Group membership is configured per device at provisioning time. A device may belong to any combination. Examples:

- GPS receiver: `anchor_watch | underway | storm` (bits 0, 1, 5)
- Wind transducer: `underway | storm` (bits 1, 5)
- Engine ECU: `engine` (bit 2)
- Tank sender: `domestic` (bit 4)

A node sending a WUF asserts one or more group bits to wake the corresponding clusters. The gateway is the typical originator but any active node may transmit a WUF. Group activation is not exclusive.

Vessel-wide mode transitions ("weighing anchor") are coordinated by the gateway: it transmits a WUF asserting the new groups; newly-woken nodes initialize and begin transmitting; nodes belonging only to the old groups time out and re-sleep via NM (§6). Transitions are not atomic — sailors should expect new instruments to come online over a few seconds.

## 4. Reserved Identifiers and Data Conventions

### 4.1 `Pelorus.WakeUp`

| Field | Value |
| --- | --- |
| DC_ID | `0x00001` |
| Priority | 0 (highest) |
| Source Address | originator's claimed address |

DLC = 8. Byte 0 carries the marine functional-group bitmask (§3, lowest six bits; bits 6–7 of byte 0 reserved, transmit zero). Bytes 1–7 reserved — transmit `0x00`, ignore on receive.

### 4.2 `Pelorus.NetworkManagement`

| Field | Value |
| --- | --- |
| DC_ID | `0x00002` |
| Priority | 6 (below safety-critical) |

DLC = 8. Layout:

| Byte | Field |
| --- | --- |
| 0 | NM state — `0x00` ready-sleep, `0x01` repeat, `0x02` normal-operation, `0x03` prepare-bus-sleep |
| 1 | Active groups (low byte) — bitmap of groups the sender is keeping awake |
| 2–7 | Reserved — transmit zero, ignore on receive |

## 5. Power States

| State | MCU | Transceiver | Bus monitoring | Typical current (non-isolated) |
| --- | --- | --- | --- | --- |
| **Active** | Running | Normal | Yes | Application-specific (declared) |
| **Standby** | Low-power running | Normal | Yes | Device-specific declared |
| **Sleep** | Off or retention | Selective wake mode | Yes (via WUF detection) | ≤100 µA |
| **Deep Sleep** | Off | Standby (no WUF) | No | ≤10 µA |

Sleep targets for galvanically isolated devices are in [`02-physical.md §8.5`](./02-physical.md).

### 5.1 Transitions

| From | To | Trigger | Notes |
| --- | --- | --- | --- |
| Active | Standby | Idle timeout (application-defined) | Drain pending TX first |
| Active | Sleep | Coordinated cluster sleep (§6) | Initiated by NM, not unilateral |
| Standby | Active | Application event (sensor reading, RX traffic) | No bus signaling required |
| Standby | Sleep | Coordinated cluster sleep (§6) | |
| Sleep | Active | WUF group match | Via Standby; transceiver wakes MCU |
| Sleep | Deep Sleep | Vessel-wide power-down command | Optional; not all devices support |
| Deep Sleep | Active | External wake (RTC, manual switch, hardwired event) | Bus traffic does not wake from Deep Sleep |

Unilateral sleep is forbidden for any node that other nodes depend on. A node may only enter Sleep if (a) all consumers of its data have indicated they no longer need it, or (b) the cluster has reached coordinated sleep through NM.

### 5.2 Wake-Up Latency

| Phase | Typical |
| --- | --- |
| Transceiver INH/MCU wake | 100–500 µs |
| MCU boot and CAN init | 5–50 ms |
| Application initialization | 10–500 ms |
| First valid data transmission | 50 ms – 2 s (sensor-dependent) |

Pelorus does not mandate a wake-up latency target. Manufacturers shall declare typical and maximum elapsed time from WUF reception to first valid data, per supported functional group.

## 6. Network Management

Modelled on AUTOSAR CanNm (R23-11), simplified for marine use. Each node periodically transmits an NM message indicating intent to keep the cluster active. When all nodes stop transmitting NM, the cluster transitions to coordinated sleep.

### 6.1 Cadence

| Parameter | Value |
| --- | --- |
| NM message period | 200 ms ± 20 ms |
| Repeat-message duration | 1.0 s |
| Wait-bus-sleep duration | 2.0 s |
| Total transition to Sleep | ~3.0 s after last keep-active |

Final cadence subject to wake-up latency measurements from prototype hardware.

### 6.2 NM States

| State | Behaviour |
| --- | --- |
| **Bus-Sleep** | Transceiver in selective wake mode. No NM transmission. |
| **Prepare-Bus-Sleep** | No NM transmission. Wait 2.0 s for any node to break silence. If a frame is observed, return to Repeat. Else transition to Bus-Sleep. |
| **Ready-Sleep** | No NM transmission, but listening. Other nodes' NM keeps cluster alive. After 1.0 s with no NM traffic, enter Prepare-Bus-Sleep. |
| **Normal-Operation** | Transmit NM every 200 ms. Application is operating. |
| **Repeat-Message** | Transmit NM every 200 ms for 1.0 s after waking. Ensures cluster-membership announcement. |

### 6.3 Wake-Up to Active Sequence

1. Sleeping nodes' transceivers detect a matching WUF.
2. Transceivers wake their MCUs.
3. Each woken node enters Repeat-Message and transmits NM for 1.0 s.
4. Each node either continues to Normal-Operation if it has work, or transitions to Ready-Sleep.
5. If all woken nodes reach Ready-Sleep with no traffic for 1.0 s, the cluster proceeds toward Bus-Sleep.

### 6.4 Sleep Coordination Failure Modes

If a node fails to transmit NM but has pending application work, other nodes may incorrectly initiate cluster sleep. Mitigations:

- Application work that requires the bus shall keep the node in Normal-Operation
- Watchdog timers detect stuck Ready-Sleep states
- The gateway acts as cluster monitor and may rebroadcast a WUF if it detects premature sleep

## 7. Frame Error Counter

Per ISO 11898-2:2016 §5.9.4.4, a transceiver in selective wake mode increments a Frame Error Counter (FEC) on each frame that fails validation through the CRC delimiter. When FEC reaches its threshold, the transceiver suspends WUF detection until the bus quiets.

Pelorus implementations configure FEC threshold to **31** errors. Resets occur when:

- A valid CAN frame is observed (regardless of WUF match)
- The host MCU explicitly resets via the transceiver's SPI interface
- The transceiver re-enters normal mode

If the FEC saturates (severely degraded bus), the transceiver enters a fault-tolerant state that does not wake the MCU. The bus must quiet, and the host must explicitly reset the counter, before WUF detection resumes. Implementers shall not bypass FEC — it is the primary defense against errant noise causing battery drain through repeated false wake-ups.

## 8. Bus Biasing

A CAN bus requires active biasing — at least one node driving toward recessive — for proper signal integrity. In a network with at least one Active node, that node's transceiver provides biasing.

When all nodes are in Sleep or Deep Sleep, the bus is unbiased. WUFs still propagate correctly: the transmitting node biases during transmission; receivers in selective wake mode synchronise to WUF edges from the unbiased state per ISO 11898-2:2016 §5.10.

Repeater nodes bridge two segments and shall maintain at least one transceiver in Standby (not Sleep) on each segment whenever any node on either segment is Active. If both segments reach coordinated Bus-Sleep, the repeater itself may sleep.

Pelorus does not specify external bias resistors beyond what compliant CAN FD transceivers integrate. Split termination per [`02-physical.md §5`](./02-physical.md) is sufficient.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
