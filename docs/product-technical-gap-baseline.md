# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-4ed375df.md`; the owner-era CHANGELOG is preserved at `docs/archive/CHANGELOG-through-4ed375df.md`. Earlier decision surfaces remain under `docs/archive/`, and focused authority/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, procedure-volatility, procedure-parallel-safety, procedure-kind, procedure-security-definer, procedure-leakproof, procedure-definition, procedure-owner, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; raw `pg_proc.provolatile`; raw `pg_proc.proparallel`; raw `pg_proc.prokind='f'`; raw `pg_proc.prosecdef`; raw `pg_proc.proleakproof`; exact implementation language/definition material from `prolang`/`prosrc`/`probin`/`prosqlbody`; exact `pg_proc.proowner` plus same-generation `pg_roles.oid -> rolname`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

The detailed implementation-definition and function-owner decisions, lineage, rejected alternatives, and source/test traceability remain preserved in `docs/archive/product-technical-gap-baseline-through-4ed375df.md`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-definition-integrity.md`, and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-owner-integrity.md`.

## EXCLUDE implementation-function local configuration integrity

Review `5231254550` on exact owner predecessor `4ed375df5ba0e8a1d248d67a1b9f66c22f954dc9` found the next bounded P1: the governed chain preserved exact executable definition and owner identity but omitted `pg_proc.proconfig`, the function-local run-time configuration array.

PostgreSQL 18 stores `proconfig` independently as local run-time settings. `ALTER FUNCTION ... SET`, `SET FROM CURRENT`, `RESET`, and `RESET ALL` can change those settings without changing the function's input identity. This is security-material in PostgreSQL's own model: the `CREATE FUNCTION` guidance for `SECURITY DEFINER` explicitly recommends a safe function-local `search_path` to prevent an untrusted writable schema or temporary schema from masking intended objects. Two otherwise identical ordinary-EXCLUDE implementation functions can therefore execute under materially different name-resolution or run-time settings if `proconfig` is omitted from governed source identity.

The repair introduces `IndexExclusionConstraintOperatorProcedureConfigurationMaterial`, `IndexExclusionConstraintOperatorProcedureConfigurationObservation`, and `IndexExclusionConstraintOperatorProcedureConfigurationSnapshot` over the exact function-owner predecessor. The source boundary consumes exact same-row nullable `pg_proc.proconfig`, distinguishes `NULL` from an empty array, preserves catalog array order and exact entry bytes in domain-separated framing, and immediately reduces the raw array to SHA-256. Receipts expose only `is_configured`, entry count, and the digest; arbitrary/custom GUC plaintext is not propagated downstream.

Configuration lineage:

- finding review `5231254550`;
- structural source/compile RED `f5baf631e9add518322cbab8db471acdf4fe1d4f`; the new public configuration types did not yet exist, so no executed compiler failure is claimed;
- production successor `4c7117fa7587f6c85e0588f00a76c1a4e1dbb117`;
- public composition `0354ecf1c003a021b1401932cbc82f74bd5cbdf4`;
- PostgreSQL/APA decision record `a7d631a3d9a3c98fb5636ef9211c79801aaa5ce8`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-configuration-integrity.md`;
- pre-configuration CHANGELOG preserved byte-for-byte at `docs/archive/CHANGELOG-through-4ed375df.md` by `2f9bbc4fa1f464c86b54b214a22eeb34dacc4fc3`;
- pre-configuration product/technical-gap baseline preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-4ed375df.md` by `7519d0ba39a39e6ac8f5c681150c59bf48722ff0`.

Configuration invariants are exact coordinate/key completeness, uniqueness, exact operator/function binding to the owner predecessor, one-based key positions, and exact receipt coordinates. `NULL`, empty array, entry-byte changes, and source-order changes remain distinct digest states. This source identity does not parse or normalize GUC assignments and does not itself decide whether a `search_path` is secure; a policy decision must be separately governed with the required security context.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18 `pg_proc`, which defines `proconfig` as the function's local run-time settings, and PostgreSQL 18 `ALTER FUNCTION`, which permits `SET`/`RESET` changes without altering input identity. PostgreSQL 18 `CREATE FUNCTION` further documents `search_path` as a SECURITY DEFINER security boundary. Focused rationale, privacy treatment, alternatives, and APA 7 references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-configuration-integrity.md`.

Current configuration traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5231254550`
- structural RED: `f5baf631e9add518322cbab8db471acdf4fe1d4f`
- production: `4c7117fa7587f6c85e0588f00a76c1a4e1dbb117`
- composition: `0354ecf1c003a021b1401932cbc82f74bd5cbdf4`
- doctoring: `a7d631a3d9a3c98fb5636ef9211c79801aaa5ce8`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_configuration.rs`
- test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_configuration_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureOwnerSnapshot`
- exact catalog fact: `pg_proc.proconfig`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-configuration/owner/definition/leakproof/security-definer/kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, and `proconfig`; resolve the exact same-generation `proowner` through `pg_roles.oid -> rolname`; resolve `prolang` to exact `pg_language.lanname`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. Owner, definition material, and function-local configuration must come from the exact joined `pg_proc` row and may not be reconstructed from routine name, schema ownership, session identity, current GUC values, decompiled DDL, or another generation.

A real ordinary-EXCLUDE implementation function with its actual owner/language/definition/configuration tuple is the positive control. Synthetic owner, source, and GUC variants are digest-distinguishability unit controls only; they are not evidence that arbitrary principals, bodies, or GUC settings are valid PostgreSQL exclusion-operator configurations.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09` on base `a9c6477d326b84411f492ba4cac29f52deebcac3`, OPEN / Draft / mergeable. Commit `653466de...` repaired the two prior owner-qualified durable repository identity REDs. The most recent fresh exact-current workflow inventory remains nonterminal: CodeQL PR `35177301855` is pending; Security Scan `35177301852`, Python Security `35177301847`, SAST Semgrep `35177301881`, and Agent Review Runtime Quality CI `35177301854` are queued. ConceptWeave does not compete with that owner lane or manufacture wake/no-op evidence.

Product bootstrap #35 remains source-stable at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its SAST and Security Scan are terminal success, but CodeQL PR `34434790860` remains terminal failure, so Ready/mergeable is not normal-landing authority. Fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including the new configuration successor -> bounded PostgreSQL 18 live differential including exact `proconfig`, owner and implementation-definition material plus every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
