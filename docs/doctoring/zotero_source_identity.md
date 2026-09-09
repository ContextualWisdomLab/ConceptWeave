# Zotero source identity admission

Status: **Source repaired — executable evidence pending**  
Owner lane: Research Intake PR #9  
Provider-bound RED: `8294caf3ac1ef20fdabd1ce0f07bcea851be7409`  
Minimal source repair: `2715a9d507facaa77d603f247b27d39b6bd3799e`

## Problem

The immutable research report treats each Zotero `key` as the stable source identity used for item inventory, parent/child linkage, duplicate provenance and later steward replay. Before this repair, the reader guard rejected blank/whitespace keys and item revisions newer than the advertised library version, but the Local API adapter did not reject other syntactically invalid nonblank keys before the page could enter snapshot accumulation.

That was weaker than the provider contract. Zotero's own test support defines `zoteroObjectKeyRe` as `^[23456789ABCDEFGHIJKLMNPQRSTUVWXYZ]{8}$` and notes that it is based on `Zotero.Utilities::generateObjectKey()`. Accepting arbitrary nonblank strings therefore permitted malformed provider identity to enter an otherwise immutable report and made downstream provenance look stronger than source admission proved.

## Constraints

- Treat provider identity as evidence; do not trim, case-fold, rewrite or invent a replacement key.
- Reject malformed Local API identity before it can participate in snapshot accumulation, relationship traversal or duplicate grouping.
- Preserve the existing snapshot-change failure semantics, future-item-version guard, duplicate-key guard, resource/deadline budgets and read-only boundary.
- Do not add a `cfg(test)` identity bypass.
- Keep the injectable reader core as a deterministic post-admission boundary. Its short synthetic keys are not provider evidence and therefore need not impersonate Zotero object keys; the provider contract is tested through the real loopback Local API adapter instead.
- Classification of an already captured in-memory fixture remains a separate deterministic boundary and does not prove Local API source admission.

## Reality RED

Initial source inspection RED `3a11bdd119ae941e11bda7a28ab7a3e520ed18fe` showed that arbitrary nonblank keys were admitted by the shared reader. Provider-bound successor `8294caf3ac1ef20fdabd1ce0f07bcea851be7409` moved the regression to the loopback Local API path so it exercises the production adapter rather than an injected `FetchedPage`.

`crates/conceptweave-zotero/src/tests/metadata_transport.rs::snapshot_rejects_noncanonical_zotero_object_keys` requires these nonblank identities to fail closed:

- `ABCDEFG` — seven characters;
- `ABCDEFGHI` — nine characters;
- `ABC1DEFG` — forbidden digit `1`;
- `ABCOEFGH` — forbidden letter `O`;
- `abcdefgh` — lowercase;
- `ABC-DEF2` — punctuation.

`2A3B4C5D` is the positive control. At pre-repair source `0df993f8350407649dd0cc99ba708457e4de969d`, Local API parsing returned these invalid nonblank controls without provider-key validation.

## Decision and repair

`2715a9d507facaa77d603f247b27d39b6bd3799e` adds one exact Zotero object-key predicate and validates every parsed Local API page before returning `FetchedPage` to snapshot accumulation. Valid keys are retained byte-for-byte. Malformed keys fail with the existing `ReadError::SnapshotChanged`; no normalization, case-folding, replacement identity or semantic classification change was introduced.

The existing production wrapper fixture was updated from synthetic key `A` to provider-shaped `2A3B4C5D` because that fixture exercises the Local API adapter. In contrast, injected reader-core fixtures remain intentionally below the provider ACL and can continue to use concise synthetic identities without becoming a production bypass.

A permissive "nonblank string" rule is rejected because it does not establish provider identity. Silent normalization is rejected because it fabricates provenance and can collapse distinct malformed inputs. A test-only exception in the Local API adapter is rejected because it would make the production contract untestable.

## Acceptance

The source repair is not GREEN until one unchanged successor head demonstrates:

1. canonical-key negative and positive Local API regressions;
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
| Pre-repair ConceptWeave admission | `crates/conceptweave-zotero/src/lib.rs` at `0df993f8350407649dd0cc99ba708457e4de969d`: Local API parse has no provider-key syntax validation; shared reader checks only blank/whitespace key plus future item revision |
| Initial reality RED | `crates/conceptweave-zotero/src/tests/metadata_transport.rs` at `3a11bdd119ae941e11bda7a28ab7a3e520ed18fe` |
| Provider-bound RED | same test at `8294caf3ac1ef20fdabd1ce0f07bcea851be7409`, exercised through `read_local_snapshot` loopback transport |
| Minimal source repair | `crates/conceptweave-zotero/src/lib.rs` at `2715a9d507facaa77d603f247b27d39b6bd3799e` |
| Review owner | PR #9 thread `PRRT_kwDOUKg5E86fVadN`; current reply `3954276613` |

## Reference

Zotero. (2026). *test/content/support.js* [Source code]. GitHub. `fc17dcd24ad34686cb24e6b3ffb06a6a7a5e0e5d`.
