# Zotero source-resolution identity

Status: `SOURCE_TEST_REPAIRED_PENDING_CI` on PR #40. Constructor-side provider identity and stored-artifact restoration are causally source/test repaired. This note is not executable GREEN, semantic approval, protected integration, or Zotero write authority.

## Problem boundary

`PendingSourceResolution` carries the report `library_version` and Local API `server_id`, and `prepare_source_resolution_review` compares those coordinates with the classification report before accepting item version/type/parent identity. Admission fails closed when the report server identity is missing or blank and rejects cross-library/cross-server reuse when decisions are prepared from the report.

The earlier optional-identity defect was fixed by RED `25fa3bfe03365cf1893bfc46f9eb758c31e1a53c` and source repair `82a392cde56e54ef22169107ca1bdc89f112dfff`: a typed review cannot be constructed from a report that lacks a stable provider identity. Later tests cover empty-pending and blank-identity paths.

A separate artifact-boundary defect existed in the former direct-deserialization path. Stored review JSON could retain a valid aggregate `server_id` while a nested decision carried `null`, blank text, or another server identity and still regain the trusted review type. Reality RED `5718e4f62122454c3a989814d80bf5e62edcc073` fixed that historical bypass as a concrete regression.

## Causal repair and current restoration path

Source repair `665e1bbf896fba3f1eec766f2e3651cae5f124e2` first required every nested resolution identity to match the validated aggregate server identity. Subsequent provenance work strengthened the boundary further: trusted `SourceResolutionReview` no longer implements direct `Deserialize`.

Current stored restoration is explicitly two-stage. `restore_source_resolution_review` decodes bytes only into private `StoredSourceResolutionWire`, whose DTO boundaries reject undeclared fields. The wire validator checks nonblank aggregate identity, nested server/library agreement, canonical pending keys, expected identities, nonblank reasons and ordered complete decisions. Restoration then compares the stored envelope with the caller-supplied immutable `ClassificationReport` and invokes `prepare_source_resolution_review` so key/version/type/parent identity is re-established against report inventory before a trusted `SourceResolutionReview` is returned.

The public review aggregate remains serialize-only with private constructor-bound fields and immutable accessors. It cannot be externally created by struct literal, mutated after construction, or restored from JSON without the report-binding path.

A future canonical report digest may strengthen this binding only through an explicit versioned contract and an appropriate integrity primitive. It must not silently replace the caller-supplied report as the trust root with another mutable self-declared field.

## Acceptance and traceability

Constructor path: `PendingSourceResolution` → `prepare_source_resolution_review` → `SourceResolutionReview`.

Stored restoration path: untrusted JSON → private `StoredSourceResolutionWire` → `validate_stored_source_resolution_wire` → envelope equality against `ClassificationReport` → `prepare_source_resolution_review` → trusted `SourceResolutionReview`.

The implementation is in `crates/conceptweave-zotero/src/lib.rs`; public regressions span the source-resolution contract, server/library identity, stored completeness, coordinated identity rewrite, strict unknown-field and pending-set integrity tests.

Required GREEN is one unchanged exact PR #40 successor passing:

- `cargo +1.98.0 test --workspace --locked`;
- `cargo +1.98.0 fmt --all -- --check`;
- `cargo +1.98.0 clippy --workspace --all-targets --locked -- -D warnings`;
- `RUSTDOCFLAGS='-D warnings' cargo +1.98.0 doc --workspace --no-deps --locked`;
- warnings-denied release build;
- owned production function / normalized-region / branch coverage at 100%;
- applicable hosted checks and independent review evidence.

Predecessor execution does not transfer. Keep still-valid artifact-boundary findings unresolved until one unchanged exact successor supplies the executable and hosted evidence above.
