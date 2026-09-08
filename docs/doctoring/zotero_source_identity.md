# Zotero source identity admission

Status: **Reality RED — source repair pending**  
Owner lane: Research Intake PR #9  
Reality RED: `3a11bdd119ae941e11bda7a28ab7a3e520ed18fe`

## Problem

The immutable research report treats each Zotero `key` as the stable source identity used for item inventory, parent/child linkage, duplicate provenance and later steward replay. The current reader guard rejects blank/whitespace keys and item revisions newer than the advertised library version, but it does not reject other syntactically invalid nonblank keys before page accumulation.

That is weaker than the provider contract. Zotero's own test support defines `zoteroObjectKeyRe` as `^[23456789ABCDEFGHIJKLMNPQRSTUVWXYZ]{8}$` and notes that it is based on `Zotero.Utilities::generateObjectKey()`. Accepting arbitrary nonblank strings therefore permits a malformed provider identity to enter an otherwise immutable report and makes downstream provenance look stronger than the source admission actually proved.

## Constraints

- Treat provider identity as evidence; do not trim, case-fold, rewrite or invent a replacement key.
- Reject malformed source identity before it can participate in snapshot accumulation, relationship traversal or duplicate grouping.
- Preserve the existing snapshot-change failure semantics, future-item-version guard, duplicate-key guard, resource/deadline budgets and read-only boundary.
- Do not add a `cfg(test)` identity bypass. Existing reader fixtures that use short synthetic keys are fixture debt and must migrate to canonical Zotero-shaped keys when the source repair lands.
- Classification of an already captured in-memory fixture remains a separate deterministic boundary; this finding concerns Local API source admission.

## Reality RED

`crates/conceptweave-zotero/src/tests/metadata_transport.rs::snapshot_rejects_noncanonical_zotero_object_keys` requires these nonblank identities to fail closed:

- `ABCDEFG` — seven characters;
- `ABCDEFGHI` — nine characters;
- `ABC1DEFG` — forbidden digit `1`;
- `ABCOEFGH` — forbidden letter `O`;
- `abcdefgh` — lowercase;
- `ABC-DEF2` — punctuation.

`2A3B4C5D` is the positive control. At pre-RED source `0df993f8350407649dd0cc99ba708457e4de969d`, admission checks only `item.key.trim().is_empty()` for source-key syntax, so the invalid nonblank controls are admitted. The regression is intentionally RED until production admission implements the provider contract.

## Decision

Implement one exact Zotero object-key predicate at the source-admission boundary and fail with the existing snapshot-integrity error when a page contains a noncanonical key. Keep the key byte-for-byte unchanged when valid.

A permissive "nonblank string" rule is rejected because it does not establish provider identity. Silent normalization is rejected because it fabricates provenance and can collapse distinct malformed inputs. A test-only exception is rejected because it would make the production contract untestable; migrate synthetic reader fixtures instead.

## Acceptance

The repair is not GREEN until one unchanged successor head demonstrates:

1. canonical-key negative and positive regressions;
2. existing blank-key, duplicate-key, parent/child, pending-source, future-version, budget and deadline behavior;
3. locked Rust 1.98 workspace tests and formatting;
4. all-target Clippy and warnings-denied rustdoc/release;
5. owned production function, normalized-region and branch coverage at 100%;
6. applicable hosted exact-head checks.

A passing classifier or zero pending source keys does not grant review, publication or write authority.

## Traceability

| Evidence | Exact binding |
| --- | --- |
| Provider identity syntax | `zotero/zotero@test/content/support.js`, `zoteroObjectKeyRe = /^[23456789ABCDEFGHIJKLMNPQRSTUVWXYZ]{8}$/`, exact source snapshot `fc17dcd24ad34686cb24e6b3ffb06a6a7a5e0e5d` |
| Pre-RED ConceptWeave guard | `crates/conceptweave-zotero/src/lib.rs` at `0df993f8350407649dd0cc99ba708457e4de969d`: source admission checks blank/whitespace key plus future item revision only |
| Reality RED | `crates/conceptweave-zotero/src/tests/metadata_transport.rs` at `3a11bdd119ae941e11bda7a28ab7a3e520ed18fe` |
| Review owner | PR #9 thread `PRRT_kwDOUKg5E86fVadN`; RED reply `3954276613` |

## Reference

Zotero. (2026). *test/content/support.js* [Source code]. GitHub. `fc17dcd24ad34686cb24e6b3ffb06a6a7a5e0e5d`.
