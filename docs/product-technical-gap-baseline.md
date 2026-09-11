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
- representation/index successor #46: base #45 above. Tablespace RED `1a47d6b16838006e5f7a75407e69464740f368b1` is source-repaired by `5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce` plus public export `b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`. Review `5177806003` and RED `d425bdfb00652321367dd1fef0c77930d2ade327` were subsequently superseded by review `5177862263` after PostgreSQL 18 source proved index `relkind` is derived from the owning relation kind; the impossible RED was removed ordinary-forward at `96e2b683ad21ebdd1137f618b6f1d2c56ae544c2`.

Protected central `.github/main` was freshly verified at `cb0872c9a20d5584703dffacca65c096fc034c6c`. `.github#2051@558693e0333e48012beea142f739bc634b0674a7` and stacked `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remain owner prerequisites: reconcile them ordinary/non-force onto current main, preserve terminal-job-set/atomic-wake behavior, land a backward-compatible protected handler, and obtain fresh exact terminal GREEN before unchanged-head #35 acceptance.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op/manual retrigger, synthetic status, mutable supplier dependency or routine administrator bypass is acceptance evidence.

## Source Observation bounded-context contract

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content digest identity, evidence locations and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority or foreign product truth.

Historical v2 evidence is frozen. The v2 digest domain and `/schemas/{schema}/tables/{table}` vocabulary retain identical meaning and reproducibility. New PostgreSQL facts must not be appended under the v2 identity domain.

## PostgreSQL 18 representation-v3 current state

The #45 -> #46 lineage now carries materially more complete PostgreSQL index evidence than the predecessor baseline described. Ordinary-forward commits are inspected and adopted rather than treated as a race.

Implemented source repair includes:

- relation-kind-aware owning-relation coordinates and exact qualified type/domain resolution;
- key/`INCLUDE` layout invariants, at least one key, and expression exclusion from `INCLUDE`;
- exact per-key collation, operator-class coordinate, access-method-specific `indoption` bits and operator-class parameters;
- uniqueness/`NULLS NOT DISTINCT` consistency;
- first-class `pg_index` flags `indisprimary`, `indisexclusion`, `indimmediate`, `indisclustered`, `indcheckxmin`, `indisreplident`;
- index `pg_class.reloptions` as canonical exact name/value evidence with unobserved state distinct from an observed empty set;
- index `pg_class.reltablespace` as first-class resolved tablespace evidence, preserving unobserved vs observed database-default vs explicit named assignment as distinct states;
- deterministic v3 digest framing for the above while frozen v2 framing remains unchanged.

Recent exact repair lineage:

- review `5176683395` -> RED `908d1b10e63254aa4cb85eda9c078ee79f950c0c` -> repair `68efaf0fc735faa74202b420df9bf031069e5116`: reject impossible non-unique + `NULLS NOT DISTINCT=true` evidence;
- review `5176751905` -> RED `0e6c7314bdd36f313abb6c09231d1e1271383ce0` -> repair/export/frozen-v2 correction through `f24242708cf82f4405c12ed2b8fa7153b1c58b24`: preserve the remaining material `pg_index` state vector;
- RED `f796bf51110863e98e5d4d16a7f7bbea689b4705` -> repair `70455fdfbc28dffc8f306619e806b79ca9678693` -> public export `3eab943ad85584b535417b770f05c67192a7a081`: preserve `pg_class.reloptions` identity, canonicalize option order, reject duplicate names, and distinguish unobserved from observed-empty state;
- review `5176975409` -> RED `1a47d6b16838006e5f7a75407e69464740f368b1` -> production repair `5021ed6b6fc8c6af136f8560c5d0c80c5da6c7ce` -> public export `b56a38de7f5a1c7419fa0ea2105c9cd7422a59a3`: preserve exact resolved index tablespace identity without promoting catalog OIDs into the governed contract;
- review `5177806003` -> provisional RED `d425bdfb00652321367dd1fef0c77930d2ade327` -> superseding review `5177862263` -> ordinary-forward removal `96e2b683ad21ebdd1137f618b6f1d2c56ae544c2`: deeper PostgreSQL 18 source verification showed ordinary/partitioned index `relkind` is not an independent valid-state dimension and the same-owner RED admitted impossible source state.

### Tablespace repair source-complete, exact-head acceptance pending

PostgreSQL 18 `pg_class.reltablespace` is material index configuration: it identifies the tablespace where an index is stored, with zero meaning the database's default tablespace. `pg_database.dattablespace` is the database-default OID used when `reltablespace` is zero; `pg_tablespace.spcname` provides the exact cluster-wide tablespace name. `CREATE INDEX ... TABLESPACE` and `ALTER INDEX ... SET TABLESPACE` can change this fact.

The production repair adds an immutable `IndexTablespace` value object with exact resolved name plus an explicit database-default marker. `Option<IndexTablespace>` preserves three materially different states: unobserved, observed database-default, and observed explicit named assignment. The constructor rejects blank names with `index_tablespace_name`; `IndexObservation::with_tablespace` admits the evidence; `IndexObservation::tablespace` exposes it without mutable state; and `encode_index` frames presence, database-default state, and exact name into v3 source-content identity. Public export is present at `b56a38de...`.

The behavioral tablespace RED remains an acceptance oracle: distinct named tablespaces must produce distinct v3 digests; observed database-default must remain distinct from unobserved; explicit named assignment must not collapse into the database-default marker even when the resolved name is identical; and blank names must fail closed. Catalog OIDs remain adapter-local join coordinates. The eventual adapter must resolve `reltablespace = 0` through the database default and resolve nonzero OIDs through `pg_tablespace` before crossing the domain boundary.

### Derived index-relkind invariant — no duplicate domain identity

PostgreSQL 18 `pg_class.relkind` distinguishes ordinary indexes (`i`) from partitioned indexes (`I`), and partitioned indexes are virtual parents whose actual data is held by child indexes. Initial review `5177806003` treated that catalog field as an independent v3 identity dimension. The deeper owner-source check disproved that modeling assumption for valid PostgreSQL index creation.

In PostgreSQL 18 `DefineIndex`, the `partitioned` decision is derived directly from the indexed relation: `rel->rd_rel->relkind == RELKIND_PARTITIONED_TABLE`. The command then sets `INDEX_CREATE_PARTITIONED`; `index_create` deterministically assigns `RELKIND_PARTITIONED_INDEX` when that flag is set and `RELKIND_INDEX` otherwise. The provisional test constructed both ordinary and partitioned index kinds on the same partitioned owning relation, which is not a valid state produced by this contract.

Because `RelationObservation.kind` is already first-class and framed into v3 identity, adding an independently mutable `IndexRelationKind` would duplicate a derived invariant and create contradictory domain states. The provisional RED was therefore removed rather than implemented. The useful control belongs at the future PostgreSQL Anti-Corruption Layer: capture the index relation's catalog `relkind` and fail closed if it disagrees with the owning relation-derived invariant before constructing governed `IndexObservation`; do not persist the redundant field as a second source of truth.

Authoritative basis:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html
- PostgreSQL 18 source, `src/backend/commands/indexcmds.c`: `DefineIndex` derives `partitioned` from the owning relation and sets `INDEX_CREATE_PARTITIONED`.
- PostgreSQL 18 source, `src/backend/catalog/index.c`: `index_create` maps that flag to `RELKIND_PARTITIONED_INDEX` versus `RELKIND_INDEX`.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_database*. https://www.postgresql.org/docs/18/catalog-pg-database.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE INDEX*. https://www.postgresql.org/docs/18/sql-createindex.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER INDEX*. https://www.postgresql.org/docs/18/sql-alterindex.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Tablespaces*. https://www.postgresql.org/docs/18/manage-ag-tablespaces.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_tablespace*. https://www.postgresql.org/docs/18/catalog-pg-tablespace.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

Catalog OIDs may be adapter-local join coordinates but are not governed semantic identity. `pg_get_indexdef`/`pg_get_expr` remain reconstructed provenance text, never the sole semantic carrier.

## Representation acceptance still required

#46/#45/#6 can return to source-repaired status after removal of the invalid RED, but cannot claim native/Product GREEN until one unchanged exact successor passes repository-pinned Rust 1.98:

- `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests including frozen-v2 and all retained v3 index representation contracts;
- rustdoc/doc tests and release build;
- owned production docstring/test/edge-case coverage requirements;
- applicable Product/security/dependency/review workflows bound to the exact head.

