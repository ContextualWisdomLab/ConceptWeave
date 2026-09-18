# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-volatility surface through exact source head `f7593e834660efa56e3331c99bcf15d908cd46bc` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-f7593e83.md`; its matching changelog is preserved at `docs/archive/CHANGELOG-through-f7593e83.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The current behavior-bearing parallel-safety successor is `6373c77652f63dae874e2b4bdb881b54f5680bfe`: review `5250289810` identified the missing raw converter `pg_proc.proparallel`, RED `0fc0f567386d39b848444879a41c4877e9f996b6` first added the missing public contract, production `66846c34d668202a6f8fbbdfb3658fba908aec0d` added raw converter-parallel-safety behavior, and `6373c776...` publicly composed it through `index_partition.rs`.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains. The active chain binds exact constraint/backing-index identity, ordered keys and exclusion operators, target-procedure scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, converter definition, converter owner, converter object-level `EXECUTE` ACL, converter nullable `proconfig`, converter raw `prosecdef`, converter raw `proleakproof`, converter raw `proisstrict`, converter raw `provolatile`, and now converter raw `proparallel`.

Quoted PostgreSQL transform-type identifiers remain collision-safe: schema and type-name components are percent-encoded independently before the canonical `.` separator. Valid pairs such as (`payload.domain`, `json`) and (`payload`, `domain.json`) therefore cannot collapse to one provenance location.

## Transform-converter parallel-safety integrity

PostgreSQL 18 stores `pg_proc.proparallel` independently from volatility and other routine attributes. `CREATE FUNCTION` exposes `PARALLEL SAFE`, `PARALLEL RESTRICTED`, and `PARALLEL UNSAFE` independently, and PostgreSQL's parallel-safety documentation states that the planner cannot derive the correct label for arbitrary user-defined functions. A transform binds converter functions to type/direction semantics but does not make parallel safety part of the transform coordinate. Converter definition, owner, ACL, local configuration, security-definer state, leakproof state, strictness, and volatility therefore cannot stand in for raw same-row `proparallel`.

`IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot` is an observational successor over `IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot`. Catalog states `s`, `r`, and `u` are all representable and domain-separated; no parallel-safe-only transform policy is inferred. Every exact predecessor `(constraint coordinate, key position, transform type, direction)` must receive one observation with matching converter schema/function identity. Missing or extra coordinates, duplicates, binding drift, invalid parallel-safety discriminators, zero positions, blank converter identifiers, and unknown receipt coordinates fail closed.

Traceability:

- finding review: `5250289810`;
- RED: `0fc0f567386d39b848444879a41c4877e9f996b6`;
- production implementation: `66846c34d668202a6f8fbbdfb3658fba908aec0d`;
- public-composition successor: `6373c77652f63dae874e2b4bdb881b54f5680bfe`;
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety.rs`;
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_contract.rs`;
- doctoring: `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-parallel-safety-integrity.md`.

## Acceptance boundary

**Source repaired is not GREEN.** The first pull-request workflow inventory query for exact `6373c776...` returned no runs. Repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage have not been demonstrated on the current successor lineage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must retain all predecessor ordinary-EXCLUDE evidence and resolve every nonzero selected transform converter to the same exact-generation `pg_proc` row, independently capturing definition, owner/ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, and raw `proparallel`. Missing resolution, mixed-generation joins, inferred parallel safety, or coercing every converter to parallel safe is capture failure.

## Residual material gap

Converter planner support and cost remain independently mutable converter-function facts that are not yet governed by the converter successor chain. They remain separate review-gated successors; neither is inferred from parallel safety or folded into its digest domain. No publication or immutable semantic release is authorized from this Draft stack before exact-head acceptance and the bounded PostgreSQL 18 differential.

## Canonical prerequisite state

Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise. The branches are materially diverged and the shared scheduler changed on both sides. #2040 production remains repository-identity RED; protected main is only partially hardened.

The next valid central movement is ordinary/non-force path-wise protected-main reconciliation. The resolved scheduler must preserve #2040's v2 CodeQL producer, removal of source-neutral restamps, repository-scoped Actions credential/selected-token proof, stale-run revalidation, rationale/docstrings/tests; adopt compatible protected-main queue/coalescing/capacity behavior; and enforce the stronger repository-identity invariant.

Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their hosted evidence remains independent and does not transfer into #2040. Source-fix #2175 remains exact `6e5f75dd40a428d174f691e35e210d08a29eb270`; it intentionally cannot self-modify `.github/` or `scripts/ci/` control-plane paths.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its Security Scan and SAST success do not override terminal CodeQL PR failure; no unchanged-head manual rerun or leaf-source workaround is authorized.

## Required order

`.github#2040` path-wise reconciliation with repository-identity repair -> focused scheduler GREEN -> fresh exact-head central terminal checks plus qualifying non-self approval -> fresh compatible #35 acceptance and normal landing -> exact #46 Rust 1.98 native/hosted GREEN -> bounded PostgreSQL 18 differential including converter `proparallel` -> review converter planner support -> cost -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
