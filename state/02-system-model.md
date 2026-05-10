# Pelorus State — System Model

**Version:** 0.1 Draft
**Last Updated:** May 10, 2026
**Trust:** Unverified

The static structure State operates on: entities, coordinate frames, and the static transforms between them. **No live values** — these live in [`04-world-snapshot.md`](./04-world-snapshot.md). **No semantic identity** — that lives in [`05-situation-model.md`](./05-situation-model.md).

## 1. Entities

An **entity** is a typed, identified thing State knows about. Each entity has a stable identifier and a fixed type for its lifetime.

### 1.1 Entity Types

| Type | Purpose | Lifetime |
|---|---|---|
| `vessel-self` | The own-ship | Static; one per State instance |
| `sensor` | A measurement device mounted on the vessel | Static; configured at install |
| `contact` | An external object detected by sensors (other vessels, marks, hazards) | Dynamic; created/retired by tracking |
| `static-feature` | A charted, position-fixed feature relevant to navigation (buoy, light, hazard area) | Static; from charts |
| `region` | A bounded area relevant to policy (anchorage, restricted zone, traffic separation) | Static or chart-derived |

### 1.2 Sensor Sub-types

| Sub-type | Examples |
|---|---|
| `gnss` | GPS, Galileo, GLONASS receivers |
| `radar` | Marine radar antenna |
| `ais` | AIS receiver |
| `imu` | Inertial measurement unit |
| `gyro` | Gyrocompass, fluxgate compass |
| `depth` | Echosounder |
| `wind` | Anemometer |
| `log` | Speed log (paddle, doppler) |

Contact sub-types (vessel, buoy, sea-mark, etc.) are assigned by the situation model, not here.

### 1.3 Entity Identifier

A stable, locally-unique identifier of the form:

```
<type>:<instance>
```

| Field | Form |
|---|---|
| `type` | An entity type from §1.1 |
| `instance` | UUIDv7 for dynamic entities; installer-assigned slug for static entities |

Examples: `vessel-self:0`, `sensor:radar-bow`, `sensor:gnss-primary`, `contact:018f3c2b-9a4d-7c80-…`.

The identifier is opaque to consumers beyond the `type:` prefix. Mapping to wire identities (Core NAME, AIS MMSI, vessel name) is the situation model's job, not §02's.

## 2. Coordinate Frames

All frames are right-handed.

### 2.1 Earth-Fixed Frames

| Frame | Use | Convention |
|---|---|---|
| **WGS84 geographic** | Absolute position (vessel, contacts) | EPSG:4326. Latitude, longitude in degrees; altitude in metres above ellipsoid |
| **ECEF** | Geometry across long distances | Earth-Centered Earth-Fixed cartesian, metres |
| **NED** (local tangent plane) | Short-range relative geometry | North–East–Down, metres, anchored at a reference point (typically own-ship's current position) |

### 2.2 Body Frame

The own-ship body frame:

- **x** — forward (toward bow)
- **y** — starboard
- **z** — down

Origin: a documented structural reference, fixed for the configuration's lifetime.

Vessel attitude relative to NED:

| Quantity | Convention |
|---|---|
| **Heading** | Clockwise from true north, body **x** projected onto local horizontal, range [0°, 360°) |
| **Roll** | Rotation about body **x**, positive starboard-down |
| **Pitch** | Rotation about body **y**, positive bow-up |
| **Yaw rate** | Rotation rate about body **z**, positive bow-right |

Angles are radians on the wire; degrees are permitted only in human-facing UI.

### 2.3 Sensor Frames

Each sensor has its own right-handed frame whose pose relative to the body frame is static configuration: a translation `(x, y, z)` in metres and a rotation expressed as a unit quaternion `(w, x, y, z)`.

| Sensor type | Frame convention |
|---|---|
| `radar` | x = bore-sight (zero-bearing reference), y = right-hand rule, z = up |
| `gnss` | Antenna phase centre at origin; orientation immaterial (position-only) |
| `imu` | Manufacturer-specified; remapped to body convention via the static rotation |
| `gyro` | x = forward, y = starboard, z = down (typically aligned with body) |

## 3. Frame Relationships

Three transforms are sufficient:

```
T_sensor → body  : static (per sensor, from §4 calibration)
T_body   → NED   : live   (from attitude in the snapshot)
T_NED    → WGS84 : live   (from own-ship position in the snapshot)
```

A measurement in any frame is transformed to any other by composing these three. State shall not introduce shortcut transforms that bypass the body frame.

## 4. Calibration

Static configuration that completes the system model:

| Field | Per | Form |
|---|---|---|
| Body-frame origin offset | Vessel | `(x, y, z)` metres from a documented hull reference |
| Sensor mount translation | Sensor | `(x, y, z)` metres in body frame |
| Sensor mount rotation | Sensor | Unit quaternion `(w, x, y, z)` |

Calibration is provided as part of vessel commissioning; provisioning format is out of scope.

Calibration is **read-only reference data** for State. Recalibration requires a State restart with the new configuration; mid-session calibration changes are not supported in v1.0.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](../LICENSE.md).
