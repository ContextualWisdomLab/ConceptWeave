# Zotero source-resolution identity

Status: `REALITY_RED_SOURCE_FIX_PENDING` on PR #40 at exact head `669a44e61de225964f2b599772daa8e409a351e9`. Constructor-side provider identity is source/test repaired, but a fresh JSON artifact-boundary RED remains. This note is not executable GREEN, semantic approval, protected integration, or Zotero write authority.

## Problem boundary

`PendingSourceResolution` carries the report `library_version` and Local API `server_id`, and `prepare_source_resolution_review` compares those values with the classification report before accepting item version/type/parent identity. Admission now fails closed when the report server identity is missing or blank and rejects cross-library/cross-server reuse when decisions are prepared from the live report.

The earlier optional-identity defect was fixed by RED `25fa3bfe03365cf1893bfc46f9eb758c31e1a53c` and source repair `82a392cde56e54ef22169107ca1bdc89f112dfff`: a typed review cannot be constructed from a report that lacks a stable provider identity. Later tests cover the empty-pending and blank-identity paths, and the public `SourceResolutionReview` projection owns its JSON fields.

A separate artifact-boundary defect remains after those constructor repairs. `SourceResolutionReview` has a custom JSON deserializer that verifies only the aggregate `server_id` and then accepts `resolved_sources` as deserialized. A persisted or hand-edited review can therefore carry top-level `server_id = "local-server"` while a nested decision carries `null`, blank text, or another server identity. Such an artifact would deserialize as the typed exact-snapshot review even though `prepare_source_resolution_review` would reject the same decision.

Reality RED `5718e4f62122454c3a989814d80bf5e62edcc073` adds `source_resolution_review_json_rejects_nested_server_identity_mismatch`. It starts from a valid prepared review, mutates only `resolved_sources[0].server_id` to `null`, whitespace, and `other-server`, and requires all three JSON artifacts to fail deserialization. Current production accepts them, so this is intentionally RED.

## Current integration state

Research Intake #9 advanced from `e8f7f83ee0d7f7ca3d2bb0b655040974786c1e6c` to test-only successor `28fb662ee754e9820d9eb8ad9e8cec4de6cdec85` on the same `source_resolution_contract.rs` path. PR #40 temporarily conflicted. Ordinary two-parent integration `669a44e61de225964f2b599772daa8e409a351e9` preserves the #9 commit while retaining #40's stricter superset fixture, including exact server identity, owned JSON round-trip, library binding, missing-inventory/error rendering, and the new nested-artifact RED. No force push or destructive rebase was used.

## Minimal causal repair

After validating that the aggregate `server_id` is nonblank, `SourceResolutionReview` deserialization must require every `resolved_sources[*].server_id` to be present, nonblank, and exactly equal to the aggregate identity. The least-widening implementation can use exact equality with the already-validated aggregate value. It must not infer provider identity from numeric library/item coordinates, broaden Zotero mutation authority, reinterpret classification, or promote the review artifact to semantic publication authority.

A future canonical report digest may strengthen this binding only through an explicit versioned contract; it must not silently replace or reinterpret existing evidence.

## Acceptance and traceability

Current owner path: `PendingSourceResolution` -> `prepare_source_resolution_review` -> `SourceResolutionReview::{serialize, deserialize}` in `crates/conceptweave-zotero/src/lib.rs`, with public regression coverage in `crates/conceptweave-zotero/tests/source_resolution_contract.rs`.

Required GREEN is one unchanged exact PR #40 successor passing:

- `cargo +1.98.0 test --workspace --locked`;
- `cargo +1.98.0 fmt --all -- --check`;
- `cargo +1.98.0 clippy --workspace --all-targets --locked -- -D warnings`;
- `RUSTDOCFLAGS='-D warnings' cargo +1.98.0 doc --workspace --no-deps --locked`;
- warnings-denied release build;
- owned production function / normalized-region / branch coverage at 100%;
- applicable hosted checks and independent review evidence.

Predecessor execution does not transfer. Keep the new artifact-boundary review finding unresolved until the causal source repair and one unchanged exact successor supply the executable and hosted evidence above.
