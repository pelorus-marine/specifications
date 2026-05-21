# Pelorus Core — Conformance and Self-Declaration

Conformance test plan and manufacturer self-declaration template. Conformance is established by self-testing against reference implementations; no third-party certification body is required for v1.0.

## 1. Scope

| Class under test | Normative primary documents |
| --- | --- |
| **Class S** node | [`02`](./02-physical.md), [`03`](./03-data-link.md), [`04`](./04-power.md), [`05`](./05-addressing.md), [`06`](./06-signal-catalog.md), [`07`](./07-dcid-registry.md) |
| **Class D** node | [`02`](./02-physical.md), [`03`](./03-data-link.md), [`04`](./04-power.md), [`05`](./05-addressing.md), [`07`](./07-dcid-registry.md), [`08`](./08-redundancy.md) |
| **Class H** hub | [`02`](./02-physical.md), [`03`](./03-data-link.md), [`05`](./05-addressing.md), [`07`](./07-dcid-registry.md), [`08`](./08-redundancy.md), [`09`](./09-network.md) |
| **Gateway** | [`09`](./09-network.md), plus applicable node class for each Core port |
| **Repeater** (non-hub) | [`02`](./02-physical.md), [`04`](./04-power.md), [`05`](./05-addressing.md), [`09`](./09-network.md) |
| **Any device with a writable image** | Above for its class, plus [`12-firmware-update.md`](./12-firmware-update.md) |

Any device that exposes a writable firmware image — regardless of node class — shall implement the open firmware update protocol in [`12-firmware-update.md`](./12-firmware-update.md). A device that rejects firmware update sessions from third-party tools based on initiator identity is non-conformant.

Tests marked **(D)** apply only to dual-bus / path-redundancy declarations.

## 2. Test Equipment and Setup

- **Reference bus.** Two CAN FD channels (for (D) tests) or one channel at 250 kbit/s arbitration / 500 kbit/s data, ISO 11898-1:2015 compliant, with logging.
- **Reference companion.** Reference implementation node(s) from [`10-implementation.md §1`](./10-implementation.md) (when published) or golden-trace replay fixture.
- **Fault injection (D).** Programmable open / disconnect on Bus A or Bus B backbone; optional stuck-recessive / dominant injector behind isolation.
- **Power.** 9–32 V supply with current measurement for sleep-state tests.

## 3. Test Categories

### 3.1 Physical Layer

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-02-001** | Measure termination resistance, power-off, on a completed segment | ~60 Ω between CAN_H and CAN_L at segment ends |
| **P-02-002** | Verify M12 A-coded pinout continuity | Matches [`02-physical.md §3`](./02-physical.md) |
| **P-02-003 (D)** | Class D: two independent bus pairs from device to field | No DC continuity between Bus A pair and Bus B pair |

### 3.2 Data Link — Duplicate Discard

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-03-001 (D)** | Transmit identical `Pelorus.BusHealth` PRH frame on A then B within 10 ms | DUT accepts one application delivery; duplicate counter increments |
| **P-03-002 (D)** | Same SA+DC_ID+payload compatibility frame on A then B within `DISCARD_WINDOW` | One delivery to application |
| **P-03-003** | `Pelorus.AddressClaim` on both buses (D) | DUT processes both claims independently per [`08-redundancy.md §6.2`](./08-redundancy.md) |
| **P-03-004 (D)** | Increment `Pelorus.BusHealth` sequence; resend same sequence on second bus within `DISCARD_WINDOW` | Second copy discarded |
| **P-03-005 (D)** | Failover under steady traffic: 10 Hz Class D producer, kill Bus A mid-stream | Receiver application sees no message gap larger than `DISCARD_WINDOW + 100 ms` |
| **P-03-006 (D)** | Bus return without false duplicates: restore Bus A after 10 s outage while Bus B continues | Receiver does not re-deliver any message already accepted from Bus B; DDT entries remain valid |
| **P-03-007 (D)** | PRH 16-bit sequence wrap on `Pelorus.BusHealth` (drive past 65535) | Receiver continues correct duplicate discard across wrap; no spurious suppression |

### 3.3 Addressing

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-05-001** | Contested NAME / SA | Loser selects new SA per [`05-addressing.md §3`](./05-addressing.md) |
| **P-05-002 (D)** | Class D: claim on A succeeds, forced fail on B | DUT enters degraded single-bus or re-claims per [`08-redundancy.md §5`](./08-redundancy.md) |

### 3.4 Power Management vs DDT

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-04-001 (D)** | Sleep → Active; Bus Health wake generation increments | Peer clears DDT for that SA or accepts first post-wake frame without false duplicate discard |

### 3.5 Data Contract Registry — Bus Health

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-07-001 (D)** | Class D powered Active | `Pelorus.BusHealth` on each bus at ≤ 2.5 s observed cadence; layout matches [`08-redundancy.md §8.1`](./08-redundancy.md) |

### 3.6 Network and Hub

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-09-001** | Repeater between two segments | Valid frame from seg1 appears on seg2 unmodified |
| **P-09-002 (D)** | Class S node on hub downstream; frame to hub | Identical frame on Bus A and Bus B backbones |
| **P-09-003 (D)** | Two upstream Class D producers run active-active; one downstream Class S consumer attached to a hub | Class S receives one copy of each logical frame; hub increments duplicate counter on whichever ingress bus arrived second |
| **P-09-004 (D)** | Force one hub backbone port into bus-off while traffic continues | Hub continues forwarding between surviving backbone and downstream segments; `Pelorus.BusHealth` reports `Bus state = 3`; missed-frame counter for failed bus increments |

