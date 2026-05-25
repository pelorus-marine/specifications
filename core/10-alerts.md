# Pelorus Core — Alert System

Pelorus-native alert system: structured propagation of operator-facing alert conditions across the Core bus, with lifecycle, acknowledgement, multi-source coordination, and bridging to NMEA 2000 alert PGNs. Wire-level DC assignments are normative in [`07-dcid-registry.md §1.8`](./07-dcid-registry.md); this document specifies semantics, state machine, and payload layouts. Alert history, summary views, and the alarm-management UI are Stream-side concerns and out of scope here.

The alert system is semantically aligned with IEC 61924-2 (Bridge Alert Management) and NMEA 2000 alert PGNs (126983–126988), but uses Pelorus-native identifiers, ack semantics, and bus encoding. Bridge to legacy alerts is normative in §11.

## 1. Categories

Every alert belongs to exactly one of four categories. The category determines lifecycle behaviour, ack requirements, and Core arbitration priority.

| Category | Code | Lifecycle | Operator action | Core priority |
| --- | --- | --- | --- | --- |
| Alarm | `0` | Persistent until **both** condition clears **and** operator acknowledges | Required | 2 |
| Warning | `1` | Persistent until condition clears; ack closes UI but is not required | Recommended | 2 |
| Caution | `2` | Self-clears when condition clears; no ack required | None | 3 |
| Info | `3` | Self-clearing notification | None | 4 |

Sources shall assign category at design time per the safety analysis of the originating function; categories are not user-configurable on the wire.

## 2. Severity

Within a category, severity orders alerts for display when multiple are concurrently active. Severity is a 3-bit unsigned integer (0–7); lower is more severe.

Severity is advisory for display only. It does not alter wire priority or override category lifecycle. Two alerts with the same category and severity are equally prominent; receivers may break ties by time-of-occurrence.

## 3. Alert Identifier

The globally unique identifier for an alert is the triple `(source NAME, alert_id, instance)`:

- **source NAME** — 64-bit J1939-81 NAME of the device that raised the alert, per [`05-addressing.md`](./05-addressing.md). Identifies which physical device claims this alert.
- **alert_id** — 16-bit identifier assigned by the source device within its own namespace. A given source vendor partitions alert_id values across its product line; the same alert_id from different sources means different things.
- **instance** — 16-bit per-source-per-alert_id discriminator. Lets a source report the same alert from multiple causes simultaneously (e.g. two cylinder banks each over-temperature ⇒ same alert_id, distinct instance).

A receiver shall deduplicate by the full triple. Two frames with the same triple are the same alert; two frames differing in any field are distinct alerts.

## 4. Lifecycle

```text
       ┌─────────┐
       │ Inactive│ ◀───────────────────────────────────┐
       └────┬────┘                                     │
            │ condition arises                         │
            ▼                                          │
       ┌─────────┐                                     │
       │ Active  │─────silence──────▶ Active-Silenced  │
       └────┬────┘                          │          │
            │                               │          │
            │ ack                           │ ack      │
            ▼                               ▼          │
       ┌─────────────┐                ┌─────────────┐  │
       │Acknowledged │ ◀── ack ───── │   (same)    │  │
       └──────┬──────┘                └──────┬──────┘  │
              │ condition clears             │         │
              ▼                              ▼         │
       ┌──────────┐                                    │
       │ Cleared  │ ─────────────────────────────────▶ │
       └──────────┘                                    │
                                                       │
       (alarm category only:)                          │
       Active + clears-before-ack ▶ Cleared-Unacked ───┘
       Cleared-Unacked + ack ▶ Inactive
```

State semantics:

| State | Code | Description |
| --- | --- | --- |
| `Inactive` | `0` | No active alert for this triple. Sources do not transmit `AlertAnnounce` for inactive alerts. |
| `Active` | `1` | Condition is currently true and unacknowledged. |
| `Active-Silenced` | `2` | Condition still true; operator has silenced audible/visual annunciation but has not formally acked. |
| `Acknowledged` | `3` | Operator has acked; condition may still be true. |
| `Cleared-Unacked` | `4` | Alarm-category only: condition cleared before ack; alert persists in UI until acked, then transitions to `Inactive`. |

