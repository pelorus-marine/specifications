# Pelorus Catalog — Structure

Root, top-level branches, VSS syntax, leaf attributes, and worked examples. Instance handling lives in [`03-instances.md`](./03-instances.md); Pelorus overlay attributes live in [`04-overlays.md`](./04-overlays.md).

## 1. Root and Top-Level Branches

Root: `Vessel`. All signals live under `Vessel.*`. Top-level branches:

- `Vessel.Propulsion` — engines, thrusters, sail drives
- `Vessel.Navigation`
- `Vessel.Environment`
- `Vessel.Electrical`
- `Vessel.Tanks`
- `Vessel.Anchoring`
- `Vessel.Safety`
- `Vessel.Domestic`
- `Vessel.Network`

New top-level branches are added by pull request against this document and `catalog/vessel.vspec` together; the registered set above is the authoritative one for v1.0.

## 2. VSS Syntax

Standard VSS `.vspec` (YAML) format:

- **Branches** — structural nodes that group leaves and sub-branches; declared with `type: branch`.
- **Leaves** — signals carrying value; declared with `type: sensor | actuator | attribute` (VSS leaf kind) plus mandatory `datatype`, `unit`, `description`, and `min`/`max`/`enum` where applicable.
- **Pelorus overlay attributes** — applied via `vss-tools`: `data_contract`, `dc-field`, `instance-field`, `pelorus-priority`. Normative semantics in [`04-overlays.md`](./04-overlays.md).

## 3. Example — Instanced Propulsion Signal

A sensor signal bridged from a J1939 PGN, projected to an instanced engine entry:

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
        type: sensor
        datatype: float
        unit: rpm
        min: 0
        description: Engine crankshaft rotational speed
        data_contract: Pelorus.EngineSpeed
```

## 4. Example — Vessel-Wide Time Reference

The time reference published by the elected Time Master per [`../core/08-redundancy.md §8.2`](../core/08-redundancy.md):

```yaml
Vessel:
  Network:
    type: branch
    description: Network-level metadata and infrastructure signals

    Time:
      type: branch
      description: Vessel time reference and trust metadata from the elected Time Master

      UTC:
        type: sensor
        datatype: uint32
        unit: ms
        description: |
          CoreTime. Milliseconds since UTC midnight when SourceClass ∈ {1,2,3,4,6};
          monotonic millisecond counter when SourceClass ∈ {0,5}. Consumers shall
          check SourceClass before interpreting as wall-clock UTC.
        data_contract: Pelorus.TimeSync
        dc-field: CoreTime

      SourceClass:
        type: sensor
        datatype: uint8
        description: Trust class of the current time source (enum per 08-redundancy.md §8.2.3)
        min: 0
        max: 7
        data_contract: Pelorus.TimeSync
        dc-field: SourceClass

      AccuracyBucket:
        type: sensor
        datatype: uint8
        description: Coarse current UTC offset bound (enum per 08-redundancy.md §8.2.3)
        min: 0
        max: 7
        data_contract: Pelorus.TimeSync
        dc-field: AccuracyBucket

      SpoofSuspect:
        type: sensor
        datatype: boolean
        description: Receiver-level spoofing/jamming indication or peer cross-check disagreement
        data_contract: Pelorus.TimeSync
        dc-field: SpoofSuspect

      LeapPending:
        type: sensor
        datatype: boolean
        description: Leap second announced for the current UTC day
        data_contract: Pelorus.TimeSync
        dc-field: LeapPending
```

Protocol DCs like `Pelorus.TimeSync` are not instanced (there is one Time Master per dual-bus domain), so no `instance-field` overlay is set.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
