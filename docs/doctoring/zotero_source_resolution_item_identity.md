# Zotero source-resolution stored item identity

Status: **SOURCE_REPAIRED — exact-head checks pending**.

## Problem

`prepare_source_resolution_review` admits each pending-source decision only when its `item_key`, `item_version`, `item_type`, and `parent_item_key` match the retained source in the immutable classification report. The stored `SourceResolutionReview` envelope now retains an independent expected identity vector containing those coordinates.

JSON restoration rejects a constructor-valid review after `resolved_sources[*].item_version`, `item_type`, or `parent_item_key` has been changed. The custom deserializer compares each mutable decision to the separately persisted constructor-bound identity before the artifact regains typed review status.

This conflicts with the repository invariant that pending Zotero ancestry is resolved only through a typed aggregate bound to the exact report item key/version/type/parent identity.

## Regression and repair

Commit `6fa6c667e1f936f505ed6eae69e637b9e89dab71` adds `stored_source_resolution_review_rejects_item_identity_drift` in `crates/conceptweave-zotero/tests/source_resolution_stored_completeness.rs`.

The regression starts from a real `ClassificationReport`, constructs a valid review, serializes it, then mutates exactly one nested coordinate at a time:

- `item_version`: `41 -> 40`
- `item_type`: `attachment -> note`
- `parent_item_key`: empty -> `PARENT`

Each restored artifact must be rejected. The regression is now GREEN locally after adding the independent identity vector and exact deserializer comparison.

## Least-widening repair

Persist constructor-bound expected pending-source coordinates separately from steward decisions and require exact equality during deserialization. The expected record contains the item key, item version, item type, and parent key. Server identity and library version remain envelope coordinates and keep their existing exact nested equality checks. Stored artifacts without the new field fail closed because no migration contract exists.

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
