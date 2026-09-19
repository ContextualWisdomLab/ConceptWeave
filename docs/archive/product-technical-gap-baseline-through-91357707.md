# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-parallel-safety surface through exact head `1fbbfadec507999710a8ca5c00378685d017326a` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-1fbbfade.md`; its matching changelog is preserved at `docs/archive/CHANGELOG-through-1fbbfade.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The current behavior-bearing planner-support successor is `9bbb668767ddaa24f6934568a7340ef1cbc504ad`: review `5250887046` identified missing converter `pg_proc.prosupport`, RED `f7a92af4d18e964680f514514020b712942ffc49` first added the missing public contract, production `3b048dae7cefed4c1615d5bfe5659be57d044eb9` added converter planner-support behavior, and `9bbb668...` publicly composed it through `index_partition.rs`.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains. The active chain binds exact constraint/backing-index identity, ordered keys and exclusion operators, target-procedure scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, converter definition, converter owner, converter object-level `EXECUTE` ACL, converter nullable `proconfig`, converter raw `prosecdef`, converter raw `proleakproof`, converter raw `proisstrict`, converter raw `provolatile`, converter raw `proparallel`, and now converter `prosupport` absence or exact resolved support-function identity.

Quoted PostgreSQL transform-type identifiers remain collision-safe: schema and type-name components are percent-encoded independently before the canonical `.` separator. Valid pairs such as (`payload.domain`, `json`) and (`payload`, `domain.json`) therefore cannot collapse to one provenance location.

## Transform-converter planner-support integrity

PostgreSQL 18 defines `pg_proc.prosupport` as a `regproc` reference to another `pg_proc` row, with zero meaning no planner support function. `CREATE FUNCTION` exposes `SUPPORT support_function` independently from volatility, strictness, security mode, parallel safety, cost, and configuration. Section 36.11 further defines the attached support function as planner knowledge for the target function rather than a fact derived from the target's other routine attributes.

`IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot` is an observational successor over `IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot`. Support absence and exact support-function identity are both representable and domain-separated; no support-required policy is inferred. Every exact predecessor `(constraint coordinate, key position, transform type, direction)` must receive one observation with matching converter schema/function identity. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, and unknown receipt coordinates fail closed.

The support function is stored as a qualified call signature rather than an unqualified name. That preserves namespace and overload identity while matching PostgreSQL's callable-function identity model. Cost remains outside this successor because `pg_proc.procost` is another independently mutable function fact.

Traceability:

- finding review: `5250887046`;
- RED: `f7a92af4d18e964680f514514020b712942ffc49`;
- production implementation: `3b048dae7cefed4c1615d5bfe5659be57d044eb9`;
- public-composition successor: `9bbb668767ddaa24f6934568a7340ef1cbc504ad`;
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_planner_support.rs`;
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_planner_support_contract.rs`;
- doctoring: `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-planner-support-integrity.md`.

## Acceptance boundary

**Source repaired is not GREEN.** The first pull-request workflow inventory query for exact `9bbb668...` returned no runs. Repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage have not been demonstrated on the current successor lineage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must retain all predecessor ordinary-EXCLUDE evidence and resolve every nonzero selected transform converter to the same exact-generation `pg_proc` row, independently capturing definition, owner/ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, and `prosupport`. A nonzero `prosupport` must resolve to the exact support-function call identity from that generation. Missing resolution, mixed-generation joins, name-only support resolution, or inferred support is capture failure.

## Residual material gap

Converter cost is the remaining independently mutable converter-function fact in the current review sequence. It remains a separate review-gated successor rather than being inferred from planner support or folded into its digest domain. No publication or immutable semantic release is authorized from this Draft stack before exact-head acceptance and the bounded PostgreSQL 18 differential.

## Canonical prerequisite state

Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise. The branches are materially diverged and the shared scheduler changed on both sides. #2040 production remains repository-identity RED; protected main is only partially hardened.

The next valid central movement is ordinary/non-force path-wise protected-main reconciliation. The resolved scheduler must preserve #2040's v2 CodeQL producer, removal of source-neutral restamps, repository-scoped Actions credential/selected-token proof, stale-run revalidation, rationale/docstrings/tests; adopt compatible protected-main queue/coalescing/capacity behavior; and enforce the stronger repository-identity invariant.

Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their hosted evidence remains independent and does not transfer into #2040. Source-fix #2175 remains exact `6e5f75dd40a428d174f691e35e210d08a29eb270`; it intentionally cannot self-modify `.github/` or `scripts/ci/` control-plane paths.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its Security Scan and SAST success do not override terminal CodeQL PR failure; no unchanged-head manual rerun or leaf-source workaround is authorized.

## Required order

`.github#2040` path-wise reconciliation with repository-identity repair -> focused scheduler GREEN -> fresh exact-head central terminal checks plus qualifying non-self approval -> fresh compatible #35 acceptance and normal landing -> exact #46 Rust 1.98 native/hosted GREEN -> bounded PostgreSQL 18 differential including converter `prosupport` -> review converter cost -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
