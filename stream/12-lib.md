# Pelorus Stream — Reference Library

The reference Rust library that implements Pelorus Stream and the public API surface other code links against. Cross-subsystem implementation guidance, including the Pelorus Platform repository pointer, is in [`../implementation/`](../implementation/).

## 1. Crate Inventory

| Crate | Purpose | Status |
| --- | --- | --- |
| `pelorus-stream` | Top-level entry; re-exports the public API | Planned |
| `pelorus-stream-id` | Stream ID (UUIDv7) types and helpers | Planned |
| `pelorus-stream-cbor` | Pelorus-CBOR-1 deterministic encoder/decoder | Planned |
| `pelorus-stream-quic` | QUIC transport, dual-fabric connection management, datagram header | Planned |
| `pelorus-stream-redundancy` | Dual-fabric state machine, DDT, RedBox primitives | Planned |
| `pelorus-stream-radar` | Radar video and control framing | Planned |
| `pelorus-stream-charts` | HTTP/3 client/server for S-100 distribution | Planned |
| `pelorus-stream-discovery` | mDNS-SD service browse and advertise | Planned |
| `pelorus-stream-registry` | Local registry implementation | Planned |
| `pelorus-mdns` | `no_std` mDNS responder for embedded targets | Planned |
| `statime` (third-party) | IEEE 802.1AS / IEEE 1588 (gPTP) | External |

All Pelorus crates publish under the `pelorus-marine` GitHub organisation. License is MIT or Apache 2.0 per crate.

## 2. Implementation Principles

- **Rust 2024 edition** (or latest stable at release).
- **`forbid(unsafe_code)`** at every crate root. Unsafe is permitted only behind FFI boundaries.
- **`no_std`-friendly** for `pelorus-stream-id` and `pelorus-stream-cbor`. Transport, discovery, registry crates require `std`.
- **No heap allocation in the realtime media path.** Buffers are pre-allocated; PUs are encoded into caller-provided slices.
- **Async via `tokio` for `std` builds.** Embedded targets use `embassy` async or callback APIs.
- **Determinism over cleverness.** Where the spec gives one canonical encoding, the library exposes one canonical encoder.

## 3. Reference QUIC Stack

| Concern | Crate |
| --- | --- |
| QUIC transport | `quinn` (pure Rust QUIC) |
| TLS 1.3 | `rustls` (pure Rust) |
| Async runtime | `tokio` |

The entire Stream transport stack is implementable in pure Rust with no C dependencies. HTTP/3 (for the chart distribution service) layers on top of `quinn` via `h3` or equivalent.

## 4. Public API Sketch

Signatures are illustrative; full API surfaces stabilise before v1.0 ships.

### 4.1 `Stream` — opaque handle

```rust
pub struct Stream { /* ... */ }

impl Stream {
    pub fn id(&self) -> StreamId;
    pub fn metadata(&self) -> &Metadata;
    pub fn state(&self) -> StreamState;
    pub fn close(self);
}
```

### 4.2 Publisher

```rust
pub struct Publisher { /* ... */ }

impl Publisher {
    pub fn new(node: NodeIdentity, fabric: FabricBindings) -> Result<Self>;

    pub fn announce_radar_video(&mut self, instance: u16, meta: RadarMetadata) -> Result<RadarVideoStream>;
    pub fn announce_telemetry(&mut self, meta: TelemetryMetadata) -> Result<TelemetryStream>;
    pub fn serve_charts(&mut self, root: &Path, meta: ChartMetadata) -> Result<ChartServer>;

    pub fn shutdown(self);
}
```

`RadarVideoStream`, `TelemetryStream`, and `ChartServer` extend `Stream` with type-specific emit APIs.

### 4.3 Subscriber

```rust
pub struct Subscriber { /* ... */ }

impl Subscriber {
    pub fn new(fabric: FabricBindings) -> Result<Self>;
    pub fn subscribe(&mut self, sid: StreamId) -> Result<Subscription>;
}

pub struct Subscription { /* ... */ }

impl Subscription {
    pub fn next_pu(&mut self) -> Option<Pdu<'_>>;
    pub fn events(&self) -> &EventChannel;
    pub fn unsubscribe(self);
}
```

`next_pu()` blocks on async runtimes; on `no_std` it integrates with the embedded executor.

### 4.4 Registry

```rust
pub struct Registry { /* ... */ }

impl Registry {
    pub fn new(fabric: FabricBindings) -> Result<Self>;
    pub fn list(&self) -> Vec<RegistryEntry>;
    pub fn get(&self, id: StreamId) -> Option<RegistryEntry>;
    pub fn watch(&self) -> RegistryWatcher;
}
```

`RegistryWatcher` yields add/remove/update notifications.

### 4.5 Errors

All public APIs return `Result<T, StreamError>` where `StreamError` carries the codes from [`11-events-and-errors.md`](./11-events-and-errors.md). Errors are non-exhaustive enums so adding new variants does not break callers.

## 5. Embedded Targets

| Target | Status |
| --- | --- |
| Linux x86_64 (gateway, head units) | Primary |
| Linux ARM (Raspberry Pi-class gateways) | Primary |
| FreeBSD | Best-effort |
| `no_std` ARM Cortex-M (embedded sensors) | Publish/subscribe only; no registry |
| `no_std` ESP32 / similar Wi-Fi MCUs | Subscribe only |

Embedded targets ship as a profile that excludes the registry, full QUIC, and full mDNS responder. They depend on a gateway-class node to handle service discovery.

## 6. Testing

The reference library ships with:

- **Unit tests** for every crate, run on every commit.
- **Test vectors** for CBOR encodings: encoding tests assert byte-equality against canonical hex strings.
- **Conformance fixtures** under `tests/conformance/`: end-to-end exchanges with golden traces.
- **Loom tests** for any concurrency-sensitive code paths.
- **Fuzz harnesses** for the CBOR decoder and the envelope parser, run continuously in CI.

A device or software component is considered Pelorus Stream conformant only by passing the conformance fixtures and self-declaring per a future `stream-conformance` document.

## 7. Bindings

C ABI bindings will be provided via a `pelorus-stream-c` crate using `cbindgen`. The C ABI exposes a subset of the API suitable for non-Rust embedders. Python (`pyo3`) and Node.js (`napi`) bindings are out of scope for v1.0 reference work; community projects are welcome.

## 8. Configuration

A typical Pelorus Stream node has a small configuration surface:

| Field | Default | Notes |
| --- | --- | --- |
| `fabric_a_interface` | first M12 X-coded port found | Fabric A Ethernet interface |
| `fabric_b_interface` | second M12 X-coded port found | Fabric B Ethernet interface (Class D only) |
| `pub_id` | hostname-derived | Publisher identifier |
| `default_lease_ms` | 60000 | Default subscription lease |
| `idle_close_ms` | 300000 | Idle window before publisher closes |
| `node_class` | `D` if two interfaces present, else `S` | Class D / Class S |

Configuration is loaded from a TOML file in the reference library. Values may be overridden at runtime via the `pelorus-stream` API.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
