# Pelorus Implementation — Installation

Vessel install patterns, commissioning sequence, dual-bus install ergonomics, and troubleshooting. **Non-normative.** Topology, cabling, termination, BPI, isolation, and dual-bus rules are normative in the subsystem specs.

## 1. Pre-Installation Planning

1. Measure the vessel; plan Core segments so backbone length, drop length, node count, and stub length stay within [`../core/02-physical.md §4`](../core/02-physical.md) and [`../core/09-network.md §1`](../core/09-network.md).
2. Decide on topology — small vessels: single linear Core bus; large vessels: star with central gateway recommended ([`../core/09-network.md §4`](../core/09-network.md)).
3. Plan repeater locations to respect the hop limit ([`../core/09-network.md §2`](../core/09-network.md)).
4. Identify Core BPI injection points and size them per [`../core/02-physical.md §7`](../core/02-physical.md).
5. Decide per-device isolation requirements using the categories in [`../core/02-physical.md §8`](../core/02-physical.md).
6. Prepare a critical zone map ([`../core/08-redundancy.md §12`](../core/08-redundancy.md)): assign C0 / C1 / C2 to each function and decide where Bus A and Bus B run.
7. For Stream installs, plan dual-fabric segregation and PoE budget per [`../stream/03-physical.md`](../stream/03-physical.md) and [`../stream/07-redundancy.md`](../stream/07-redundancy.md).

## 2. Cabling and Connectors

Cable type, connector pinout, and termination are normative in [`../core/02-physical.md §2–§5`](../core/02-physical.md) (Core) and [`../stream/03-physical.md`](../stream/03-physical.md) (Stream). On-site practice:

- Label every cable and drop.
- For dual-bus, label A vs B at both ends and at every tee.
- Verify termination resistance (~60 Ω between CAN_H and CAN_L on a Core segment) with the network powered off.

## 3. BPI Installation

The Core BPI is the bus's most critical reliability component ([`../core/02-physical.md §7.8`](../core/02-physical.md)).

- Mount in an accessible, dry, ventilated location.
- Fuse the BPI input per the cable capacity in [`../core/02-physical.md §2`](../core/02-physical.md).
- For dual-bus C0/C1 zones, install one BPI per bus from independent fused feeds.

## 4. Repeater and Gateway Placement

- Mount repeaters where they create clean isolated segments.
- In star topology, connect repeaters directly to the central gateway.
- Install the central gateway in an accessible location with Wi-Fi or Ethernet access for the provisioning UI ([`../core/09-network.md §5`](../core/09-network.md)).
- Verify galvanic isolation between every repeater port pair before commissioning.

## 5. Commissioning Sequence

1. Install and terminate all segments before powering anything.
2. Power up one segment at a time and verify no bus errors.
3. Connect the gateway and confirm it claims an address ([`../core/05-addressing.md`](../core/05-addressing.md)).
4. Provision the binding table via the gateway UI ([`../core/06-instance-binding.md`](../core/06-instance-binding.md)).
5. Test the power state machine ([`../core/04-power.md`](../core/04-power.md)): Active → Sleep → Wake.
6. Verify multi-segment forwarding and inter-segment isolation.
7. Perform a full network test with every device connected.

## 6. Dual-Bus Path Redundancy

For C0 / C1 zones per [`../core/08-redundancy.md`](../core/08-redundancy.md):

- Run two independent backbone pairs (Bus A, Bus B).
- Use separated cable trays and bulkhead penetrations where feasible; document residual risk in the critical zone map.
- Use independent fused feeds to the Bus A and Bus B BPIs.
- Commission `Pelorus.BusHealth` on a test display before declaring the dual-bus domain complete.
- Verify `DISCARD_WINDOW` against the formula in [`../core/08-redundancy.md §6.3.3`](../core/08-redundancy.md).

## 7. Troubleshooting

| Symptom | First check | Spec reference |
| --- | --- | --- |
| Bus errors or no communication | Termination and stub lengths | [`../core/02-physical.md §5`](../core/02-physical.md) |
| High standby current | Isolation and transceiver sleep behaviour | [`../core/04-power.md`](../core/04-power.md) |
| Address conflicts | NAME uniqueness | [`../core/05-addressing.md §2`](../core/05-addressing.md) |
| Binding table not updating | Gateway powered? Binding is out of band in v1.0 — use gateway UI, export/import, or diagnostic tool | [`../core/06-instance-binding.md`](../core/06-instance-binding.md) |
| Intermittent faults | Connectors and cable shielding | [`../core/02-physical.md §9`](../core/02-physical.md) |
| One bus silent on dual-bus helm | Bus Health on surviving bus; inspect failed backbone for opens, terminators, or transceiver damage | [`../core/08-redundancy.md §10`](../core/08-redundancy.md) |
| BPI fault flags | Inspect `Pelorus.PowerInjector` reports | [`../core/02-physical.md §7.2.5`](../core/02-physical.md) |
| Stream fabric degraded | Per-fabric `_pelorus-stream-health._quic.local` reports | [`../stream/10-services-nav.md §6`](../stream/10-services-nav.md) |

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
