# Pelorus Stream — Reference Library (Core Interface)

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specifies the **reference Rust library** that implements Pelorus Stream and the public API surface that other code links against. The companion document for Pelorus Core is [`core/11-reference-implementations.md`](../core/11-reference-implementations.md); this is the Stream-specific equivalent.

The library is the *Core Interface* in the sense of "the entry point of the `pelorus-stream` crate" — not Pelorus Core. The two are related only by name.

---

## 1. Crate Inventory

| Crate | Purpose | Status |
|---|---|---|
| `pelorus-stream` | Top-level entry; re-exports the public API | Planned |
| `pelorus-stream-id` | Stream ID (UUIDv7) types and helpers | Planned |
| `pelorus-stream-cbor` | Pelorus-CBOR-1 deterministic encoder/decoder | Planned |
| `pelorus-stream-transport` | UDP unicast/multicast/QUIC transport | Planned |
| `pelorus-stream-audio` | Opus integration, format codes, frame helpers | Planned |
| `pelorus-stream-discovery` | mDNS-SD service browse and advertise | Planned |
| `pelorus-stream-registry` | Local registry implementation | Planned |
| `pelorus-mdns` | `no_std` mDNS responder (used by stream-discovery on embedded targets) | Planned |

All crates publish under the `pelorus-marine` GitHub organization. License is MIT or Apache 2.0 per crate.

---

## 2. Implementation Principles

The Stream reference library follows Pelorus's general rules ([`core/01-overview.md` §9](../core/01-overview.md)) plus stream-specific ones:

- **Rust 2024 edition** (or latest stable at the time of release).
- **`forbid(unsafe_code)`** at every crate root. Unsafe is permitted only behind FFI boundaries (e.g. `libopus`).
- **`no_std`-friendly** for `pelorus-stream-id`, `pelorus-stream-cbor`, and the on-wire types in `pelorus-stream-audio`. The transport, discovery, and registry crates require `std`.
- **No heap allocation in the realtime audio path.** Buffers are pre-allocated; PUs are encoded into caller-provided slices.
- **Async via `tokio` for `std` builds.** Embedded targets use `embassy` async or callback APIs.
- **Determinism over cleverness.** Where the spec gives one canonical encoding, the library exposes one canonical encoder.

---

## 3. Public API Sketch

The top-level `pelorus_stream` crate exposes the surfaces below. Signatures are illustrative; full API surfaces stabilize before v1.0 ships.

### 3.1 `Stream` — opaque handle

```rust
pub struct Stream { /* ... */ }

impl Stream {
    pub fn id(&self) -> StreamId;
    pub fn metadata(&self) -> &Metadata;
    pub fn state(&self) -> StreamState;
    pub fn close(self);
}
```

A `Stream` is created by a publisher or subscriber API. It owns its session resources; `Drop` closes gracefully.

### 3.2 Publisher

```rust
pub struct Publisher { /* ... */ }

impl Publisher {
    pub fn new(node: NodeIdentity, network: NetworkBindings) -> Result<Self>;

    pub fn announce_audio(
        &mut self,
        format: AudioFormat,
        meta: AudioMetadata,
    ) -> Result<AudioStream>;

    pub fn announce_telemetry(
        &mut self,
        meta: TelemetryMetadata,
    ) -> Result<TelemetryStream>;

    pub fn shutdown(self);
}
```

`AudioStream` and `TelemetryStream` extend `Stream` with type-specific PU emit APIs.

### 3.3 Subscriber

```rust
pub struct Subscriber { /* ... */ }

impl Subscriber {
    pub fn new(network: NetworkBindings) -> Result<Self>;

    pub fn subscribe(&mut self, sid: StreamId) -> Result<Subscription>;
}

pub struct Subscription { /* ... */ }

impl Subscription {
    pub fn next_pu(&mut self) -> Option<Pdu<'_>>;
    pub fn events(&self) -> &EventChannel;
    pub fn unsubscribe(self);
}
```

