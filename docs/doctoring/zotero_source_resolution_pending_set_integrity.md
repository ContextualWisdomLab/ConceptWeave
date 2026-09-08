# Zotero source-resolution pending-set integrity

Status: **REALITY_RED_SOURCE_FIX_PENDING**

## Problem

`prepare_source_resolution_review` treats `ClassificationReport.pending_source_item_keys` as the constructor authority for the complete pending-source set. The normal classifier produces a sorted unique sequence, but the public report type can be supplied or mutated by an external caller before source-resolution admission.

At reviewed head `c749c3e91bbef82c750c52e2982712a47d2840ef`, the constructor immediately collects those keys into a `BTreeMap`. A duplicate key is therefore silently collapsed, and a noncanonical key order is silently sorted for lookup while the original sequence is copied into the returned trusted review. A malformed report can consequently regain `SourceResolutionReview` status even though its pending-set evidence is not canonical.

## Reality RED

Commit `13e70410c4285bf037e9b1ca49fd50654f2829dc` adds two public-contract regressions:

- duplicate pending keys must be rejected before typed review construction;
- a non-strictly-increasing pending-key sequence must be rejected rather than normalized implicitly.

Both inputs preserve valid server/library/item identity and valid one-per-source decisions so the failing variable is the report pending-set invariant itself.

## Least-widening repair

Validate `report.pending_source_item_keys` as strictly increasing before building the lookup map. Reject `pair[0] >= pair[1]` explicitly through a source-resolution admission error; do not deduplicate, sort, rewrite, or otherwise repair malformed report evidence inside the constructor.

The existing classifier remains responsible for producing the canonical sorted unique set. Source-resolution admission independently verifies that invariant because it crosses a public trust boundary.

## Unchanged authority boundary

This repair must not change classification, source inventory derivation, resolution dispositions, Zotero read/write behavior, semantic publication, steward approval or release authority. It only prevents malformed report evidence from being promoted to the trusted source-resolution aggregate.

## Acceptance

A single unchanged exact successor must pass both new pending-set regressions plus locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks and independent review. Predecessor execution does not transfer.
