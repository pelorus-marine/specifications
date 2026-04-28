# Pelorus Stream — Session

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **session lifecycle**: how a stream comes into being, how subscribers join and leave, and how the stream eventually ends. A session is the time-bounded reality of a stream — the Stream ID is permanent for the life of the session, and only one session ever exists per Stream ID ([`02-stream-id.md`](./02-stream-id.md)).

---

## 1. Session States

Every stream session moves through this state machine on the publisher side:

```
   ┌──────────┐    open      ┌──────────┐
   │   IDLE   │──────────────▶ ANNOUNCED │
   └──────────┘              └────┬─────┘
                                  │ first subscriber attaches
                                  ▼
                            ┌──────────┐
                            │  ACTIVE  │
                            └────┬─────┘
                                 │ last subscriber detaches OR
                                 │ publisher chooses to keep open
                                 ▼
                            ┌──────────┐
                            │  IDLE-   │  optional, configurable lifetime
                            │ ATTACHED │  (continue advertising)
                            └────┬─────┘
                                 │ close()  /  lease expiry
                                 ▼
                            ┌──────────┐
                            │  CLOSED  │ (terminal)
                            └──────────┘
```

The same state is observable on subscriber side via [`20-stream-state.md`](./20-stream-state.md). This document defines the publisher-side transitions.

---

## 2. Open

A publisher opens a session by:

1. Minting a fresh UUIDv7 Stream ID.
2. Constructing the metadata record ([`06-stream-metadata.md`](./06-stream-metadata.md)).
3. Allocating local transport resources (UDP socket, optional QUIC endpoint).
4. Publishing an mDNS-SD announcement ([`23-discovery.md`](./23-discovery.md)) with the metadata in its TXT record.

After step 4 the session is in **ANNOUNCED**. The stream is visible in the registry but no payload is being transmitted.

A publisher **shall not** transmit payload PUs while in ANNOUNCED. A subscriber that attaches early sees no PUs until the publisher transitions to ACTIVE.

---

## 3. Subscriber Attach

A subscriber attaches by sending a `subscribe` message ([`24-subscription.md`](./24-subscription.md)) to the publisher's well-known control endpoint advertised in mDNS. On accepting the subscription, the publisher:

- Adds the subscriber to its dispatch set (unicast UDP) or accepts the subscriber on the multicast group (passive).
- Transitions to ACTIVE if this is the first subscriber.
- Begins or continues PU emission.

The first PU after a transition into ACTIVE may be a key/anchor PU defined per-type (for `audio` see [`16-audio-format.md`](./16-audio-format.md); for `telemetry` it is the next regular sample).

---

## 4. Active

In ACTIVE, the publisher emits PUs at the cadence appropriate to the stream type:

- `audio`: ~50 PUs/s for 20 ms Opus frames
- `telemetry`: per `cad` metadata, or as data is generated
- `control`: per command issued

The session remains ACTIVE while at least one subscriber is attached **or** the publisher's policy keeps the stream alive without subscribers.

For multicast streams, the publisher generally does not know which subscribers exist; it transitions to ACTIVE on the first explicit subscribe message, then stays ACTIVE on a publisher-defined lease (default 60 s). A subscriber on a multicast stream is **expected** to send periodic subscribe-renew messages but the publisher never refuses traffic to subscribers it does not know about.

---

## 5. Lease and Renewal

A subscription is leased. Default lease is **60 seconds**. A subscriber renews by re-sending `subscribe` before lease expiry. A publisher with no live subscribers and no other policy reason to remain open transitions to IDLE-ATTACHED, then closes after a publisher-defined idle window (default 5 minutes).

The lease is **advisory** for multicast: a publisher cannot eject an unwanted multicast listener; the lease is what gates publisher-side resource cleanup.

---

## 6. Close

A publisher closes the session by:

1. Sending a `closing` event to all known subscribers and on the announced control endpoint ([`19-stream-event.md`](./19-stream-event.md)).
2. Withdrawing the mDNS announcement (TTL=0).
3. Releasing transport resources.

The Stream ID is now retired and shall not be reused ([`02-stream-id.md` §3](./02-stream-id.md)).

A publisher that exits without sending `closing` is permitted; subscribers detect this through the loss of mDNS announcements and the stop of PU arrivals. Subscribers shall surface `publisher-disappeared` ([`25-stream-error.md`](./25-stream-error.md)) only after the lease has expired with no renewal traffic.

---

## 7. Crash and Recovery

If a publisher crashes and restarts, the new process **shall** mint a new Stream ID — never recover the old one. Subscribers see the old stream age out of the registry and a new stream appear. State subsystem logic, not Stream, decides whether to auto-attach to the replacement.

If the network partitions, ANNOUNCED and ACTIVE sessions on either side continue independently. Reconverge is handled by the registry ([`22-stream-registry.md`](./22-stream-registry.md)) and is best-effort.

---

## 8. Multiple Sessions per Publisher

A publisher may host an arbitrary number of concurrent sessions, each with its own Stream ID and its own state machine. The publisher's `pub` field (in metadata) is shared across all sessions; the Stream IDs differentiate them.

There is no aggregation primitive in v1.0. A multi-session publisher publishes N independent mDNS records.

---

## 9. Open Items

- Whether to define an explicit "pause emission, hold session" sub-state distinct from ACTIVE for streams where the publisher wants to suppress PUs without closing (currently no — close and re-open).
- Lease default values — 60 s subscription / 5 min idle are first guesses; revisit after field measurements.
- Whether the `closing` event should specify a reason code (currently optional `extra`-style).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
