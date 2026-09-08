# Zotero source-resolution stored item identity

Status: **REALITY_RED — trusted report binding pending**.

## Problem

`prepare_source_resolution_review` admits each pending-source decision only when its `item_key`, `item_version`, `item_type`, and `parent_item_key` match the retained source in the immutable classification report.

Commit `610582a70f66503a694d4f1e72d34183a6652cdb` repairs unilateral decision drift by persisting `expected_source_identities` separately from `resolved_sources` and comparing version/type/parent during deserialization. That is a valid repair for an altered decision payload when the expected vector remains intact.

It does not yet prove constructor provenance after a coordinated stored-artifact rewrite. Both vectors are mutable fields in the same JSON document. If a stored artifact changes `expected_source_identities[0].item_version` and `resolved_sources[0].item_version` from `41` to `40` together, all current envelope, ordering, completeness, server/library and expected-vs-decision comparisons still agree. Direct deserialization can therefore return the trusted `SourceResolutionReview` type even though the restored coordinates were never admitted against the original classification report.

The typed review is described as exact-snapshot bound. Internal agreement between two mutable copies is not equivalent to revalidation against the immutable report that supplied those coordinates.

## RED → partial repair → remaining RED

Commit `6fa6c667e1f936f505ed6eae69e637b9e89dab71` adds `stored_source_resolution_review_rejects_item_identity_drift`. It starts from a real report and a constructor-valid review, then changes only one decision coordinate at a time:

- `item_version`: `41 -> 40`
- `item_type`: `attachment -> note`
- `parent_item_key`: empty -> `PARENT`

Source repair `610582a70f66503a694d4f1e72d34183a6652cdb` adds `PendingSourceIdentity`, persists the expected identity vector, requires its keys to equal the canonical pending-key sequence, and rejects unilateral version/type/parent drift. The earlier RED is therefore causally repaired in source, subject to unchanged-head execution.

Commit `a3cd9b0d68b4ecc28322860d04a23867412d611f` adds the next reality RED, `stored_source_resolution_review_rejects_coordinated_identity_rewrite`. It changes the expected and decision `item_version` together. Current source accepts that coordinated rewrite by direct inspection because every comparison is internal to the altered JSON artifact.

## Least-widening causal repair

Do not add a third mutable copy or a self-declared digest and call it provenance. A hash stored in the same unauthenticated artifact can be rewritten together with its payload.

Treat stored JSON as an untrusted wire representation and require a trusted external binding before it regains the exact-snapshot review type. The narrowest existing authority is the immutable `ClassificationReport` (or a separately authenticated immutable report receipt derived from it). Restoration must revalidate the stored pending-key set and every decision key/version/type/parent against that trusted report, while retaining the existing exact server/library checks. One sound type-state shape is a deserializable stored/wire type plus an explicit report-bound restoration function that returns `SourceResolutionReview`; direct `Deserialize` into the trusted review type must not bypass that validation.

This repair must not infer parents, rewrite report evidence, mutate Zotero, add semantic approval/publication authority, or treat a self-contained unauthenticated digest as a trust root. If an authenticated receipt/signature contract is introduced instead, its canonical owner and immutable verification contract must be explicit.

## Acceptance

On one unchanged exact successor:

```text
cargo +1.98.0 test --locked --workspace --all-targets
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --locked --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.98.0 doc --locked --workspace --no-deps
cargo +1.98.0 build --locked --workspace --release
```

Owned production function/normalized-region/branch coverage must reach 100% under the repository's pinned coverage procedure, followed by applicable hosted checks and independent review. No predecessor execution transfers to a changed head. The coordinated-rewrite RED stays open until restoration is externally report-bound or equivalently authenticated.