The `next_pu()` method blocks on async runtimes; on `no_std` it integrates with the embedded executor.

### 3.4 Registry

```rust
pub struct Registry { /* ... */ }

impl Registry {
    pub fn new(network: NetworkBindings) -> Result<Self>;
    pub fn list(&self) -> Vec<RegistryEntry>;
    pub fn get(&self, id: StreamId) -> Option<RegistryEntry>;
    pub fn watch(&self) -> RegistryWatcher;
}
```

The `RegistryWatcher` yields add/remove/update notifications.

### 3.5 Errors

All public APIs return `Result<T, StreamError>` where `StreamError` carries the codes from [`25-stream-error.md`](./25-stream-error.md) and [`26-transport-error.md`](./26-transport-error.md). Errors are non-exhaustive enums so adding new variants does not break callers.

---

## 4. Embedded Targets

Pelorus Stream's reference library targets:

| Target | Status |
|---|---|
| Linux x86_64 (gateway, head units) | Primary |
| Linux ARM (Raspberry Pi-class gateways) | Primary |
| FreeBSD | Best-effort |
| `no_std` ARM Cortex-M (embedded amplifiers) | Audio publish/subscribe only; no registry |
| `no_std` ESP32 / similar Wi-Fi MCUs | Audio subscribe only |

Embedded targets ship as a profile that excludes the registry, QUIC, and full mDNS responder. They depend on a gateway-class node to handle service discovery. On a vessel without a gateway, embedded nodes still publish via the embedded mDNS responder but at higher resource cost.

---

## 5. Testing

The reference library ships with:

- **Unit tests** for every crate, run on every commit.
- **Test vectors** matching [`13-serialization.md` §8](./13-serialization.md). Encoding tests assert byte-equality against canonical hex strings.
- **Conformance fixtures** under `tests/conformance/`: end-to-end exchanges with golden traces.
- **Loom tests** for any concurrency-sensitive code paths.
- **Fuzz harnesses** for the CBOR decoder and the envelope parser, run continuously in CI.

A device or software component is considered Pelorus Stream conformant only by passing the conformance fixtures and self-declaring per a future `stream-conformance` document (analogous to [`core/15-conformance-test-plan.md`](../core/15-conformance-test-plan.md)).

---

## 6. Linking and Distribution

Pelorus Stream crates publish to **crates.io** under their canonical names. Pre-release versions track the specification minor version (`0.1.x` for spec v0.1).

Implementations targeting v0.x should pin to a specific minor; v1.0 of the library will follow v1.0 of the specification.

---

## 7. Bindings

C ABI bindings will be provided via a `pelorus-stream-c` crate using `cbindgen`. The C ABI exposes a subset of the API suitable for non-Rust embedders. C bindings are best-effort and do not block the Rust release.

Python bindings (via `pyo3`) and Node.js bindings (via `napi`) are out of scope for v1.0 reference work. Community projects are welcome to add them.

---

## 8. Configuration

A typical Pelorus Stream node has a small configuration surface:

| Field | Default | Notes |
|---|---|---|
| `interface` | first M12 D-coded port found | Stream Ethernet interface |
| `pub_id` | hostname-derived | Publisher identifier |
| `default_lease_ms` | 60000 | Default subscription lease |
| `idle_close_ms` | 300000 | Idle window before publisher closes |
| `enable_quic` | true | Whether to advertise `mode-quic` |
| `enable_multicast` | true | Whether to advertise multicast modes |
| `max_subscribers_unicast` | 4 | Per-stream subscriber cap on unicast streams |

Configuration is loaded from an embedded TOML file in the reference library. Values may be overridden at runtime via the `pelorus-stream` API.

---

## 9. Open Items

- Crate boundaries (top-level vs. split) — currently the seven-crate layout above.
- Async runtime selection — `tokio` recommended; library should compile against alternative runtimes via feature flags.
- C ABI surface — to settle once Rust API is stable.
- Conformance test plan document number — TBD when first conformance documentation is drafted.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
