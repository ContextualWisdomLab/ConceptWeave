# Product / Technical Gap Baseline

**Snapshot:** 2026-09-12

This is the code-current ConceptWeave product/technical gap authority for the active Source Observation lane. Exact SHAs, reviews and runs are evidence snapshots, not mutable dependencies. A moved head invalidates predecessor execution evidence unless the successor itself produces equivalent evidence.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. Source-system business truth remains with its canonical owner.

- `semantic-data-portal`: catalog, governance and consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- `keyverse`: identity/authentication trust evidence; ConceptWeave owns authorization of ConceptWeave proposal/base/semantic resources.
- consumers: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL and mutable sibling-head dependencies are invalid integration mechanisms.

## Live stack and protected prerequisites

Fresh authority entering this update:

- protected/default ConceptWeave `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`;
- Product-CI bootstrap #35: `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mechanically mergeable, still waiting on central protected-workflow settlement;
- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft;
- Source Observation #6: `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft;
- representation-v3 parent #45: `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6;
- representation/index successor #46 source head before this documentation commit: `8efc1e5d24c3fb4670237515f3f760b73ee5303e`, OPEN Draft and mechanically mergeable.

Protected central `.github/main` was freshly verified at `cb0872c9a20d5584703dffacca65c096fc034c6c`; `.github#2051@558693e0333e48012beea142f739bc634b0674a7` remains Draft on historical `main@7fd571db...`, with `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` stacked on it. The central owner must land a backward-compatible handler, ordinary/non-force reconcile those PRs onto current protected main and obtain terminal GREEN before unchanged #35 can receive fresh acceptance and normal merge.

Protected/default ConceptWeave `main` still lacks repository-local `.github/workflows` authority for the Product `pull_request` workflow. A PR branch cannot bootstrap its own default-branch trigger. Draft/Ready toggles, no-op commits and manual retriggers are not acceptance evidence.

## Source Observation bounded context

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, exact evidence locations and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority or foreign product truth.

Historical v2 evidence is frozen. Its digest domain and `/schemas/{schema}/tables/{table}` coordinate vocabulary retain identical meaning. New PostgreSQL facts use successor contracts; new identity-bearing evidence requires an exact verified receipt coordinate without reinterpreting predecessor paths.

## PostgreSQL 18 representation-v3 state

Preserved source repairs include:

- relation-kind-aware owner/child coordinates and exact qualified type resolution;
- relation-backed composite row-type resolution for non-sequence modeled relations;
- shared schema-local `pg_type` namespace checks across domains, enums, relation row types and observed true arrays;
- exact `pg_type.typarray`/`typelem` true-array observations without underscore-name inference, OID identity or `search_path` dependence;
- one element -> one associated true array, no observed array-of-array relation, same-schema reciprocity and observed-empty versus unobserved array state;
- exact array evidence receipts through `ArrayTypeLocation`/`ArrayTypeSourceReceipt`, binding the public array-aware digest while leaving previous `SchemaObjectLocation` meanings frozen;
- relation-scoped index evidence for key/`INCLUDE` layout, expression keys, per-key collation/operator class/opaque `indoption`, operator-class parameters, `NULLS NOT DISTINCT`, material `pg_index` flags, `pg_class.reloptions`, resolved tablespace state and reconstructed-definition/comment provenance;
- schema-local `pg_class` namespace consistency across owning relations and nested indexes;
- local index admission only on ordinary tables, partitioned tables and materialized views;
- represented table constraints on ordinary/partitioned tables, CHECK-only on foreign tables, and no modeled table constraints on views/materialized views/sequences/standalone composite-type relations;
- at most one represented primary key per relation and exact `nullable = false` evidence for every primary-key column;
- explicitly observed PRIMARY KEY/UNIQUE timing as `NotDeferrable`, `InitiallyImmediate` or `InitiallyDeferred`, with exact relation/constraint coordinates, complete inventory, observed-empty versus unobserved state and a separate `conceptweave.postgres_schema_snapshot.v3.constraint_timings.v1` digest layer.

### Key-constraint timing lineage

PostgreSQL 18 stores constraint timing in `pg_constraint.condeferrable`/`condeferred`. Source repair is preserved as:

