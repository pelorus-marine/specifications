# Pelorus Core — Signal Catalog

The canonical semantic data model for Pelorus Core signals, expressed in COVESA VSS under a `Vessel.*` root. Protocol-agnostic: meaning, type, units, and instance binding live here; Pelorus Data Contracts (DCs) and CAN FD wire contracts live in [`07-dcid-registry.md`](./07-dcid-registry.md).

## 1. Catalog Structure

Root: `Vessel`. All signals live under `Vessel.*`. Top-level branches:

- `Vessel.Propulsion` (engines, thrusters, sail drives)
- `Vessel.Navigation`
- `Vessel.Environment`
- `Vessel.Electrical`
- `Vessel.Tanks`
- `Vessel.Anchoring`
- `Vessel.Safety`
- `Vessel.Domestic`
- `Vessel.Network`

The catalog is maintained as a standalone `Vessel.*` tree and is not contributed upstream to COVESA in v1.0.

## 2. VSS Syntax

Standard VSS `.vspec` (YAML) format:

- **Branches** (structural nodes)
- **Leaves** (signals) with mandatory attributes: `type`, `unit`, `description`, `min`/`max`/`enum`
- **Pelorus overlay attributes** via vss-tools: `data_contract` (references a `Pelorus.<Name>`), `instance-field`, `pelorus-priority`

## 3. Instance Handling and Binding

Pelorus uses numeric indexed arrays as the canonical form:

- `Vessel.Propulsion.Engines[0].Speed`
- `Vessel.Propulsion.Engines[1].Speed`
- … up to a practical maximum (e.g. `[0..15]`)

Named branches (Port/Starboard) are not used in the canonical catalog because they do not scale to boats with arbitrary numbers of identical devices.

**Binding table.** The mapping `(Source Address + 64-bit NAME + DC_ID + DC-internal instance field) → VSS array index [n]` is stored in a binding table. Sailor-assigned friendly labels ("Port Main", "Starboard", "Wing Engine #3", "Generator") live as metadata on each entry.

**v1.0 distribution.** Binding-table contents are not defined for on-bus publication over Pelorus Core CAN in v1.0. Distribution is out of band: gateway/local configuration export, diagnostic session, Pelorus Stream, companion app, or NV backup restored by the operator. A future revision may assign a dedicated DC or `Pelorus.NetworkManagement` / `Pelorus.WakeUp` payload fields for binding sync.

## 4. Fault Tolerance

The binding table must not create a single point of failure.

- Any authorised role (primary gateway, secondary display head, diagnostic tool) can hold binding authority: merge edits in NV and distribute updates out of band.
- Nodes that need semantics cache the latest binding table in their own non-volatile memory.
- The primary gateway typically provides the web UI for editing/provisioning but is not required for continued Core operation.
- If the gateway is absent or failed: Core raw DC traffic continues unaffected; semantic consumers fall back to the last cached binding table (or to raw `DC_ID` + instance display); new devices join and transmit data immediately in raw mode.
- When a gateway or tool returns, it reapplies the authoritative table through the same out-of-band channels.

## 5. LMDE Compatibility

Pelorus signals carry the same semantic information sailors already see on LMDE networks. Those semantics are exchanged on Classical CAN segments; Pelorus Core carries equivalent meaning in CAN FD frames where mapped. Gateways translate identifiers and reframe between formats; the signal catalog and binding table describe meaning independent of which side produced the frame.

Where a Pelorus DC corresponds to a legacy J1939 / NMEA 2000 message, the DC declares a `bridges[*]` entry naming the legacy identifier. Gateways use the bridge table to translate between the Pelorus DC_ID on Core and the legacy PGN on LMDE. These mappings derive from public observation of live networks and open-source reverse-engineering (e.g. canboat). Proprietary or vendor-specific extensions are not carried forward.

## 6. Three-Layer Roles

| Layer | Representation | Responsibility | Authoritative in |
|---|---|---|---|
| **Semantics** | `Vessel.*` path in COVESA VSS | Units, types, valid range, human meaning, relationships | This document |
| **Data Contract** | `Pelorus.<Name>` with `dc_id`, priority, payload layout, optional `bridges[*]` | Naming, prioritisation, payload bit layout, legacy-protocol bridging | [`07-dcid-registry.md`](./07-dcid-registry.md) |
| **Wire** | 29-bit identifier `[PRIO 3b \| DC_ID 18b \| SA 8b]` | Bus arbitration, transmission, addressing | [`03-data-link.md §2`](./03-data-link.md) |
| **Instance binding** | `(SA + NAME + DC_ID + DC-internal instance field) → VSS index` | Which physical device / bus instance maps to `Vessel.*[n]` | Binding table — out-of-band distribution for v1.0; see §3–4 |
| **Pelorus Stream** | UUIDv7 stream ID, not a CAN frame | High-bandwidth or media sessions | [`stream/`](../stream/) — metadata may reference `Vessel.*` paths |

Rules:

- VSS does not define CAN bitpacking. The catalog references a Pelorus DC via the `data_contract` overlay attribute; bit layouts live in the DC registry.
- A DC does not define nautical meaning. Two signals with the same rough name in different namespaces must resolve to distinct `Vessel.*` leaves or explicit aliases.
- Stream telemetry that mirrors Core quantities should carry the optional `vss` metadata key (full `Vessel.*` path) per [`stream/02-data-model.md`](../stream/02-data-model.md).

## 7. Criticality

C0 / C1 / C2 criticality and Class S / D / H node roles are installation and product attributes, normative in [`08-redundancy.md`](./08-redundancy.md). The `Vessel.*` catalog does not add parallel branches for criticality — doing so would duplicate `08` and invite drift.

## 8. Tooling

- Source: `catalog/vessel.vspec` plus overlay files
- Validation: `vss-tools` with the Pelorus overlay profile
- Code generation: Rust structs, validation, and TypeScript definitions generated by reference crates listed in [`10-implementation.md`](./10-implementation.md)
- Runtime: only nodes that need semantics carry the binding cache; low-power sensors ignore it entirely

## 9. Example

```yaml
Vessel:
  type: branch
  description: Root of the Pelorus marine signal catalog

  Propulsion:
    type: branch
    description: Propulsion systems

    Engines:
      type: branch
      instances: [0,15]
      description: Individual propulsion engines (indexed by binding table)

      Speed:
        type: float
        unit: m/s
        description: Engine crankshaft rotational speed
        data_contract: Pelorus.EngineController1
        instance-field: engine-instance
```

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
