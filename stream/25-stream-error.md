# Pelorus Stream — Stream Errors

**Version:** 0.1 Draft  
**Last Updated:** April 27, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document defines the **application-layer error taxonomy** for Pelorus Stream — errors that arise from the *meaning* of stream operations, as distinct from transport-level errors ([`26-transport-error.md`](./26-transport-error.md)). Errors are surfaced via the `error` control message and as locally-raised conditions in the reference library.

Stream errors are **information for State**, not authority over State. Stream surfaces what happened; State decides what to do.

---

## 1. Error Surface

There are three places a stream error appears:

1. **Local exception or `Result::Err` in the reference library** — for the local application that initiated an operation.
2. **`error` control message (`kind=0x00FE`)** — sent across the wire when the local error is relevant to a remote peer.
3. **`event` of a specific name** — when the error is a per-stream condition observable by all subscribers (e.g. `format-mismatch`).

Implementations choose the surface that matches the audience of the error. A decoder failing on a single PU emits a `decode-error` event to subscribers and, optionally, an `error` control message to the publisher; it does not crash.

---

## 2. `error` Message Body

```cbor
{
  "code": "<error-code>",      ; required, identifier from §3
  "level": "warn"|"err"|"fatal",
  ? "details": {<context-specific>},
  ? "ref_seq": <u32>            ; sequence number that caused the error
}
```

Severity:

- `warn` — recoverable; the offending operation will be retried or dropped.
- `err` — non-recoverable for this operation/PU; the stream continues.
- `fatal` — non-recoverable for this **session**; the sender will close.

---

## 3. Stream Error Code Registry

Codes are short stable strings, kebab-case. Forward compatibility: receivers shall ignore unknown codes (log only).

| Code | Level | Meaning | Source |
|---|---|---|---|
| `protocol-error` | err | Malformed envelope; required field missing. | Either |
| `decode-error` | err | CBOR decode failure or codec decode failure. | Receiver |
| `format-mismatch` | err | Received PU does not match negotiated format. | Subscriber |
| `metadata-conflict` | err | Static metadata changed mid-session. | Subscriber |
| `payload-too-large` | err | PU exceeded subscriber's configured maximum. | Subscriber |
| `caps-incompatible` | warn | Subscribe negotiation failed; no usable caps. | Publisher |
| `subscriber-cap-exhausted` | warn | Per-stream subscriber limit reached. | Publisher |
| `not-active` | warn | Operation requested on an inactive stream. | Either |
| `not-seekable` | warn | `seek` requested on a non-seekable stream. | Publisher |
| `out-of-scope` | err | Playback control targets a non-Stream entity. | Publisher |
| `vendor-required` | warn | Vendor capability is required but not advertised. | Publisher |
| `data-loss` | warn | Detected loss not closed within buffer. | Subscriber |
| `clock-discipline` | warn | Severe clock drift detected. | Either |
| `closing` | warn | The peer is shutting down. | Either |
| `internal` | err | Unspecified internal error; details should describe. | Either |

Codes prefixed with `vendor:<reverse-dns>:<name>` are vendor-defined and ignored unless the vendor capability is negotiated.

---

## 4. `out-of-scope` — A Special Case

The `out-of-scope` error is the wire-level enforcement of the boundary in [`01-overview.md` §2](./01-overview.md). A publisher receiving any control message that:

- targets a Pelorus Core entity, or
- attempts to actuate hardware outside Stream's purview, or
- requests Stream to influence Core,

shall reject with `code=out-of-scope, level=err`. The message is **not** acted upon.

`out-of-scope` is the protocol's last line of defense against accidental cross-subsystem coupling. Implementations shall log the offending message and the source address so that a misbehaving controller can be identified and fixed.

---

## 5. Error Propagation

| Origin | Surface |
|---|---|
| Subscriber decoding fails | Local `Result::Err`, `decode-error` event for State. |
| Subscriber format mismatch | Local `Result::Err`, `error` to publisher, `format-mismatch` event. |
| Publisher rejects subscribe | `subscribe-ack` with `result=rejected`; no `error` message. |
| Publisher receives invalid playback-control | `error` back to controller; no event. |
| Either side hits internal panic | local crash, no wire message; recovery via session expiry. |

`error` messages over the wire are best-effort UDP. They may be lost. Senders shall not depend on a peer receiving an `error` message; the local effect of the error is what matters.

---

## 6. Logging

The reference library ([`27-lib.md`](./27-lib.md)) shall log every error at the level configured by the host application. The default level is `warn`; `info`-level messages (joined, deactivated, …) are not errors and are logged separately.

Log lines should include:

- Timestamp
- Stream ID (full or short — implementation choice)
- Error code
- Level
- Brief details

Logs shall not include payload contents (privacy and log size).

---

## 7. Recovery

| Code | Recovery |
|---|---|
| `protocol-error`, `decode-error`, `format-mismatch` | Drop the offending PU. Continue. |
| `metadata-conflict`, `payload-too-large` | Unsubscribe; re-subscribe if metadata reconverges. |
| `caps-incompatible`, `subscriber-cap-exhausted` | Wait, retry after metadata update. |
| `not-active`, `not-seekable` | Surface to controller; State decides next action. |
| `out-of-scope` | Log, do not retry. Bug at the controller. |
| `data-loss` | Apply concealment; continue. |
| `clock-discipline` | Engage drift compensation. |
| `closing` | Tear down; consider re-attaching to a successor stream when one appears. |
| `internal` | Surface; next steps are publisher-specific. |

These recoveries are reference behavior. Production deployments may add their own retry/backoff policies.

---

## 8. Open Items

- Whether to formalize `vendor:` prefix as a protocol-recognized namespace (currently informal).
- A standard rate-limit on `error` messages to avoid log storms (currently best practice, not normative).
- Severity rules for `data-loss` per stream type (currently uniform `warn`).

---

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
