# Pelorus Stream — Specification Document Index

**Version:** Living  
**Last Updated:** April 27, 2026  
**Status:** Active  
**Trust:** Trusted

---

## About This Document

Numbered list of all documents that constitute a complete Pelorus Stream specification. Numbers are stable references — once assigned, they do not change. New documents get new numbers; deprecated documents are marked but keep their numbers.

**Layout:** All Pelorus Stream documents (`00`–`27`) live in the repository's [`stream/`](.) directory (e.g. `specifications/stream/01-overview.md`). Project-wide community files (`README.md`, `LICENSE.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`) live at the **repository root** next to `core/` and `stream/` and apply to both subsystems.

This index is the authoritative list of Pelorus Stream specification documents. Update when documents are added, drafted, or completed.

Related: [`core/00-document-index.md`](../core/00-document-index.md) lists the safety-critical Pelorus Core specifications.

---

## 1. Trust Levels

Every document is annotated with a trust level so contributors know what to rely on:

- **Trusted** — written deliberately against external sources or published suite summaries; cited content is verified.
- **Unverified** — provisional draft of unknown provenance. Content has not been validated, may contradict trusted documents, may invent terms. Treat as a starting guess until reviewed.
- **Final** — frozen and not expected to change.

---

## 2. Document Index

### Root

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 00 | [`00-document-index.md`](./00-document-index.md) | This index | Living | Trusted |
| 01 | [`01-overview.md`](./01-overview.md) | What Pelorus Stream is, architecture role, draft design targets (§9) | v0.1 draft | Unverified |

### Tier 1 — Model

The abstract, transport-independent description of what a Stream is.

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 02 | [`02-stream-id.md`](./02-stream-id.md) | Stream identifier format and lifetime rules | v0.1 draft | Unverified |
| 03 | [`03-stream-type.md`](./03-stream-type.md) | Stream type classification (audio, video, telemetry, file, control) | v0.1 draft | Unverified |
| 04 | [`04-stream-priority.md`](./04-stream-priority.md) | Soft priority hint model — non-authoritative, non-Core-influencing | v0.1 draft | Unverified |
| 05 | [`05-stream-payload.md`](./05-stream-payload.md) | Payload abstraction, framing, opaque-vs-typed distinction | v0.1 draft | Unverified |
| 06 | [`06-stream-metadata.md`](./06-stream-metadata.md) | Per-stream metadata schema and lifecycle | v0.1 draft | Unverified |

### Tier 2 — Transport

The wire-level realization of streams over Pelorus Stream Ethernet.

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 07 | [`07-session.md`](./07-session.md) | Session lifecycle: open, advertise, close, lifetime | v0.1 draft | Unverified |
| 08 | [`08-connection.md`](./08-connection.md) | Connection model: UDP unicast, UDP multicast, QUIC reliable | v0.1 draft | Unverified |
| 09 | [`09-routing.md`](./09-routing.md) | Routing: IPv6 link-local, multicast group allocation, scope | v0.1 draft | Unverified |
| 10 | [`10-encoding.md`](./10-encoding.md) | Wire encoding policies: control plane vs. payload plane | v0.1 draft | Unverified |

### Tier 3 — Protocol

The on-wire control-plane protocol that announces, opens, and tears down streams.

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 11 | [`11-message.md`](./11-message.md) | Stream control message taxonomy and required fields | v0.1 draft | Unverified |
| 12 | [`12-envelope.md`](./12-envelope.md) | Common message envelope, header, integrity | v0.1 draft | Unverified |
| 13 | [`13-serialization.md`](./13-serialization.md) | Deterministic CBOR serialization for the control plane | v0.1 draft | Unverified |
| 14 | [`14-versioning.md`](./14-versioning.md) | Protocol version field, capability negotiation, compatibility | v0.1 draft | Unverified |

### Tier 4 — Media subsystem

Audio is the only media class specified for v1.0. Video, sonar, and radar are deferred.

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 15 | [`15-audio-stream.md`](./15-audio-stream.md) | Audio stream specialization, channel model, latency budget | v0.1 draft | Unverified |
| 16 | [`16-audio-format.md`](./16-audio-format.md) | Codec (Opus), sample rate, framing, packetization | v0.1 draft | Unverified |
| 17 | [`17-playback-control.md`](./17-playback-control.md) | Play, pause, seek, volume — soft control, no Core authority | v0.1 draft | Unverified |
| 18 | [`18-buffering.md`](./18-buffering.md) | Adaptive jitter buffer, bounded latency, drop policy | v0.1 draft | Unverified |

