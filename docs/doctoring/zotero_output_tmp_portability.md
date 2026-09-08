# Zotero report output temporary-directory portability

Status: `SOURCE_TEST_REPAIRED_PENDING_CI`

## Problem

PR #40 intentionally supports the host system temporary directory without requiring a POSIX `/tmp` directory. Intervening test-only commit `a8609387cc5edf0919ed54596749bf1675aa93f5` removed the existing `Path::new("/tmp").exists()` guard from `output_path_helpers_keep_parent_policy_and_temp_names_bounded`, making the test require `/tmp` even though production `allowed_output_parents()` treats conventional `/tmp` discovery as optional. That recreates the portability failure previously found on Windows/non-POSIX hosts.

## Constraints and alternatives

Changing production admission to require `/tmp` was rejected because it would contradict `env::temp_dir()` ownership and the existing Windows regression. Platform-skipping the whole helper test was also rejected because the deterministic parent-policy and temporary-name assertions remain useful on every supported host.

The least-widening repair is to keep the helper test cross-platform and gate only the positive `/tmp` assertion on actual path existence.

## Repair and traceability

- Reality RED/intervening regression: `a8609387cc5edf0919ed54596749bf1675aa93f5`.
- Minimal repair: `bf5c38bd1f4b9a0eca8b736b765f4d5202ec39e5`.
- Production behavior changed: none.
- Preserved boundaries: output-path admission, create-new publication, post-publication cleanup, Zotero transport, research classification, source-resolution construction/restoration, semantic authority.

The repair is an ordinary successor; the intervening commit remains in ancestry. No force-push, destructive rebase, coverage exclusion, or threshold weakening is used.

## Acceptance

The repair is not executable GREEN until one unchanged exact successor passes the repository-pinned Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target warnings-denied Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks, and independent review. This automation runtime has no Rust toolchain, so predecessor execution evidence does not transfer.