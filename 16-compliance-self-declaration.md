# Pelorus Core — Compliance Self-Declaration

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification (normative for v1.0)

---

## About This Document

This document provides the official manufacturer self-declaration template for Pelorus Core conformance.

Any manufacturer claiming that a device, repeater, gateway, or firmware is Pelorus Core conformant must complete and publish this declaration. It is a formal attestation that the product has been tested against the full specification set (documents 01–15) and passes all applicable tests in `15-conformance-test-plan.md`.

**Design decision (locked):** Conformance is based on self-testing against the reference implementations. No third-party certification body is required for v1.0.

---

## Manufacturer Self-Declaration Template

**Product Identification**  
- Manufacturer: _______________________________  
- Model / Part Number: _______________________________  
- Hardware Revision: _______________________________  
- Firmware Version: _______________________________  
- Date of Declaration: _______________________________  

**Declaration**

We, the undersigned, declare that the above-identified product meets the requirements of the Pelorus Core specification version 0.1 and is therefore **Pelorus Core conformant**.

Specifically, the product has been verified to comply with:

- Physical Layer (`02-physical-layer.md`)
- Data Link Layer (`03-data-link-layer.md`)
- Addressing (`05-addressing.md`)
- Power Management (`04-power-management.md`)
- Signal Catalog and Binding (`06-signal-catalog.md`)
- PGN Registry (`07-pgn-registry.md`)
- Network Architecture (`08-network-architecture.md`)
- Gateway Behavior (`09-gateway-specification.md`)
- Repeater Behavior (`10-repeater-specification.md`)
- Reference Implementation Rules (`11-reference-implementations.md`)
- Hardware Design Requirements (`12-hardware-design-guide.md`)
- Firmware Design Requirements (`13-firmware-design-guide.md`)
- Installation Requirements (`14-installation-guide.md`)
- Conformance Test Plan (`15-conformance-test-plan.md`)

**Test Results**  
All mandatory tests in `15-conformance-test-plan.md` were executed and passed. Test logs and results are available upon request.

**Signature**  

Manufacturer Representative: _______________________________  
Printed Name: _______________________________  
Title: _______________________________  
Date: _______________________________  

---

## How to Use This Template

1. Fill in the product details and sign the declaration.
2. Publish the completed declaration alongside the product documentation (e.g. on the product webpage or in the user manual).
3. Include the Pelorus Core conformant logo (when available) on the product, packaging, and marketing materials.
4. Retain test records for at least 5 years in case of dispute.

---

## Open Items (to be resolved before v1.0 promotion)

- Official Pelorus Core conformance logo and usage guidelines
- Digital signing method for declarations (optional for v1.0)
- Public registry of declared-conformant products
- Revocation procedure for non-conformant products

---

*This document, together with documents 01–15, completes the minimum viable specification for Pelorus Core.*