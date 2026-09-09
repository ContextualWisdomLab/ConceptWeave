# Zotero source-resolution pending-set integrity

Status: **SOURCE_TEST_REPAIRED_PENDING_CI**

## Problem

`prepare_source_resolution_review` treats `ClassificationReport.pending_source_item_keys` as the constructor authority for the complete pending-source set. The normal classifier produces a sorted unique sequence, but the public report type can be supplied or mutated by an external caller before source-resolution admission.

At reviewed head `c749c3e91bbef82c750c52e2982712a47d2840ef`, the constructor immediately collected those keys into a `BTreeMap`. A duplicate key was therefore silently collapsed, and a noncanonical key order was silently sorted for lookup while the original sequence was copied into the returned trusted review. A malformed report could consequently regain `SourceResolutionReview` status even though its pending-set evidence was not canonical.

## Reality RED

Commit `13e70410c4285bf037e9b1ca49fd50654f2829dc` adds two public-contract regressions:

- duplicate pending keys must be rejected before typed review construction;
- a non-strictly-increasing pending-key sequence must be rejected rather than normalized implicitly.

Both inputs preserve valid server/library/item identity and valid one-per-source decisions so the failing variable is the report pending-set invariant itself.

## Causal source repair

Commit `9fee299ea1d068a0c9c829865d5fe647d37c0951` adds `SourceResolutionError::InvalidPendingKeySet` and validates the report pending-key sequence before lookup-map construction. The implementation canonicalizes a temporary copy only for comparison; it does not rewrite, sort, deduplicate, or otherwise repair the report evidence returned by the caller. Any duplicate or noncanonical sequence is rejected before the constructor can promote it to a trusted review.

Ordinary merge `c849cb2e9bc5d26f647a0b9381fb064b2d54a98e` preserves the source repair together with the committed RED and doctoring ancestry without force or destructive rebase.

The existing classifier remains responsible for producing the canonical sorted unique set. Source-resolution admission independently verifies that invariant because it crosses a public trust boundary.

Exact successor `c0907676727ec914bb2c41a0610dc49e11203314` extends the same fail-closed boundary to blank or whitespace pending keys. Its regression supplies an otherwise valid empty-key source and confirms that no trusted review is produced. The successor also replaces the output-path root panic with an `InvalidInput` result and asserts that contract. Local Rust 1.98 workspace tests, formatting, strict Clippy and warnings-denied rustdoc pass; raw coverage and hosted protection gates remain incomplete.

## Unchanged authority boundary

The repair does not change classification, source inventory derivation, resolution dispositions, Zotero read/write behavior, semantic publication, steward approval or release authority. It only prevents malformed report evidence from being promoted to the trusted source-resolution aggregate.

## Acceptance

The source defect is repaired, but executable GREEN does not transfer to the changed head. A single unchanged exact successor must pass both pending-set regressions plus locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks and independent review. Predecessor execution does not transfer.
