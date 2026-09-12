# Product / Technical Gap Baseline

**Snapshot:** 2026-09-12

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, reviews, checks, and runs are evidence coordinates, not mutable dependencies. Evidence from an earlier PR head does not transfer after head movement unless the successor reproduces it.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. Source-system business truth remains with its canonical owner.

`semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, `contextual-orchestrator` owns production LLM/provider routing, and Keyverse owns authentication/identity trust evidence. Consumers use only released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Live stack and protected prerequisites

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, active Source Observation successor.

#46 stays Draft until one unchanged exact head has repository-pinned Rust 1.98 plus applicable Product/security/dependency/review terminal evidence. #45/#6 must ordinary/non-force adopt the complete verified child delta only after #46 exact-head GREEN; partial cherry-picks or duplicate fixes are not valid succession.

Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable. Its central workflow prerequisite has advanced to `.github#2040@85522306949bada2b5939608dc911f6374125f1b` on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`. #2040 remains Draft because current-head CodeQL is not terminal GREEN; ConceptWeave must not copy, weaken, synthetically satisfy, or manually retrigger the central owner gate.

## Source Observation bounded context

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, exact evidence locations, and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority, or foreign product truth.

Historical v2 evidence is frozen. PostgreSQL successor facts are additive and domain-separated. Catalog OIDs are capture-local join coordinates, never governed semantic identity.

## PostgreSQL 18 representation-v3 state

The active successor preserves these source-authoritative invariants:

- relation-kind-aware exact coordinates and schema-local `pg_class` namespace consistency;
- exact `pg_type` namespace, true-array `typarray`/`typelem`, direct type kind, domain base, and reciprocal range/multirange evidence without underscore-name, display-text, OID, or `search_path` inference;
- independent array/type-kind families compose without inventing true-array or temporal-range semantics;
- cross-schema Base/Range/Multirange evidence is admitted only inside the observation request's authorized schemas;
- relation-scoped index key/`INCLUDE` shape, expression keys, collation/operator-class/opaque `indoption`, operator-class parameters, `NULLS NOT DISTINCT`, material `pg_index` flags, reloptions, tablespace, access method, and provenance remain explicit;
- represented constraints are restricted to PostgreSQL relation kinds that can own them; PK cardinality and non-nullability remain fail-closed;
- PRIMARY KEY/UNIQUE timing comes directly from `pg_constraint.condeferrable`/`condeferred` and is checked against exact same-name supporting-index role/shape/`indimmediate`/key order/non-partial/null treatment/exclusion-GiST evidence;
- `pg_constraint.conperiod` is the declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/operator shape never invents it;
- temporal final columns resolve through observed type/domain-base evidence to range or multirange, including PostgreSQL 18.4+ domains over range/multirange;
- governed PostgreSQL 18 FK evidence admits `MATCH SIMPLE`/`MATCH FULL` and rejects unimplemented `MATCH PARTIAL` while preserving frozen historical vocabulary;
- PERIOD FKs require valid shape, `NO ACTION`/`NO ACTION`, an in-snapshot referenced `WITHOUT OVERLAPS` PK/UNIQUE on exact columns, and exact referenced-key `NOT DEFERRABLE` timing when the target is represented locally.

## Current temporal exclusion-operator repair

Concurrent ordinary-forward work after the previous baseline added resolved `pg_constraint.conexclop` signatures to `ConstraintPeriodObservation`, including position, operator namespace/name, and qualified binary operand types. That delta is adopted rather than rewritten.

Finding review `5186120516` identified a remaining P1: the operator family enforced presence, contiguous positions, arity, and digest materiality but admitted arbitrary operator names. The regression itself treated a prefix `=#` operator as a coherent `WITHOUT OVERLAPS` temporal key.

PostgreSQL 18 defines `UNIQUE/PRIMARY KEY (..., valid_at WITHOUT OVERLAPS)` as exclusion semantics equivalent to prefix key columns `WITH =` and the final range/multirange column `WITH &&`. `pg_constraint.conexclop` is the per-column operator vector. Therefore operator arity alone is insufficient source validation.

Causal lineage:

- finding review `5186120516` on #46 exact predecessor `6679cd3c29d6457a644dec358ea8f9003cca8ee5`;
- behavioral RED `8b5c3ba74fc9f8dfb0c8f6500634d62c8feaf14f` in `constraint_period_exclusion_operator_contract.rs`;
- production repair `2333d7f7640931ab158734e12dacd8729aa9983f` in `constraint_period.rs`;
- regression-boundary adjustment `5fa8112359a28850e73cf6912223b883d2c7f21d`;
- primary-source doctoring `ef289379d842f366674bc4ba54306df7b5a7cd61` at `docs/doctoring/source-observation-without-overlaps-exclusion-operator-semantics.md`.

