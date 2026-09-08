# Zotero source-resolution construction boundary

Status: **REALITY_RED_SOURCE_FIX_PENDING**

## Problem

`SourceResolutionReview` is the typed, report-bound review aggregate returned only after `prepare_source_resolution_review` or `restore_source_resolution_review` validates the immutable Zotero snapshot coordinates. At reviewed head `d7f750644fd254a4977f137cae7cfa3b179d46a9`, the type itself and all seven fields are public. An external crate can therefore construct a `SourceResolutionReview` literal or mutate a returned value without passing either validation path. That makes the trusted type stronger than its construction boundary.

This is separate from stored-wire validation. `deny_unknown_fields`, envelope checks, and report-bound restoration protect persisted JSON, but they do not protect direct Rust construction or post-construction mutation.

## Reality RED

Commit `cefc2e084f0a746d6a8a40880eef0fdfef417472` adds `source_resolution_review_does_not_expose_mutable_construction_fields`. The public-contract regression requires the seven constructor-bound fields to stop being externally writable while keeping the review type itself available to consumers.

The RED is intentionally source-shape based because the defect is a Rust visibility/API-surface property: the current declaration exposes `pub` fields and therefore fails the contract before runtime validation can participate.

## Least-widening repair

Keep `SourceResolutionReview` public and serializable, but make its fields private. Expose only immutable accessors needed by consumers:

- `zotero_version() -> &str`
- `server_id() -> Option<&str>`
- `library_version() -> u64`
- `rule_revision() -> &str`
- `pending_source_item_keys() -> &[String]`
- `expected_source_identities() -> &[PendingSourceIdentity]`
- `resolved_sources() -> &[PendingSourceResolution]`

Migrate repository-local external field reads to these accessors. Do not add setters, mutable slice access, public constructors, `Default`, unchecked deserialization, or any path that bypasses `prepare_source_resolution_review` / `restore_source_resolution_review`.

## Unchanged authority boundary

This repair must not change source-resolution disposition semantics, Zotero read/write behavior, classification, publication, steward approval, or semantic authority. The aggregate remains read-only evidence for a steward workflow.

## Acceptance

A single unchanged exact successor must pass the new visibility regression, locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks, and independent review. Predecessor execution does not transfer.