- finding review `5181035661` on `ccd7e245010dab994a6540fe2de5e15db3f742ea`;
- behavioral RED `0a11a3285e7d5f371494656eac890b5b3a632796`, `constraint_timing_contract.rs`;
- timing VO `85d462795a462abd11e4c0782a80e0d82a5c7727`;
- domain-separated snapshot integration `929451e41b0f977f6bbb01299b843c4bfa2fbc68`;
- completeness regression `f0e97b4de53db9bdf6326abad36bfdfc4bf9b12e` and admission repair `72bdb7752483fc974a09fae0443cbd5ab231e919`;
- canonical coordinate repair `8a6092733c2417fd3eae88bd279e0dbdcc880e85`;
- code-current timing baseline `9020f3620afbe932aec7a0ca6f1bb3dbf5b0d316`.

The timing family is additive. It does not mutate `PrimaryKeyObservation`/`UniqueConstraintObservation` shared with frozen v2. Existing constraint receipt paths remain stable and bind the public digest, including timing when explicitly observed.

### Active P1 — key constraint to supporting-index coherence

Review `5181223180` on exact `9020f3620afbe932aec7a0ca6f1bb3dbf5b0d316` found that v3 now models both sides of PostgreSQL's key-constraint/index relationship but does not bind them. `pg_constraint.conindid` identifies the index supporting PRIMARY KEY/UNIQUE, PostgreSQL creates the supporting unique index with the same constraint name, and a deferrable unique/primary constraint is backed by a non-immediate index (`pg_index.indimmediate = false`).

Initial behavioral RED `c0cec50ed29e2435cf1552302e166a7ff7494924` added `constraint_backing_index_contract.rs`. Fixture review then caught a non-canonical UTC provenance timestamp; ordinary-forward refinement `8efc1e5d24c3fb4670237515f3f760b73ee5303e` changes it to exact `Z` UTC so the test reaches the intended backing-index invariant rather than failing on provenance validation first.

The current RED requires the public v3 aggregate to fail closed when explicitly observed key-constraint timing disagrees with represented index evidence:

- every represented PRIMARY KEY/UNIQUE must resolve to one same-relation, same-name supporting index;
- PRIMARY KEY must bind an observed unique index with `indisprimary = true`;
- UNIQUE must bind an observed unique non-primary index;
- `NotDeferrable` must agree with `indimmediate = true`;
- `InitiallyImmediate` and `InitiallyDeferred` are both deferrable and therefore require `indimmediate = false`;
- coherent evidence remains admissible.

`conindid` remains an adapter-local join coordinate, not governed semantic identity. The canonical domain relationship is the exact relation + constraint/index coordinate and observed catalog state.

This P1 is **RED-active**. Production aggregate admission has not yet been changed to satisfy `constraint_backing_index_contract`; no source GREEN or native/Product GREEN claim is valid on this head.

### Preserved high-value repair lineage

- `5176683395 -> 908d1b10e63254aa4cb85eda9c078ee79f950c0c -> 68efaf0fc735faa74202b420df9bf031069e5116`: reject impossible non-unique + `NULLS NOT DISTINCT=true` state.
- `5176751905 -> 0e6c7314bdd36f313abb6c09231d1e1271383ce0 -> f24242708cf82f4405c12ed2b8fa7153b1c58b24`: preserve material `pg_index` flags.
- `f796bf51110863e98e5d4d16a7f7bbea689b4705 -> 70455fdfbc28dffc8f306619e806b79ca9678693 -> 3eab943ad85584b535417b770f05c67192a7a081`: preserve canonical `pg_class.reloptions` and observed-empty state.
- `5176975409 -> 1a47d6b16838006e5f7a75407e69464740f368b1 -> 5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce -> b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`: exact resolved index tablespace identity.
- `5177885832 -> b820c7b80b6c95e6ae419515882d850d79e578ec -> 1bede23588956c11500beb9a54f1f617ceb5429f`: schema-local `pg_class` namespace enforcement.
- `5178743508 -> f81ae51af614a39e648a9c314776ba79bf99d64e -> a8e9fac16896e3e6d48ec5bc20cafae8c855da39`: reject indexes on non-indexable relation kinds.
- `5179341855 -> 089df3d4d59a45cd87afd30c4d86390a96c8c674 -> 50b8d05e286181a3d39e88186116a4af173a4285`: relation-kind/constraint admission.
- `5180058119 -> eca8adb2e5667048c220a37ad863971a8457e9d3 -> 66130c568705092ffd4dabc9bf56bf2a8c88da3a`: exact relation-backed composite row-type identity.
- `5180207753 -> 51075e48da8da3059cb6ec764f8c45b88b1f933c -> 8d8bd115f080fbcc11fb2be755f41436ba3b8886 -> 21c216aae61009d54dcb4a593f502fdd8654598d -> a7d20d90f1de5a4b94ac23e1d22737be3c4d9c1d -> da6fe0fd51431fa0f902566a9d8d3fac6bd8caf9`: exact true-array identity, reciprocity, digest and receipt coordinates.
- `5180938301 -> 65be0a02da340ecc4ab96b34b8a5325d8f8b197c -> 27de693b18d6cb1936e572a44e91c5ace2b7209a`: primary-key cardinality/nullability invariant.

