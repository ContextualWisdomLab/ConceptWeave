# Product / Technical Gap Baseline

**Snapshot:** 2026-09-12

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, reviews, checks, and runs are evidence coordinates, not mutable dependencies. If a PR head moves, predecessor execution evidence does not transfer unless the successor reproduces the required evidence.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. Source-system business truth remains with its canonical owner.

- `semantic-data-portal`: catalog, governance, consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- `keyverse`: identity/authentication trust evidence; ConceptWeave owns authorization of ConceptWeave proposal/base/semantic resources.
- consumers: tenant/purpose authorization and physical execution.

Consumers may use only released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Live stack and protected prerequisites

The last verified protected/default ConceptWeave `main` entering this update is `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

The active Source Observation stack remains:

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft;
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6;
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, carrying the active representation/index successor.

#46 must remain Draft until one unchanged exact head has the repository-pinned Rust/Product/security/dependency/review evidence listed below. #45 and #6 must not duplicate or partially cherry-pick #46; they ordinary/non-force adopt the complete verified child delta only after #46 exact-head GREEN.

Product-CI bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft, and depends on the central protected `.github` owner. The last verified central protected `.github/main` entering this update is `cb0872c9a20d5584703dffacca65c096fc034c6c`; the current-main reconciliation authority was `.github#2040@3b2de64c2c4c95c56d2f5099a480a0825304d038`, Draft, with hosted acceptance still outstanding. ConceptWeave must not copy, weaken, or locally replace that owner workflow.

## Source Observation bounded context

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, exact evidence locations, and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority, or foreign product truth.

Historical v2 evidence is frozen. Its digest domain and `/schemas/{schema}/tables/{table}` coordinate vocabulary keep identical meaning. PostgreSQL successor facts use additive, domain-separated contracts and exact verified source coordinates.

## PostgreSQL 18 representation-v3 state

The current successor preserves the following source-authoritative invariants:

- relation-kind-aware owner/child coordinates and exact qualified type resolution;
- schema-local `pg_class` namespace consistency across represented relations and indexes;
- exact `pg_type` namespace, true-array `typarray`/`typelem`, direct type kind, domain base, and reciprocal range/multirange evidence without underscore-name, OID, display-text, or `search_path` inference;
- independent array/type-kind families compose while Base evidence never invents true-array or temporal-range semantics;
- direct qualified Base/Range/Multirange evidence in another schema is admitted only when that schema is explicitly authorized by the observation request;
- relation-scoped index key/`INCLUDE` shape, expression keys, per-key collation/operator-class/opaque `indoption`, operator-class parameters, `NULLS NOT DISTINCT`, material `pg_index` flags, `pg_class.reloptions`, tablespace, access method, and reconstructed provenance;
- represented table constraints only on PostgreSQL relation kinds that can own them, single primary-key cardinality, and exact PK non-nullability evidence;
- explicit PRIMARY KEY/UNIQUE timing from `pg_constraint.condeferrable`/`condeferred`, with same-name supporting-index role, shape, `indimmediate`, key-order, non-partial, null-treatment, and exclusion/GiST coherence;
- explicit `pg_constraint.conperiod` as the authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK semantics; index shape never creates `conperiod` truth;
- `conperiod=true` final constrained columns resolve through observed direct type/domain-base evidence to range or multirange, including PostgreSQL 18.4+ domains over range/multirange;
- governed PostgreSQL 18 FK evidence admits `MATCH SIMPLE`/`MATCH FULL` and rejects unimplemented `MATCH PARTIAL` without changing frozen historical vocabulary;
- PERIOD FKs require an equality-key prefix, exact observed `NO ACTION`/`NO ACTION`, an in-snapshot referenced `WITHOUT OVERLAPS` PK/UNIQUE on the exact columns, and exact referenced-key `NOT DEFERRABLE` timing when that target is represented locally.

### Active temporal backing-index repair

Finding review `5185780899` identified that the PK/UNIQUE branch of `canonicalize_constraint_periods()` checked temporal backing-index coherence only when a same-name index and material catalog flags happened to be present. Missing index evidence or missing `pg_index` flag evidence therefore bypassed the invariant.

Behavioral RED `c80d07816863f814e9b8fbb716661310d8b2b150`, retained warning-clean at `3c38cfb1d2c1134dc30a19c511379969428c530b`, adds `constraint_period_backing_index_presence_contract.rs` with three witnesses:

- `conperiod=true` PK/UNIQUE without a same-name backing index fails as `constraint_period_backing_index`;
- a same-name index without material `IndexCatalogFlags` fails through the same invariant;
- explicit same-name GiST plus `indisexclusion=true` evidence is admitted.

Primary-source doctoring is `bfbce8b040a411c89bd9cba3d19b9b5b14a84da6` at `docs/doctoring/source-observation-without-overlaps-backing-index.md`.

Production repair `43126e4b8b53415b0c4547b532b89ff88291ed71` closes only this causal gap. For PRIMARY KEY/UNIQUE with explicit `conperiod=true`, the aggregate now requires the exact same-name represented index, material catalog flags, `indisexclusion=true`, and exact observed access method `gist`. For `conperiod=false`, index-family observation remains optional; if an index and flags are explicitly present, contradictory exclusion truth still fails closed. The repair does not infer `conperiod` from GiST/exclusion evidence and does not make index observation mandatory for ordinary keys. Exact-source review `5185928607` records the repaired boundary.

This slice is **source-repaired / native-and-Product-acceptance-pending**. It is not Ready, merged, adopted by #45/#6, transported to PostgreSQL, published, or released.

## Preserved high-value repair lineage

The detailed narrative remains available in repository history and the cited doctoring documents. The current baseline keeps the causal coordinates required for reconstruction:

- `5180207753 -> 51075e48da8da3059cb6ec764f8c45b88b1f933c -> 8d8bd115f080fbcc11fb2be755f41436ba3b8886 -> 21c216aae61009d54dcb4a593f502fdd8654598d -> a7d20d90f1de5a4b94ac23e1d22737be3c4d9c1d -> da6fe0fd51431fa0f902566a9d8d3fac6bd8caf9`: exact true-array identity, reciprocity, digest, and receipt coordinates.
- `5181223180 -> c0cec50ed29e2435cf1552302e166a7ff7494924 -> 8efc1e5d24c3fb4670237515f3f760b73ee5303e -> 40337bc086f5b12b981c0f602bfc6413287644ea -> 5181883061`: key constraint/supporting-index role and timing coherence.
- `5182343267 -> a54ea984d72b5b0c1609e54826efd6e70101cc7b -> 3c962108d09a8c02f3349ad79285ea8175ec1fa8 -> 5182395681`: supporting-index shape and null-treatment coherence.
- `5182818948 -> b7c8c11eee1444a96523d265f31277570bcc68b7 -> 5f00f911359451a2021f03392e8d0439e88c3fc0 -> 5183334477`: exclusion/GiST coherence without inferring `conperiod`.
- `5183704353 -> 39bdccbb9d3cf8a26f46ba05f6ce59896f390f06 -> 69d4c734c5954e3ccf37b6965ca65ff238aa45b7 -> 3f2ecba28fdd5742de5af1cac68227767baadbed`: explicit `pg_constraint.conperiod` family.
- `5184007447 -> 03e4443b5834383f4d25a8e83786cccb62e003be -> 1a77e006553a39e3752ee3e9f08c57e9160dac78 -> 5184074266`: PERIOD-FK action evidence and correction chain.
- `5184299133 -> 6c77cb6fb1664cd7ffeb517ad7bcd85382ebd825 -> fa21b47653192af83627ac77d14c9141f4419cbd -> 194612f3ea5586484980f25f51cba133b5b187d1`: referenced temporal key exact NOT DEFERRABLE evidence.
- `5184492366 -> ccf1a0558389dcf6b7d5af456c83f58f018a8704 -> 635a9ea05af9daffd78a469bc72e69df4633317e -> 67da1f9d7ac34e9aa75eb92696c7ec695553e481 -> 8581766af558a596c8548fde11742bea829ce90a`: constraint evidence relation-kind coordinates.
- `5184685945 -> e367b81bd1af07ea28f3a044a92e8dd39239048e -> d60de513d559c6359d55149ddc8707d1520cd6b0 -> 5184707338`: PostgreSQL 18 `MATCH PARTIAL` fail-closed boundary.
- `5184917000 -> ef86be21c06477d210a9725874f10a969f4f37a9 -> 5185079749 -> 375242bb02cc165a410e72e318f9abec712764bf -> 7bfe5eb46a38e3a4bcfe91aac2688b956526b8b7 -> 22b6577979a03e2622648d5a63851f1ec9cfe2ba`: temporal final-column type/domain/range evidence.
- `5185362949 -> ef9c8d61d0864e7b867902746969ea4d33592407 -> e96367b2944ce3bb33f2b33be04dd36bb0f84e33 -> 5185490815`: array/type-kind observed-family composition.
- `5185657110 -> 670fd280d32a1fcfa2cf776bdd779812e10d31bf -> 058b434640816a36914923803b6c4946733ec483 -> 5185671332`: exact Base binding without array/temporal promotion.
- `5185712815 -> c166247a2ae0604a81f315994a66b199dd98a183 -> 050a2ad679bf2246046638be1e8b89692ee8c251 -> 5185723369 -> d377b3a558e85e4e35376db5a0532d1590b45546`: request-authorized cross-schema type-kind evidence.
- `5185780899 -> c80d07816863f814e9b8fbb716661310d8b2b150 -> bfbce8b040a411c89bd9cba3d19b9b5b14a84da6 -> 3c38cfb1d2c1134dc30a19c511379969428c530b -> 43126e4b8b53415b0c4547b532b89ff88291ed71 -> 5185928607`: temporal PK/UNIQUE backing-index presence/material-flags/GiST-exclusion enforcement.

## Acceptance still required

Before any Ready/adoption/merge claim, one unchanged exact #46 successor must produce all applicable evidence:

- repository-pinned Rust 1.98 `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests, frozen-v2 regressions, and retained v3 contracts including `constraint_period_backing_index_presence_contract`, `constraint_period_contract`, `constraint_period_action_contract`, `constraint_period_reference_timing_contract`, `constraint_period_type_contract`, `constraint_timing_contract`, `constraint_backing_index_contract`, `constraint_backing_index_shape_contract`, `constraint_temporal_index_contract`, `foreign_key_match_contract`, `constraint_catalog_relation_kind_contract`, `array_type_kind_composition_contract`, true-array identity/digest/schema/receipt contracts, and primary-key invariants;
- rustdoc/doc tests, release build, and owned production docstring/test/edge-case coverage;
- applicable Product/security/dependency/review workflows terminal on the same exact head.

No predecessor GREEN transfers. Draft status, bot-only status, mechanical mergeability, or manual/no-op reruns are not acceptance evidence.

## Concrete PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must:

- use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate and passing dependency/SBOM review;
- resolve least-privilege credentials only for the authorized source key and immutable policy binding;
- use one explicit `REPEATABLE READ READ ONLY` catalog transaction with bounded connect/query/cancellation time; do not hold an explicit transaction/lock while waiting on LLM or long external computation;
- use catalog OIDs only for adapter-local joins, then cross the ACL with exact names/coordinates;
- collect complete `pg_type`, `pg_range`, `pg_class`, `pg_index`, and `pg_constraint` families needed by the released representation in one coherent bounded snapshot;
- preserve exact qualified type coordinates and request-authorized schema scope without `search_path` or unrelated-object authorization proxies;
- preserve exact PK/UNIQUE timing, `conindid` support relationships, `conperiod`, FK action/match values, key/index role/shape, and exclusion/GiST state from direct catalog evidence;
- for every represented `conperiod=true` PK/UNIQUE, resolve the same-name backing index and material `pg_index` flags and require `indisexclusion=true` plus GiST before crossing the ACL; never synthesize `conperiod` from those facts;
- resolve temporal final constrained types through explicit type/domain/range evidence rather than names, display text, GiST, or reconstructed DDL;
- reject unsupported `confmatchtype='p'` in governed PostgreSQL 18 evidence while preserving SIMPLE/FULL exactly;
- enforce policy-admitted row/byte/concurrency ceilings and complete-or-fail snapshot construction.

## Standards and primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE* — PRIMARY KEY/UNIQUE, `WITHOUT OVERLAPS`, PERIOD foreign keys, match types, referential actions, deferrability, and generated supporting indexes.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint* — `condeferrable`, `condeferred`, `conindid`, `conperiod`, `confupdtype`, `confdeltype`, `confmatchtype`.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index* — `indisunique`, `indisprimary`, `indisexclusion`, `indimmediate`, `indnullsnotdistinct`, `indkey`, `indpred`, and index state.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_type / pg_range / pg_class / CREATE INDEX*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes* — domains over range/multirange for `WITHOUT OVERLAPS`.

Catalog OIDs are adapter-local joins, never governed semantic identity. `pg_get_indexdef`/`pg_get_expr` are reconstructed provenance text, never the sole semantic carrier.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams unchanged. |
| Truth/publication lifecycle | SOURCE_REPAIR_ACCEPTANCE_PENDING_NO_PUBLICATION | No protected immutable semantic release exists. |
| Source Observation | TEMPORAL_BACKING_INDEX_SOURCE_REPAIRED | RED `c80d078...`; production repair `43126e4...`; exact-head native/Product acceptance still required. |
| Product CI | BLOCKED_OWNER_ACCEPTANCE | #35 still depends on central protected `.github` workflow settlement. |
| Quality gate | NATIVE_ACCEPTANCE_REQUIRED | No Ready/adoption/merge before one unchanged head passes Rust 1.98 and hosted gates. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| PostgreSQL 18 temporal keys | SOURCE_REPAIRED_ACCEPTANCE_PENDING | `conperiod` remains declaration authority; temporal backing-index evidence is now fail-closed. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Keep #46 Draft and acquire one unchanged exact-head Rust 1.98/Product/security/dependency/review acceptance for the repaired temporal backing-index invariant.
2. If a real failure appears, ordinary-forward repair only that causal failure and restart exact-head acceptance; no predecessor GREEN transfers.
3. Ordinary/non-force adopt the complete verified #46 delta into #45 and obtain fresh parent acceptance; then adopt #45 into #6.
4. In parallel, the central `.github` owner must obtain fresh exact-head hosted/security/review evidence for its current-main reconciliation and land it normally before unchanged #35 can obtain Product acceptance.
5. Only after representation/adapter prerequisites are GREEN may the bounded PostgreSQL adapter and frozen conformance fixture be implemented, followed by discovery/alignment/deterministic validation/independent evaluation/steward review/immutable publication under canonical owner boundaries.

Adapters remain outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence.
