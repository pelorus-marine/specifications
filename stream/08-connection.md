# Pelorus Stream — Connection

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document specifies the **connection model** — the wire-level relationships that carry payload PUs from publisher to subscriber. Pelorus Stream supports three connection modes; all three coexist on the same network and differ only in addressing and reliability.

The draft transport targets are summarized in [`01-overview.md` §9](./01-overview.md#9-draft-design-targets-summary).

---

## 1. Connection Modes

| Mode | Transport | Addressing | Reliability | Typical use |
|---|---|---|---|---|
| **Unicast UDP** | UDP/IPv6 | Subscriber link-local address | Best-effort | Targeted media, intercom point-to-point |
| **Multicast UDP** | UDP/IPv6 (ASM or SSM) | Per-stream multicast group | Best-effort | One-to-many audio, telemetry fan-out |
| **Reliable QUIC** | QUIC over UDP/IPv6 | Subscriber link-local address | Reliable, in-order | File transfer, control plane retries |

A single stream uses **one** connection mode for its lifetime. A publisher may decide its mode based on metadata (e.g. `prio`, `type`) or static policy. The mode is announced via the TXT record (`mode={u,m,q}`) so subscribers know what to expect.

---

## 2. Port Allocation

| Purpose | UDP port |
|---|---|
| Stream control plane (subscribe / unsubscribe / heartbeat / events) | **`5354`** |
| Stream payload (unicast and multicast) | **`5355`** |
| Reserved for future Stream extensions | `5356`–`5360` |

Choices: 5354 sits adjacent to 5353 (mDNS) for clarity; 5355 is also assigned to LLMNR — Pelorus Stream **does not implement LLMNR** and the port collision is by accident in IANA's registry; vessels run no LLMNR responders, and the port is used here for a private link-local protocol. If real-world deployment shows interop pain, this port assignment will be revisited (Open Items).

QUIC uses the same `5355` UDP port and is multiplexed against unreliable UDP by the QUIC version field per RFC 9000.

---

## 3. Unicast UDP

The simplest mode. The publisher transmits PUs as UDP datagrams to each subscriber's link-local address and port `5355`.

**Strengths:** No multicast configuration required; works through any switch.

**Weaknesses:** Per-subscriber CPU and bandwidth cost on the publisher; bad fit for fan-out.

A publisher of a unicast stream limits itself to a small subscriber set (recommended ≤ 4). A subscriber set hitting that bound triggers a switch to multicast in v1.1+; in v1.0 the publisher refuses additional subscribers.

---

## 4. Multicast UDP

Each multicast stream is bound to a multicast group address allocated per [`09-routing.md`](./09-routing.md). The publisher sends PUs to the group; subscribers join the group via MLD2.

**Source-Specific Multicast (SSM)** is preferred when the network supports MLDv2 (most modern managed switches do). SSM means subscribers join `(source, group)` — they will only receive traffic from the publisher's address. This avoids cross-talk from misconfigured publishers.

**Any-Source Multicast (ASM)** is the fallback for unmanaged switches that do not snoop IGMP/MLD properly.

The mode (SSM vs ASM) is signaled by the address range alone:

- `ff32::/32` — SSM
- `ff02::/16` — ASM, link-local

See [`09-routing.md` §3](./09-routing.md) for allocation.

### 4.1 Multicast Reliability

Multicast UDP is best-effort. There is no per-receiver retransmission. Subscribers absorb loss via [`18-buffering.md`](./18-buffering.md). Publishers shall not assume a delivered datagram has been received by anyone.

### 4.2 Subscribe Messaging

Even on multicast, subscribers send `subscribe` messages on port `5354` to the publisher. This is how the publisher learns:

- Who is listening (for telemetry of subscriber count)
- When to keep the stream alive vs. release ([`07-session.md` §5](./07-session.md))
- Capability negotiation results

A subscriber that joins the multicast group without sending `subscribe` will still receive PUs — multicast is open — but is invisible to the publisher and contributes no lease pressure.

---

## 5. Reliable QUIC

For streams that cannot tolerate loss (file transfer, configuration push, paginated registry sync), the connection uses QUIC per RFC 9000.

QUIC is the right tool here because:

- Same UDP port as unreliable transport — no extra firewall holes.
- 0-RTT or 1-RTT setup against a known peer — fast.
- Per-stream multiplexing — multiple Stream-streams over one QUIC connection.
- Forward-compatible with TLS encryption when v1.0 deployments warrant it.

**v1.0 QUIC deployments shall:**

- Use the `pelorus-stream/0.1` ALPN identifier.
- Negotiate TLS 1.3 with self-signed certificates whose subject is the publisher's link-local address. Trust evaluation is policy of the subscriber and is permitted to be "accept any onboard certificate" for v1.0.
- Use a single QUIC connection per `(publisher, subscriber)` pair regardless of how many Stream-streams are multiplexed over it.

QUIC is **not** a Pelorus Stream prerequisite. A v1.0 conformant subscriber that does not implement QUIC is permitted; it will simply skip streams whose announcements list `mode=q`.

---

## 6. Mode Announcement

The TXT record carries:

```
mode=<u|m|q>
addr=<link-local-or-multicast-address>
port=<5355 or another reserved>
```

Examples:

```
mode=u  addr=fe80::1234:5678:9abc:def0  port=5355
mode=m  addr=ff32::dead:beef             port=5355
mode=q  addr=fe80::1234:5678:9abc:def0  port=5355
```

The `addr` for unicast is the publisher's address that the publisher accepts subscriptions on (and from which it sources packets). Subscribers subscribe to that address and receive PUs to their own address.

---

## 7. Health and Heartbeats

For unicast and QUIC modes, the publisher shall emit a heartbeat every 5 seconds on the control port if there are no PUs to send. The heartbeat is a `keepalive` event ([`19-stream-event.md`](./19-stream-event.md)). Multicast does not require heartbeats — the mDNS announcement and the multicast group activity serve.

A subscriber that does not see PUs **or** a heartbeat for 3 × heartbeat interval (15 s) shall surface `transport-stalled` ([`26-transport-error.md`](./26-transport-error.md)).

---

## 8. Open Items

- Whether to default to QUIC for the control plane (announce, subscribe, heartbeat) instead of plain UDP — currently plain UDP for simplicity; revisit after measuring jitter buffer interactions.
- Whether to specify a TLS-on-multicast profile (currently no — multicast is in-vessel, plaintext).
- Multicast snooping requirements for the recommended switch list — to be settled in [`12-hardware-design-guide.md`](../core/12-hardware-design-guide.md) once Stream switches are sourced.
- Port `5355` collision with LLMNR — monitor for interop issues; reserve a fallback in `5356`–`5360`.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
