# Pelorus Stream — Routing

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines **routing** for Pelorus Stream: how IPv6 addresses are assigned, which multicast groups a publisher uses, and what scope a stream lives in. Draft design targets are summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary). This document is the operational reference.

---

## 1. Address Plan

| Layer | Address class | Source |
|---|---|---|
| Unicast | IPv6 link-local (`fe80::/10`) | RFC 4862 SLAAC; modified EUI-64 from interface MAC |
| Unicast | IPv6 ULA (`fd00::/8`) | Optional, vessel-wide static prefix; configured by gateway |
| Multicast | IPv6 link-local multicast (`ff02::/16`) | Per-stream, allocated by hashing |
| Multicast | IPv6 SSM (`ff32::/32`) | Per-stream, allocated by hashing |

A v1.0 Stream node **shall** auto-configure a link-local address on every interface. ULA is optional; if present, ULA addresses may also be used for unicast streams whose subscribers are on the vessel-wide segment beyond a single link.

Global IPv6 addressing is **out of scope** for v1.0. A vessel that uplinks to the Internet does so via a separate gateway; cross-Internet streaming is not specified here.

---

## 2. Interface Selection

Most Pelorus Stream nodes have a single Ethernet interface. Nodes with multiple interfaces (gateway nodes, multi-segment vessels) shall:

- Bind their Stream sockets to a single, configured "Stream interface" — typically the M12 D-coded port.
- Not forward Stream packets between interfaces. There is no Stream router.

If multi-segment Stream connectivity is needed, it is provided by a Layer-2 switch that bridges segments. Layer-3 Stream routing is not part of v1.0.

---

## 3. Multicast Group Allocation

Multicast streams need a unique IPv6 multicast group per stream. v1.0 uses deterministic, decentralized allocation:

```
group_address = base_prefix || H(stream_id)[lower 96 bits]
```

Where:

- `base_prefix` is `ff32::/32` (SSM) or `ff02::/16` extended with `::pelorus:` for ASM (TBD assignment).
- `H` is BLAKE3 (or any 256-bit cryptographic hash; BLAKE3 is the reference for its speed in Rust, MIT/Apache 2.0).
- Concatenation drops trailing bits to reach 128 bits total.

Concretely, for SSM:

```
ff32:0040:<48-bit-zero>:<top-48-bits-of-H(stream_id)>
```

Collisions are vanishingly rare (~1 in 2⁴⁸ for any given pair) and are detected at session-open by the publisher checking its own multicast joins; on collision the publisher may re-mint the Stream ID (and thus the group).

Subscribers compute the same group address from the announced Stream ID and the SSM source address (the publisher's link-local). No central allocator exists.

---

## 4. Multicast Scope

| Scope flag | Meaning | v1.0 status |
|---|---|---|
| `ff02` | Link-local | Default and only scope in v1.0 |
| `ff05` | Site-local | Reserved for v1.1 multi-segment vessels |
| `ff0e` | Global | Not used |

v1.0 multicast Stream traffic shall not leave the link-local broadcast domain. Any cross-segment streaming requires explicit gateway translation, which is out of scope for v1.0.

---

## 5. Source Address Stability

A publisher's link-local address is derived from its MAC address. If the publisher is replaced (hardware change), its Stream ID changes anyway (new session, [`07-session.md`](./07-session.md)) and so does its multicast group; subscribers re-resolve via mDNS and rejoin.

ULA addresses, if used, are stable across MAC changes and provide a vessel-wide identity. ULA usage is policy of the gateway, not Stream-mandated.

---

## 6. MLD2 / IGMP

Stream nodes implementing multicast subscription shall support **MLDv2** (RFC 3810) for IPv6 multicast group join/leave. Switches in the vessel network are expected to MLD-snoop; if they do not, multicast streams are flooded but still work, at increased bandwidth cost.

Recommended onboard switches are listed in [`core/12-hardware-design-guide.md`](../core/12-hardware-design-guide.md) once Stream hardware sourcing is finalized.

---

## 7. Routing Table Independence

Stream **shall not** install routes, run a routing protocol, or modify the host routing table beyond standard SLAAC behavior. There is no Stream OSPF, no Stream RIP. A vessel's Layer-3 architecture, where it exists, is owned by the gateway.

---

## 8. Loopback and Self-Subscribe

A publisher subscribing to its own stream is permitted. Implementations shall:

- Loop traffic locally without round-tripping through the network when possible.
- Otherwise, allow the OS multicast loopback path — `IPV6_MULTICAST_LOOP=1`.

This pattern is useful for the **Pelorus State subsystem** to observe locally what is also being multicast; it runs as a subscriber and sees the same stream as remote subscribers.

---

## 9. Open Items

- The exact `base_prefix` for ASM. Currently a placeholder; needs a clean allocation that does not collide with documented IANA assignments.
- Whether to assign a Pelorus IANA multicast block (probably not for v1.0; link-local does not require IANA).
- Site-scope (`ff05`) usage in v1.1 for vessels with multiple bridged Ethernet segments.
- Behavior when the publisher's link-local address changes after session open (e.g. interface bounce) — currently force a session close; a graceful re-claim is a v1.1 candidate.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
