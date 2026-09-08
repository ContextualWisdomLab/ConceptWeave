# Zotero source-resolution snapshot identity

Status: `SOURCE_TEST_REPAIRED_PENDING_CI`

## Finding

`prepare_source_resolution_review` accepts a caller-supplied `ClassificationReport`, including reports that were fabricated or mutated after deterministic classification. Before this repair, the constructor rejected absent/blank Local API `server_id`, stale library/item coordinates, malformed pending sets, and ambiguous retained inventory, but it did not reject blank `zotero_version` or blank `rule_revision`.

Those fields are part of the review's immutable snapshot identity and are serialized into `SourceResolutionReview`. Accepting blank values therefore permitted a trusted review to be constructed without enough provenance to identify the Zotero/classifier snapshot that produced it.

## Reality RED

Commit `938a9154be617e2d51f724a9447208e715d0e1a2` adds `source_resolution_rejects_blank_snapshot_identity_coordinates`. On the predecessor implementation, both blank-Zotero-version and blank-rule-revision cases reach successful review construction, so the assertions are behavioral RED rather than compile-only checks.

## Causal repair

Commit `7c3280ef7b8223ed685377d3861a7a22a933d2e5` adds the typed `SourceResolutionError::InvalidSnapshotIdentity` admission failure and rejects a report when either `zotero_version.trim()` or `rule_revision.trim()` is empty. No disposition, Zotero transport, publication, mutation, or approval semantics are widened.

Commit `19380afd18d148d0a435849136e29afc159ca58f` pins both cases to the typed error rather than generic failure. The internal formatter branch is also covered by a dedicated unit assertion.

## Acceptance contract

The source/test repair is not executable GREEN until one unchanged exact successor passes the repository-required Rust 1.98 locked workspace tests, `cargo fmt --all -- --check`, all-target strict Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks, and qualifying independent review. Predecessor CI or coverage evidence does not transfer across these source/test commits.

This boundary remains read-only and proposal/review-oriented. It does not grant Zotero write access, semantic publication authority, or steward approval authority.
