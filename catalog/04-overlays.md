# Pelorus Catalog — Overlay Attributes

Pelorus extends the COVESA VSS leaf attribute set with overlay attributes that link catalog leaves to Pelorus Core's wire-level behaviour without making the catalog itself protocol-aware.

| Attribute | Purpose | Normative value source |
| --- | --- | --- |
| `data_contract` | Names the Pelorus DC that projects to this leaf on Pelorus Core | [`../core/07-dcid-registry.md`](../core/07-dcid-registry.md) |
| `dc-field` | Names which field of the DC's payload this leaf reads. Omitted when the DC carries a single quantity (the leaf reads the whole payload). Required when one DC projects to multiple catalog leaves. | [`../core/07-dcid-registry.md §2.1`](../core/07-dcid-registry.md) |
| `instance-field` | For instanced leaves whose DC carries the instance value in its payload: names that payload field. Omitted when the DC's instancing is via address-claim NAME (one device per instance, distinguished by Source Address + NAME). | [`../core/07-dcid-registry.md`](../core/07-dcid-registry.md) |
| `pelorus-priority` | Pelorus Core arbitration priority hint (0 highest, 7 lowest) | [`../core/03-data-link.md §2.2`](../core/03-data-link.md) |

Overlays are applied via `vss-tools` per [`05-tooling.md`](./05-tooling.md). They are advisory: a node that does not consume Pelorus Core may ignore them.

The catalog adds no per-leaf attributes for Pelorus Stream or Pelorus State, and LMDE bridges live on the Pelorus DC rather than on catalog leaves. Cross-subsystem linkage is summarised in [`01-overview.md §2–§3`](./01-overview.md); bridge mechanics are normative in [`../core/07-dcid-registry.md §2`](../core/07-dcid-registry.md).

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
