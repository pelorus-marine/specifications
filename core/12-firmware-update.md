# Pelorus Core — Firmware Update

Open, vendor-neutral firmware update protocol for Pelorus Core devices. The mechanism layers on the multi-frame transport in [`03-data-link.md §4`](./03-data-link.md) and uses the firmware-update Data Contracts allocated in [`07-dcid-registry.md §1.7`](./07-dcid-registry.md).

## 1. Why This Is Normative

Vendor-locked firmware update is endemic in the legacy marine ecosystem: chartplotter A can only update devices from manufacturer A, with no documented protocol for third-party tools or other-brand chartplotters to perform the same operation. This is one of the concrete failures Pelorus exists to correct.

A vessel owner who keeps a boat for 20 years will outlive the original manufacturer of at least some installed devices. Without an open update path, a discontinued device becomes either unmaintainable or unsafe (no security fixes). Open firmware update is therefore a precondition for the Pelorus durability premise — devices that work for ten or more years.

This document is normative for every Pelorus Core device that exposes a writable image.

## 2. Scope

In scope:

- Discovery and capability query
- Manifest format and signing model
- Slot model (A/B vs single + recovery)
- Image transfer via multi-frame transport
- Progress reporting
- Activation, rollback, recovery from interruption

Out of scope for v1.0:

- Per-vessel role-based authorization (any addressable node may initiate)
- Vendor key publication infrastructure (signing scheme is normative; key distribution is guidance)
- Application-defined post-install verification beyond image CRC and signature
- Updates of recovery loaders themselves (recovery loaders are fixed ROM)

## 3. Vendor Neutrality

A compliant device shall accept firmware update sessions from any tool that follows this protocol, regardless of the tool's vendor identity. Specifically:

- A device shall not reject `Pelorus.FirmwareUpdateBegin` based on the initiator's NAME manufacturer code.
- A device may reject based on: manifest signature failure (if signing is required), `device_kind` mismatch in the manifest, version-compatibility failure (declared `min_hardware_revision` higher than the device), or insufficient resources (e.g. busy with another session). These rejections shall not implicitly correlate with initiator identity.
- A device that *only* accepts updates initiated from a specific brand of chartplotter, dongle, or app is non-conformant.

Vendors that want to lock down which *images* run on their hardware may require signed manifests (§5). The verification key shall be published in the device's public documentation, so any tool can submit a properly signed image. The signing scheme locks images, not tools.

## 4. Data Contracts

All firmware-update DCs use priority 7 (bulk).

| DC | DC_ID | Direction | Purpose |
| --- | --- | --- | --- |
| `Pelorus.FirmwareUpdateQuery` | `0x0000A` | initiator → device | Request capabilities |
| `Pelorus.FirmwareUpdateResponse` | `0x0000B` | device → initiator | Capabilities reply |
| `Pelorus.FirmwareUpdateBegin` | `0x0000C` | initiator → device | Carries manifest; opens session |
| `Pelorus.FirmwareUpdateProgress` | `0x0000D` | device → all | Progress + status |
| `Pelorus.FirmwareUpdateActivate` | `0x0000E` | initiator → device | Commit / switch slot |
| `Pelorus.FirmwareUpdateRollback` | `0x0000F` | initiator → device | Revert to previous image |

Detailed payload bit layouts are normative below.

### 4.1 `Pelorus.FirmwareUpdateQuery`

Payload (8 bytes):

| Bytes | Field |
| --- | --- |
| 0 | Target SA |
| 1–7 | Reserved — transmit `0x00`, ignore on receive |

A broadcast query (target SA = `0xFF`) prompts every writable-image device to respond.

### 4.2 `Pelorus.FirmwareUpdateResponse`

Payload (up to 64 bytes):

| Bytes | Field |
| --- | --- |
| 0 | Initiator SA (the SA from the corresponding Query) |
| 1 | Slot model: `0` = single + recovery loader, `1` = A/B |
| 2 | Signing model: `0` = unsigned accepted, `1` = signature required |
| 3 | Current slot identifier (`0` or `1` for A/B; always `0` for single) |
| 4–7 | Current image version (semver: `major`, `minor`, `patch`, `build`) |
| 8 | Hardware revision |
| 9 | Maximum concurrent ingress sessions (informative; v1.0 = `1`) |
| 10–13 | Maximum supported `total_size` (`uint32` LE, bytes) |
| 14–15 | `signature_key_id` (only meaningful if signing model = `1`) — identifies which published verification key applies |
| 16–63 | Reserved — transmit `0x00`, ignore on receive |

### 4.3 `Pelorus.FirmwareUpdateBegin`

Payload carries the manifest (up to 64 bytes); larger manifests use multi-frame transport with `content_DC_ID = Pelorus.FirmwareUpdateBegin's DC_ID`.