Sources retransmit `Pelorus.AlertAnnounce` at 1 Hz while in any non-`Inactive` state. Receivers expire stale alerts (no announce received for 3× the expected period) and surface the timeout as a Caution-class alert against the alert-bus health itself.

## 5. Wire Encoding

Three DCs implement the alert wire protocol, allocated in [`07-dcid-registry.md §1.8`](./07-dcid-registry.md).

### 5.1 `Pelorus.AlertAnnounce` (DC_ID `0x00010`)

Priority 2. Single CAN FD frame, 16 bytes.

| Offset | Size | Field | Notes |
| --- | --- | --- | --- |
| 0 | 8 | source NAME | J1939-81 NAME of the source. |
| 8 | 2 | alert_id | Source-namespace identifier (little-endian). |
| 10 | 2 | instance | Per-source-per-alert_id discriminator (little-endian). |
| 12 | 1 | category | `0`=alarm, `1`=warning, `2`=caution, `3`=info. Upper 5 bits reserved, transmit 0. |
| 13 | 1 | severity + state | Low 3 bits: severity (0–7, lower more severe). High 3 bits: state (`0`–`4` per §4). Bits 6–7 reserved. |
| 14 | 1 | flags | Bit 0: condition cleared (`Cleared-Unacked` only). Bit 1: silenced. Bit 2: power-on (set on first announce after source boot, cleared after first remote ack). Bits 3–7 reserved. |
| 15 | 1 | trigger code | Source-defined sub-cause of this alert occurrence (e.g. which threshold was crossed). 0 if not used. |

Transmission cadence: 1 Hz while in any non-`Inactive` state. On state transitions (active→acked, condition clear, etc.) the source emits an immediate frame, then resumes the 1 Hz cadence.

### 5.2 `Pelorus.AlertResponse` (DC_ID `0x00011`)

Priority 3. Single CAN FD frame, 22 bytes.

| Offset | Size | Field | Notes |
| --- | --- | --- | --- |
| 0 | 8 | source NAME | Of the *alerting* device (echoed from the announce). |
| 8 | 2 | alert_id | Echoed from the announce. |
| 10 | 2 | instance | Echoed from the announce. |
| 12 | 8 | responder NAME | NAME of the device sending the response. |
| 20 | 1 | response | `0`=ack, `1`=silence, `2`=reset (force back to `Inactive`; alarm-recovery only, requires authority). |
| 21 | 1 | flags | Bit 0: cross-station — this response is being forwarded by a redundant authority. Bits 1–7 reserved. |

A source receiving its own `AlertResponse` updates its state machine and emits the next `AlertAnnounce` reflecting the new state immediately (does not wait for the next 1 Hz tick).

### 5.3 `Pelorus.AlertText` (DC_ID `0x00012`)

Priority 4. Multi-frame via `Pelorus.MultiFrameData` per [`03-data-link.md §4`](./03-data-link.md). Carries human-readable description of an alert.

Payload after the alert triple header:

| Offset | Size | Field | Notes |
| --- | --- | --- | --- |
| 0 | 8 | source NAME | Of the alerting device. |
| 8 | 2 | alert_id | |
| 10 | 2 | instance | |
| 12 | 5 | locale | BCP-47 language tag (`en`, `de`, `fr-CA`, …), space-padded, ASCII. |
| 17 | 2 | text length | Bytes of UTF-8 text following (little-endian). |
| 19 | N | text | UTF-8 description, ≤ 512 bytes. |

`AlertText` is sent on request (via `Pelorus.Request`) or pushed once on the first `AlertAnnounce` for a given triple after source boot. Receivers cache text by triple and do not re-request unless cache is invalidated.

Multiple locale variants for the same alert may be transmitted; receivers select preferred locale. A source that does not provide text for a requested locale shall respond with the source-default locale.

## 6. Acknowledge vs Silence

These are distinct operator actions with distinct wire effects:

| Action | Wire `response` | State effect | Audible/visual |
| --- | --- | --- | --- |
| Acknowledge | `0` | `Active → Acknowledged` (or `Cleared-Unacked → Inactive`) | Stops audible; alert may close from UI per category lifecycle |
| Silence | `1` | `Active → Active-Silenced` | Stops audible; alert remains visually present |
| Reset | `2` | Any → `Inactive` | Stops all annunciation; alert dropped from history active-view |

