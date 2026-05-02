# Pelorus monorepo alignment

**Purpose:** Concrete traceability and a minimal feedback loop between **`specifications/`** (normative source of truth, **this repository**), **`platform/`** (Rust implementation), **`reference-implementations/`** (consumer/reference crates), and **`ecdis/`** (S-100 / chart-side integration).

**Status:** Living checklist — update rows when crates move or spec sections stabilize.

**Canonical location:** This file is stored in the [**pelorus-marine/specifications**](https://github.com/pelorus-marine/specifications) repository at the path **`PELORUS_ALIGNMENT.md`**.

---

## 1. Layering (who owns what)

| Layer | Role | Drift rule |
|-------|------|------------|
| `specifications/` | Wire semantics, DCIDs, trust levels | Changes here drive tests and docs elsewhere |
| `platform/` | Executable Core / Stream / State / tooling | API or frame behavior must match spec or carry a **tracked deviation** (issue + comment in code) |
| `reference-implementations/` | Focused samples (e.g. GNSS stack pairing) | Validates “can a third crate integrate?” — gaps feed spec clarity or `pelorus-core` API fixes |
| `ecdis/` | ENC / IHO / UI consumers of Pelorus types | Uses **`pelorus-core`** via path dependency — semver and type churn surface here first |

---

## 2. Traceability matrix (Core-focused)

Normative Core documents live under **`core/`** in this repository. Implementation paths below assume a **combined checkout** (sibling clones: `specifications`, `platform`, `ecdis`, `reference-implementations`).

| Spec | Topic | `platform/` anchor | `ecdis/` touchpoint | `reference-implementations/` |
|------|--------|---------------------|---------------------|------------------------------|
| `02-physical-layer.md` | PHY / CAN FD plant | Bus-facing config in gateways / MCU bring-up (`pelorus-m7/`, future hardware paths) | Indirect (timing assumptions for updates) | Board-level pairing notes in crates as applicable |
| `03-data-link-layer.md` | Frames, addressing usage | `platform/pelorus-core/src/canbus/` | — | — |
| `04-power-management.md` | NM / selective wake | Policy encoded where PM appears in stack (see crate milestones) | — | — |
| `05-addressing.md` | Source address / claiming | Address-related logic colocated with Core bus stack when implemented | — | — |
| `06-signal-catalog.md` | VSS / correlation | `pelorus-core` semantics / correlation (`src/semantics.rs`, `src/correlation.rs`) | `pelorus-ecdis` snapshot models vs Core semantics | — |
| `07-dcid-registry.md` | DCID assignments | `platform/pelorus-core/src/dcid/` | Types bridged via `pelorus-ecdis` / adapter | — |
| `09-gateway-specification.md` | LMDE ↔ Pelorus gateway | Future `pelorus-gateway` / inspector flows | — | — |
| `11-reference-implementations.md` | Logical ↔ physical map | **11** §1 maps components to **`platform/`** tree (`pelorus-core/src/dcid`, …); gateway scaffold in **`reference-implementations/pelorus-gateway`** | `ecdis/pelorus-ecdis`, adapters | `reference-implementations` pairing / gateway scaffolds |
| `15-conformance-test-plan.md` | Procedures / fixtures | CI and integration tests under each workspace | Add ECDIS-specific conformance cases when defined | — |

**Optional** standalone `pelorus-dcid` / `pelorus-pm`-style crates are **TBD**; **`pelorus-core`** in **`platform/`** is the integration home today — see **11** §1.

---

## 3. Known cross-artifact gaps (prioritize these)

Track remaining review debt in **`core/00-document-index.md`** (*Next priorities*). **Resolved in-repo:** **03** / **07** DCID band wording and **04** / **07** **WUF** & **NM** on-wire layouts — **07** defers to **04** §7 for payloads.

Outstanding example: **instance binding** across **06** and gateways vs live traffic patterns.

When you close an item in prose, update **this matrix** and add or adjust a **test or fixture** in `platform/` (and note ECDIS impact if own-ship or DCIDs move).

---

## 4. Feedback loop — repeatable rituals

### A. Spec change landed (`specifications/` PR merged)

1. Identify affected rows in §2.
2. Update `platform/` implementation or file a **short deviation issue** if implementation lags.
3. Run **`platform`** checks (embedded + default features), e.g.:
   - `cargo test --workspace` from `platform/`
   - `cargo check -p pelorus-core --no-default-features --features canbus,alloc`
   - `cargo check -p pelorus-core --no-default-features --features canbus_heapless`
4. From `ecdis/`, run tests after Core API changes: `cargo test --workspace` (path dependency pulls local `pelorus-core`).
5. If `reference-implementations/` consumes `pelorus-core` in the future, run its workspace tests the same way.

### B. Implementation discovered a spec bug or ambiguity

1. Open an issue on **`pelorus-marine/specifications`** (or cross-post link) with **minimal repro**: frame hex, DCID, and expected vs actual behavior.
2. Label or prefix **`spec-gap`** for triage.
3. Do not “fix” wire meaning only in Rust — either amend the spec or document **temporary** behavior in code with a `TODO(spec)` and issue ID.

### C. ECDIS integration friction

1. Classify: **type/API** issue (fix in `platform/pelorus-core` or adapter), **nautical semantics** (fix in spec + Core), or **pure chart/UI** (stay in `ecdis/`).
2. If `pelorus-ecdis` types diverge from `07-dcid-registry` / `06-signal-catalog`, add a row note under §2 or a short `ARCHITECTURE.md` subsection in the affected crate (avoid duplicating normative prose).

### D. Periodic polish (suggested cadence: monthly or per release tag)

1. Re-read **`core/00-document-index.md`** completion counts vs open **spec-gap** issues.
2. Diff **path dependency** consumers (`ecdis/pelorus-ecdis/Cargo.toml` → `pelorus-core`) against last known good **spec git revision** you trust.
3. Archive or close deviation issues that are now resolved in spec + code.

---

## 5. Version / clone expectations

Use a **single tree** with `specifications`, `platform`, `ecdis`, and `reference-implementations` as sibling directories when validating cross-layer work. Standalone clones of `ecdis` may use **git submodule / CI checkout** of `platform` — see `ecdis/.github/workflows` and companion actions for automation context.

---

## 6. Related reading

- **Architecture narrative (non-normative):** [`ARCHITECTURE.md`](./ARCHITECTURE.md)
- **Core document index:** [`core/00-document-index.md`](./core/00-document-index.md)
- **Platform workspace:** [platform `README.md`](https://github.com/pelorus-marine/platform/blob/main/README.md)
- **ECDIS workspace:** [ecdis `README.md`](https://github.com/pelorus-marine/ecdis/blob/main/README.md)
