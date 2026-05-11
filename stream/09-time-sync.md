# Pelorus Stream — Time Synchronisation

**Version:** 0.2 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

IEEE 802.1AS (gPTP) on the Stream Ethernet plant. The datagram header timestamp ([`04-transport.md §5`](./04-transport.md)) uses the gPTP epoch.

## 1. Requirement

**IEEE 802.1AS (gPTP) is mandatory for all Class D Stream nodes.**

Radar spoke timestamps must be accurate to better than one antenna beamwidth in time. At 24 RPM with a 2° beamwidth, this is ~1.4 ms. Sub-millisecond synchronisation is required. IEEE 802.1AS achieves sub-microsecond on commodity hardware.

Class S nodes should implement gPTP where the service they participate in requires accurate timestamps. Stream services that do not require timestamps may run without gPTP, but the receiver shall mark the datagram header `time sync valid` flag (bit 0 of `Flags`) as 0 to inform the consumer.

## 2. Per-Fabric Operation

gPTP runs **independently on Fabric A and Fabric B**.

- Each fabric has its own Best Master Clock (BMC) selection per IEEE 802.1AS.
- Each fabric typically has its own grandmaster — usually the GNSS-disciplined node.
- Nodes use the better of the two fabric time references (configured policy: lowest BMC priority, smallest accumulated path delay, or operator-configured preference).

Cross-fabric time-source coordination is out of scope for v1.0 — each fabric is treated as an independent time domain.

## 3. Grandmaster Selection

The GNSS node is the natural grandmaster (GPS 1PPS gives sub-microsecond accuracy). GNSS nodes shall advertise themselves as gPTP grandmaster candidates with:

- `priority1 = 64` (above default)
- Clock class corresponding to GNSS-locked operation
- Time source = `GPS` (per IEEE 1588 §7.6.2.6)

Nodes without a primary time reference (most service nodes) shall advertise themselves with default priority and let the BMC algorithm select the GNSS node automatically.

If GNSS is unavailable or the GNSS node fails, the BMC algorithm selects the next-best clock among remaining nodes. A node that becomes grandmaster as the result of GNSS loss shall:

- Continue to serve gPTP from its local oscillator.
- Mark the datagram header `time sync valid` flag = 0 on its own datagrams once the local oscillator drift exceeds the service's tolerance (publisher policy; default ±1 ms).

## 4. Datagram Header Timestamps

Every Pelorus Stream datagram carries an 8-byte timestamp field ([`04-transport.md §5`](./04-transport.md)) encoded as nanoseconds since the gPTP epoch. When `Flags` bit 0 is set, the timestamp is valid; when clear, the timestamp is best-effort and consumers shall treat it as advisory.

## 5. Reference Implementation

`statime` (pure Rust IEEE 1588 / 802.1AS implementation, MIT/Apache 2.0). Suitable for both `std` and `no_std` builds.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
