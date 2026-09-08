# Zotero source-resolution construction boundary

Status: **SOURCE_TEST_REPAIRED_PENDING_CI**

## Problem

`SourceResolutionReview` is the typed, report-bound review aggregate returned only after `prepare_source_resolution_review` or `restore_source_resolution_review` validates the immutable Zotero snapshot coordinates. At reviewed head `d7f750644fd254a4977f137cae7cfa3b179d46a9`, the type itself and all seven fields were public. An external crate could therefore construct a `SourceResolutionReview` literal or mutate a returned value without passing either validation path. That made the trusted type stronger than its construction boundary.

This is separate from stored-wire validation. `deny_unknown_fields`, envelope checks, and report-bound restoration protect persisted JSON, but they do not protect direct Rust construction or post-construction mutation.

## Reality RED

Commit `cefc2e084f0a746d6a8a40880eef0fdfef417472` adds `source_resolution_review_does_not_expose_mutable_construction_fields`. The public-contract regression requires the seven constructor-bound fields to stop being externally writable while keeping the review type itself available to consumers.

The RED is intentionally source-shape based because the defect is a Rust visibility/API-surface property: the reviewed declaration exposed `pub` fields and therefore failed the contract before runtime validation could participate.

## Causal source repair

Commit `f39fc7f874e01ae2fc043d6211ce24fd863930b8` performs the least-widening repair. `SourceResolutionReview` remains public, cloneable, comparable and serializable, while all seven constructor-bound fields become private. Read access is preserved only through immutable, documented accessors:

- `zotero_version() -> &str`
- `server_id() -> Option<&str>`
- `library_version() -> u64`
- `rule_revision() -> &str`
- `pending_source_item_keys() -> &[String]`
- `expected_source_identities() -> &[PendingSourceIdentity]`
- `resolved_sources() -> &[PendingSourceResolution]`

Repository-local integration-test reads are migrated to accessors. No setter, mutable slice, public unchecked constructor, `Default`, or direct trusted deserialization is introduced. The ordinary two-parent integration `d7ebef510df2481a55550eed76571589c04b3ede` preserves both this repair and the committed RED/doctoring/baseline lineage without force or destructive rebase.

## Unchanged authority boundary

The repair does not change source-resolution disposition semantics, Zotero read/write behavior, classification, publication, steward approval, or semantic authority. The aggregate remains read-only evidence for a steward workflow.

## Acceptance

The source defect is repaired, but no executable GREEN is transferred to the changed head. A single unchanged exact successor must pass the visibility regression, locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks, and independent review. Predecessor execution does not transfer.