Manifest fields:

| Field | Type | Notes |
| --- | --- | --- |
| `device_kind` | 8 bytes | NAME class — Industry Group, Device Class, Function (matches J1939-81 NAME structure) |
| `version` | 4 bytes | semver as in §4.2 |
| `min_hardware_revision` | 1 byte | Device shall reject if its hardware revision < this |
| `target_slot` | 1 byte | `0` or `1` for A/B; `0` for single |
| `image_size` | 4 bytes | `uint32` LE, bytes |
| `image_crc32` | 4 bytes | CRC32 over the binary image |
| `content_session_id` | 2 bytes | `session_id` the initiator will use for the binary transfer (referenced by `Pelorus.MultiFrameControl{Open}` that follows) |
| `signature_present` | 1 byte | `0` or `1` |
| `signature_key_id` | 2 bytes | Identifies the published verification key (only meaningful if `signature_present = 1`) |
| `signature` | 64 bytes | Ed25519 over the rest of the manifest (only present if `signature_present = 1`) |

A device that requires signing shall verify `signature` before opening the multi-frame session for the binary; failure → `OpenNak{reason=SignatureInvalid}`.

### 4.4 `Pelorus.FirmwareUpdateProgress`

Transmitted by the device under update at minimum 1 Hz during transfer and on every state change. Broadcast — any node on the bus may subscribe and display progress.

Payload (8 bytes):

| Bytes | Field |
| --- | --- |
| 0–1 | `session_id` |
| 2 | Status: see §6 |
| 3 | Sub-state code (status-specific) |
| 4–7 | `frames_received` (`uint32` LE) |

The total frame count is known from the manifest's `image_size` and the multi-frame transport's frame size; receivers compute percent complete locally.

### 4.5 `Pelorus.FirmwareUpdateActivate`

Payload (8 bytes):

| Bytes | Field |
| --- | --- |
| 0–1 | `session_id` (must match the session whose transfer just completed) |
| 2 | Target SA |
| 3 | Activation mode: `0` = immediate, `1` = at next power cycle |
| 4–7 | Reserved |

On immediate activation with A/B slots, the device flips the boot slot and resets. On single-slot, the device commits the staged image to the active slot and resets. The device emits a final `Pelorus.FirmwareUpdateProgress{Status=ActivationComplete}` before the reset where possible.

### 4.6 `Pelorus.FirmwareUpdateRollback`

Payload (8 bytes):

| Bytes | Field |
| --- | --- |
| 0 | Target SA |
| 1–7 | Reserved |

Valid only on A/B-slot devices. Switches the boot slot back to the previously active image and resets.

## 5. Signing Model

Two paths:

- **Unsigned.** Manifest has `signature_present = 0`. Device accepts the update unconditionally (subject to other manifest checks). Appropriate for hobbyist devices and development.
- **Signed.** Manifest has `signature_present = 1`. Device verifies the Ed25519 signature against a verification key identified by `signature_key_id`. The key is published in the device's public documentation; any tool can submit signed images.

The signing scheme is **Ed25519 over the entire manifest excluding the `signature` field itself**, computed in little-endian field order as defined above. Vendors who use a different scheme are non-conformant.

**Key publication.** A vendor that ships devices requiring signed images shall publish the corresponding verification key(s) in product documentation that is freely accessible online without registration or payment. Publication format is informative (PEM file in datasheet, key on product webpage, etc.), but accessibility is normative.

**Why publication is normative.** A signing scheme whose key authority is private is functionally equivalent to vendor lock-in — only the vendor's tools can produce valid signatures. Publication breaks the lock without breaking the signing.

## 6. Status Codes

`Pelorus.FirmwareUpdateProgress` byte 2 (`Status`):

| Code | Status | Meaning |
| --- | --- | --- |
| `0x00` | Idle | No active session |
| `0x01` | ManifestVerifying | Validating manifest fields and signature |
| `0x02` | Receiving | Multi-frame transfer in progress |
| `0x03` | ImageVerifying | CRC32 verification after receive complete |
| `0x04` | ReadyToActivate | Image staged; awaiting `Activate` |
| `0x05` | Activating | Slot switch in progress |
| `0x06` | ActivationComplete | New image active; final progress message before reset |
| `0x07` | RolledBack | Rollback complete |
| `0xE0` | ManifestRejected | See sub-state for reason |
| `0xE1` | TransferFailed | Multi-frame session aborted |
| `0xE2` | ImageInvalid | CRC32 or signature verification failed |
| `0xE3` | ActivationFailed | Slot switch failed; device remains on previous image |
| `0xE4` | InsufficientResources | Storage, RAM, or session quota |

Sub-state codes are status-specific:

For `ManifestRejected` (`0xE0`): `0x01` SignatureInvalid · `0x02` DeviceKindMismatch · `0x03` HardwareRevisionTooOld · `0x04` UnknownKeyId · `0x05` MalformedManifest.

For `TransferFailed` (`0xE1`): mirrors `Pelorus.MultiFrameControl` reason codes from [`03-data-link.md §4.6`](./03-data-link.md).

## 7. Slot Model

### 7.1 A/B Slots (recommended)

The device has two image slots in flash. The boot loader selects between them based on a slot-active bit in retained or flash storage.

- New images are written to the inactive slot. The active slot continues to run.
- `Activate` flips the slot-active bit and resets. On boot, the loader validates the new slot's CRC; on failure, it falls back to the previous slot automatically (rollback).
- Failed-boot detection: if the new image fails to reach a healthy state within an implementation-defined window (e.g. fails to transmit `Pelorus.NetworkManagement` within 5 s of expected boot), the loader rolls back automatically on the next reset.
- `Rollback` explicitly flips back to the previous slot.

A/B slots provide atomic switchover; a power loss during update never bricks the device.

### 7.2 Single Slot with Recovery Loader

The device has one image slot plus a fixed recovery loader in ROM (not user-replaceable). The recovery loader accepts firmware updates via the same protocol when the main image is corrupt or absent.

- New images are written *in place* to the single slot. The device is non-functional during transfer and resets at the end.
- A power loss mid-transfer leaves the slot invalid. On reset, the loader detects the invalid image and enters recovery mode, accepting a new update.
- The recovery loader exposes a minimum capability set: `Pelorus.FirmwareUpdateQuery`, `Pelorus.FirmwareUpdateResponse`, `Pelorus.FirmwareUpdateBegin`, multi-frame transport, `Pelorus.FirmwareUpdateProgress`.

Single slot is appropriate for resource-constrained devices (≤ 64 KB flash). It is operationally more fragile than A/B because of the recovery-mode interruption visible to the operator.

## 8. Authorization

v1.0: any addressable node may initiate firmware update. There is no node-level access control. A device that requires signed images relies on signature verification (§5) to restrict *what runs*, not *who initiates*.

Per-vessel role-based authorization (e.g. "only nodes claiming the installer role can update propulsion ECUs") is deferred to a future extension. Implementations are welcome to add optional vessel-policy checks above this layer, but the baseline shall remain open.

## 9. Recovery

A device that loses power or otherwise interrupts a session shall retain enough state to allow the initiator to resume via the multi-frame transport's session-id + timeout mechanism in [`03-data-link.md §4.7`](./03-data-link.md):

- Receivers shall retain partial-session state for at least 60 seconds following the last received frame.
- Resumption uses the same `session_id` with `Open` advancing `next_expected_seq` past already-received frames.

A/B-slot devices preserve the inactive slot's partial-receive state across resets where flash storage permits. Single-slot devices in recovery mode preserve nothing — interrupted transfers must restart.

## 10. Examples (informative)

### 10.1 Open update by a third-party tool

1. Tool broadcasts `Pelorus.FirmwareUpdateQuery{target_SA=0x42}` on a vessel containing a wind sensor at SA `0x42`.
2. Sensor responds with `Pelorus.FirmwareUpdateResponse{slot_model=1, signing_model=1, current_version=2.3.1.405, signature_key_id=0x0007}`.
3. Tool fetches the device documentation (online) for the wind sensor's public verification key matching key id `0x0007`. Tool produces a manifest for the new image (version `2.4.0.418`), signed with the matching private key.
4. Tool sends `Pelorus.FirmwareUpdateBegin{manifest}`. Sensor verifies signature, accepts, emits `Progress{Status=ManifestVerifying}`.
5. Tool opens a multi-frame session via `Pelorus.MultiFrameControl{Open}` with the manifest's `content_session_id`, total size = image_size, content CRC32 = image_crc32.
6. Image streams over `Pelorus.MultiFrameData`. Sensor emits `Progress{Status=Receiving, frames_received=N}` at ≥ 1 Hz.
7. On `Close`, sensor verifies CRC32; emits `Progress{Status=ReadyToActivate}`.
8. Tool sends `Pelorus.FirmwareUpdateActivate{activation_mode=0}`. Sensor flips slot, resets, boots new image, emits `Progress{Status=ActivationComplete}` from the new image.

The tool is not affiliated with the sensor's vendor; the only vendor input is the published verification key.

### 10.2 Recovery from interrupted update

Same as §10.1 through step 6, but power fails at frame N=42000 of 75000. After power restoration, the tool retries `Pelorus.MultiFrameControl{Open}` with the same `session_id`. Sensor responds with `OpenAck{next_expected_seq=42001, window_size_granted=...}`. Tool resumes from frame 42001 without re-transmitting frames 0–42000.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
