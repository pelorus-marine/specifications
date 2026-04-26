# Pelorus Core — Conformance Test Plan

**Version:** 0.1 Draft  
**Last Updated:** April 26, 2026  
**Status:** Pre-specification  
**Trust:** Unverified

---

## About This Document

This document will define the conformance test plan (categories, fixtures, pass/fail) for Pelorus Core devices. The **conformance model** (self-test against reference implementations; no third-party cert for v1.0) is stated once in [01-overview.md §9](./01-overview.md#9-locked-decisions-authoritative-summary) and in [16-compliance-self-declaration.md](./16-compliance-self-declaration.md). **Normative** test procedures will live here when authored.

> **Stub — content TBD.** This file is a placeholder. The real test plan has not been authored yet. Until it is, treat any reference to "the conformance test plan" elsewhere in the specification as referring to a document that does not yet exist. Do not infer test procedures from the empty sections below.

---

## 1. Scope

*To be authored. Will enumerate the device classes covered (node, repeater, gateway) and the boundary between this plan and external standards (marine environmental qualification, accelerated-life testing, etc.).*

---

## 2. Test Equipment and Setup

*To be authored. Will define the standard test fixture, required instruments, and the reference companion node used to drive each test.*

---

## 3. Test Categories

*To be authored. Will group tests by the specification document they verify (`02-physical-layer.md`, `03-data-link-layer.md`, `04-power-management.md`, `05-addressing.md`, `06-signal-catalog.md`, `07-pgn-registry.md`, `09-gateway-specification.md`, `10-repeater-specification.md`).*

---

## 4. Requirements Traceability Matrix

*To be authored. Will map every "shall" statement in documents 02–14 to one or more test IDs defined in `Section 3`.*

---

## 5. Pass/Fail Criteria

*To be authored. Will define the conditions under which a device may be declared Pelorus Core conformant per `16-compliance-self-declaration.md`.*

---

## 6. Open Items (to be resolved before v1.0 promotion)

- Author all sections above. This document is currently a placeholder.
- Decide whether environmental and reliability tests (vibration, salt fog, accelerated life) are referenced normatively here or left to external marine standards.
- Define a machine-readable test report format so reference implementations can publish conformance results automatically.
- Set normative thresholds for sleep current, repeater forwarding latency, and gateway bridge round-trip latency.

---

*This document, together with documents 01–14 and 16, will complete the minimum viable specification for verifying Pelorus Core conformance once authored.*