## Acceptance still required

The current #46 lineage is not native/Product GREEN. After the backing-index RED is causally repaired, one unchanged exact successor must produce:

- repository-pinned Rust 1.98 `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests including frozen-v2 and retained v3 index/type/array/constraint contracts plus `constraint_timing_contract`, `constraint_backing_index_contract`, `primary_key_invariants_contract`, `array_type_identity_contract`, `array_type_digest_contract`, `array_type_schema_contract` and `array_type_receipt_contract`;
- rustdoc/doc tests, release build and owned production docstring/test/edge-case coverage;
- applicable Product/security/dependency/review workflows terminal on the same exact head.

No predecessor GREEN transfers. Draft/bot-only status is not acceptance.

## Concrete PostgreSQL adapter boundary

No transport is admitted before representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must:

- use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate and passing dependency/SBOM review;
- resolve least-privilege credentials only for the authorized source key and immutable policy binding;
- use one explicit `REPEATABLE READ READ ONLY` catalog transaction and one non-resetting connect/query/cancellation budget;
- use catalog OIDs only for adapter-local joins, then cross the ACL with exact names/coordinates;
- resolve `pg_class.reltype`/`pg_type.typrelid`, `pg_type.typarray`/`typelem`, and exact relation/type namespaces;
- collect complete `pg_constraint` PK/UNIQUE timing and `conindid` support relationships, resolving supporting indexes before crossing the ACL;
- validate `condeferrable`/`condeferred` against `pg_index.indimmediate`, PK against `indisprimary`/`indisunique`, and exact constraint/index names rather than persisting OIDs;
- validate schema-local `pg_class`, derived index `relkind`, tablespace/options, relation-kind constraint rules, single-PK cardinality and PK NOT NULL consistency;
- enforce policy-admitted row/byte/concurrency ceilings and complete-or-fail snapshot construction.

## Standards and primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE — UNIQUE/PRIMARY KEY indexes, deferrability, constraint naming and primary-key semantics*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint — `condeferrable`, `condeferred`, `conindid`*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index — `indisunique`, `indisprimary`, `indimmediate` and index state*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_type and the PostgreSQL Type System*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class and CREATE INDEX*.
- PostgreSQL 18 source `src/backend/catalog/index.c` for deferrable supporting-index `indimmediate` behavior and `src/backend/parser/parse_utilcmd.c` for deferrable-index/constraint compatibility.

Catalog OIDs are adapter-local joins, never governed semantic identity. `pg_get_indexdef`/`pg_get_expr` are reconstructed provenance text, never the sole semantic carrier.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams unchanged. |
| Truth/publication lifecycle | RED_ACTIVE_NO_PUBLICATION | No protected immutable semantic release exists. |
| Source Observation | REPRESENTATION_V3_BACKING_INDEX_RED_ACTIVE | Timing/array/index/type repairs are preserved; corrected `constraint_backing_index_contract` is intentionally unsatisfied pending causal aggregate admission repair. |
| Product CI | BLOCKED_OWNER_RECONCILIATION | Protected/default ConceptWeave `main` still lacks Product workflow authority; #35 waits on central owner settlement. |
| Quality gate | RED_ACTIVE | No Ready/adoption/merge before source repair and unchanged-head Rust/Product/security/dependency/review evidence. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Repair the public v3 aggregate so explicitly observed PK/UNIQUE timing and represented backing-index evidence satisfy the corrected active RED without persisting `conindid` as semantic identity.
2. On the repaired unchanged #46 head, produce repository-pinned Rust 1.98 and applicable hosted Product/security/dependency/review acceptance; causally repair any real failure.
3. Ordinary/non-force adopt verified #46 into #45 and obtain fresh parent acceptance; then adopt #45 into #6. Never transfer predecessor GREEN.
4. In parallel, central owner lands the backward-compatible protected handler, reconciles #2051/#2056 onto current protected `.github/main`, obtains terminal GREEN, then unchanged #35 gets fresh acceptance and normal merge.
5. Foundation ordinary/non-force restacks after #35; descendants consume only released/versioned owner contracts.
6. Only then implement the bounded PostgreSQL adapter and frozen conformance fixture, followed by discovery/alignment/deterministic validation/independent evaluation/steward review/immutable publication under the canonical owner boundaries.

Adapters remain outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence.
