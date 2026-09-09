# Zotero stored source-resolution strict-wire doctoring

Status: `SOURCE_TEST_REPAIRED_PENDING_CI`.

## Problem and boundary

`SourceResolutionReview` is a governed, report-bound artifact. Restoration already reconstructs the trusted aggregate against a caller-supplied immutable `ClassificationReport`, but its persisted JSON boundary previously used Serde's default unknown-field tolerance. That allowed undeclared top-level or nested fields to be discarded before the trusted review was returned. A newer or stale artifact could therefore carry semantics the current reader did not understand without failing closed.

This repair is limited to stored source-resolution wire admission. It does not change source classification, constructor/report identity binding, steward disposition semantics, Zotero mutation authority, semantic publication, or approval authority.

## RED → repair trace

Reality RED `a7b2588baa727ce973323c900e1d00b9c76425b2` adds `crates/conceptweave-zotero/tests/source_resolution_unknown_fields.rs`. Starting from a constructor-valid review, it injects three undeclared fields independently: top-level `approval`, nested expected-identity `source_digest`, and nested decision `semantic_authority`. Restoration must return `SourceResolutionRestoreError::InvalidStoredArtifact` for each mutation.

Causal source repair `2760ebd494ee9d8f25d8768bb57e877ed3a3a678` adds `#[serde(deny_unknown_fields)]` to exactly the persisted object boundaries that can consume those values: `StoredSourceResolutionWire`, `PendingSourceIdentity`, and `PendingSourceResolution`. The trusted `SourceResolutionReview` remains serialize-only and restoration still reconstructs through `prepare_source_resolution_review(report, ...)`.

Docs successor `adee2d35c0145f1363e51618ca17a3a970f4c150` records the invariant in `AGENTS.md` and the product/technical gap baseline. Intervening commits are ordinary ancestry; no force or destructive rebase is required.

## Evidence and acceptance

The repair lineage records focused source-resolution tests and strict Clippy as locally passing, but that evidence does not make a changed docs successor or the PR globally GREEN. Exact-head acceptance still requires one unchanged successor through locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks, and independent review.

Unknown-field rejection is deliberately fail-closed. Forward evolution of the persisted contract requires an explicit versioned schema/migration decision rather than silently dropping fields. No predecessor execution transfers to a changed source/base head, and no stored source-resolution artifact grants Zotero write, semantic publication, or approval authority.
