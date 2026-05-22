# Pelorus Catalog — Overview

The protocol-agnostic semantic data model for Pelorus, expressed in COVESA VSS under a `Vessel.*` root. The catalog defines **meaning** — signal names, types, units, valid ranges, instance handling. It does not define wire bit layouts, transport, or arbitration; those live in subsystem-specific normative documents.

## 1. What the Catalog Is

A single source of truth for vessel-level signal semantics, shared across all Pelorus subsystems. Every quantity a Pelorus system might publish, subscribe to, or reason about lives under `Vessel.*`. The catalog is maintained as a standalone `Vessel.*` VSS tree and is not contributed upstream to COVESA in v1.0.

## 2. Who Consumes the Catalog

| Subsystem | How it uses `Vessel.*` | Detailed reference |
| --- | --- | --- |
| **Pelorus Core** (CAN FD fieldbus) | Wire-level Data Contracts declare which `Vessel.*` paths they project to via the `data_contract` overlay; the binding table maps bus identifiers to indexed-array elements | [`../core/07-dcid-registry.md`](../core/07-dcid-registry.md), [`../core/06-instance-binding.md`](../core/06-instance-binding.md) |
| **Pelorus Stream** (Ethernet) | Telemetry payloads use `Vessel.*` paths as CBOR map keys; stream metadata declares mirrored semantics via the `vss` field | [`../stream/02-data-model.md`](../stream/02-data-model.md), [`../stream/10-services-nav.md`](../stream/10-services-nav.md) |
| **Pelorus State** (fused world model) | Snapshot, situation, and policy stages all reference `Vessel.*` paths as their semantic input | [`../state/01-overview.md`](../state/01-overview.md) |

The catalog itself is subsystem-agnostic. No subsystem owns it; all subsystems import from it.

## 3. Layer Roles

| Layer | Representation | Responsibility | Authoritative in |
| --- | --- | --- | --- |
| **Semantics** | `Vessel.*` path in COVESA VSS | Units, types, valid range, human meaning, relationships | This catalog ([`02`](./02-structure.md)–[`04`](./04-overlays.md)) |
| **Data Contract** | `Pelorus.<Name>` with `dc_id`, priority, payload layout, optional `bridges[*]` | Naming, prioritisation, payload bit layout, legacy-protocol bridging | [`../core/07-dcid-registry.md`](../core/07-dcid-registry.md) |
| **Wire** | 29-bit identifier `[PRIO 3b \| DC_ID 18b \| SA 8b]` | Bus arbitration, transmission, addressing | [`../core/03-data-link.md §2`](../core/03-data-link.md) |
| **Instance binding** | `(SA + NAME + DC_ID + DC-internal instance field) → Vessel.*[n]` | Bus identifier triple resolved to catalog index | [`../core/06-instance-binding.md`](../core/06-instance-binding.md) |

Cross-cutting rules:

- A Pelorus DC does not redefine nautical meaning. Two signals with the same rough name in different namespaces must resolve to distinct `Vessel.*` leaves or explicit aliases.
- Stream telemetry that mirrors catalog quantities carries the optional `vss` metadata key (full `Vessel.*` path) per [`../stream/02-data-model.md`](../stream/02-data-model.md).

## 4. Design Principles

- **Protocol-agnostic.** The catalog says what a signal means and what its units are. It does not say how it travels.
- **Indexed arrays over named branches.** Repeated identical devices (engines, tanks, batteries) are addressed by integer index, not by names like Port/Starboard, so the catalog scales to any vessel topology. See [`03-instances.md`](./03-instances.md).
- **Subsystem-agnostic identity.** Catalog leaves do not embed subsystem-specific identifiers (no source addresses, no UUIDs, no MAC addresses). Subsystems map their own identifier conventions to catalog indices via their own binding documents.
- **One source of truth.** A signal is defined in exactly one place. Subsystems reference it; they do not redefine it.

## 5. Criticality

C0 / C1 / C2 criticality and Class S / D / H node roles are installation and product attributes, normative in [`../core/08-redundancy.md`](../core/08-redundancy.md). The `Vessel.*` catalog does not add parallel branches for criticality — doing so would duplicate `08` and invite drift.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
