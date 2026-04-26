# Pelorus Core — Compliance Self-Declaration

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document provides the manufacturer self-declaration template for Pelorus Core conformance. The attestation model is summarized in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary); use this file for the **exact** declaration text and checklist once [15-conformance-test-plan.md](./15-conformance-test-plan.md) is fully authored (stub today).

---

## 1. Manufacturer Self-Declaration Template

**Product Identification**  
- Manufacturer: _______________________________  
- Model / Part Number: _______________________________  
- Hardware Revision: _______________________________  
- Firmware Version: _______________________________  
- Date of Declaration: _______________________________  

**Declaration**

We, the undersigned, declare that the above-identified product meets the requirements of the Pelorus Core specification version 0.1 and is therefore **Pelorus Core conformant**.

Specifically, the product has been verified to comply with the Pelorus Core documents in the repository **`core/`** directory:

- Physical Layer (`core/02-physical-layer.md`)
- Data Link Layer (`core/03-data-link-layer.md`)
- Addressing (`core/05-addressing.md`)
- Power Management (`core/04-power-management.md`)
- Signal Catalog and Binding (`core/06-signal-catalog.md`)
- PGN Registry (`core/07-pgn-registry.md`)
- Network Architecture (`core/08-network-architecture.md`)
- Gateway Behavior (`core/09-gateway-specification.md`)
- Repeater Behavior (`core/10-repeater-specification.md`)
- Reference Implementation Rules (`core/11-reference-implementations.md`)
- Hardware Design Requirements (`core/12-hardware-design-guide.md`)
- Firmware Design Requirements (`core/13-firmware-design-guide.md`)
- Installation Requirements (`core/14-installation-guide.md`)
- Conformance Test Plan (`core/15-conformance-test-plan.md`)

**Test Results**  
All mandatory tests in `core/15-conformance-test-plan.md` were executed and passed. Test logs and results are available upon request.

**Signature**  

Manufacturer Representative: _______________________________  
Printed Name: _______________________________  
Title: _______________________________  
Date: _______________________________  

---

## 2. How to Use This Template

1. Fill in the product details and sign the declaration.
2. Publish the completed declaration alongside the product documentation (e.g. on the product webpage or in the user manual).
3. Include the Pelorus Core conformant logo (when available) on the product, packaging, and marketing materials.
4. Retain test records for at least 5 years in case of dispute.

---

## 3. Open Items (to be resolved before v1.0 promotion)

- Official Pelorus Core conformance logo and usage guidelines
- Digital signing method for declarations (optional for v1.0)
- Public registry of declared-conformant products
- Revocation procedure for non-conformant products

---

*This document, together with documents 01–15, completes the minimum viable specification for Pelorus Core.*