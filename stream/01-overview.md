# Pelorus Stream — Overview

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified (structural overview + **draft** design targets; wire-level decisions are **not** interoperability commitments until validated on hardware — see §7 and [`00-document-index.md`](./00-document-index.md) §4).

---

## About This Document

This is the entry point to the Pelorus Stream specification. It states what Stream is, what it is **not**, how it relates to Pelorus Core and the State subsystem, and which documents to read next. **Normative** requirements live in downstream documents (02 onward). [§9](#9-draft-design-targets-summary) collects **draft design targets** in one place so other documents can cross-reference instead of repeating them; for wire-level requirements, always use the numbered specification for that topic.

Pelorus Stream is **intentionally** the Pelorus Ethernet substrate: it aims to take what is genuinely good in **NMEA OneNet-style** IP marine networking (IPv6, rich media paths, complementary to a fieldbus) while **rejecting** closed certification walls, mandatory product programs, and stacks that entangle high-bandwidth data with safety authority. Stream **mates cleanly with Pelorus Core** (CAN FD) using **tiered gateways**—**standard** for Core→Stream and **capable bidirectional** only where Stream→Core is required (**§3.1**). Ordinary Stream nodes do **not** originate Core traffic; **no** actuation or safety authority through Stream beyond §2 bounds; LMDE joins remain gateway-mediated per [`core/09-gateway-specification.md`](../core/09-gateway-specification.md).

Read this first. Then go to [`02-stream-id.md`](./02-stream-id.md) for the data model, or [`08-connection.md`](./08-connection.md) for the transport substrate.

---

## 1. What Pelorus Stream Is

Pelorus Stream is the high-bandwidth, non-safety-critical communication subsystem of the Pelorus marine data network. It transports media (audio for v1.0; video, sonar, and radar deferred), telemetry, and auxiliary data over a best-effort Ethernet network layer.

Stream is a **transport and media substrate**. It is not a database, not a state store, and not a coordinator. It moves bytes between nodes after higher-level policy (see the **Pelorus State** subsystem — [GitHub Issue #2](https://github.com/pelorus-marine/specifications/issues/2)) has decided that those bytes should move.

The specification, the reference implementations, the test fixtures, and the documentation are released under permissive open licenses. Documents are CC BY 4.0; code is MIT or Apache 2.0.

---

## 2. What Pelorus Stream Is Not

Stream **does not**:

- Carry safety-critical signals. Safety-critical traffic lives on Pelorus Core (CAN FD). See [`core/01-overview.md`](../core/01-overview.md).
- Represent authoritative system state. Authoritative state lives on Core; the **State subsystem** derives operational views from Core. See [Issue #2](https://github.com/pelorus-marine/specifications/issues/2).
- Make decisions. Prioritization, suppression, coordination, and arbitration are owned by the **State subsystem**. Stream advertises and transports; Stream does not choose.
- Influence Core behavior from ordinary Stream endpoints (anything toward Core is **gateway-only**—§3.1). A failed Stream subsystem must leave Core fully functional. A misbehaving Stream node must not be able to degrade Core.
- Guarantee delivery. The default transport is best-effort UDP. Reliable delivery is opt-in (QUIC) and is still subordinate to State decisions.
- Provide a control bus for actuators. Helm, autopilot, engine, and thruster commands belong on Core; Stream playback control (volume, pause) is the upper bound of what may be expressed on Stream.

These constraints are normative. A node that violates them is not Pelorus Stream conformant.

---

## 3. Three-Layer Architecture

Pelorus has two physical layers (Core and Stream) and one logical layer (State) that sits above both.

```
                 ┌───────────────────────────┐
                 │   Pelorus State           │  decisions, coordination
                 │   (spec: `state/` — #2)   │  prioritization, suppression
                 └─────────┬─────────────────┘
                           │ observes / commands
              ┌────────────┴────────────┐
              ▼                         ▼
   ┌──────────────────────┐   ┌──────────────────────┐
   │     Pelorus Core     │   │    Pelorus Stream    │
   │       (CAN FD)       │   │      (Ethernet)      │
   │ safety-critical,     │   │ best-effort, high    │
   │ authoritative state  │   │ bandwidth, non-      │
   │                      │   │ authoritative        │
   └──────────────────────┘   └──────────────────────┘
```

### 3.1 The Stream ↔ Core Boundary

Integration uses **gateway tiers** (summary also in [`ARCHITECTURE.md`](../ARCHITECTURE.md) §3; normative gateway responsibilities in [`core/09-gateway-specification.md`](../core/09-gateway-specification.md) §8):

1. **Core → Stream — standard gateway.** Core traffic (telemetry, identity, DCID-aligned metadata) is bridged onto the Ethernet substrate through the **standard gateway** path—the usual product bridge from Pelorus Core **CAN FD** to Pelorus Stream.

2. **Stream → Core — capable bidirectional gateway only.** Ethernet-origin traffic **shall not** be injected onto the Core fieldbus by arbitrary Stream publishers, unconstrained applications, or generic Stream stacks pretending to be Core talkers. **Reverse bridging** onto Core is permitted **only** through a **capable bidirectional gateway** that explicitly implements and validates the Stream→Core policy surface (future conformance tests in `stream/` and **`core/09`**).

3. **Ordinary Stream endpoints** shall **not** transmit on Core, originate Core frames, request Core-level actions outside gateway-mediated interfaces, or hold authoritative Core resources—except where (2) defines the gateway as the sole injection boundary.

Stream may **read** Core via published mechanisms **via (1)** (e.g. gateway-published identity tied to a Core NAME; mirrored telemetry).

A node that participates in both Core and Stream runs them as separate stacks with no shared safety-critical state. The two stacks may share an SoC and a clock; they shall not share an authoritative data path except through the gateway tiers above.

### 3.2 The Stream → State Boundary

Stream emits events ([`19-stream-event.md`](./19-stream-event.md)) and exposes per-stream observable state ([`20-stream-state.md`](./20-stream-state.md)). The **State subsystem** subscribes, aggregates, and derives intents; it does **not** perform transport. Stream itself never calls into State; State consumes Stream APIs.

This is enforced by direction-of-dependency: Stream code shall not link or import State APIs. State imports Stream.

The normative decomposition of State (event ingestion, reconstruction, policy, intents) is **not** defined in this directory; it is tracked in [**Define Pelorus State subsystem · Issue #2**](https://github.com/pelorus-marine/specifications/issues/2) and planned as a `state/` document set — see **[`state/00-document-index.md`](../state/00-document-index.md)** for the **non-overlapping** file layout (replaces the original 12-doc sketch in the issue).

### 3.3 Stream identities vs Core DCIDs (and VSS)

- **Pelorus Stream** identifies a *media or telemetry session* with a **stream ID** (UUIDv7 — [`02-stream-id.md`](./02-stream-id.md)). That is independent of any single CAN frame.
- **Pelorus Core** uses **DCIDs** (Data Contract IDs) as wire-level message contracts on CAN FD ([`core/07-dcid-registry.md`](../core/07-dcid-registry.md)).
- **Meaning** for Core-centric signals is canonical in the **`Vessel.*`** signal catalog ([`core/06-signal-catalog.md`](../core/06-signal-catalog.md)). The long-term relationship between **DCID evolution** and **VSS** is an **open design thread**: see [**Issue #3**](https://github.com/pelorus-marine/specifications/issues/3) and §6 in **`core/06-signal-catalog.md`**.

Telemetry on Stream should eventually trace to catalog semantics where applicable; until DCID and VSS roles are frozen, treat stream metadata and catalog paths as **paired documentation**, not as a single merged identifier space.

**Practical rule:** If a `telemetry` stream’s payloads are *the same quantities* a sailor would also find on **`Vessel.*`** (e.g. repeated GNSS position), the publisher **SHOULD** publish an optional metadata key `vss` (full `Vessel.*` path string) so gateways, tooling, and the State subsystem can correlate Stream bytes with Core semantics. The meaning of that path is defined only in [`core/06-signal-catalog.md`](../core/06-signal-catalog.md); Stream does not redefine VSS. See **`core/06` §6** (roles of VSS vs DCID) and [GitHub Issue #3](https://github.com/pelorus-marine/specifications/issues/3) for DCID evolution.

---

## 4. Physical and Network Substrate

Pelorus Stream runs on the Ethernet plant defined in [`core/01-overview.md` §3.2](../core/01-overview.md):

- M12 D-coded 4-pin connectors at 100 Mbit/s for v1.0
- M12 X-coded 8-pin reserved for future Gigabit profiles
- IPv6 link-local (`fe80::/10`) addressing on every interface
- mDNS-SD service discovery ([`23-discovery.md`](./23-discovery.md))
- No IPv4 dependency; nodes that bridge to legacy IPv4 networks do so out-of-scope of this specification

PoE strategy and switch topology are out of scope of this v0.1 draft and are tracked with [Pelorus Stream — Issue #1](https://github.com/pelorus-marine/specifications/issues/1) (design targets, bench validation, and deployment posture). Stream protocol behavior shall not depend on the presence of PoE.

---

## 5. v1.0 Scope

The v1.0 Stream specification covers the 28 documents listed in [`00-document-index.md`](./00-document-index.md). Within those:

| Area | In v1.0 | Deferred |
|---|---|---|
| Stream identity, type, payload, metadata | Yes (02–06) | — |
| UDP unicast / multicast transport | Yes (07–10) | — |
| QUIC reliable transport | Yes, opt-in (08) | DTLS-only profile |
| Control message protocol over UDP | Yes (11–14) | — |
| Audio streams | Yes (15–18) | Music libraries, multi-room sync |
| Video streams | Mentioned, not specified | v2.0+ |
| Sonar / radar streams | Mentioned, not specified | v2.0+ |
| File transfer streams | Type registered (03), not specified | v1.1 |
| mDNS discovery | Yes (23) | DNS-SD over Internet |
| Subscriptions and registry | Yes (22, 24) | Cross-segment federation |
| Errors | Yes (25–26) | Predictive failure analytics |
| Reference implementation | Rust crate skeleton (27) | Production-ready release |

### Explicitly Deferred From v1.0

- Video, sonar, radar, and chartplotter image streams. The model in 02–06 is general enough to admit them without renumbering; concrete formats are a v2.0+ activity.
- Reliable multicast (e.g. NORM, PGM). v1.0 reliable transport is unicast QUIC only.
- Encryption for in-vessel traffic. v1.0 assumes a trusted onboard LAN. TLS/QUIC encryption is supported by the transport but key management is deferred.
- Cross-vessel and cross-LAN streaming. v1.0 is single-vessel, link-local scope.

---

## 6. Design Principles

These guide every concrete decision in downstream documents.

- **Strictly non-safety-critical.** No exceptions. If a feature would create a safety dependency on Stream, it does not belong on Stream.
- **State decides, Stream transports.** Prioritization, suppression, and coordination are **State subsystem** concerns ([Issue #2](https://github.com/pelorus-marine/specifications/issues/2)). Stream documents specify mechanism, not policy.
- **Best-effort by default.** UDP is the default transport. Reliability is opt-in and bounded.
- **Bounded latency over guaranteed delivery.** For media, fresh data beats complete data. Buffering policy enforces this.
- **Discoverable, not configured.** Stream nodes appear via mDNS-SD with all the metadata a subscriber needs. Static configuration is permitted but never required for v1.0 in-vessel operation.
- **Open all the way down.** Specification, reference implementations, test fixtures. No purchases required to participate.
- **Honest about tradeoffs.** Open Items in each document record what is unresolved; nothing is hidden.

### 6.1 Alternatives under evaluation (non-normative)

The following are **design responses** to review feedback; they are **not** final choices until reflected in numbered Stream documents and validated on a bench reference.

| Topic | Current draft | Alternatives to evaluate |
|---|---|---|
| **Media datagram framing** | CBOR envelope + opaque PU body on UDP ([`12-envelope.md`](./12-envelope.md)) | **RTP/RTCP profile** (RFC 3550) for audio/video payloads with a thin Pelorus session header; or **fixed 16-byte binary header** + raw codec access unit for CPU-bound devices |
| **Multicast group allocation** | Hash-derived groups ([`09-routing.md`](./09-routing.md)) | **Central allocator** on the gateway (DHCP-style option or static table in announcement); or ** administratively assigned** pool per vessel with collision detection only |
| **UDP port plan** | `5354`/`5355` adjacent to mDNS ([`08-connection.md`](./08-connection.md)) | **IANA application-specific port** request or **dynamic port** in SRV only (no fixed payload port); eliminates LLMNR collision risk |
| **Service discovery load** | One mDNS instance per stream ([`23-discovery.md`](./23-discovery.md)) | **Pelorus Stream directory** agent (unicast + small mDNS pointer); subscribers browse directory first, reducing PTR churn on busy nets |
| **Security posture** | Trusted LAN; unauthenticated control; permissive QUIC certs ([`11-message.md`](./11-message.md)) | **Named profiles**: `open-boat` (today) vs `hardened` (mandatory TLS client auth, EDHOC/PSK, or COSE-signed control once key provisioning exists) — comparable to OneNet cyber ambition without mandating a certification monopoly |

---

## 7. Status and stability

The v0.1 specification is pre-release. [§9](#9-draft-design-targets-summary) lists **draft design targets** only. **No wire encoding, port, or codec choice is an interoperability commitment** until at least one reference triple (publisher, subscriber, display/listener) passes the smoke tests described in [`00-document-index.md`](./00-document-index.md) §4.

**Target directions** (subject to change before v1.0):

- Stream identifier format (UUIDv7, 128-bit) — *intended* baseline ([`02-stream-id.md`](./02-stream-id.md))
- Control-plane serialization (deterministic CBOR) — *intended* ([`13-serialization.md`](./13-serialization.md))
- Audio codec (Opus) at 48 kHz — *intended* ([`16-audio-format.md`](./16-audio-format.md))
- Sequential document numbering — contractual for *this repo* ([Issue #1](https://github.com/pelorus-marine/specifications/issues/1)); not a claim about field stability of every field inside each doc
- Service-discovery name (`_pelorus-stream._udp`) — *intended*; may add directory indirection later

Implementations targeting v0.x should expect frequent revision until the first bench-validated drop.

---

## 8. Where to Go Next

| If you want to... | Read |
|---|---|
| Understand what a stream *is* | [`02-stream-id.md`](./02-stream-id.md), [`03-stream-type.md`](./03-stream-type.md) |
| Implement the wire transport | [`08-connection.md`](./08-connection.md), [`09-routing.md`](./09-routing.md) |
| Implement an audio source or sink | [`15-audio-stream.md`](./15-audio-stream.md) → [`18-buffering.md`](./18-buffering.md) |
| Discover streams on a vessel | [`23-discovery.md`](./23-discovery.md), [`22-stream-registry.md`](./22-stream-registry.md) |
| Build a reference implementation | [`27-lib.md`](./27-lib.md) |
| See the Core companion | [`core/01-overview.md`](../core/01-overview.md) |
| Define the State subsystem (intents, policy) | [Issue #2 — Pelorus State](https://github.com/pelorus-marine/specifications/issues/2) (planned `state/`) |
| DCID evolution / PGN heritage | [Issue #3 — DCID exploration](https://github.com/pelorus-marine/specifications/issues/3) |
| See what is decided and why (Core + repo-wide) | [`ARCHITECTURE.md`](../ARCHITECTURE.md) |

---

## 9. Draft design targets (summary)

Downstream documents (02–27) state **normative-style** requirements and rationale as *drafts*. This section collects cross-cutting **targets** so other documents can cross-reference without repeating them. **Treat every item below as provisional** until bench validation; supersede with explicit RFCs or numbered doc revisions.

When Core’s **DCID** model and Issue #3 converge, update Stream cross-references (§3.3) so telemetry and catalog bindings stay aligned.

- **Boundary (this document, §2–3):** Stream is **strictly non-safety-critical**. Ordinary Stream endpoints shall **not** influence Core behavior directly (**§3.1**); any Stream→Core effect appears **only** through a **capable bidirectional gateway**. Decision logic (prioritization, suppression, coordination) is owned by the **State subsystem** ([Issue #2](https://github.com/pelorus-marine/specifications/issues/2)). Stream advertises and transports; State decides.

- **Physical layer:** Pelorus Stream uses the Ethernet plant defined in [`core/01-overview.md` §3.2](../core/01-overview.md). M12 D-coded 4-pin at 100 Mbit/s for v1.0; X-coded reserved for future Gigabit.

- **Network layer:** IPv6 link-local addressing on every Stream interface; no IPv4 requirement; mDNS-SD for service discovery.

- **Stream identity (02):** Each stream is identified by a 128-bit **UUIDv7**. Identifiers are publisher-minted, globally unique, time-sortable.

- **Stream type (03):** Type is a closed enumeration in v1.0: `audio`, `video` (reserved), `telemetry`, `file` (reserved), `control`. Unknown types shall be ignored by receivers.

- **Priority (04):** Priority is a **hint**, not authority. It is a small unsigned integer carried in announcements and may be mapped to DSCP for transport scheduling. Stream priority **shall not** preempt or otherwise interact with Core arbitration.

- **Transport (08):** Default transport is **UDP** (unicast or ASM/SSM multicast). Reliable transport is **opt-in QUIC** over the same UDP port range. No TCP. No DCCP.

- **Routing (09):** Multicast scope is **link-local** in v1.0 (`ff02::/16`). Unicast may use any link-local or stable ULA address advertised in mDNS.

- **Control plane encoding (10, 13):** Control messages use **deterministic CBOR** (RFC 8949 §4.2.1 deterministic encoding). No JSON on the wire. No XML. No vendor-defined binary frames.

- **Payload encoding (10):** Payload is opaque to the control plane. Per-type payload formats are defined in the Media subsystem (15–18).

- **Audio (15–18):** Audio uses **Opus** (RFC 6716) at **48 kHz**, mono or stereo, **20 ms** frames. Other sample rates are negotiated capabilities; 48 kHz is the mandatory baseline.

- **Versioning (14):** The protocol uses **semantic versioning** at the major-minor level (`vX.Y`). Capability negotiation is bit-vector based and forward-compatible. Receivers shall ignore unknown capability bits.

- **Discovery (23):** Streams are advertised via mDNS-SD under the service type **`_pelorus-stream._udp`**. The TXT record schema is defined in [`23-discovery.md`](./23-discovery.md).

- **Registry (22):** The registry is **distributed and eventually-consistent**. No single node is the authoritative registry. Caches are advisory.

- **Reference implementation (27):** The official reference is **Rust**, **`forbid(unsafe_code)`** at the crate root, with the realtime media path designed to avoid heap allocation.

- **Sequential numbering (Issue #1):** Document numbering 00–27 is part of the specification contract. Numbers shall not be reassigned or reordered without a documented version bump.

---

## 10. License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
