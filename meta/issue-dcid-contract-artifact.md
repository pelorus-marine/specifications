# GitHub issue draft: Machine-readable DCID contract artifact

Paste into a new issue on [pelorus-marine/specifications](https://github.com/pelorus-marine/specifications) when [**Issue #3 — DCID evolution**](https://github.com/pelorus-marine/specifications/issues/3) converges on a stable on-wire layout.

---

## Title

**Generate and publish `dcid-contract` machine-readable artifact from `Vessel.*` + registry**

## Problem

Today, `dcid` appears in `catalog/vessel.vspec` and narrative docs in `core/07-dcid-registry.md`, but there is no **single generated file** that tooling (code generators, gateways, CI) can consume to validate “this DCID ↔ this VSS leaf ↔ these bitfields” once Issue #3 finalizes versioning/namespaces.

## Proposal

1. **After** Issue #3 and `07-dcid-registry.md` agree on the DCID **header** and **core payload + TLV** rules, add a build step (likely in a future `signal-catalog` or `dcid-rs` repo) that emits:
   - `dcid-contract.json` (or CBOR equivalent), **or**
   - Generated Rust/TypeScript modules **from** that JSON schema.
2. **Source of truth order:**  
   - Semantics: `catalog/vessel.vspec` + overlays  
   - Wire: `core/07-dcid-registry.md` + compatibility tables  
   - Generated artifact: **must be reproducible** from those inputs; not hand-edited.
3. **CI:** PRs that change any `dcid:` in the catalog or normative fields in **07** must either update the artifact in the same PR or fail the build until regenerated.

## Acceptance criteria

- [ ] Schema documented (JSON Schema or CDDL) in **`core/`** or **`meta/`**
- [ ] One command reproduces artifact from clean checkout
- [ ] `CONTRIBUTING.md` cross-links to this issue or a successor “closed” design note

## Blockers

- Open design in Issue #3 (64-bit header vs legacy 18-bit DCID map, TLV rules, governance).

---

*This file is editorial only; it is not normative.*