The repaired value-object boundary requires every non-final resolved operator name to be `=` and the final operator name to be `&&`. Namespace and qualified operand types remain retained/digested evidence because operator names are overloadable. This validation does not infer `conperiod`, does not collapse OIDs into governed identity, and does not hard-code operator namespace.

The immediately preceding backing-index repair remains retained: `5185780899 -> c80d07816863f814e9b8fbb716661310d8b2b150 -> 3c38cfb1d2c1134dc30a19c511379969428c530b -> bfbce8b040a411c89bd9cba3d19b9b5b14a84da6 -> 43126e4b8b53415b0c4547b532b89ff88291ed71 -> 5185928607`. A represented `conperiod=true` PK/UNIQUE therefore also requires its same-name index, material catalog flags, `indisexclusion=true`, and exact observed `gist` access method; ordinary `conperiod=false` keys do not acquire mandatory index-family observation.

This lane is **source-repaired / exact-head native-and-Product-acceptance-pending**. It is not Ready, merged, adopted by #45/#6, transported to PostgreSQL, published, or released.

## Acceptance still required

Before Ready/adoption/merge, one unchanged exact #46 successor must produce:

- repository-pinned Rust 1.98 `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace and doc tests, including `constraint_period_exclusion_operator_contract`, `constraint_period_backing_index_presence_contract`, retained period/action/reference/type/timing/index contracts, frozen-v2 regressions, and array/type-kind contracts;
- release build and owned production docstring/test/edge-case coverage;
- applicable Product/security/dependency/review workflows terminal on the same exact head.

Draft state, bot-only status, mechanical mergeability, predecessor GREEN, manual/no-op reruns, or synthetic statuses are not acceptance evidence.

## Concrete PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate; obtain least-privilege credentials only through the authorized source/policy binding; execute bounded catalog capture in one `REPEATABLE READ READ ONLY` transaction; and never hold an explicit database transaction/lock while waiting on LLM or long external computation.

Catalog OIDs may be used only for adapter-local joins. The ACL must cross with exact qualified names and complete bounded evidence from `pg_type`, `pg_range`, `pg_class`, `pg_index`, `pg_constraint`, and the operator catalogs needed to resolve `conexclop`. For represented `conperiod=true` keys the adapter must preserve the complete per-column operator vector, resolve each OID to durable namespace/name/type evidence, and fail closed unless prefix operators are equality and the final temporal operator is overlap. It must also retain the same-name GiST/exclusion backing index, temporal type/domain chain, exact constraint timing/action/match evidence, and policy-admitted row/byte/concurrency ceilings. Reconstructed DDL is provenance text, never the sole semantic carrier.

## Standards and primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE* — `WITHOUT OVERLAPS` behaves as `EXCLUDE USING GIST` with `=` on non-temporal key columns and `&&` on the final range/multirange column; generated supporting-index behavior and PERIOD FK semantics.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint* — direct `conperiod` and per-column `conexclop` catalog evidence, timing/action/match coordinates.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index / pg_type / pg_range / pg_class / CREATE INDEX*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes* — domains over range/multirange for `WITHOUT OVERLAPS`.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner seams unchanged. |
| Source Observation | TEMPORAL_OPERATOR_SOURCE_REPAIRED | `5186120516 -> 8b5c3ba... -> 2333d7f... -> 5fa8112... -> ef28937...`; exact-head native/Product acceptance required. |
| Product CI | BLOCKED_OWNER_ACCEPTANCE | #35 unchanged; `.github#2040@8552230...` remains Draft with non-GREEN CodeQL settlement. |
| Quality gate | NATIVE_ACCEPTANCE_REQUIRED | No Ready/adoption/merge before one unchanged head passes Rust 1.98 and hosted gates. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| Truth/publication lifecycle | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Keep #46 Draft and obtain one unchanged exact-head Rust 1.98/Product/security/dependency/review acceptance for the repaired temporal operator and backing-index invariants.
2. If a real failure appears, ordinary-forward repair only that causal failure and restart exact-head acceptance.
3. Ordinary/non-force adopt the complete verified #46 delta into #45 and obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, the central `.github` owner must land its backward-compatible CodeQL bootstrap/settlement normally before unchanged #35 can obtain Product acceptance.
5. Only after representation and Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by discovery/alignment/deterministic validation/independent evaluation/steward review/immutable publication under canonical owner boundaries.
