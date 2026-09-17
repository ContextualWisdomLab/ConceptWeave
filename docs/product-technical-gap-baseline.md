# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline and CHANGELOG through exact `781f8d57c83c722596fd5f3759f54dc6a001a803` are preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-781f8d57.md` and `docs/archive/CHANGELOG-through-781f8d57.md`. Earlier surfaces remain under `docs/archive/`; focused decision/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and its released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner. Consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative; no issued predecessor digest domain is rewritten. For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained source identity includes the independent constraint and exact backing index; timing/enforcement/validation/period/no-inherit state; ordered `conkey`/`conexclop`; `pg_operator.oprkind`, independently resolved `oprcom`, exact `oprcode -> pg_proc`, and independent Boolean `oprresult`/`prorettype`; target-function `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prokind`, `prosecdef`, `proleakproof`; implementation language/definition (`prolang`, `prosrc`, `probin`, `prosqlbody`); exact `proowner`; exact nullable `proconfig`; exact `proacl` with same-generation ACL role resolution; exact optional `prosupport`; exact positive-finite `procost`; operator-family/strategy; constraint/index namespace/name coupling; access-method exclusion capability; backing-index role/lifecycle; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain separate owner families and are not reclassified as ordinary EXCLUDE.

## EXCLUDE implementation-function transform-type integrity

Review `5232628816` on exact predecessor `781f8d57c83c722596fd5f3759f54dc6a001a803` found the next bounded P1: the governed target-function chain omitted independent nullable `pg_proc.protrftypes`.

PostgreSQL 18.6 defines `protrftypes` as the nullable array of argument/result data-type OIDs for which a routine applies transforms selected by its `TRANSFORM FOR TYPE` clause. `CREATE FUNCTION` defines those transforms as conversions between SQL types and language-specific data types. Target signature, implementation language/source, ACL, planner support, and planner cost therefore do not prove equivalent transform selection.

The repair adds `IndexExclusionConstraintOperatorProcedureTransformTypesObservation`, `IndexExclusionConstraintOperatorProcedureTransformTypesSourceReceipt`, and `IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot` over the exact planner-cost predecessor. The observation preserves either PostgreSQL's documented NULL state or a nonempty set of same-generation resolved `QualifiedTypeName` values. Non-null types are canonicalized by qualified name for deterministic set identity; duplicate types and an explicit empty array fail closed. Distinct NULL/selection states and distinct selected type sets produce distinct successor digests.

This successor intentionally does not authenticate the mutable `pg_transform` converter row. `pg_transform` independently binds a type/language pair to optional `trffromsql` and `trftosql` functions. Exact converter-function identity and implementation evidence is a separate material gap and must not be inferred from `protrftypes` alone.

Transform-type lineage:

- finding review `5232628816`;
- structural source/compile RED `6c2aa9f3470c00a4fdd34ed6025a35640455ca58`; the public transform-type types did not yet exist, so no executed compiler failure is claimed;
- RED contract refinement `92d8141e102c33a18b89c05a586ee8c2054efbcd`, preserving NULL explicitly and rejecting an explicit empty array;
- production successor `e12b961179a9b22ca9429eeaf1844d8fe8563323`;
- public composition `ba11bd4333003b824a4128d32782a19a7a02a0ca`;
- pre-transform-type active baseline archive `bb8a93cddadb7b1e25bc19f537a98632619dd9d5`;
- pre-transform-type active CHANGELOG archive `71d1e8de1ff0a6607bf15e9c346c48e3a930d459`;
- PostgreSQL/APA decision record `923a120a9305e1305e5d33a3cacfec72ae53ed57`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-types-integrity.md`.

Focused contract coverage includes exact nullable `protrftypes` provenance, NULL-versus-selection digest separation, transform-set ordering invariance, explicit-empty and duplicate rejection, exact operator/target-function binding, complete unique coordinate coverage, one-based positions, exact receipt coordinates, and public composition.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc`, which defines `protrftypes`; PostgreSQL 18.6 `CREATE FUNCTION`, which defines `TRANSFORM FOR TYPE`; and PostgreSQL 18.6 `pg_transform`, which separately owns converter functions for a type/language pair. The focused rationale, alternatives, rejected claims, and residual converter-binding risk are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-types-integrity.md`.

Current transform-type traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5232628816`
- structural RED: `6c2aa9f3470c00a4fdd34ed6025a35640455ca58`
- RED refinement: `92d8141e102c33a18b89c05a586ee8c2054efbcd`
- production: `e12b961179a9b22ca9429eeaf1844d8fe8563323`
- composition: `ba11bd4333003b824a4128d32782a19a7a02a0ca`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_types.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_types_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureCostSnapshot`
- exact catalog fact: `pg_proc.protrftypes`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the transform-type contract plus every retained procedure/operator/Source Observation/relation-partition contract, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and all applicable hosted quality/security/review gates. Any source movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; follow `oprcode` to the exact target `pg_proc` row and independently read `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, `proconfig`, `proacl`, `prosupport`, `procost`, and `protrftypes`. It must preserve NULL-default ACL semantics and same-generation ACL role resolution; resolve any nonzero `prosupport` to the exact support `pg_proc` row; preserve NULL `protrftypes`; and resolve every selected transform-type OID to the exact same-generation qualified type identity. Unresolved nonzero support or transform-type OIDs are capture failures, not absence. Operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in that same v3 generation.

A real ordinary-EXCLUDE implementation function and its actual `protrftypes` state are the positive control. Synthetic transform selections are digest-distinguishability unit controls only. No live differential may claim exact transform converter identity until a separately reviewed `pg_transform` successor binds it.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09`, OPEN / Draft / mergeable. Owner-qualified source repair is complete. Exact-current CodeQL PR `35177301855`, Security Scan `35177301852`, Python Security `35177301847`, SAST Semgrep `35177301881`, and Agent Review Runtime Quality CI `35177301854` remain queued; terminal settlement is therefore still open.

Product bootstrap #35 remains a separate prerequisite at exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`; Ready/mergeable is not normal-landing authority while its required CodeQL acceptance debt remains unresolved. Fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_TYPES_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_TYPES_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_IDENTITY_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_COST_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable semantic publication, version/tag/package/SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including transform-type evidence -> bounded PostgreSQL 18 live differential including exact `protrftypes` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review exact `pg_transform` converter identity as the next material catalog gap -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
