# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-planner-support surface through exact head `913577072165cb09fbf52bbdb2e1216fa2b21005` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-91357707.md`; its matching changelog is preserved at `docs/archive/CHANGELOG-through-91357707.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The current behavior-bearing converter-cost successor is `5032b9cbeacd784f6b36d1393476a24b00f7529a`: review `5250941523` identified missing converter `pg_proc.procost`, RED `c876bb3907f9d33da3973196937fd854ac3aae85` first added the missing public contract, production `f82ca656ac238c1ce7015cddd327169abd7ce054` added converter-cost behavior, and `5032b9cb...` publicly composed it through `index_partition.rs`.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains. The active chain binds exact constraint/backing-index identity, ordered keys and exclusion operators, target-procedure scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, converter definition, converter owner, converter object-level `EXECUTE` ACL, converter nullable `proconfig`, converter raw `prosecdef`, converter raw `proleakproof`, converter raw `proisstrict`, converter raw `provolatile`, converter raw `proparallel`, converter `prosupport` absence or exact resolved support-function identity, and now converter raw `procost` `float4` bits.

Quoted PostgreSQL transform-type identifiers remain collision-safe: schema and type-name components are percent-encoded independently before the canonical `.` separator. Valid pairs such as (`payload.domain`, `json`) and (`payload`, `domain.json`) therefore cannot collapse to one provenance location.

## Transform-converter planner-cost integrity

PostgreSQL 18 defines `pg_proc.procost` as a `float4` estimated execution cost in units of `cpu_operator_cost`, with per-row semantics for set-returning functions. `CREATE FUNCTION` requires `COST execution_cost` to be positive and documents language-dependent defaults when it is omitted. Section 36.11 distinguishes this declarative constant cost from optional non-constant cost estimates supplied by a planner support function. Converter planner-support identity therefore cannot stand in for the same-row `procost` catalog fact.

`IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot` is an observational successor over `IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot`. Each positive finite PostgreSQL `float4` value is retained by raw `f32` bits and domain-separated in the successor digest; no product-specific cost threshold is inferred. Every exact predecessor `(constraint coordinate, key position, transform type, direction)` must receive one observation with matching converter schema/function identity. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, non-positive/non-finite costs, and unknown receipt coordinates fail closed.

Traceability:

- finding review: `5250941523`;
- RED: `c876bb3907f9d33da3973196937fd854ac3aae85`;
- production implementation: `f82ca656ac238c1ce7015cddd327169abd7ce054`;
- public-composition successor: `5032b9cbeacd784f6b36d1393476a24b00f7529a`;
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_cost.rs`;
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_cost_contract.rs`;
- doctoring: `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-cost-integrity.md`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage must be demonstrated on the final exact head. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must retain all predecessor ordinary-EXCLUDE evidence and resolve every nonzero selected transform converter to the same exact-generation `pg_proc` row, independently capturing definition, owner/ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, `prosupport`, and exact raw `procost`. A nonzero `prosupport` must resolve to the exact support-function call identity from that generation; `procost` must be captured as the actual catalog `float4` rather than reconstructed from language defaults. Missing resolution, mixed-generation joins, name-only support resolution, inferred support/cost, or presentation rounding that changes cost identity is capture failure.

## Residual material gap

The named converter-function fact sequence reviewed in this Source Observation lane now includes definition, owner, ACL, local configuration, security-definer state, leakproof state, strictness, volatility, parallel safety, planner-support identity, and planner cost. No additional named converter successor from this review sequence is currently outstanding. That is not a claim that all future PostgreSQL/catalog semantics are exhausted: the next semantic gap must be derived from a fresh code/catalog review, not presumed closed.

Publication and immutable semantic release remain unauthorized until exact-head Rust/coverage acceptance, the bounded PostgreSQL 18 differential, central workflow prerequisites, and downstream stack adoption are complete.

## Canonical prerequisite state

Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise. The branches are materially diverged and the shared scheduler changed on both sides. #2040 production remains repository-identity RED; protected main is only partially hardened.

The next valid central movement is ordinary/non-force path-wise protected-main reconciliation. The resolved scheduler must preserve #2040's v2 CodeQL producer, removal of source-neutral restamps, repository-scoped Actions credential/selected-token proof, stale-run revalidation, rationale/docstrings/tests; adopt compatible protected-main queue/coalescing/capacity behavior; and enforce the stronger repository-identity invariant.

Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their hosted evidence remains independent and does not transfer into #2040. Source-fix #2175 remains exact `6e5f75dd40a428d174f691e35e210d08a29eb270`; it intentionally cannot self-modify `.github/` or `scripts/ci/` control-plane paths.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its Security Scan and SAST success do not override terminal CodeQL PR failure; no unchanged-head manual rerun or leaf-source workaround is authorized.

## Required order

`.github#2040` path-wise reconciliation with repository-identity repair -> focused scheduler GREEN -> fresh exact-head central terminal checks plus qualifying non-self approval -> fresh compatible #35 acceptance and normal landing -> exact #46 Rust 1.98 native/hosted GREEN -> bounded PostgreSQL 18 differential including converter `prosupport` and exact `procost` -> fresh next semantic-gap review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.