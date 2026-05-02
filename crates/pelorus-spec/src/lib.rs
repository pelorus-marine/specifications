//! **Pelorus specifications** — identity and deep links for the [`specifications`](https://github.com/pelorus-marine/specifications)
//! repository. This crate is for **host tools** (lints, generators, tests): it does **not** embed
//! normative prose — browse [`core/`](https://github.com/pelorus-marine/specifications/tree/main/core)
//! and [`stream/`](https://github.com/pelorus-marine/specifications/tree/main/stream) on GitHub or read the **mdBook** build.

pub const GITHUB_ORG: &str = "pelorus-marine";
pub const GITHUB_REPO: &str = "specifications";

/// Repository root on GitHub (`https://github.com/pelorus-marine/specifications`).
#[must_use]
pub fn repo_https_root() -> &'static str {
    concat!("https://github.com/", "pelorus-marine/specifications")
}

/// Latest crate release line — kept aligned with repository tags when publishing.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitRef<'a> {
    /// Default branch (`main`).
    Main,
    /// Annotated or lightweight tag (`v0.1.0-alpha.1`, …).
    Tag(&'a str),
    /// Raw revision (`sha` prefix acceptable).
    Rev(&'a str),
}

impl GitRef<'_> {
    fn path_segment(self) -> String {
        match self {
            GitRef::Main => "main".to_string(),
            GitRef::Tag(t) => t.to_string(),
            GitRef::Rev(r) => r.to_string(),
        }
    }
}

/// View a file at `relative_path` from repository root (use `/`, no leading slash).
/// `relative_path` must stay inside the corpus — rejects `..`, NUL, and Windows separators.
#[must_use]
pub fn github_blob_url(relative_path: &str, git_ref: GitRef<'_>) -> Option<String> {
    if relative_path.is_empty() || relative_path.contains('\0') {
        return None;
    }
    if relative_path.contains("..") || relative_path.starts_with('/') {
        return None;
    }
    let normalized = relative_path.replace('\\', "/");
    let seg = git_ref.path_segment();
    Some(format!(
        "{}/blob/{}/{}",
        repo_https_root(),
        seg,
        normalized.trim_start_matches('/')
    ))
}

/// Permanent link to the markdown-book artifact instructions (`SPEC_BOOK.md`).
#[must_use]
pub fn book_guide_url() -> &'static str {
    concat!(
        "https://github.com/",
        "pelorus-marine/specifications/blob/main/SPEC_BOOK.md"
    )
}

/// Known corpus roots under `specifications/`.
pub mod paths {
    pub const CORE_INDEX: &str = "core/00-document-index.md";
    pub const STREAM_INDEX: &str = "stream/00-document-index.md";
    pub const ARCHITECTURE: &str = "ARCHITECTURE.md";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob_main_roundtrip() {
        let u = github_blob_url(paths::CORE_INDEX, GitRef::Main).unwrap();
        assert!(u.contains("/blob/main/"), "unexpected URL: {u}");
        assert!(u.ends_with("core/00-document-index.md"));
    }

    #[test]
    fn rejects_traversal() {
        assert!(github_blob_url("../Cargo.toml", GitRef::Main).is_none());
        assert!(github_blob_url("..\\evil", GitRef::Main).is_none());
    }

    #[test]
    fn tag_ref_blob_path() {
        let u = github_blob_url("CHANGELOG.md", GitRef::Tag("v0.1.0-alpha.1")).unwrap();
        assert!(u.contains("/blob/v0.1.0-alpha.1/"));
    }
}
