# Zotero stored source-resolution completeness

Status: `SOURCE_TEST_REPAIRED_PENDING_CI`

## Problem

`prepare_source_resolution_review` has always required exactly one resolution for every pending source key in the supplied classification report. Earlier stored-review restoration did not preserve an independent expected pending-key set, so deleting one decision after serialization could still yield a typed `SourceResolutionReview`.

That defect was an artifact-boundary completeness failure: a truncated stored review could be mistaken for complete exact-snapshot resolution evidence even though constructor admission would never have produced it.

## Reality RED → source repair

Commit `6b28fc38afaae455a70901c57b28116b0b88510d` adds `crates/conceptweave-zotero/tests/source_resolution_stored_completeness.rs`.

The test builds a real classification report with two pending records, constructs a valid complete review, serializes it, removes exactly one `resolved_sources` entry, and requires restoration to fail.

Causal repair `c19c787cd9153ed39f3c34370d2f9fa18fef7c8a` persists the canonical `pending_source_item_keys` sequence in the review/wire envelope. Current restoration does not deserialize directly into the trusted review type. `restore_source_resolution_review` decodes untrusted JSON into private `StoredSourceResolutionWire`, validates the stored key sequence and decision sequence, binds the envelope to the caller-supplied immutable `ClassificationReport`, then reconstructs the trusted aggregate through `prepare_source_resolution_review`.

Later report-side pending-set RED `13e70410c4285bf037e9b1ca49fd50654f2829dc` and repair `9fee299ea1d068a0c9c829865d5fe647d37c0951` close the complementary constructor boundary: duplicate or noncanonical `ClassificationReport.pending_source_item_keys` are rejected before lookup-map construction instead of being silently normalized by a map.

## Current contract

The pending-source key sequence is constructor-derived evidence, not a decision-derived count. It must be canonical, strictly ordered and duplicate-free. Stored restoration requires exact agreement between the persisted pending-key sequence, persisted expected identities, persisted decision keys and the immutable report supplied by the caller. The trusted `SourceResolutionReview` itself is serialize-only and exposes constructor-bound state through immutable accessors; external code cannot regain trusted status through direct deserialization or field mutation.

The new evidence field intentionally has no legacy default. A stored artifact that predates the completeness evidence cannot silently regain typed completeness.

A future canonical report digest may strengthen report binding only through an explicit versioned contract and an appropriate integrity primitive. It must not replace external report binding with another self-declared mutable field.

## Acceptance

On one unchanged exact successor:

- the stored-completeness regression and report-side pending-set integrity regressions pass;
- existing source-resolution round-trip, server identity, library binding, strict-wire, duplicate/ordering, blank-field, missing/unknown/stale and rule-revision regressions pass;
- locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc and release build pass;
- owned production function/normalized-region/branch coverage is 100% with the restoration branches exercised;
- applicable hosted checks and independent review are terminal GREEN.

Until then the repaired source contract remains `SOURCE_TEST_REPAIRED_PENDING_CI`; #40 stays Draft. No predecessor execution transfers to a changed source or docs head, and no Zotero write, semantic publication or approval authority is implied.
