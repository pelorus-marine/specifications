# Pelorus Catalog — Instance Handling

Repeated identical devices (engines, tanks, batteries, AIS targets) are represented as indexed arrays under their branch. This document specifies the canonical form and points to subsystem-specific resolution.

## 1. Indexed Array Canonical Form

Pelorus uses numeric indexed arrays for repeated devices:

- `Vessel.Propulsion.Engines[0].Speed`
- `Vessel.Propulsion.Engines[1].Speed`
- … up to a practical maximum (typically `[0..15]`)

The `instances: [low, high]` attribute on a branch declares its index range. The range is closed on both ends.

## 2. No Named Sub-Branches for Repeated Devices

Named branches (`Vessel.Propulsion.Port`, `Vessel.Propulsion.Starboard`) are **not** used in the canonical catalog. They do not scale to vessels with arbitrary numbers of identical devices (wing engines, multiple thrusters, multi-engine generators) and they bake an assumption about vessel layout into a vocabulary that is supposed to be vessel-agnostic.

Sailor-assigned friendly labels ("Port Main", "Starboard", "Wing Engine #3", "Generator") live as metadata on each entry, not as branch names. They are preferred for display and stable for the sailor; they are not used as identifiers.

## 3. Subsystem Resolution of Instance Identities

Catalog indices are protocol-agnostic. Each subsystem resolves its own identifier conventions to a catalog index:

| Subsystem | Identifier source | Resolution mechanism | Detailed reference |
| --- | --- | --- | --- |
| **Pelorus Core** | Source Address + 64-bit NAME + DC_ID + DC-internal instance field | Per-vessel binding table; out-of-band distribution in v1.0 | [`../core/06-instance-binding.md`](../core/06-instance-binding.md) |
| **Pelorus Stream** | mDNS service instance + `instance` metadata field | Service discovery + metadata | [`../stream/02-data-model.md`](../stream/02-data-model.md), [`../stream/08-discovery-and-registry.md`](../stream/08-discovery-and-registry.md) |
| **Pelorus State** | Imported from Core and Stream | Indices preserved from origin subsystem; State does not assign new indices | [`../state/01-overview.md`](../state/01-overview.md) |

Catalog leaves do **not** carry subsystem identifiers. A sensor that publishes the same quantity via both Core and Stream produces two independent identity paths (Core SA+NAME triple, Stream service+instance) that both resolve to the same catalog index by way of subsystem-specific binding.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
