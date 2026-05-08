# Pelorus Core — Conformance Test Plan

**Version:** 0.1 Draft  
**Last Updated:** May 4, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **conformance test plan** (categories, fixtures, pass/fail) for Pelorus Core devices. The **conformance model** (self-test against reference implementations; no third-party cert for v1.0) is stated in [01-overview.md §9](./01-overview.md#9-cross-cutting-decisions-authoritative-summary) and in [16-compliance-self-declaration.md](./16-compliance-self-declaration.md). **Normative** requirements under test are cited from **`core/02`**–**`17`**; this document assigns **test IDs** and procedures.

---

## 1. Scope

| Class under test | Normative primary documents |
|------------------|-----------------------------|
| **Class S** node | **02**, **03**, **04**, **05**, **06**, **07** |
| **Class D** node | **02** §13, **03** §6, **04** §13–14, **05** §7, **07** §1.3, **17** |
| **Class H** hub | **02** §13, **08**, **10** §3, **03** §6, **05** §7, **07** §1.3, **17** |
| **Gateway** | **09**, plus applicable node class for each Core port |
| **Repeater** (non-hub) | **10** §1–2, **04**, **05**, **08** |

Tests marked **(D)** apply only to dual-bus / path-redundancy declarations.

---

## 2. Test Equipment and Setup

- **Reference bus:** Two (for **(D)** tests) or one CAN FD channels at **250 kbit/s** arbitration / **500 kbit/s** data, ISO 11898-1:2015 compliant, with logging.
- **Reference companion:** Reference implementation node(s) from **[11-reference-implementations.md](./11-reference-implementations.md)** (when published) or golden-trace replay fixture.
- **Fault injection (D):** Programmable open / disconnect on Bus A or Bus B backbone; optional stuck-recessive / dominant injector behind isolation.
- **Power:** 9–32 V supply with current measurement for sleep-state tests (**04**).

---

## 3. Test Categories and Test IDs

### 3.1 Physical layer (**02**)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-02-001** | Measure termination resistance power-off on a completed segment | ≈ 60 Ω between CAN_H and CAN_L at segment ends per **02** §6.4 |
| **P-02-002** | Verify M12 A-coded pinout continuity | Matches **02** §4.2 |
| **P-02-003 (D)** | Class D: two independent bus pairs from device to field | No DC continuity between Bus A pair and Bus B pair |

### 3.2 Data link — duplicate discard (**03** §6)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-03-001 (D)** | Transmit identical PRH frame (0x0FF82) on A then B within 10 ms | DUT accepts one application delivery; duplicate counter increments or second copy discarded |
| **P-03-002 (D)** | Same SA+DCID+payload compatibility frame on A then B within `DISCARD_WINDOW` | One delivery to app layer |
| **P-03-003** | Address Claimed on both buses (D) | DUT processes both claims independently (**03** §6.2 exemption) |
| **P-03-004 (D)** | Increment Bus Health sequence; resend same sequence on second bus within `DISCARD_WINDOW` | Second copy discarded |
| **P-03-005 (D)** | Failover under steady traffic: 10 Hz Class D producer, kill Bus A mid-stream | Receiver application layer sees no message gap larger than `DISCARD_WINDOW + 100 ms` (**[17 §3.1](./17-criticality-and-redundant-paths.md#31-failover-convergence-c0--c1)**) |
| **P-03-006 (D)** | Bus return without false duplicates: restore Bus A after a 10 s outage while Bus B continues | Receiver does not re-deliver any message already accepted from Bus B during the outage; DDT entries remain valid (**[03 §6.6](./03-data-link-layer.md#66-interaction-with-power-management-and-bus-return)**) |
| **P-03-007 (D)** | PRH 16-bit sequence wrap on 0x0FF82 (drive past 65535) | Receiver continues to apply duplicate discard correctly across wrap; no spurious duplicate suppression (**[03 §6.3 / §6.4.1](./03-data-link-layer.md#63-prh--pelorus-redundancy-header-pelorus-native-dcids)**) |

### 3.3 Addressing (**05**)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-05-001** | Contested NAME / SA | Loser selects new SA per **05** §3 |
| **P-05-002 (D)** | Class D: claim on A succeeds, forced fail on B | DUT enters degraded single-bus or re-claims per **05** §7 |

### 3.4 Power management vs DDT (**04** §13, **03** §6.6)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-04-001 (D)** | Sleep → Active; Bus Health wake gen increments | Peer clears DDT for that SA or accepts first post-wake frame without false duplicate discard |

### 3.5 DCID registry — Bus Health (**07** §1.3)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-07-001 (D)** | Class D powered Active | 0x0FF82 on each bus at ≤ 2.5 s observed cadence; layout matches **07** |

### 3.6 Network architecture & hub (**08**, **10**)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-08-001** | Repeater between two segments | Valid frame from seg1 appears on seg2 unmodified identifier+data (**10** §1) |
| **P-10-001 (D)** | Class S node on hub downstream; frame to hub | Identical frame on Bus A and Bus B backbones |
| **P-10-002 (D)** | Two upstream Class D producers run active-active; one downstream Class S consumer attached to a hub | Class S receives **one** copy of each logical frame; hub increments duplicate counter on whichever ingress bus arrived second (**[10 §3.4](./10-repeater-specification.md#34-hub-bidirectional-duplicate-discard)**) |
| **P-10-003 (D)** | Force one hub backbone port into bus-off while traffic continues | Hub continues forwarding between surviving backbone and downstream segments; 0x0FF82 on surviving bus reports `Bus state = Degraded-Single`; missed-frame counter for failed bus increments (**[10 §3.5](./10-repeater-specification.md#35-hub-bus-off-and-degraded-backbone)**) |

### 3.7 Criticality & declaration cross-check (**17**, **16**)

| ID | Procedure (summary) | Pass criteria |
|----|---------------------|---------------|
| **P-17-001** | Review submitted critical zone map + declaration | C0/C1 zones show dual-bus where required; no undocumented single-bus C0 paths |
| **P-17-002 (D)** | Review failover convergence claim against measured P-03-005 / P-03-006 logs | Declared maximum gap is consistent with **[17 §3.1](./17-criticality-and-redundant-paths.md#31-failover-convergence-c0--c1)** and matches measured worst-case |

---

## 4. Requirements Traceability Matrix (starter)

| Requirement (document §) | Test IDs |
|--------------------------|----------|
| **02** §13 dual transceiver | P-02-003 |
| **03** §6 duplicate discard | P-03-001, P-03-002, P-03-004 |
| **03** §6.2 exemptions | P-03-003 |
| **03** §6.3 PRH (sequence wrap) | P-03-007 |
| **03** §6.6 bus return | P-03-006 |
| **04** §13 wake generation | P-04-001 |
| **05** §7 dual claim | P-05-002 |
| **07** §1.3 Bus Health | P-07-001 |
| **10** §3.1 hub replication | P-10-001 |
| **10** §3.4 hub bidirectional dedup | P-10-002 |
| **10** §3.5 hub backbone bus-off | P-10-003 |
| **17** §2–§6 | P-17-001 |
| **17** §3.1 failover convergence | P-03-005, P-17-002 |

Expand this matrix as additional **SHALL** statements are added to **02**–**17**.

---

## 5. Pass/Fail Criteria

A device **passes** Pelorus Core conformance for a declared **configuration** when:

1. Every **mandatory** test for that configuration (single-bus **Class S** vs **(D)** dual-bus) is **executed** and **passes**.
2. No **SHALL** in the cited normative documents is violated by observed behavior during the test campaign.
3. Test logs are retained per **[16](./16-compliance-self-declaration.md)**.

**Fail:** Any mandatory test failure, or any safety-critical deviation (e.g. application data on one bus before dual claim completes — **05** §7).

---

## 6. Open Items (to be resolved before v1.0 promotion)

- Automated trace capture format and golden-trace library
- Reference implementation pinning (commit hash) per release
- Environmental qualification scope (salt fog, vibration) — normative here vs external IEC only
- Machine-readable test report JSON schema

---

*This document, together with documents 01–14, **16**, and **17**, supports Pelorus Core conformance verification.*