### 3.7 Firmware Update — Open Access

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-12-001** | Issue `Pelorus.FirmwareUpdateQuery` from a non-vendor third-party tool to a writable-image device | Device responds with `Pelorus.FirmwareUpdateResponse` carrying version, slot model, and signing requirement |
| **P-12-002** | Initiate a firmware update from a non-vendor tool using a manifest signed with the published verification key | Device accepts the session and emits `Pelorus.FirmwareUpdateProgress` at ≥ 1 Hz throughout the transfer |
| **P-12-003** | Initiate a firmware update where the manifest carries a valid signature but the initiator's NAME manufacturer code does not match the device's manufacturer | Device proceeds with the update — no rejection based on initiator identity per [`12-firmware-update.md`](./12-firmware-update.md) |
| **P-12-004** | Interrupt an in-progress update by cycling power; resume with the same `session_id` within the receiver's timeout | Update resumes from `next_expected_seq` without re-transmitting received frames |

### 3.8 Criticality and Declaration Cross-Check

| ID | Procedure | Pass criteria |
| --- | --- | --- |
| **P-08-001** | Review submitted critical zone map + declaration | C0/C1 zones show dual-bus where required; no undocumented single-bus C0 paths |
| **P-08-002 (D)** | Review failover convergence claim against measured P-03-005 / P-03-006 logs | Declared maximum gap is consistent with [`08-redundancy.md §10.1`](./08-redundancy.md) and matches measured worst-case |

## 4. Requirements Traceability

| Requirement | Test IDs |
| --- | --- |
| `02 §3` connector pinout | P-02-002 |
| `08 §4` dual transceiver | P-02-003 |
| `08 §6.3` duplicate discard | P-03-001, P-03-002, P-03-004 |
| `08 §6.2` exemptions | P-03-003 |
| `08 §7` PRH (sequence wrap) | P-03-007 |
| `08 §10.2` bus return | P-03-006 |
| `08 §9` wake generation | P-04-001 |
| `08 §5` dual claim | P-05-002 |
| `08 §8.1` Bus Health | P-07-001 |
| `09 §3.1` hub replication | P-09-002 |
| `09 §3.3` hub bidirectional dedup | P-09-003 |
| `09 §3.4` hub backbone bus-off | P-09-004 |
| `08 §2`–`§12` criticality | P-08-001 |
| `08 §10.1` failover convergence | P-03-005, P-08-002 |
| `12 §"Vendor-neutral"` open firmware update | P-12-001, P-12-002, P-12-003 |
| `12 §"Recovery"` resumable update | P-12-004 |

Expand this matrix as additional `shall` statements are added.

## 5. Pass/Fail

A device passes Pelorus Core conformance for a declared configuration when:

1. Every mandatory test for that configuration (single-bus Class S vs (D) dual-bus) is executed and passes.
2. No `shall` in the cited normative documents is violated by observed behaviour during the test campaign.
3. Test logs are retained per §6.

**Fail:** any mandatory test failure, or any safety-critical deviation (e.g. application data on one bus before dual claim completes).

## 6. Self-Declaration Template

### Product Identification

- Manufacturer: _______________________________
- Model / Part Number: _______________________________
- Hardware Revision: _______________________________
- Firmware Version: _______________________________
- Date of Declaration: _______________________________

### Path Redundancy

- Node class: **Class S** / **Class D** / **Class H** (circle one; see [`08-redundancy.md §3`](./08-redundancy.md))
- Criticality tier claimed: **C2-only** / **includes C1** / **includes C0** (definitions in [`08-redundancy.md §2`](./08-redundancy.md))
- Critical zone map attached: **Yes** / **No** (required when C0/C1 or Class D/H dual-bus is claimed)
- Dual-bus conformance tests (IDs marked (D) above): **executed** / **not applicable**

### Declaration

> We, the undersigned, declare that the above-identified product meets the requirements of the Pelorus Core specification version 0.3 and is therefore **Pelorus Core conformant**.
>
> Specifically, the product has been verified to comply with the Pelorus Core documents in `specifications/core/`:
> [`02-physical.md`](./02-physical.md), [`03-data-link.md`](./03-data-link.md), [`04-power.md`](./04-power.md), [`05-addressing.md`](./05-addressing.md), [`06-signal-catalog.md`](./06-signal-catalog.md), [`07-dcid-registry.md`](./07-dcid-registry.md), [`08-redundancy.md`](./08-redundancy.md) (when dual-bus or C0/C1 is claimed), [`09-network.md`](./09-network.md), [`10-implementation.md`](./10-implementation.md), [`11-conformance.md`](./11-conformance.md), and [`12-firmware-update.md`](./12-firmware-update.md) (when a writable image is exposed).
>
> All mandatory tests in §3 above were executed and passed. Test logs and results are available upon request.

### Signature

- Manufacturer Representative: _______________________________
- Printed Name: _______________________________
- Title: _______________________________
- Date: _______________________________

### Use

1. Fill in the product details and sign the declaration.
2. Publish the completed declaration alongside the product documentation (product webpage or user manual).
3. Include the Pelorus Core conformant logo (when available) on product, packaging, and marketing.
4. Retain test records for at least 5 years in case of dispute.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
