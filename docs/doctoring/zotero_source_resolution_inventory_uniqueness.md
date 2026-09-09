# Zotero source-resolution retained-inventory uniqueness

Status: `SOURCE_TEST_REPAIRED_PENDING_CI`

## Problem

`prepare_source_resolution_review` binds a steward decision to the retained `ClassificationReport.unclassified_items` record for the same pending key. The report type is intentionally inspectable and remains mutable at this stack position, so the resolution boundary must not assume that a pending key occurs exactly once in retained inventory.

Before this repair, lookup used `Iterator::find`. A report containing two retained records with the same pending key therefore selected whichever matching record appeared first. That made the supposedly exact snapshot identity depend on vector order and allowed a fabricated or corrupted caller-supplied report to choose one of two contradictory item versions/types/parents.

## RED

Commit `e30e519c44270450213f06fcf895dc0dcfc1c25c` adds `source_resolution_rejects_ambiguous_retained_inventory_identity`. It starts from a constructor-valid report, appends a second retained record with the same `SOURCE` key and a different item version, then requires source-resolution construction to reject the report rather than bind to the first match.

This is a resolution-boundary invariant. It does not claim that the Research Intake owner may accept duplicate provider keys: the live Local API reader already rejects duplicate keys. The defensive check exists because #40 accepts a caller-supplied `ClassificationReport` and must fail closed if that input no longer represents an unambiguous exact snapshot.

## Causal repair

Commit `1799b2a1815dbfb2c57a8eb1bee992158c3cdb44` replaces first-match lookup with exact-cardinality admission for each pending key. Zero retained matches still return `MissingInventory`; more than one returns the new `AmbiguousInventory(key)` error. Exactly one record proceeds to the existing server/library/item-version/item-type/parent binding checks.

Commit `aa021414843037f598e3585108504e4323bc832e` pins the typed `AmbiguousInventory("SOURCE")` result and its actionable error text so the new error/display branch is covered by the focused regression.

## Boundary preserved

The repair does not change disposition semantics, Research Intake classification, provider reads, Zotero mutation, semantic publication or approval authority. `SourceResolutionReview` remains constructor-bound and serialize-only; stored restoration continues to re-enter the same report-bound constructor.

## Acceptance

This lineage is not executable GREEN until one unchanged exact #40 head passes the repository's locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target strict Clippy, warnings-denied rustdoc/release, owned production function/normalized-region/branch 100% coverage, applicable hosted checks and qualifying independent review. No predecessor evidence transfers across these commits.
