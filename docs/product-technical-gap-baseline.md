# Product / Technical Gap Baseline

**Snapshot:** 2026-09-12

This document is the code-current ConceptWeave product/technical gap authority for the active Source Observation lane. Exact SHAs, reviews and runs are evidence snapshots, not mutable dependencies. A moved head invalidates predecessor execution evidence unless the successor itself produced equivalent evidence.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical client release-consumption contract. Source-system business truth remains with its canonical owner.

- `semantic-data-portal`: catalog, governance and consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- `keyverse`: identity/authentication trust evidence; ConceptWeave owns authorization of ConceptWeave proposal/base/semantic resources.
- consumers: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL and mutable sibling-head dependencies are invalid integration mechanisms.

## Live protected truth and stack

Fresh authority entering this baseline update:

- protected/default ConceptWeave `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`;
- Product-CI bootstrap #35: `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mechanically mergeable, awaiting central protected-workflow settlement;
- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft;
- Source Observation #6: `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft;
- representation-v3 parent #45: `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6;
- representation/index successor #46 source head before this documentation commit: `da6fe0fd51431fa0f902566a9d8d3fac6bd8caf9`, OPEN Draft and mechanically mergeable.

Protected central `.github/main` was freshly verified at `cb0872c9a20d5584703dffacca65c096fc034c6c`; `.github#2051@558693e0333e48012beea142f739bc634b0674a7` and stacked `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remain the protected-workflow owner prerequisites. #2051 remains Draft on historical `main@7fd571db...`; #2056 remains Draft stacked on #2051. Product-CI #35 remains unchanged and therefore still waits on a backward-compatible protected handler, ordinary/non-force current-main reconciliation and fresh exact terminal evidence.

No force push, destructive rebase, self-approval, review dismissal, gate weakening, fail-open scanner substitution, synthetic status, no-op/manual retrigger or mutable supplier dependency is acceptance evidence.

## Source Observation bounded context

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, exact evidence locations and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority or foreign product truth.

Historical v2 evidence is frozen. Its digest domain and `/schemas/{schema}/tables/{table}` coordinate vocabulary retain identical meaning. New PostgreSQL facts are represented only by successor contracts; every new identity-bearing evidence kind requires an exact verified receipt coordinate without reinterpreting predecessor paths.

## PostgreSQL 18 representation-v3 source state

Source-repaired behavior now includes:

- relation-kind-aware owning-relation/child coordinates and exact qualified type resolution;
- relation-backed composite row-type resolution for modeled `pg_class` relations whose `reltype` is nonzero, while Sequence remains non-type evidence;
- schema-local `pg_type` collision checks across domains, enums, relation-backed row types and observed true-array rows;
- exact true-array/element observations corresponding to `pg_type.typarray` and `typelem`, without catalog OIDs, `search_path`, display-text identity or underscore-name inference;
- one observed element coordinate maps to at most one associated true array; an observed array cannot be another observed array's element; associated array and element must share a schema;
- observed-empty array inventory remains distinct from an unobserved array family;
- array-aware snapshots preserve original qualified array bindings in governed identity and bind successor receipts to the public array-aware digest, while legacy `PostgresSchemaSnapshotV3::new` retains its existing v3 digest contract;
- exact array-type receipt coordinates through `ArrayTypeLocation` and `ArrayTypeSourceReceipt`; unknown or unobserved coordinates fail closed and canonical `/schemas/{schema}/array-types/{name}` paths use RFC 6901 escaping without changing pre-array `SchemaObjectLocation` meanings;
- exact relation-scoped index evidence: key/`INCLUDE` layout, expressions, per-key collation/operator class/opaque `indoption`, operator-class parameters, `NULLS NOT DISTINCT`, material `pg_index` flags, `pg_class.reloptions`, resolved tablespace state and index definition/comment provenance;
- schema-local `pg_class` name consistency across modeled owning relations and nested indexes;
- local index evidence admitted only on ordinary tables, partitioned tables and materialized views;
- represented table constraints admitted on ordinary/partitioned tables, CHECK-only for foreign tables, and rejected on views/materialized views/sequences/standalone composite-type relations.

### Exact true-array repair lineage

PostgreSQL 18 makes catalog coordinates authoritative: element `typarray` points to the associated true-array row and that row's `typelem` points back to the element. Array names can change because of truncation/collision, multidimensional arrays use the same true-array type, and the associated array follows the element type's schema.

The active lineage is:

- finding review `5180207753`;
- initial RED `856934e561cf41b9ef546570b9a41524549b57f2`;
- fixture-verification review `5180344773`;
- corrected RED `51075e48da8da3059cb6ec764f8c45b88b1f933c`, `array_type_identity_contract.rs`;
- exact `ArrayTypeObservation` VO `8d8bd115f080fbcc11fb2be755f41436ba3b8886`;
- array-aware aggregate/digest/receipt repair `21c216aae61009d54dcb4a593f502fdd8654598d`;
- identity/order/public-digest/observed-empty regressions `dde345e34958fa0ddedf98ae3b34518f38558492`, `array_type_digest_contract.rs`;
- same-schema reciprocity finding review `5180706765`;
- behavioral RED `a4b8dd55f451459e3aadce12835e31bfe1d6943a`, `array_type_schema_contract.rs`;
- same-schema causal repair `a7d20d90f1de5a4b94ac23e1d22737be3c4d9c1d`;
- source review `5180721275`;
- first code-current baseline `a0f4766425dae85ffc63a735850455e76fcc7141` / review `5180747959`;
- exact receipt-coordinate finding review `5180793356`;
- receipt behavioral RED `60d34c2ab52f8215701d865b6b830b5391f91a29`, `array_type_receipt_contract.rs`;
- `ArrayTypeLocation`/`ArrayTypeSourceReceipt` repair `975bd892d57ced8a546cceaebd6b456857ac4700`;
- aggregate export/exact verification seam `da6fe0fd51431fa0f902566a9d8d3fac6bd8caf9`;
- exact-current source review `5180826533`.

The repair intentionally keeps the original v3 constructor and existing successor location semantics stable. `new_with_array_types` is a separate observed-family admission path with domain separator `conceptweave.postgres_schema_snapshot.v3.array_types.v1`; it canonicalizes exact array observations and binds the array inventory plus original qualified domain/column type bindings into the public source digest. The private pre-array representation is used only as compatibility validation after array bindings are projected to their exact element coordinates; public evidence retains the original array coordinate and the public digest prevents that projection from collapsing identity.

True-array receipts are likewise additive. `SchemaObjectLocation` remains frozen for relation/column/constraint/index/domain/enum successor evidence. `ArrayTypeLocation` owns `/schemas/{schema}/array-types/{array}` and receipt issuance verifies the exact observed immutable array inventory before binding source key, connection-policy revision, public array-aware digest, extractor revision and observation time.

### Preserved preceding repair lineage

- `5176683395 -> 908d1b10e63254aa4cb85eda9c078ee79f950c0c -> 68efaf0fc735faa74202b420df9bf031069e5116`: reject impossible non-unique + `NULLS NOT DISTINCT=true` state.
- `5176751905 -> 0e6c7314bdd36f313abb6c09231d1e1271383ce0 -> f24242708cf82f4405c12ed2b8fa7153b1c58b24`: preserve remaining material `pg_index` flags.
- `f796bf51110863e98e5d4d16a7f7bbea689b4705 -> 70455fdfbc28dffc8f306619e806b79ca9678693 -> 3eab943ad85584b535417b770f05c67192a7a081`: preserve canonical `pg_class.reloptions` and observed-empty state.
- `5176975409 -> 1a47d6b16838006e5f7a75407e69464740f368b1 -> 5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce -> b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`: exact resolved index tablespace identity.
- `5177806003 -> d425bdfb00652321367dd1fef0c77930d2ade327 -> 5177862263 -> 96e2b683ad21ebdd1137f618b6f1d2c56ae544c2`: remove an invalid independent index-relkind model after PostgreSQL source proved it derived.
- `5177885832 -> b820c7b80b6c95e6ae419515882d850d79e578ec -> 1bede23588956c11500beb9a54f1f617ceb5429f -> 5178298921`: schema-local `pg_class` namespace enforcement.
- `5178743508 -> f81ae51af614a39e648a9c314776ba79bf99d64e -> a8e9fac16896e3e6d48ec5bc20cafae8c855da39`: reject local indexes on non-indexable relation kinds.
- `5179341855 -> 089df3d4d59a45cd87afd30c4d86390a96c8c674 -> 50b8d05e286181a3d39e88186116a4af173a4285`: relation-kind/constraint admission.
- `5180058119 -> eca8adb2e5667048c220a37ad863971a8457e9d3 -> 66130c568705092ffd4dabc9bf56bf2a8c88da3a`: exact relation-backed composite row-type identity.

## Representation acceptance still required

#46 is source-repaired for the current true-array identity and receipt P1s but is **not** native/Product GREEN. One unchanged exact successor must still produce repository-pinned Rust 1.98 and hosted acceptance evidence:

- `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests including frozen-v2, retained v3 index/namespace/relation-kind/constraint-kind/relation-backed-type contracts, `array_type_identity_contract`, `array_type_digest_contract`, `array_type_schema_contract` and `array_type_receipt_contract`;
- rustdoc/doc tests and release build;
- owned production docstring/test/edge-case coverage requirements;
- applicable Product/security/dependency/review workflows on the exact head.

At earlier source/docs heads, pull-request workflow runs were absent and combined commit status contained only CodeRabbit success. Draft/bot-only status is not acceptance. Every successor, including documentation-only successors, requires a fresh exact-head cycle before adoption.

## Concrete PostgreSQL adapter boundary

Do not attach transport before representation acceptance. After exact-head GREEN and ordinary/non-force adoption through #45/#6, the adapter must:

- use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate and passing dependency/SBOM review;
- resolve least-privilege credentials only for the authorized source key and immutable policy binding, rejecting stale bindings before source I/O;
- use one explicit `REPEATABLE READ READ ONLY` catalog transaction and one non-resetting connect/query/cancellation budget;
- resolve catalog OIDs only as adapter-local joins and cross the ACL with exact names/coordinates;
- resolve `pg_class.reltype`/`pg_type.typrelid` for relation-backed row types;
- resolve `pg_type.typarray`/`typelem` for exact associated array/element coordinates without generated-name inference;
- validate shared schema-local `pg_type` namespace, one-element/one-array reciprocity, no array-of-array interpretation and same-schema array/element reciprocity;
- emit exact array-type evidence coordinates/receipts only after the array row is part of the immutable snapshot;
- validate schema-local `pg_class`, index owning relation kind, derived index `relkind`, index tablespace/options and relation-kind constraint rules;
- enforce policy-admitted row/byte/concurrency ceilings and complete-or-fail snapshot construction.

## Standards and primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_type*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: The PostgreSQL Type System*, §36.2.2.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TYPE — Array Types and Notes*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TYPE*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE INDEX*.
- PostgreSQL 18 source `src/include/catalog/pg_class.h`, `src/backend/commands/indexcmds.c`, and `src/backend/catalog/index.c` for schema-local relation namespace and derived index relation-kind invariants.

Catalog OIDs are adapter-local join coordinates, never governed semantic identity. `pg_get_indexdef`/`pg_get_expr` are reconstructed provenance text, never the sole semantic carrier.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams unchanged. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | No protected immutable semantic release exists. |
| Source Observation | REPRESENTATION_V3_ARRAY_TYPE_SOURCE_REPAIRED | True-array identity + receipt-coordinate source repair through `da6fe0fd...`; native/Product acceptance still absent. |
| Product CI | BLOCKED_OWNER_RECONCILIATION | #35 still depends on central backward-compatible handler/current-main reconciliation and exact terminal GREEN. |
| Quality gate | SOURCE_REPAIRED_NATIVE_ACCEPTANCE_PENDING | Keep #46 Draft until one unchanged exact head has Rust/Product/security/dependency/review evidence. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Treat the current true-array identity + receipt source delta as repaired, then obtain repository-pinned Rust 1.98 plus applicable hosted Product/security/dependency/review acceptance on one unchanged exact #46 head. Real failures require causal repair and a new exact-head cycle.
2. Ordinary/non-force adopt verified #46 into #45 and obtain fresh parent acceptance; then adopt #45 into #6. Never transfer predecessor GREEN.
3. In parallel, central owner must land the backward-compatible protected handler, reconcile #2051/#2056 onto current protected `.github/main`, obtain terminal GREEN, then give unchanged-head #35 fresh acceptance and normal merge.
4. Foundation ordinary/non-force restacks after #35; descendants consume only released/versioned owner contracts.
5. Only after Source Observation representation and protected prerequisites are current, implement the bounded PostgreSQL adapter and frozen conformance fixture.
6. Continue discovery/alignment/deterministic validation/independent evaluation/steward review/immutable publication under the canonical owner boundaries. Production LLM calls remain behind released `contextual-orchestrator` contracts.

Adapters remain outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence.
