# Zotero source-resolution identity

Status: `REALITY_RED_SOURCE_FIX_PENDING` on PR #40. This note records a fail-closed provenance requirement for the pending-source resolution aggregate; it is not executable GREEN, semantic approval, protected integration, or Zotero write authority.

## Problem boundary

`PendingSourceResolution` now carries the report `library_version` and optional Local API `server_id`, and `prepare_source_resolution_review` compares those values with the classification report before accepting item version/type/parent identity. This prevents cross-library-version reuse and prevents cross-server reuse when Zotero supplied a server identity.

The remaining gap is the optional case. When both the classification report and a decision carry `server_id = None`, equality succeeds. Numeric library version plus item key/version/type/parent coordinates are not a globally unique Zotero-source identity, so a decision can still be rebound to another Local API instance that omits the server identity and happens to expose the same coordinates.

Reality RED `25fa3bfe03365cf1893bfc46f9eb758c31e1a53c` adds `source_resolution_fails_closed_when_snapshot_has_no_server_identity`. It constructs a classification report with no server identity and an otherwise exact decision and requires `prepare_source_resolution_review` to reject it as stale. The current implementation accepts that decision, so this is intentionally RED.

## Least-widening repair

Source-resolution admission must fail closed when the classification report lacks a stable provider identity. When `server_id` is present, retain exact equality between the decision and report. The lower-authority classification/report path may continue to represent `server_id = None`; only creation of the higher-authority source-resolution review is blocked because its origin cannot be proven.

Do not infer provider identity from numeric library/item coordinates, broaden Zotero mutation authority, rewrite the classification report, or treat the resolution record as semantic publication authority. If a future canonical report digest is introduced, it must be versioned and evidence-bound before replacing this fail-closed boundary.

## Acceptance and traceability

Current owner path: `PendingSourceResolution` -> `prepare_source_resolution_review` -> `SourceResolutionReview` in `crates/conceptweave-zotero/src/lib.rs`.

Required GREEN is one unchanged exact PR #40 successor passing:

- `cargo +1.98.0 test --workspace --locked`;
- `cargo +1.98.0 fmt --all -- --check`;
- `cargo +1.98.0 clippy --workspace --all-targets --locked -- -D warnings`;
- `RUSTDOCFLAGS='-D warnings' cargo +1.98.0 doc --workspace --no-deps --locked`;
- warnings-denied release build;
- owned production function / normalized-region / branch coverage at 100%;
- applicable hosted checks and independent review evidence.

Predecessor local evidence does not transfer. Keep the associated review thread unresolved until the RED has a causal source repair and exact-head GREEN.
