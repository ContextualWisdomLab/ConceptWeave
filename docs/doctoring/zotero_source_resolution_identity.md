# Zotero source-resolution identity

Status: `SOURCE_TEST_REPAIRED_PENDING_CI` on PR #40. This note records a fail-closed provenance repair for the pending-source resolution aggregate; it is not executable GREEN, semantic approval, protected integration, or Zotero write authority.

## Problem boundary

`PendingSourceResolution` carries the report `library_version` and optional Local API `server_id`, and `prepare_source_resolution_review` compares those values with the classification report before accepting item version/type/parent identity. This prevents cross-library-version reuse and prevents cross-server reuse when Zotero supplied a server identity.

The remaining predecessor gap was the optional case. When both the classification report and a decision carried `server_id = None`, equality succeeded. Numeric library version plus item key/version/type/parent coordinates are not a globally unique Zotero-source identity, so a decision could be rebound to another Local API instance that omitted the server identity and happened to expose the same coordinates.

Reality RED `25fa3bfe03365cf1893bfc46f9eb758c31e1a53c` added `source_resolution_fails_closed_when_snapshot_has_no_server_identity`. It constructs a classification report with no server identity and an otherwise exact decision and requires `prepare_source_resolution_review` to reject it as stale.

## Causal repair

Source repair `82a392cde56e54ef22169107ca1bdc89f112dfff` adds the least-widening fail-closed admission rule: `prepare_source_resolution_review` now rejects any report whose `server_id` is absent before accepting an otherwise matching resolution. Existing source-resolution contract fixtures that exercise other rejection paths were moved to an explicit `local-server` identity so they continue testing their intended causes rather than being short-circuited by the new provider-identity gate.

Docs successor `a31d5c22500c4e9e0c69757b3895658a89f526d5` records the same boundary in `AGENTS.md`: optional identity remains acceptable for lower-authority classification, but not for a typed resolution review. Exact equality between decision and report is still required when `server_id` is present.

The repair does not infer provider identity from numeric library/item coordinates, broaden Zotero mutation authority, rewrite the classification report, or treat the resolution record as semantic publication authority. If a future canonical report digest is introduced, it must be versioned and evidence-bound before replacing this fail-closed boundary.

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

Predecessor local evidence does not transfer. Keep the associated review thread unresolved until one unchanged exact successor supplies the executable and hosted evidence above.
