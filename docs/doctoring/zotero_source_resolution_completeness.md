# Zotero stored source-resolution completeness

Status: `REALITY_RED_SOURCE_FIX_PENDING`

## Problem

`prepare_source_resolution_review` admits a review only when every pending source key in the immutable classification report has exactly one resolution. The stored JSON representation does not preserve the expected pending-key set independently from `resolved_sources`. Its custom deserializer therefore revalidates provider identity, library revision, nonblank keys/reasons, ordering, and rule revision, but cannot detect that a constructor-valid artifact lost one decision after serialization.

A restored `SourceResolutionReview` is a typed review aggregate. Accepting a truncated decision array contradicts its public contract of one resolution for every pending source and can let a downstream consumer mistake incomplete review evidence for a complete exact-snapshot resolution.

## Reality RED

Commit `6b28fc38afaae455a70901c57b28116b0b88510d` adds `crates/conceptweave-zotero/tests/source_resolution_stored_completeness.rs`.

The test builds a real classification report with two pending records, constructs a valid complete review, serializes it, removes exactly one `resolved_sources` entry, and requires deserialization to fail. Current production deserialization accepts the remaining sorted, nonblank, server/library-consistent entry, so the regression is intentionally RED pending the causal source repair.

## Least-widening repair contract

Persist the canonical expected pending-source key set in the review envelope, derived from the report during constructor admission, and require exact equality between that expected set and the restored decision keys during deserialization. The set must be nonblank, strictly ordered, and duplicate-free. Do not infer completeness from `resolved_sources.len()` alone, fabricate missing decisions, relax exact report item identity, or grant Zotero write/semantic approval authority.

This repair is an unreleased Draft wire-contract change, so compatibility must be made explicit rather than silently defaulting the new evidence field to an empty set.

A stronger future report-digest binding may supersede the duplicated key receipt, but a digest must use a versioned canonical representation and an appropriate integrity primitive; do not introduce an ad-hoc hash solely to make this RED pass.

## Acceptance

On one unchanged exact successor:

- the new stored-completeness regression passes;
- existing source-resolution round-trip, server identity, library binding, duplicate/ordering, blank-field, missing/unknown/stale and rule-revision regressions pass;
- locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc and release build pass;
- owned production function/normalized-region/branch coverage is 100% with the new deserialize branches exercised;
- applicable hosted checks and independent review are terminal GREEN.

Until then the finding remains unresolved and #40 remains Draft. No predecessor execution is transferred to the new test/docs heads.
