# Pelorus Stream — Transport Errors

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **transport-layer error taxonomy** for Pelorus Stream — errors arising from the wire substrate (UDP, IPv6, sockets, multicast, QUIC) rather than from the application meaning of stream operations. The application companion is [`25-stream-error.md`](./25-stream-error.md).

Transport errors are mostly handled by the OS or by the QUIC stack; this document specifies how they are *surfaced* to Stream-aware code so the **Pelorus State subsystem** can react.

---

## 1. Error Surface

Transport errors appear:

- As OS socket errors during send/receive.
- As QUIC connection-level events (RFC 9000 §20).
- As Stream-level events ([`19-stream-event.md`](./19-stream-event.md)) when a transport condition is observable from the application's perspective.

Where the OS surface is sufficient (e.g. `ENETUNREACH` on a single send), the reference library does not emit a wire-level event. Where the condition affects ongoing dispatch (e.g. multicast group join failure, QUIC connection close), the library surfaces a transport event.

---

## 2. Transport Error Code Registry

Codes are short stable strings, kebab-case. Receivers shall ignore unknown codes.

| Code | Level | Meaning |
|---|---|---|
| `transport-stalled` | warn | Heartbeats absent for 3× interval ([`08-connection.md` §7](./08-connection.md)). |
| `mtu-exceeded` | err | Outbound datagram exceeds path MTU; fragmentation forbidden. |
| `socket-bind` | fatal | Failed to bind UDP socket to required port. |
| `multicast-join` | err | Failed to join multicast group (MLD failure or unsupported interface). |
| `multicast-leave` | warn | Failed to leave multicast group; already left or interface gone. |
| `quic-handshake-failed` | err | QUIC TLS handshake failed during subscribe. |
| `quic-connection-closed` | warn | QUIC connection closed unexpectedly. |
| `quic-stream-reset` | warn | QUIC stream-level reset. |
| `link-down` | err | Underlying network interface link is down. |
| `address-unreachable` | err | Subscriber address no longer reachable (ICMPv6 unreachable). |
| `port-unreachable` | err | ICMPv6 port-unreachable from peer. |
| `route-disappeared` | err | Local routing table no longer has a route to the peer. |
| `mdns-conflict` | warn | mDNS-level service-instance name conflict ([`23-discovery.md` §9](./23-discovery.md)). |
| `interface-bounce` | warn | The Stream interface flipped down/up; sessions reset. |
| `internal` | fatal | Unspecified transport-layer internal error. |

Severity follows the rules of [`25-stream-error.md` §2](./25-stream-error.md).

---

## 3. UDP-specific Behavior

UDP has weak inherent error reporting: a "connected" UDP socket may surface `ECONNREFUSED` from a prior ICMP, but unconnected sockets used for multicast generally do not. Implementations:

- Use `connect()` on unicast UDP sockets where possible to get ICMP error feedback.
- Treat `port-unreachable` as a hint, not a hard fault — the peer may rejoin shortly.
- Treat `ENETUNREACH` and `EHOSTUNREACH` as `address-unreachable`.

A publisher whose unicast subscriber's address becomes unreachable shall:

1. Surface `address-unreachable`.
2. Stop dispatching to that subscriber.
3. Allow the subscription lease to expire.
4. Not "remember" the subscriber across an interface bounce — the subscriber's lease will be refreshed if it returns.

---

## 4. Multicast Errors

Multicast group operations may fail at OS level:

- **Join failure** (`multicast-join`): the interface does not support multicast, or MLD2 is broken on the upstream switch. Publishers and subscribers shall not retry rapidly; back off to ~10 s.
- **Leave failure** (`multicast-leave`): nearly always benign. Log only.

A subscriber that fails to join a multicast group has effectively no subscription. It shall surface the error to State and not silently fall back to unicast — that decision belongs to State.

---

## 5. QUIC Errors

QUIC carries rich error information per RFC 9000 §20:

- **Handshake failure** maps to `quic-handshake-failed`. Common in v1.0 due to self-signed certificate validation policy mismatch.
- **Stream reset** (peer aborted a QUIC stream) maps to `quic-stream-reset`. Stream-level recovery is QUIC's job; the Pelorus Stream session may continue if other streams in the connection are healthy.
- **Connection close** (peer or transport idle timeout) maps to `quic-connection-closed`. The Stream session is unrecoverable on this connection; the subscriber must re-subscribe.

QUIC TLS error codes shall **not** be exposed verbatim in `details`; map to the abstract code above. Specific TLS alerts are debug-only.

---

## 6. MTU and Fragmentation

Pelorus Stream is single-PU-per-datagram in v1.0 ([`05-stream-payload.md` §3](./05-stream-payload.md)). Fragmentation is forbidden on v1.0 publishers:

- Publishers shall set `IPV6_DONTFRAG` (or platform equivalent) where available.
- A datagram that would fragment is dropped, and `mtu-exceeded` is surfaced.
- The publisher shall close the offending stream and re-open with a smaller-PU profile.

Path MTU discovery is **not** performed by Stream itself in v1.0. Onboard Ethernet has a stable 1500-byte MTU; the assumption is safe. If experience shows otherwise, v1.1 will add PLPMTUD per RFC 8899.

---

## 7. Recovery Patterns

| Error | Reference recovery |
|---|---|
| `transport-stalled` | Subscriber re-checks subscription; sends a fresh `subscribe`. |
| `multicast-join` | Back off 10 s, retry. After three failures, surface to State. |
| `quic-connection-closed` | Subscriber re-discovers, re-subscribes (new QUIC connection). |
| `link-down` | Hold sessions; on link-up surface `interface-bounce`, then re-discover. |
| `port-unreachable` | Drop the unreachable subscriber; expire its lease. |
| `mtu-exceeded` | Publisher closes; sailor adjusts encoder profile. |

These are reference behaviors. Implementations may make different reasonable choices.

---

## 8. Sustained-failure Behavior

A node that observes the same transport error continuously for over **30 seconds** shall:

1. Reduce log cadence (one line per 30 s, not per occurrence).
2. Stop attempting recovery for the offending session.
3. Mark the affected subscription/stream as failed and let State decide.

Stream code shall not enter a tight retry loop; that escalates a network problem into a CPU storm.

---

## 9. Open Items

- A standard "transport health" telemetry stream that aggregates per-link counters (currently per-stream metrics suffice).
- Path-MTU-discovery profile for v1.1.
- Behavior on persistent multicast snooping failure on the switch fabric — currently degrade-and-log; v1.1 may add an automatic unicast fallback negotiation.

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