### Tier 5 — Events

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 19 | [`19-stream-event.md`](./19-stream-event.md) | Stream event model: emitted, never authoritative | v0.1 draft | Unverified |
| 20 | [`20-stream-state.md`](./20-stream-state.md) | Per-stream observable state machine | v0.1 draft | Unverified |
| 21 | [`21-stream-update.md`](./21-stream-update.md) | Update messages: metadata, capability, state-change publication | v0.1 draft | Unverified |

### Tier 6 — Registry & Discovery

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 22 | [`22-stream-registry.md`](./22-stream-registry.md) | Distributed, eventually-consistent registry of active streams | v0.1 draft | Unverified |
| 23 | [`23-discovery.md`](./23-discovery.md) | mDNS-SD service types, TXT records, scope rules | v0.1 draft | Unverified |
| 24 | [`24-subscription.md`](./24-subscription.md) | Subscribe/unsubscribe semantics, lease, renewal | v0.1 draft | Unverified |

### Tier 7 — Errors

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 25 | [`25-stream-error.md`](./25-stream-error.md) | Application-level stream error taxonomy | v0.1 draft | Unverified |
| 26 | [`26-transport-error.md`](./26-transport-error.md) | Transport-level error taxonomy and recovery | v0.1 draft | Unverified |

### Tier 8 — Core Interface

| # | Filename | Purpose | Status | Trust |
|---|---|---|---|---|
| 27 | [`27-lib.md`](./27-lib.md) | Reference Rust library entry point: public API surface | v0.1 draft | Unverified |

---

## 3. Numbering Conventions

- Stream documents use numeric prefixes (`00-`, `01-`, … `27-`) under `stream/` so they sort in logical reading order.
- Numbers are **assigned at document creation and never reused**. Sequential ordering is part of the specification contract per Issue [#1](https://github.com/pelorus-marine/specifications/issues/1) and shall not be renumbered without a versioning update of the whole stream specification.
- Deprecated documents keep their number but are marked deprecated in this index.
- New documents receive the next free number; sections may grow but the numbering remains flat.

---

## 4. Completion Tracking

**Trusted or final:** 1 of 28

- `stream/00-document-index.md` (procedural index only)

**Unverified — needs review:** 27 of 28

- `stream/01-overview.md` … `stream/27-lib.md` — including **`01-overview.md`**, which carries **draft** cross-cutting targets (§9) and **must not** be treated as field-validated until [`01-overview.md`](./01-overview.md) §7 criteria are met.

**Next priorities for v0.1 specification:**

1. Pelorus Core v0.1 stabilization (`core/01`–`core/04`) takes precedence; Stream v0.1 is concurrent draft work that **must not** drive Core changes.
2. Validate the draft design targets in [`01-overview.md`](./01-overview.md) §9 against a working bench-top reference (one publisher, one subscriber, one display head) before treating any Stream wire choice as stable. Track the **Pelorus State** specification in [Issue #2](https://github.com/pelorus-marine/specifications/issues/2).
3. Resolve the open items recorded in each document's *Open Items* section. Reconcile **DCID** evolution ([Issue #3](https://github.com/pelorus-marine/specifications/issues/3)) with Stream telemetry metadata in `core/06-signal-catalog.md` §6.

---

## 5. Relationship to Pelorus Core

Pelorus Stream is **strictly non-safety-critical** and **never** influences Core behavior. See [`01-overview.md` §3](./01-overview.md) and [`core/01-overview.md` §9](../core/01-overview.md#9-cross-cutting-decisions-authoritative-summary) for Core coupling context.

- **DCID evolution / Core mapping** — open exploration: [Issue #3 — DCID (Data Contract ID)](https://github.com/pelorus-marine/specifications/issues/3). Semantic anchoring in **`Vessel.*`**: [`core/06-signal-catalog.md`](../core/06-signal-catalog.md) §6.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