Silence does not satisfy the alarm-category ack requirement; an alarm-category alert remains `Active-Silenced` until ack. Reset is privileged: receivers shall ignore a reset response whose responder NAME is not on the source's reset-authority list. The reset-authority mechanism is configuration, not wire protocol; v1.0 does not specify how the list is provisioned.

## 7. Multi-Source Coordination

When two physically distinct sources detect the same condition (two depth sounders both report shoal water; engine ECU and oil-pressure sensor both report low oil), each issues its own alert with its own `(NAME, alert_id, instance)` triple. The wire protocol does not aggregate.

Receivers MAY UI-group alerts that share category + similar alert_id, but the wire identity remains distinct. Aggregation across redundant safety devices (where agreement increases confidence) is a State subsystem concern and out of scope for Core.

A source that detects another source already reporting the same condition (via the binding table's knowledge of what each device monitors) MAY suppress its own announcement — but this is policy, not wire protocol. By default, every source reports independently.

## 8. Multi-Station Vessels

Vessels with multiple operator stations (bridge, flybridge, nav station) handle alert transfer via the silence/ack semantics in §6 plus a station-of-authority policy: at any moment, one station is the "primary." Acks from the primary close the alert across all stations; silences from a non-primary do not.

The wire protocol carries the responder NAME on every `AlertResponse`, so any source can determine which station responded. The primary-station policy lives in vessel configuration, not in the wire DC. A future revision MAY add a `Pelorus.AlertAuthorityClaim` DC if cross-station authority transfer needs on-wire coordination beyond what address-claim already provides.

For BNWAS (Bridge Navigation Watch Alarm System) interactions per IMO Resolution MSC.128(75), see open items in `../../ISSUES.md`.

## 9. Alert History (Stream)

The historical record of alerts (when raised, when acked, by whom, when cleared) is held on Pelorus Stream, not Core. Core is the real-time announcement channel; Stream is the durable record and the alarm-management UI surface. The Stream service definition is forward-referenced and tracked in `../../ISSUES.md`.

A Core-only deployment (no Stream subsystem) loses history but retains the full real-time alert function — alerts still announce, acknowledge, and clear correctly. History is an enrichment, not a dependency.

## 10. Source Configuration

The set of alert conditions a source monitors, the thresholds, and the per-alert category/severity assignments are source configuration. v1.0 does not specify on-wire configuration of alert thresholds — that follows the same out-of-band-config pattern as the binding table ([`06-instance-binding.md §2`](./06-instance-binding.md)).

NMEA 2000 PGNs 126986–126988 (Alert Configuration / Threshold / Value) define an on-wire configuration mechanism; Pelorus has not adopted this in v1.0. Future revisions MAY add equivalent DCs if a use case justifies on-wire reconfiguration.

## 11. Bridge from NMEA 2000

A Core ↔ NMEA 2000 gateway bridges legacy alerts per [`07-dcid-registry.md §2.1`](./07-dcid-registry.md). Decomposition direction (NMEA 2000 → Pelorus) is required:

| NMEA 2000 PGN | Pelorus DC | Mapping |
| --- | --- | --- |
| 126983 (Alert) | `Pelorus.AlertAnnounce` | Source NAME from PGN's Alert System field; alert_id from `AlertID`; category mapped per the table below; state mapped from `AlertState`. |
| 126984 (Alert Response) | `Pelorus.AlertResponse` | Responder NAME from PGN's command source; response code mapped per the table below. |
| 126985 (Alert Text Description) | `Pelorus.AlertText` | UTF-8 text and locale carried through. |

Category mapping (NMEA 2000 → Pelorus):

| N2K AlertType | Pelorus category |
| --- | --- |
| Emergency Alarm | Alarm |
| Alarm | Alarm |
| Warning | Warning |
| Caution | Caution |

Aggregation direction (Pelorus → NMEA 2000) is best-effort per [`07-dcid-registry.md §2.1`](./07-dcid-registry.md). A gateway that emits PGN 126983 from `Pelorus.AlertAnnounce` shall preserve the alert triple identity so a re-bridged alert round-trips to the same triple.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
