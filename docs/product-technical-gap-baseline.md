# Product / Technical Gap Baseline

**Snapshot:** 2026-09-11

This document records ConceptWeave's code-current product and technical gap baseline. Exact SHA/run coordinates are immutable evidence snapshots, never mutable supplier dependencies. Live protected branch, PR, issue, review, and workflow state supersedes a recorded coordinate when it advances. Any head movement resets exact-head execution/review evidence unless that evidence was actually produced for the successor.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical release-consumption contract. Source-system business truth stays with its canonical owner.

- `semantic-data-portal`: catalog/governance/consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- `keyverse`: identity/authentication trust evidence; ConceptWeave separately owns authorization of ConceptWeave proposal/base/semantic resources.
- consuming products: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and active prerequisites

Fresh 2026-09-11 authority:

- protected/default ConceptWeave `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; no immutable ConceptWeave semantic release exists yet;
- Product-CI bootstrap #35: `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mechanically mergeable; central CodeQL rollout/settlement remains the prerequisite to fresh acceptance;
- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft and must ordinary/non-force restack after #35;
- Source Observation #6: `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft;
- representation-v3 parent #45: `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6;
- representation/index successor #46: tablespace RED `1a47d6b16838006e5f7a75407e69464740f368b1` is source-repaired by `5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce` plus export `b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`; the provisional index-relkind RED was superseded and removed at `96e2b683ad21ebdd1137f618b6f1d2c56ae544c2`; schema-local `pg_class` namespace review `5177885832` -> RED `b820c7b80b6c95e6ae419515882d850d79e578ec` -> source repair `1bede23588956c11500beb9a54f1f617ceb5429f`; index-owner relation-kind review `5178743508` -> RED `f81ae51af614a39e648a9c314776ba79bf99d64e` -> source repair `a8e9fac16896e3e6d48ec5bc20cafae8c855da39`; relation-constraint-kind review `5179341855` -> RED `089df3d4d59a45cd87afd30c4d86390a96c8c674` -> source repair `50b8d05e286181a3d39e88186116a4af173a4285`. Native/Product acceptance remains pending.