Hosted or local execution produced for predecessor heads does not transfer. Draft-skipped or bot-only status is not native/Product acceptance.

## Concrete PostgreSQL adapter boundary

Do not attach transport while exact-head representation acceptance is absent. After representation GREEN and ordinary/non-force adoption through #45/#6, the PostgreSQL adapter must use a maintained patched Rust driver pinned by immutable lock coordinate and passing cargo-deny/SBOM review; resolve least-privilege credentials only for the authorized source key+binding; reject stale binding before credential/source I/O; use one explicit `REPEATABLE READ READ ONLY` catalog transaction; resolve catalog OIDs to exact governed coordinates before crossing the Anti-Corruption Layer; preserve complete schema/index evidence including tablespace; validate index `pg_class.relkind` against the owning relation-derived PostgreSQL invariant; consume one non-resetting operation budget across connect/query/cancellation; enforce policy-admitted row/byte/concurrency ceilings; and complete-or-fail snapshot construction.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams remain unchanged. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | No protected immutable semantic release exists. |
| Source Observation | REPRESENTATION_V3_SOURCE_REPAIRED_PENDING_EXACT_HEAD | Tablespace source repair remains; invalid relkind RED was superseded and removed ordinary-forward. Exact-head native/Product acceptance remains pending. |
| Product CI | BLOCKED_OWNER_RECONCILIATION | #35 waits on central backward-compatible handler/current-main reconciliation and exact terminal GREEN. |
| Quality gate | PENDING_EXACT_HEAD | Every head movement resets Rust/Product/security/review acceptance. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Re-read #46 after this documentation successor and obtain repository-pinned Rust 1.98 plus applicable hosted Product/security/dependency/review acceptance on one unchanged exact head. If a retained test/lint/doc failure is real, repair causally and restart exact-head evidence.
2. Ordinary/non-force adopt verified #46 into #45 and then #6; do not transfer predecessor GREEN.
3. In parallel, central owner lands the backward-compatible handler, reconciles #2051/#2056 onto current protected `.github/main`, obtains terminal GREEN, then unchanged-head #35 receives fresh acceptance and merges normally.
4. Foundation ordinary/non-force restacks after #35; descendants consume only released/versioned owner contracts.
5. Only after Source Observation representation and protected prerequisites are current, add the bounded PostgreSQL adapter and frozen conformance fixture, including fail-closed verification of derived index `relkind` consistency.
6. Continue ontology/semantic discovery, alignment, deterministic validation, independent evaluation, steward review and immutable publication under canonical owner boundaries. Production LLM calls remain behind released `contextual-orchestrator` contracts.

Adapters stay outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence.