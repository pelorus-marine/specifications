# Pelorus — Manufacturer Code Registry

The Pelorus Manufacturer Code Registry lists 11-bit Manufacturer Code values administered by the Pelorus project. The same 11-bit code identifies a vendor across NMEA 2000, NMEA OneNet, and Pelorus Core (all of which embed the J1939-81 NAME), and is the recommended source of vendor identity for Pelorus Stream metadata where present.

This document is project-wide — not Core-specific — because the Manufacturer Code is the only identity primitive that crosses every protocol Pelorus interoperates with.

## 1. Scope and Reach

| Network | Carrier of the 11-bit Manufacturer Code | Administration |
| --- | --- | --- |
| **NMEA 2000** | J1939-81 NAME (in `ISO Address Claim` PGN 60928) | NMEA, paid (~ USD 1200) |
| **NMEA OneNet** | NMEA NAME (same value as N2K; carried in the IPv6 PGN extension header and DNS-SD service records) | NMEA, paid |
| **SAE J1939** (industrial / heavy duty) | J1939-81 NAME | SAE, committee-allocated |
| **Pelorus Core** | J1939-81 NAME in `Pelorus.AddressClaim` ([`core/05-addressing.md §2`](./core/05-addressing.md)) | Pelorus, free PR-allocated for `1900`–`2047`; existing NMEA / SAE codes accepted as-is for `0`–`1899` |
| **Pelorus Stream** | mDNS TXT `vendor` field, recommended to derive from Core NAME for nodes that also participate in Core ([`stream/02-data-model.md`](./stream/02-data-model.md), [`stream/08-discovery-and-registry.md`](./stream/08-discovery-and-registry.md)) | Pelorus, free; freeform text in v1.0 |

A Manufacturer Code allocated by NMEA or SAE is usable across **all five** rows above without any Pelorus-side registration. A Manufacturer Code allocated by Pelorus (in the `1900`–`2047` range) is usable on Pelorus Core and as the Stream `vendor` derived value, but is **not registered** with NMEA or SAE — vendors using a Pelorus-allocated code who later ship to NMEA 2000 or OneNet networks will appear there as an unregistered Manufacturer Code value.

## 2. Code Space Partitioning

The 11-bit Manufacturer Code field (`0`–`2047`) is partitioned as follows for Pelorus use:

| Range | Administration | Notes |
|---:| --- | --- |
| `0` | Conventional "unassigned" | Recommended for owner-built devices per [`core/05-addressing.md §2.1`](./core/05-addressing.md). Has no defined semantic in J1939-81. |
| `1`–`1899` | NMEA / SAE | Pelorus does not allocate in this range. Vendors with NMEA-administered or SAE-administered Manufacturer Codes use their existing value directly. The [canboat](https://github.com/canboat/canboat) project maintains a community-reverse-engineered list of NMEA allocations (Apache 2.0); the highest allocation observed in that list at the time of v1.0 is in the 1400s. |
| `1900`–`2047` | **Pelorus** | PR-allocated via this registry. Free; no fees, no membership, no commercial-status check. |

This partitioning is a Pelorus convention. It is not coordinated with NMEA or SAE. The buffer between the current NMEA allocation frontier (≈ 1500) and the start of the Pelorus range (1900) is deliberate; if future NMEA allocations encroach on `1900`–`2047`, Pelorus allocations take precedence on Pelorus Core and the collision is handled per §4.

## 3. Using an Existing NMEA / SAE Code

Vendors with an existing NMEA 2000 or SAE J1939 Manufacturer Code use that value directly in their Pelorus NAME — no Pelorus-side registration or notification is required. The Pelorus registry does not duplicate the NMEA list.

If a Pelorus tool (gateway, decoder, diagnostic application) needs a name-for-value lookup for an existing NMEA code, it should consult the canboat database or NMEA's own documentation. Pelorus does not normatively reproduce that data.

## 4. Allocation Policy (Pelorus Range, `1900`–`2047`)

Allocation is **free**, by **pull request**, with no fees, no membership, and no commercial-status check.

To request a code, open a PR adding a row to [§6](#6-allocated-codes):

1. Set `Code` to a free value in `1900`–`2047`, **or** to `next` to let a maintainer assign the next available value at merge time.
2. Provide a name and a contact (email or GitHub handle). URL is optional.

The maintainer team merges valid PRs after a basic well-formedness check.

Allocations are **permanent**. A Pelorus-allocated code is not revoked even if the holder becomes inactive — receivers that have configured filters on the code shall not be invalidated by administrative action. A single code identifies a **manufacturer**, not a product; one organisation needs one code regardless of how many devices or product lines it ships.

**Future direction (informative).** A web-based allocation portal is planned to lower friction further — guided submission, automatic next-free-value assignment, and lightweight fraud / duplicate-name checks before the corresponding repository PR is opened. The PR-based path described above remains the canonical fallback after the portal exists.

## 5. Collisions

Pelorus and NMEA / SAE administer overlapping 11-bit spaces independently. Collisions are possible in two directions:

- **NMEA allocates a value in `1900`–`2047`.** A vendor with that newly-allocated NMEA code who later joins Pelorus may find the value already held in the Pelorus registry. The vendor requests a separate Pelorus allocation in `1900`–`2047` (any free value), and uses their NMEA-allocated value on NMEA 2000 / OneNet and their Pelorus-allocated value on Pelorus Core. The two identities are documented as belonging to the same vendor in the registry entry.
- **A Pelorus-registered organisation later obtains an NMEA code.** They keep their Pelorus allocation in `1900`–`2047` for Pelorus Core and use their NMEA-allocated value on NMEA 2000 / OneNet.

Pelorus tooling shall treat each frame's source identity according to the network it is observed on, and not assume a single global Manufacturer Code → vendor mapping across protocols.

## 6. Allocated Codes

### Pelorus-allocated (`1900`–`2047`)

| Code | Name | Contact | URL | Allocated |
|---:| --- | --- | --- | --- |
| — | _(no allocations yet)_ | — | — | — |

## 7. Requesting a Code

Open a pull request against this repository that adds a row to the table in §6. Suggested PR title format: `manufacturer-code: allocate <code-or-next> for <name>`.

Suggested PR body:

```text
Name:       <organisation or individual name>
Contact:    <email or GitHub handle>
URL:        <optional — project, organisation, or product page>
Code:       <free value in 1900–2047, or 'next' for maintainer assignment>
Purpose:    <brief description — e.g. "open-source masthead sensor firmware",
             "community LiDAR project", "small-batch engine monitor">
```

The maintainer team will verify the chosen code is unused and merge.

## License

This document is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](./LICENSE.md).