Protected central `.github/main` was freshly verified at `cb0872c9a20d5584703dffacca65c096fc034c6c`. `.github#2051@558693e0333e48012beea142f739bc634b0674a7` and stacked `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remain owner prerequisites: reconcile them ordinary/non-force onto current main, preserve terminal-job-set/atomic-wake behavior, land a backward-compatible protected handler, and obtain fresh exact terminal GREEN before unchanged-head #35 acceptance.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op/manual retrigger, synthetic status, mutable supplier dependency or routine administrator bypass is acceptance evidence.

## Source Observation bounded-context contract

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content digest identity, evidence locations and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority or foreign product truth.

Historical v2 evidence is frozen. The v2 digest domain and `/schemas/{schema}/tables/{table}` vocabulary retain identical meaning and reproducibility. New PostgreSQL facts must not be appended under the v2 identity domain.

## PostgreSQL 18 representation-v3 current state

Implemented source repair includes:

- relation-kind-aware owning-relation coordinates and exact qualified type/domain resolution;
- key/`INCLUDE` layout invariants, at least one key, and expression exclusion from `INCLUDE`;
- exact per-key collation, operator-class coordinate, access-method-specific `indoption` bits and operator-class parameters;
- uniqueness/`NULLS NOT DISTINCT` consistency;
- first-class `pg_index` flags `indisprimary`, `indisexclusion`, `indimmediate`, `indisclustered`, `indcheckxmin`, `indisreplident`;
- index `pg_class.reloptions` as canonical exact name/value evidence with unobserved state distinct from an observed empty set;
- index `pg_class.reltablespace` as first-class resolved tablespace evidence, preserving unobserved vs observed database-default vs explicit named assignment as distinct states;
- schema-local `pg_class` relation-name consistency across every modeled owning relation and nested index before v3 digest/receipt construction;
- fail-closed index ownership admission: nested local index evidence is accepted only for ordinary tables, partitioned tables, and materialized views, not views, foreign tables, sequences, or standalone composite-type relations;
- fail-closed represented constraint admission: ordinary and partitioned tables admit the modeled table constraints; foreign tables admit only modeled CHECK constraints; views, materialized views, sequences, and standalone composite-type relations reject modeled table constraints;
- deterministic v3 digest framing for the above while frozen v2 framing remains unchanged.

Recent exact repair lineage:

- review `5176683395` -> RED `908d1b10e63254aa4cb85eda9c078ee79f950c0c` -> repair `68efaf0fc735faa74202b420df9bf031069e5116`: reject impossible non-unique + `NULLS NOT DISTINCT=true` evidence;
- review `5176751905` -> RED `0e6c7314bdd36f313abb6c09231d1e1271383ce0` -> repair/export/frozen-v2 correction through `f24242708cf82f4405c12ed2b8fa7153b1c58b24`: preserve the remaining material `pg_index` state vector;
- RED `f796bf51110863e98e5d4d16a7f7bbea689b4705` -> repair `70455fdfbc28dffc8f306619e806b79ca9678693` -> public export `3eab943ad85584b535417b770f05c67192a7a081`: preserve `pg_class.reloptions` identity, canonicalize option order, reject duplicate names, and distinguish unobserved from observed-empty state;
- review `5176975409` -> RED `1a47d6b16838006e5f7a75407e69464740f368b1` -> repair `5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce` -> export `b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`: preserve exact resolved index tablespace identity without promoting catalog OIDs into the governed contract;
- review `5177806003` -> provisional RED `d425bdfb00652321367dd1fef0c77930d2ade327` -> superseding review `5177862263` -> removal `96e2b683ad21ebdd1137f618b6f1d2c56ae544c2`: PostgreSQL 18 source proved index `relkind` is a derived invariant, so the impossible same-owner alternative was removed rather than implemented;
- review `5177885832` -> RED `b820c7b80b6c95e6ae419515882d850d79e578ec` -> repair `1bede23588956c11500beb9a54f1f617ceb5429f` -> exact-current review `5178298921`: enforce PostgreSQL's schema-local `pg_class` relation-name namespace across modeled owning relations and nested indexes without changing frozen v2 or successor coordinate vocabulary;
- review `5178743508` -> RED `f81ae51af614a39e648a9c314776ba79bf99d64e` -> repair `a8e9fac16896e3e6d48ec5bc20cafae8c855da39`: reject impossible nested index evidence on non-indexable PostgreSQL relation kinds before governed v3 digest/receipt construction while preserving table, partitioned-table, and materialized-view indexes;
- review `5179341855` -> RED `089df3d4d59a45cd87afd30c4d86390a96c8c674` -> repair `50b8d05e286181a3d39e88186116a4af173a4285`: reject impossible represented constraint evidence by owning relation kind while preserving table/partitioned-table constraints and foreign-table CHECK evidence.

### Tablespace repair source-complete

PostgreSQL 18 `pg_class.reltablespace` is material index configuration. The production repair adds `IndexTablespace` with exact resolved name plus a database-default marker. `Option<IndexTablespace>` preserves unobserved, observed database-default, and observed explicit named assignment as distinct states. The eventual adapter resolves catalog OIDs to exact names before crossing the domain boundary.

### Derived index-relkind invariant — no duplicate domain identity

PostgreSQL 18 `DefineIndex` derives the partitioned-index decision from the owning relation's `RELKIND_PARTITIONED_TABLE`, sets `INDEX_CREATE_PARTITIONED`, and `index_create` maps that flag to `RELKIND_PARTITIONED_INDEX` versus `RELKIND_INDEX`. Because `RelationObservation.kind` is already first-class and framed into v3 identity, adding an independently mutable `IndexRelationKind` would duplicate a derived invariant and admit contradictory state. The future adapter must observe the catalog index `relkind` and fail closed if it disagrees with the owning relation-derived invariant, rather than persisting a redundant second source of truth.

### Schema-local pg_class namespace P1 — source repaired, exact-head acceptance pending

PostgreSQL stores tables, indexes, sequences, views, materialized views, partitioned relations and other relation-like objects in `pg_class`. PostgreSQL's catalog declares a unique index `pg_class_relname_nsp_index` over `(relname, relnamespace)`. Consequently an index name cannot be duplicated under two different tables in the same schema, and an index cannot share a schema-local `pg_class` name with another relation. The same name remains legal in a different schema.

Review `5177885832` recorded that the prior v3 constructor validated duplicate owning relation names and duplicate indexes only inside one `RelationObservation`. Behavioral RED `b820c7b80b6c95e6ae419515882d850d79e578ec` requires two same-schema collision cases to fail while preserving same-name indexes across different schemas.

Repair `1bede23588956c11500beb9a54f1f617ceb5429f` makes the public `PostgresSchemaSnapshotV3` the owner-level aggregate seam instead of directly re-exporting the private representation constructor. Before delegating to deterministic v3 canonicalization it builds one exact `(schema_name, relation_or_index_name)` set from every owning relation and nested index. A duplicate fails closed before digest or receipt construction; identical names in different schemas remain legal. The private representation type remains inaccessible to consumers, matching the existing v2 owner-wrapper pattern. Frozen v2 identity and successor coordinate vocabulary are unchanged.

### Index owner relation-kind P1 — source repaired, exact-head acceptance pending

PostgreSQL 18 `CREATE INDEX` defines an index on a table or materialized view. Partitioned tables use the table path and own partitioned indexes. Views, foreign tables, sequences, and standalone composite-type relations do not own local PostgreSQL indexes. The v3 domain previously allowed `RelationObservation::with_indexes` on every `RelationKind`, so an impossible snapshot such as a view carrying a nested local index could reach governed identity.

Review `5178743508` records the finding. Behavioral RED `f81ae51af614a39e648a9c314776ba79bf99d64e` admits indexed `Table`, `PartitionedTable`, and `MaterializedView` fixtures while requiring indexed `View`, `ForeignTable`, `Sequence`, and `CompositeType` fixtures to fail with `index_relation_kind` before snapshot identity. Repair `a8e9fac16896e3e6d48ec5bc20cafae8c855da39` extends the public v3 aggregate admission seam so the relation-kind invariant is enforced before deterministic digest or receipt construction. It does not add a second mutable index-kind field and does not alter frozen v2 identity.

### Relation constraint-kind P1 — source repaired, exact-head acceptance pending

The same aggregate seam previously accepted the modeled `PrimaryKey`, `Unique`, `ForeignKey`, and `Check` constraint variants on every `RelationKind`. That admits catalog states PostgreSQL cannot own. Ordinary and partitioned tables can own the modeled table constraints. PostgreSQL 18 foreign tables support CHECK and NOT NULL constraints; because this representation models NOT NULL at the column boundary rather than as a `TableConstraintObservation` variant, CHECK is the only represented table-constraint variant admissible on a foreign table. Views, materialized views, sequences, and standalone composite-type relations do not own these represented table constraints.

Review `5179341855` records the finding. Behavioral RED `089df3d4d59a45cd87afd30c4d86390a96c8c674` requires table/partitioned-table constraint fixtures and foreign-table CHECK evidence to remain admissible while foreign-table primary-key/unique/foreign-key evidence and any represented constraint on a view, materialized view, sequence, or standalone composite-type relation fail with `relation_constraint_kind`. Repair `50b8d05e286181a3d39e88186116a4af173a4285` enforces that invariant at the public v3 aggregate admission seam before deterministic digest or receipt construction. It does not alter frozen v2 behavior or invent provider-specific mutable state.

Repository-pinned Rust 1.98, strict lint/doc/release/coverage and hosted Product/security/dependency/review evidence must still be produced on one unchanged exact successor before adoption or merge.

Authoritative basis:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE INDEX*. https://www.postgresql.org/docs/18/sql-createindex.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL 18 source, `src/include/catalog/pg_class.h`: `pg_class_relname_nsp_index` is unique on `(relname, relnamespace)`.
- PostgreSQL 18 source, `src/backend/commands/indexcmds.c`: partitioned-index creation derives from the owning relation kind.
- PostgreSQL 18 source, `src/backend/catalog/index.c`: index relation kind is derived from the partitioned-create flag.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER FOREIGN TABLE*. https://www.postgresql.org/docs/18/sql-alterforeigntable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE VIEW*. https://www.postgresql.org/docs/18/sql-createview.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER MATERIALIZED VIEW*. https://www.postgresql.org/docs/18/sql-altermaterializedview.html

Catalog OIDs may be adapter-local join coordinates but are not governed semantic identity. `pg_get_indexdef`/`pg_get_expr` remain reconstructed provenance text, never the sole semantic carrier.

## Representation acceptance still required

#46/#45/#6 cannot claim representation GREEN merely because the relation invariants are source-repaired. One unchanged exact #46 successor must pass repository-pinned Rust 1.98:

- `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests including frozen-v2 and all retained v3 index/schema namespace/relation-kind/constraint-kind contracts;
- rustdoc/doc tests and release build;
- owned production docstring/test/edge-case coverage requirements;
- applicable Product/security/dependency/review workflows bound to the exact head.

