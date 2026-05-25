# Pelorus Catalog — Trace Format (ASAM MDF4 Profile)

Pelorus adopts **ASAM MDF 4.2** as the on-disk format for capturing Core (CAN FD) and Stream (Ethernet/QUIC) traffic, time-aligned across subsystems, with the metadata needed for reproducible analysis. This document defines the Pelorus profile of MDF4 — which block types and channel-group conventions Pelorus uses, the required metadata header, and the time-base alignment. The MDF4 format itself is normative in the ASAM standard; this document does not restate it.

The reference implementation is [`platform/mdf4-rs`](https://github.com/pelorus-marine/platform/tree/main/mdf4-rs) — a no_std + alloc Rust crate that reads and writes Pelorus-profile MDF4 files (CAN, CAN FD, Ethernet, DBC integration, optional zstd-class compression).

## 1. Scope

A Pelorus trace is a `.mf4` file capturing one or more of:

- Core CAN FD frames from one or more bus segments (Fabric A, Fabric B, or both)
- Stream Ethernet frames or QUIC datagrams from one or more fabrics
- Time master signal for cross-subsystem alignment
- Calibration and configuration snapshot at capture start

The format supports four distinct use cases simultaneously; consumers select on need:

| Use case | Consumer | Notes |
| --- | --- | --- |
| **Debug trace** | Developer / integrator | Short-form capture of a specific scenario; opened in Vector CANalyzer, asammdf, dewesoft, or `mdf4-rs` |
| **Conformance fixture** | Test suite | Golden trace + expected-behaviour assertions; CI-replayable |
| **Voyage Data Recorder (VDR)** | Owner / investigator | Long-form continuous capture; basis for incident reconstruction; optional SOLAS-VDR-compatible profile in future |
| **ML training data** | Community / research | Owner-shared captures into an opt-in pool; basis for marine ML datasets |

The minimum-viable format defined here covers all four. Use-case-specific extensions (privacy modes, voyage segmentation triggers, SOLAS-compatible profile, tamper-evidence) are tracked in [`../../ISSUES.md`](../../ISSUES.md) and are post-v1.0.

## 2. Format Anchor

Pelorus traces conform to **ASAM MDF 4.2.0** (or later compatible revision). The ASAM MDF specification is the normative reference for block layouts, byte order, and link semantics.

Pelorus does not extend or modify MDF4 wire structures. The Pelorus profile is expressed entirely as conventions on existing MDF4 blocks (HD, FH, DG, CG, CN, SI, MD, TX) and content of the standard ASAM bus-event channel groups (`CAN_DataFrame`, `ETH_Frame`).

## 3. Channel Group Conventions

Each captured bus segment is one MDF4 channel group, using the standard ASAM bus-event channel layout.

### 3.1 Core CAN FD Bus Segments

One channel group per Core bus segment, carrying `CAN_DataFrame` records (ASAM bus-event CAN frame).

| Channel group attribute | Pelorus convention |
| --- | --- |
| `cg_path_separator` | `.` |
| Source name (SI block) | `PelorusCore.FabricA` or `PelorusCore.FabricB` (single-bus deployments use `PelorusCore` only) |
| Master channel | Absolute timestamp in seconds (or ns; see §5), zero at HD-block start time |
| Frame records | `CAN_DataFrame` with 29-bit identifier carrying Pelorus `[PRIO 3b | DC_ID 18b | SA 8b]` per `core/03-data-link.md §2` |
| CAN FD flag | Set per frame; Pelorus Core is CAN FD throughout |
| BRS / ESI | Recorded per actual bus signalling |

The bus identifier triple (priority, DC_ID, source address) is captured raw in the CAN ID; DBC-based decoding to named signals (§6) is optional and applied at read time.

### 3.2 Stream Ethernet/QUIC

One channel group per Stream fabric, carrying `ETH_Frame` records (ASAM bus-event Ethernet frame).

| Channel group attribute | Pelorus convention |
| --- | --- |
| Source name (SI block) | `PelorusStream.FabricA` or `PelorusStream.FabricB` |
| Master channel | Same time base as the Core channel groups (§5) |
| Frame records | `ETH_Frame` with full Ethernet headers + payload (typically IPv6 + UDP + QUIC) |
| VLAN tags | Captured if present (802.1Q) |
| Direction | Tx / Rx flag per frame |
| Jumbo frames | Supported per `ETH_Frame` length field |

Stream traces capture the QUIC datagram in the UDP payload; per-service decoding (radar video spokes, route plan, etc.) is downstream of the format and lives with the per-service tooling.

### 3.3 Master Channel and Time Alignment

All channel groups within a single trace share **one absolute time base**, anchored at the HD-block start time. The time base is sourced from:

- **Pelorus TimeSync** (`Pelorus.TimeSync` per [`../core/07-dcid-registry.md`](../core/07-dcid-registry.md)) when capturing Core traffic from a synchronised vessel
- **IEEE 802.1AS / gPTP** (per [`../stream/09-time-sync.md`](../stream/09-time-sync.md)) when capturing Stream traffic
- **Capture-host monotonic clock** as a fallback when neither is available (flagged in HD-block metadata; see §4)

When both Pelorus TimeSync and gPTP are present and in agreement, recorders shall use Pelorus TimeSync as the primary master; gPTP is the cross-check. Disagreement beyond the `AccuracyBucket` declared by Pelorus TimeSync shall be recorded as a discontinuity event (`##EV` block) in the trace.

Master channel resolution shall be ≤ 1 µs; nanosecond resolution is preferred and is what `mdf4-rs` emits by default.

## 4. Required Metadata Header

The HD block and the first FH block carry Pelorus-required metadata. All fields are MDF4 standard mechanisms (HD time fields, FH comment / tool ID, MD block XML metadata, TX block text).

| Field | MDF4 location | Pelorus content |
| --- | --- | --- |
| Capture start time (absolute UTC) | HD `hd_start_time_ns` + `hd_tz_offset_min` | Time of the first record across all channel groups |
| Time-base source | HD `MD` block (XML) | One of `pelorus-timesync`, `gptp`, `host-monotonic` |
| Vessel identifier | HD `MD` block (XML) | Stable per-vessel string set at commissioning. May be a pseudonym if privacy mode is active (see ISSUES.md for privacy extensions). |
| Vessel-class taxonomy | HD `MD` block (XML) | Enum slot per the planned vessel-class taxonomy (tracked in ISSUES.md); recorded literally as `unknown` until taxonomy lands |
| Calibration ID + hash | HD `MD` block (XML) | Identifier and SHA-256 of the calibration set in effect at capture time (per [`../state/02-system-model.md §4`](../state/02-system-model.md)) |
| Software versions | First FH block, one entry per participating node | Format: `<role>@<node-name> <version>`; collected from the address-claim cache at capture start |
| Pelorus spec version | First FH block `fh_tool_id` | Pelorus specification revision the recorder was built against (e.g. `pelorus-spec/v0.x`) |
| Recorder identity | First FH block `fh_tool_id` / `fh_tool_vendor` | The recording tool identification (e.g. `mdf4-rs/0.3.1`) |
| Privacy mode flag | HD `MD` block (XML) | One of `full`, `position-quantised`, `anonymised`. v1.0 supports only `full`; quantised and anonymised modes are tracked in ISSUES.md. |

Each modification to a trace file (cut, merge, post-hoc annotation) shall append a new `##FH` block per ASAM MDF4 convention, preserving the audit chain.

## 5. Compression

Pelorus traces MAY use `##DZ` (deflate-compressed data) blocks per ASAM MDF4. Compression is recommended for VDR-class long-form captures (3–5× compaction on typical Pelorus traffic per `mdf4-rs` benchmarks). Compression MAY be omitted for short debug captures.

Recorders shall declare compression support; readers shall handle both compressed and uncompressed `##DT` / `##DZ` data blocks. `mdf4-rs` exposes this behind the `compression` feature (requires `alloc`).

## 6. DBC Mapping for Catalog Signals

A Pelorus catalog leaf identifies a Pelorus DC payload field. The relationship to MDF4 channel decoding goes through ASAM-MDF4's DBC overlay mechanism:

- A **DBC file** is generated from the catalog: each `data_contract` + `dc-field` overlay declares a CAN ID range and signal layout in DBC syntax.
- The DBC file is **distributed alongside the trace** (or embedded as an MDF4 `##AT` Attachment block) so any tool with DBC support can decode raw CAN frames to named `Vessel.*` paths.
- `mdf4-rs` provides a `CanDbcLogger` (live decoding during capture) and `DbcOverlayReader` (post-hoc decoding of raw traces) under the `dbc` feature.

Generating the DBC from `catalog/vessel.vspec` is a tooling concern (per [`05-tooling.md`](./05-tooling.md)); the format and convention for the generated DBC are not pinned in v1.0 beyond "valid DBC consumable by the open `dbc-rs` library."

## 7. Reader / Writer Profiles

Two profiles to support different deployment classes:

### 7.1 Minimal Writer Profile

Sufficient for MCU-class capture nodes that may not carry a full MDF4 writer. A recorder is **profile-conformant minimal** if it produces files containing:

- ID block (mandatory, ASAM)
- HD block with Pelorus required metadata (§4)
- One or more FH blocks (initial + per-modification)
- One DG / CG / CN tree per captured bus segment
- `##DT` data blocks (uncompressed) carrying records
- Time channel resolving to monotonic ns-or-µs scale

This subset omits: `##DZ` compression, `##SD` signal data, `##DL` data lists, `##EV` event blocks, `##AT` attachments. Readers shall accept minimal-profile files.

The fully-bounded heapless writer that runs without `alloc` is in [`mdf4-rs` ARCHITECTURE backlog](https://github.com/pelorus-marine/platform/tree/main/mdf4-rs#bounded-heapless-writers-backlog) and is not part of the v1.0 trace-format commitment.

### 7.2 Full Writer Profile

Used by gateway-class nodes, dedicated VDR appliances, and post-processing tools. Adds: `##DZ` compression, `##DL` data lists for files > 100 MB, `##EV` events for discontinuities and operator-marked points, `##AT` attachments for the trace-companion DBC and calibration export.

`mdf4-rs` with default features implements the full writer profile.

## 8. Reference Implementation

[`platform/mdf4-rs`](https://github.com/pelorus-marine/platform/tree/main/mdf4-rs) is the Pelorus reference implementation. It provides:

- Full MDF4 read/write, no_std + alloc, `#![forbid(unsafe_code)]`
- CAN, CAN FD, and Ethernet bus-event loggers in ASAM format
- DBC integration via the `dbc-rs` sibling crate
- Optional compression (`##DZ`) via `miniz_oxide`
- Streaming index for large-file random access

Implementers building Pelorus-conformant tools shall either use `mdf4-rs` directly or pass the Pelorus profile test suite that ships alongside it (test suite location and CI invocation are tracked with `../core/11-conformance.md`).

## 9. Use-Case Notes

The format is one artifact serving four use cases; consumers do not need permission from Pelorus to invoke any of them:

- **Debug traces** are short, often uncompressed, opened locally in a tool of choice. The `mdf4-rs` examples include a CLI capture-and-inspect tool.
- **Conformance fixtures** are stored alongside the Pelorus reference test suite (per [`../core/11-conformance.md`](../core/11-conformance.md)). Each fixture is a `.mf4` capture plus an expected-behaviour assertion file.
- **VDR (Voyage Data Recorder)** captures are long-form continuous recordings, typically compressed, segmented by voyage. A SOLAS-VDR-compatible profile (regulated black box for vessels > 3000 GT per IMO SOLAS / IEC 61996) is tracked in ISSUES.md and is not v1.0.
- **Community ML data** comes from owner-elected sharing of voyage captures to an opt-in pool. The pool, the anonymisation toolchain, and federated-learning infrastructure are tracked in ISSUES.md and depend on this trace format shipping first.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
