# Zotero source-resolution identity

Status: `SOURCE_TEST_REPAIRED_PENDING_CI` on PR #40. Constructor-side provider identity and the nested JSON artifact boundary are causally source/test repaired. This note is not executable GREEN, semantic approval, protected integration, or Zotero write authority.

## Problem boundary

`PendingSourceResolution` carries the report `library_version` and Local API `server_id`, and `prepare_source_resolution_review` compares those values with the classification report before accepting item version/type/parent identity. Admission fails closed when the report server identity is missing or blank and rejects cross-library/cross-server reuse when decisions are prepared from the live report.

The earlier optional-identity defect was fixed by RED `25fa3bfe03365cf1893bfc46f9eb758c31e1a53c` and source repair `82a392cde56e54ef22169107ca1bdc89f112dfff`: a typed review cannot be constructed from a report that lacks a stable provider identity. Later tests cover the empty-pending and blank-identity paths, and the public `SourceResolutionReview` projection owns its JSON fields.

A separate artifact-boundary defect remained after those constructor repairs. `SourceResolutionReview` had a custom JSON deserializer that verified only the aggregate `server_id` and then accepted `resolved_sources` unchanged. A persisted or hand-edited review could therefore carry top-level `server_id = "local-server"` while a nested decision carried `null`, blank text, or another server identity. Such an artifact deserialized as the typed exact-snapshot review even though `prepare_source_resolution_review` would reject the same decision.

Reality RED `5718e4f62122454c3a989814d80bf5e62edcc073` added `source_resolution_review_json_rejects_nested_server_identity_mismatch`. It starts from a valid prepared review, mutates only `resolved_sources[0].server_id` to `null`, whitespace, and `other-server`, and requires all three JSON artifacts to fail deserialization.

## Causal repair

Source repair `665e1bbf896fba3f1eec766f2e3651cae5f124e2` adds the least-widening deserialization invariant: after the aggregate server identity has passed the existing nonblank check, every nested resolution must carry exactly the same identity. Mismatch, `None`, and blank nested values therefore fail because none equals the validated nonblank aggregate identity. The change adds no inferred identity, Zotero mutation authority, classification change, or semantic publication authority.

The concurrent repair branch and the already-pushed RED/docs ancestry were reconciled by ordinary merge `5c566826c0bd4a5675d715c1bff83b10e959173f`; no force push or destructive rebase was used. Research Intake #9 subsequently advanced from `28fb662ee754e9820d9eb8ad9e8cec4de6cdec85` to `ea204ae970ef7639927ee0ab4c60e00ad0fd156b` with a punctuation-only abstention regression. PR #40 adopted that valid base delta while preserving its existing numeric-only control and source-resolution work, so both abstention boundary cases remain in the dependent tree.

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

Predecessor execution does not transfer. Keep the canonical artifact-boundary review finding unresolved until one unchanged exact successor supplies the executable and hosted evidence above.
