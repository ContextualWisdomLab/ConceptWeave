# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-93917259.md`; its active CHANGELOG is preserved at `docs/archive/CHANGELOG-through-93917259.md`. Earlier surfaces remain under `docs/archive/`, and focused authority/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and its released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner. Consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative; no issued predecessor digest domain is rewritten. For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained source identity includes the independent constraint and exact backing index; timing/enforcement/validation/period/no-inherit state; ordered `conkey`/`conexclop`; `pg_operator.oprkind`, independently resolved `oprcom`, exact `oprcode -> pg_proc`, and independent Boolean `oprresult`/`prorettype`; target-function `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prokind`, `prosecdef`, `proleakproof`; implementation language/definition (`prolang`, `prosrc`, `probin`, `prosqlbody`); exact `proowner`; exact nullable `proconfig`; exact `proacl` with same-generation ACL role resolution; operator-family/strategy; constraint/index namespace/name coupling; access-method exclusion capability; backing-index role/lifecycle; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain separate owner families and are not reclassified as ordinary EXCLUDE.

## EXCLUDE implementation-function planner-support integrity

Review `5231648255` on exact predecessor `939172595c56a6a4bbd3ca7e9c63589a76930b3b` found the next bounded P1: the governed target-function chain omitted independent `pg_proc.prosupport`. PostgreSQL 18 stores `prosupport` as a `regproc` reference to an optional planner support function. A support function can simplify target-function calls, including operators based on the target function, and can provide selectivity, cost, rows, and index-condition planning information. The same target-function signature and retained catalog properties therefore do not prove equivalent planner-support state.

The repair adds `IndexExclusionConstraintOperatorProcedurePlannerSupportObservation`, `IndexExclusionConstraintOperatorProcedurePlannerSupportSourceReceipt`, and `IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot` over the exact access-control predecessor. `prosupport = 0` is explicit absence. A nonzero support OID must be independently resolved in the same bounded source generation to a stable support-function signature before entering the domain boundary. The OID itself remains adapter-local.

This is observational source identity, not an EXCLUDE admission rule. ConceptWeave neither requires planner support nor infers it from language, volatility, cost, implementation definition, operator family, or other target-function facts. Missing evidence, duplicate coordinates, zero positions, operator drift, target-function drift, and unknown receipt coordinates fail closed. Absence, support-function identity A, and support-function identity B produce distinct successor digests.

Planner-support lineage:

- finding review `5231648255`;
- structural source/compile RED `e77373013fb8340038a426d84ca5af6588da63c1`; the public planner-support types did not yet exist, so no executed compiler failure is claimed;
- production successor `9212b5211ad75e17f8afe43d29dec0a309054b8a`;
- public composition `0802f62c95adba09f25fe58fdb4df84038940fd3`;
- focused fixture signature correction `f2af547d3a95affd79d2cd70c6ddc402bbdfbfb6` so the synthetic support identity uses the documented `supportfn(internal)` call signature;
- PostgreSQL/APA decision record `b9836efb39478cb17048846942e7dad92cf541f5`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-planner-support-integrity.md`;
- pre-planner-support CHANGELOG preserved byte-for-byte at `docs/archive/CHANGELOG-through-93917259.md` by `7a336f75d70fbd45af9fbf48c5571437b5a856c0`;
- pre-planner-support product/technical-gap baseline preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-93917259.md` by `a26f1c106f7c70cfd20ba564e672221ffef6d8d8`.

Focused contract coverage includes exact support presence/absence provenance, absence-versus-presence and support-identity digest separation, exact operator/target-function binding, complete unique coordinate coverage, one-based positions, exact receipt coordinates, and public composition. Synthetic support identities are unit controls only; they do not assert that the invented support functions exist or are valid production policy.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc`, which records `prosupport` independently, PostgreSQL 18.6 `CREATE FUNCTION`, which exposes the `SUPPORT` property, and PostgreSQL function-optimization documentation, which defines planner-support behavior. The focused rationale and rejected alternatives are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-planner-support-integrity.md`.

Current planner-support traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5231648255`
- structural RED: `e77373013fb8340038a426d84ca5af6588da63c1`
- production: `9212b5211ad75e17f8afe43d29dec0a309054b8a`
- composition: `0802f62c95adba09f25fe58fdb4df84038940fd3`
- fixture correction: `f2af547d3a95affd79d2cd70c6ddc402bbdfbfb6`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_planner_support.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_planner_support_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureAccessControlSnapshot`
- exact catalog fact: `pg_proc.prosupport`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new planner-support contract plus every retained procedure/operator/Source Observation/relation-partition contract, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and all applicable hosted quality/security/review gates. Any source movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; follow `oprcode` to the exact target `pg_proc` row and independently read `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, `proconfig`, `proacl`, and `prosupport`. It must preserve NULL-default ACL semantics and same-generation ACL role resolution. For nonzero `prosupport`, it must resolve that exact OID to the support `pg_proc` row in the same source-content generation and reduce the stable support-function identity; unresolved nonzero support is a capture failure, not absence. Operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in that same v3 generation.

A real ordinary-EXCLUDE implementation function and its actual `prosupport` state are the positive control. Synthetic planner-support variants are digest-distinguishability unit controls only.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09` on base `a9c6477d326b84411f492ba4cac29f52deebcac3`, OPEN / Draft / mergeable. Its owner-qualified source repair is complete. Fresh exact-current workflow settlement remains nonterminal: CodeQL PR `35177301855` is pending; Security Scan `35177301852`, Python Security `35177301847`, SAST Semgrep `35177301881`, and Agent Review Runtime Quality CI `35177301854` are queued. ConceptWeave does not compete with that owner lane or manufacture wake/no-op evidence.

Product bootstrap #35 remains source-stable at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`; its SAST/Security gates are success but CodeQL PR `34434790860` remains failure, so Ready/mergeable is not normal-landing authority. Fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable semantic publication, version/tag/package/SBOM/provenance, reproducibility and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including planner-support evidence -> bounded PostgreSQL 18 live differential including exact `prosupport` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
