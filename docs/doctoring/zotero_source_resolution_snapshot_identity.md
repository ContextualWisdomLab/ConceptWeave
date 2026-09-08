# Zotero source-resolution snapshot identity

Status: `SOURCE_TEST_REPAIRED_PENDING_CI`

## Finding

`prepare_source_resolution_review` accepts a caller-supplied `ClassificationReport`, including reports that were fabricated or mutated after deterministic classification. Before this repair, the constructor rejected absent/blank Local API `server_id`, stale library/item coordinates, malformed pending sets, and ambiguous retained inventory, but it did not reject blank `zotero_version` or blank `rule_revision`.

Those fields are part of the review's immutable snapshot identity and are serialized into `SourceResolutionReview`. Accepting blank values therefore permitted a trusted review to be constructed without enough provenance to identify the Zotero/classifier snapshot that produced it.

## Reality RED and evidence correction

Commit `938a9154be617e2d51f724a9447208e715d0e1a2` was the first RED attempt, but it called `ClassificationReport::clone()` even though that type is not `Clone`. It is therefore compile-invalid evidence and must not be cited as a valid behavioral RED.

To preserve TDD provenance without rebasing or force-pushing, corrected RED commit `995837726b448d13c1b88576e2124c5c4809e1a9` was created directly from the pre-repair predecessor `70e8c2e89b077c388aa3e24e5a01deb50f7f16b3`. It constructs two independent reports, mutates only `zotero_version` or `rule_revision`, and requires constructor rejection. On that predecessor, `prepare_source_resolution_review` has no admission check for either coordinate, so both assertions reach successful trusted construction and fail behaviorally.

Current-tree test correction `1d2564b0f891a2340e4fe7c04eab2f715bbf887b` removes the accidental `clone()` dependency while retaining typed post-repair assertions. Ordinary two-parent integration `e5575a64f52c9ba0ae4b9b08e86c151ea99a3056` preserves corrected RED `9958377...` in ancestry while keeping the repaired current tree. No force push or destructive rebase was used.

## Causal repair

Commit `7c3280ef7b8223ed685377d3861a7a22a933d2e5` adds the typed `SourceResolutionError::InvalidSnapshotIdentity` admission failure and rejects a report when either `zotero_version.trim()` or `rule_revision.trim()` is empty. No disposition, Zotero transport, publication, mutation, or approval semantics are widened.

Commit `19380afd18d148d0a435849136e29afc159ca58f` introduced typed assertions but still inherited the compile-invalid `clone()` setup; it is not standalone executable evidence. `1d2564b...` is the corrected current test tree. The internal formatter branch is covered by a dedicated unit assertion in the production module.

## Acceptance contract

The source/test repair is not executable GREEN until one unchanged exact successor passes the repository-required Rust 1.98 locked workspace tests, `cargo fmt --all -- --check`, all-target strict Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks, and qualifying independent review. Predecessor CI or coverage evidence does not transfer across these source/test commits.

This boundary remains read-only and proposal/review-oriented. It does not grant Zotero write access, semantic publication authority, or steward approval authority.
