# Zotero source-resolution stored item identity

Status: **REALITY_RED_SOURCE_FIX_PENDING**.

## Problem

`prepare_source_resolution_review` admits each pending-source decision only when its `item_key`, `item_version`, `item_type`, and `parent_item_key` match the retained source in the immutable classification report. The stored `SourceResolutionReview` envelope currently retains an independent expected pending-key set, but it does not retain independent expected version/type/parent coordinates.

As a result, JSON restoration can accept a constructor-valid review after `resolved_sources[*].item_version`, `item_type`, or `parent_item_key` has been changed. The custom deserializer still sees a matching aggregate/nested `server_id`, matching `library_version`, nonblank key/reason, strict key order, and a complete decision-key set, so the altered item identity regains typed review status without proof that it is the identity admitted by the constructor.

This conflicts with the repository invariant that pending Zotero ancestry is resolved only through a typed aggregate bound to the exact report item key/version/type/parent identity.

## Reality RED

Commit `6fa6c667e1f936f505ed6eae69e637b9e89dab71` adds `stored_source_resolution_review_rejects_item_identity_drift` in `crates/conceptweave-zotero/tests/source_resolution_stored_completeness.rs`.

The regression starts from a real `ClassificationReport`, constructs a valid review, serializes it, then mutates exactly one nested coordinate at a time:

- `item_version`: `41 -> 40`
- `item_type`: `attachment -> note`
- `parent_item_key`: empty -> `PARENT`

Each restored artifact must be rejected. Current source does not independently retain the expected values needed to reject those mutations, so the test is intentionally RED by direct source inspection.

## Least-widening repair

Persist constructor-bound expected pending-source coordinates separately from steward decisions and require exact equality during deserialization. The expected record needs only the item identity not already supplied by the aggregate envelope: key, item version, item type, and parent key. Server identity and library version remain envelope coordinates and keep their existing exact nested equality checks.

Do not derive the expected identity from `resolved_sources`, because that merely compares a mutable decision array to itself. Do not infer a parent, rewrite the classification report, add Zotero mutation, or treat the review as semantic publication/approval authority. Existing artifacts that lack the new independent evidence must fail closed unless an explicit versioned migration contract is introduced.

## Acceptance

On one unchanged exact successor:

```text
cargo +1.98.0 test --locked --workspace --all-targets
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 clippy --locked --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.98.0 doc --locked --workspace --no-deps
cargo +1.98.0 build --locked --workspace --release
```

Owned production function/normalized-region/branch coverage must reach 100% under the repository's pinned coverage procedure, followed by applicable hosted checks and independent review. No predecessor execution transfers to a changed head.
