# Pelorus Core — Instance Binding

Core-specific resolution of bus-level identifiers to catalog indices. The semantic `Vessel.*` catalog and indexed-array conventions are normative in [`../catalog/`](../catalog/); this document defines how Pelorus Core resolves a Source Address + NAME + DC_ID + DC-internal instance field to a `Vessel.*[n]` index.

## 1. Binding Table

The mapping

```text
(Source Address, 64-bit NAME, DC_ID, DC-internal instance field) → Vessel.*[n] index
```

is held in a per-vessel **binding table**. Entries are stable across power cycles and persist until the operator explicitly edits them.

Each entry may carry sailor-assigned friendly labels ("Port Main", "Starboard", "Wing Engine #3", "Generator") as metadata. Labels are for display only; they are not used as identifiers and are not transmitted on the bus.

## 2. v1.0 Distribution

Binding-table contents are **not** defined for on-bus publication over Pelorus Core CAN in v1.0. Distribution is out of band by one of:

- Gateway-resident configuration export/import (typically a web UI per [`09-network.md §5.5`](./09-network.md))
- Diagnostic session via a connected tool
- Pelorus Stream
- Companion app
- Non-volatile backup restored by the operator

A future revision may assign a dedicated DC or `Pelorus.NetworkManagement` / `Pelorus.WakeUp` payload fields for binding-table sync; v1.0 declines to specify this on the constrained Core wire.

## 3. Fault Tolerance

The binding table shall not create a single point of failure:

- Any authorised role (primary gateway, secondary display head, diagnostic tool) can hold binding authority. Edits merge in non-volatile memory and propagate via the out-of-band channels above.
- Nodes that need semantic resolution cache the latest binding table in their own non-volatile memory.
- The primary gateway typically provides the web UI for editing and provisioning but is not required for continued Core operation.
- If the gateway is absent or failed: raw Core DC traffic continues unaffected; semantic consumers fall back to the last cached binding table (or to raw `DC_ID` + instance display); new devices join and transmit data immediately in raw mode.
- When a gateway or tool returns, it reapplies the authoritative table through the same out-of-band channels.
- Secondary gateways coordinate binding authority — see [`09-network.md §5.6`](./09-network.md).

## 4. Cache Requirements for Core Nodes

A node declaring semantic awareness shall:

- Receive and validate binding-table updates from the out-of-band channel it supports
- Persist the most recent valid table in non-volatile memory
- Fall back to raw `DC_ID` + instance display when the cache is invalid or absent — and not block raw data delivery on cache availability
- Re-apply binding to incoming frames at receive time, not at storage time (so a binding edit applies to subsequent frames immediately without rewriting historical data)

## 5. Catalog and DC Registry Cross-Refs

Layer roles and the relationship to catalog semantics and DC wire layout are normative in [`../catalog/01-overview.md §3`](../catalog/01-overview.md). Worked example: the engine reporting `Pelorus.EngineController1` with `engine-instance = 2` from SA `0x14` (NAME `0x80…`) resolves to the catalog's `Vessel.Propulsion.Engines[1]`.

The binding is intentionally per-vessel — the same physical engine controller carries different friendly labels and may project to different array indices on different boats. The catalog is universal; the binding is local.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
