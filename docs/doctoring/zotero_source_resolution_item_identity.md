# Zotero source-resolution stored item identity

Status: **SOURCE_TEST_REPAIRED_PENDING_CI**.

## Problem

`prepare_source_resolution_review` admits each pending-source decision only when its `item_key`, `item_version`, `item_type`, and `parent_item_key` match the retained source in the immutable classification report.

Commit `610582a70f66503a694d4f1e72d34183a6652cdb` repaired unilateral decision drift by persisting `expected_source_identities` separately from `resolved_sources` and comparing version/type/parent during restoration. That is a valid repair for an altered decision payload when the expected vector remains intact, but it was not by itself a provenance root: both vectors were mutable fields in the same JSON document.

The stronger reality RED `a3cd9b0d68b4ecc28322860d04a23867412d611f` changes `expected_source_identities[0].item_version` and `resolved_sources[0].item_version` from `41` to `40` together. The prior direct `Deserialize` implementation compared only values carried inside that altered artifact, so the coordinated rewrite could regain the trusted `SourceResolutionReview` type even though those coordinates were never admitted against the immutable classification report.

The typed review is described as exact-snapshot bound. Internal agreement between mutable copies is not equivalent to revalidation against the immutable report that supplied those coordinates.

## RED → partial repair → report-bound repair

Commit `6fa6c667e1f936f505ed6eae69e637b9e89dab71` adds `stored_source_resolution_review_rejects_item_identity_drift`. It starts from a real report and a constructor-valid review, then changes only one decision coordinate at a time:

- `item_version`: `41 -> 40`
- `item_type`: `attachment -> note`
- `parent_item_key`: empty -> `PARENT`

Source repair `610582a70f66503a694d4f1e72d34183a6652cdb` adds `PendingSourceIdentity`, persists the expected identity vector, requires its keys to equal the canonical pending-key sequence, and rejects unilateral version/type/parent drift.

Commit `a3cd9b0d68b4ecc28322860d04a23867412d611f` adds `stored_source_resolution_review_rejects_coordinated_identity_rewrite`, which mutates the expected and decision `item_version` together and therefore proves that another mutable copy is not a trust root.

Causal source repair `453878f236cd46ae5389575719f37077302da70e` removes direct deserialization into the trusted `SourceResolutionReview`. Stored JSON is now decoded into private `StoredSourceResolutionWire`; `restore_source_resolution_review(report, stored_json)` validates the wire, then reconstructs the review through `prepare_source_resolution_review` against the caller-supplied immutable `ClassificationReport`. The wire's expected coordinates are first required to match its decisions, and the constructor binds those decisions to the report; no redundant mutable-copy comparison is used as a trust root. Ordinary merge `e72fb404375e7a29489a855069fa0150026888e8` preserves both that repair and the concurrent baseline lineage without force or destructive rebase.

The public regressions now restore stored artifacts only through the report-bound function. The coordinated version rewrite reaches `prepare_source_resolution_review` with the altered decision and fails against the report's retained item version instead of being trusted because two mutable JSON fields agree.

## Boundary retained

The repair does not infer parents, rewrite report evidence, mutate Zotero, or add semantic approval/publication authority. `SourceResolutionReview` remains a read-only resolution aggregate; a stored artifact cannot regain the trusted exact-snapshot type without the immutable report supplied separately by the caller. A self-contained digest in the same unauthenticated artifact is still not treated as a trust root.

This closes the source-level cause of the coordinated-rewrite RED only. It does not authenticate the steward's disposition or reason as a publication/approval decision, and it does not make downstream consumers authoritative. Those governance boundaries remain outside this repair.

## Acceptance

On one unchanged exact successor:

```text
cargo +1.98.0 test --locked --workspace --all-targets
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --locked --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.98.0 doc --locked --workspace --no-deps
cargo +1.98.0 build --locked --workspace --release
```

Owned production function/normalized-region/branch coverage must reach 100% under the repository's pinned coverage procedure, followed by applicable hosted checks and independent review. No predecessor execution transfers to a changed head. Keep the canonical review thread unresolved until the report-bound repair has that exact-head executable evidence.
