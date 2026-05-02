# Changelog

All notable releases of this repository are documented here. Specification **document** versions inside `core/` and `stream/` remain marked draft in front matter until declared stable; **repository releases** tag snapshots for implementers and downstream tooling.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) for **repository** tags (`MAJOR.MINOR.PATCH` with optional prerelease labels).

## [Unreleased]

### Added

- **`CONTRIBUTORS.md`** — credits and mission alignment (open, non-commercial, Rust tooling, safety); **`[workspace.package].authors`** on Rust crates.
- **Cargo workspace** ([`Cargo.toml`](./Cargo.toml)) with **`pelorus-spec`** (canonical GitHub URLs + corpus path constants) and **`xtask`** (`gen-book`, `book-build`).
- **mdBook** integration: mirrors [`core/`](./core/) and [`stream/`](./stream/) into a searchable HTML book — [`SPEC_BOOK.md`](./SPEC_BOOK.md), theme **`navy`**, sidebar folding.
- **CI** workflow *Specification book* — `cargo fmt` / `clippy` / `test`, then `mdbook build`; uploads **`book/book`** as a workflow artifact.

## [0.1.0-alpha.1] — 2026-05-02

Pre-release snapshot: **`pelorus-spec`** crate **`0.1.0-alpha.1`** — workspace **`rust-version`** **1.88**, repo-root **`LICENSE-MIT`** / **`LICENSE-APACHE`** for Rust tooling alongside **`LICENSE.md`** (CC BY 4.0) for documentation.

[0.1.0-alpha.1]: https://github.com/pelorus-marine/specifications/releases/tag/pelorus-spec-v0.1.0-alpha.1

## [0.1.0-alpha.0] — 2026-05-02

First **pre-release** snapshot of the Pelorus Marine specifications corpus.

### Included

- **Pelorus Core** drafts under [`core/`](./core/) (indexed in [`core/00-document-index.md`](./core/00-document-index.md)).
- **Pelorus Stream** drafts under [`stream/`](./stream/) (indexed in [`stream/00-document-index.md`](./stream/00-document-index.md)).
- Project framing in [`ARCHITECTURE.md`](./ARCHITECTURE.md), [`README.md`](./README.md), and contribution guidance in [`CONTRIBUTING.md`](./CONTRIBUTING.md).

### Expectations

Documents remain **draft / living**; breaking edits are expected until a stable line is declared. Use this tag as a **reference point** for tooling, citations, and reproducible builds—not as a frozen certification baseline.

[0.1.0-alpha.0]: https://github.com/pelorus-marine/specifications/releases/tag/v0.1.0-alpha.0
