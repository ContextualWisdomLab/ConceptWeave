# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-cost surface through exact head `ef4657347a605a1a5acbfac8500fadf8142ae9b9` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-ef465734.md`; its matching changelog is preserved at `docs/archive/CHANGELOG-through-ef465734.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The current behavior-bearing converter-kind successor is `75028d1c8c18a490f793f8fcde5061ec39c52158`: review `5251421078` identified missing converter `pg_proc.prokind`, RED `f60aa85fbf36ffb8ab6354afc4826d347ba8ea44` first added the missing public contract, production `b75b3f008c0baeeae1b5e5eb3fd421b14e0d1205` added converter-kind behavior, and `75028d1c...` publicly composed it through `index_partition.rs`. Edge-contract successor `98149497113369b1f8a95c7f00bb0800a854a56f` additionally pins successor-domain separation and both blank converter identifier branches.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains. The active chain binds exact constraint/backing-index identity, ordered keys and exclusion operators, target-procedure scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, converter definition, converter owner, converter object-level `EXECUTE` ACL, converter nullable `proconfig`, converter raw `prosecdef`, converter raw `proleakproof`, converter raw `proisstrict`, converter raw `provolatile`, converter raw `proparallel`, converter `prosupport` absence or exact resolved support-function identity, converter raw `procost` float4 bits, and now converter raw `prokind='f'` evidence.

Quoted PostgreSQL transform-type identifiers remain collision-safe: schema and type-name components are percent-encoded independently before the canonical `.` separator. Valid pairs such as (`payload.domain`, `json`) and (`payload`, `domain.json`) therefore cannot collapse to one provenance location.

## Transform-converter routine-kind integrity

PostgreSQL 18 stores routine kind independently in `pg_proc.prokind`: `f` for a normal function, `p` for a procedure, `a` for an aggregate, and `w` for a window function. PostgreSQL's `check_transform_function()` separately requires `prokind == PROKIND_FUNCTION`, rejects set-returning converters, requires exactly one argument, and requires that argument to be `internal`. ConceptWeave's converter base already normalizes the required call boundary, but normalized signature/definition evidence is not a substitute for the raw same-generation routine-kind field.

`IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot` is a structural observational successor over `IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot`. Every exact predecessor `(constraint coordinate, key position, transform type, direction)` must receive one observation with matching converter schema/function identity and raw `prokind='f'`. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, non-function routine kinds, and unknown receipt coordinates fail closed. The successor uses its own digest domain even though all admitted observations carry `f`.

Traceability:

- finding review: `5251421078`;
- RED: `f60aa85fbf36ffb8ab6354afc4826d347ba8ea44`;
- production implementation: `b75b3f008c0baeeae1b5e5eb3fd421b14e0d1205`;
- public-composition successor: `75028d1c8c18a490f793f8fcde5061ec39c52158`;
- edge-contract successor: `98149497113369b1f8a95c7f00bb0800a854a56f`;
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_kind.rs`;
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_kind_contract.rs`;
- doctoring: `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-kind-integrity.md`.

Primary authority is PostgreSQL 18 `pg_proc`, `CREATE TRANSFORM`, and `src/backend/commands/functioncmds.c::check_transform_function()`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage must be demonstrated on the final exact head. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must retain all predecessor ordinary-EXCLUDE evidence and resolve every nonzero selected transform converter to the same exact-generation `pg_proc` row, independently capturing definition, owner/ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, `prosupport`, raw `procost`, and raw `prokind`. Any converter direction whose resolved row is not `prokind='f'` is capture failure. Missing resolution, mixed-generation joins, inferred routine kind, or signature-only normalization is also capture failure.

## Residual material gap

Fresh PostgreSQL source review found the next structural transform-converter fact immediately adjacent to `prokind`: `check_transform_function()` independently rejects `pg_proc.proretset=true`. `proretset=false` is not encoded by the converter-kind successor and must not be inferred from scalar return-type normalization. It remains the next explicit Source Observation review target. The same checker also verifies argument count and the `internal` argument type; the current base converter contract represents one `internal` argument but a fresh review must confirm extractor-level `pronargs` evidence is not being normalized away before declaring that portion complete.

Publication and immutable semantic release remain unauthorized until exact-head Rust/coverage acceptance, the bounded PostgreSQL 18 differential, central workflow prerequisites, and downstream stack adoption are complete.

## Canonical prerequisite state

Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise. The branches are materially diverged and the shared scheduler changed on both sides. #2040 production remains repository-identity RED; protected main is only partially hardened.

The next valid central movement is ordinary/non-force path-wise protected-main reconciliation. The resolved scheduler must preserve #2040's v2 CodeQL producer, removal of source-neutral restamps, repository-scoped Actions credential/selected-token proof, stale-run revalidation, rationale/docstrings/tests; adopt compatible protected-main queue/coalescing/capacity behavior; and enforce the stronger repository-identity invariant.

Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their hosted evidence remains independent and does not transfer into #2040. Source-fix #2175 remains exact `6e5f75dd40a428d174f691e35e210d08a29eb270`; it intentionally cannot self-modify `.github/` or `scripts/ci/` control-plane paths.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its Security Scan and SAST success do not override terminal CodeQL PR failure; no unchanged-head manual rerun or leaf-source workaround is authorized.

## Required order

`.github#2040` path-wise reconciliation with repository-identity repair -> focused scheduler GREEN -> fresh exact-head central terminal checks plus qualifying non-self approval -> fresh compatible #35 acceptance and normal landing -> exact #46 Rust 1.98 native/hosted GREEN -> bounded PostgreSQL 18 differential including converter `prosupport`, exact `procost`, and raw `prokind='f'` -> converter `proretset=false` review -> fresh next semantic-gap review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.