Hosted or local execution produced for predecessor heads does not transfer. Draft-skipped or bot-only status is not native/Product acceptance.

## Concrete PostgreSQL adapter boundary

Do not attach transport while exact-head representation acceptance is absent. After representation GREEN and ordinary/non-force adoption through #45/#6, the PostgreSQL adapter must use a maintained patched Rust driver pinned by immutable lock coordinate and passing cargo-deny/SBOM review; resolve least-privilege credentials only for the authorized source key+binding; reject stale binding before credential/source I/O; use one explicit `REPEATABLE READ READ ONLY` catalog transaction; resolve catalog OIDs to exact governed coordinates before crossing the Anti-Corruption Layer; preserve complete schema/index evidence including tablespace; validate index-owning relation kind, catalog index `pg_class.relkind` against the owning relation-derived invariant, and represented constraints against relation-kind rules; consume one non-resetting operation budget across connect/query/cancellation; enforce policy-admitted row/byte/concurrency ceilings; and complete-or-fail snapshot construction.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams remain unchanged. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | No protected immutable semantic release exists. |
| Source Observation | REPRESENTATION_V3_RELATION_INVARIANTS_SOURCE_REPAIRED | Namespace RED `b820c7b...`, index-owner RED `f81ae51...`, and constraint-kind RED `089df3d...` have production repairs; exact-head Rust/Product/security/dependency/review evidence remains required. |
| Product CI | BLOCKED_OWNER_RECONCILIATION | #35 waits on central backward-compatible handler/current-main reconciliation and exact terminal GREEN. |
| Quality gate | EXACT_HEAD_PENDING | Keep #46 Draft until one unchanged successor produces required Rust/Product/security/review acceptance. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Hold #46 source stable and obtain repository-pinned Rust 1.98 plus applicable hosted Product/security/dependency/review acceptance on the unchanged exact successor. If any test/lint/doc failure is real, repair causally and restart exact-head evidence.
2. Ordinary/non-force adopt verified #46 into #45 and then #6; do not transfer predecessor GREEN.
3. In parallel, central owner lands the backward-compatible handler, reconciles #2051/#2056 onto current protected `.github/main`, obtains terminal GREEN, then unchanged-head #35 receives fresh acceptance and merges normally.
4. Foundation ordinary/non-force restacks after #35; descendants consume only released/versioned owner contracts.
5. Only after Source Observation representation and protected prerequisites are current, add the bounded PostgreSQL adapter and frozen conformance fixture.
6. Continue ontology/semantic discovery, alignment, deterministic validation, independent evaluation, steward review and immutable publication under canonical owner boundaries. Production LLM calls remain behind released `contextual-orchestrator` contracts.

Adapters stay outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence.
