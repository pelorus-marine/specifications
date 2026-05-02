# Building the specification library (mdBook)

The corpus stays **Markdown on GitHub** for collaboration. This repo also ships a **searchable HTML book** built from the same files under [`core/`](./core/) and [`stream/`](./stream/).

## Prerequisites

- **Rust** toolchain (see [`rust-toolchain.toml`](./rust-toolchain.toml)).
- [**mdBook**](https://github.com/rust-lang/mdBook): `cargo install mdbook` (once).

## Build

From the repository root (`specifications/`):

```bash
cargo run -p xtask -- book-build
```

This mirrors `*.md` into `book/src/generated/` (gitignored), writes `SUMMARY.md` + introduction, then runs `mdbook build`.

Open the site:

```bash
# Linux
xdg-open book/book/index.html
```

Serve locally with live reload:

```bash
cd book && mdbook serve
```

(`cargo xtask gen-book` regenerates the mirror only — useful when iterating on the generator.)

## Rust crates

| Crate | Role |
|-------|------|
| [`pelorus-spec`](./crates/pelorus-spec) | Stable GitHub URLs and corpus paths for tooling (`pelorus-core`, conformance tests, codegen). |
| [`xtask`](./xtask) | Maintainer automation (`gen-book`, `book-build`). |

## CI

GitHub Actions builds the book on every push/PR that touches book tooling — see [`.github/workflows/book.yml`](./.github/workflows/book.yml). The HTML artifact is attached for download.
