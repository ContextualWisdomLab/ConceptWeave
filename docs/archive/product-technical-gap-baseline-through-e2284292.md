# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-cc64bab7.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, procedure-volatility, procedure-parallel-safety, procedure-kind, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; raw `pg_proc.provolatile`; raw `pg_proc.proparallel`; raw `pg_proc.prokind='f'`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_proc.prosecdef` integrity

Review `5230117505` on exact predecessor `cc64bab7d81f2adb691f627e7de852ace6946227` found the next bounded P1. The retained chain proves the exact ordinary-EXCLUDE operator implementation function and preserves its routine kind plus result/cardinality/strictness/volatility/parallel-safety semantics, but it still omitted raw `pg_proc.prosecdef`.

PostgreSQL 18.6 stores `prosecdef` independently. `SECURITY INVOKER` executes the function with the privileges of the calling user and is the default; `SECURITY DEFINER` executes with the privileges of the function owner. `ALTER FUNCTION ... SECURITY INVOKER|SECURITY DEFINER` changes this property without changing the input-argument signature that identifies the function. Therefore identical stable operator/function bindings can have materially different privilege boundaries and must not collapse to the same governed semantic digest.

This repair is observational rather than prescriptive. No authoritative PostgreSQL ordinary-EXCLUDE rule was found that requires `prosecdef=false`. Both raw Boolean states are representable, but they are domain-separated and bound to the exact predecessor position.

Ordinary-forward lineage:

- finding review `5230117505` on predecessor `cc64bab7d81f2adb691f627e7de852ace6946227`;
- structural source/compile contract `4127151673d52b647e40e42f3cf310d958ee409e`; the test referenced the new public security-context types before production existed, so no executed compiler failure is claimed;
- production successor `27b9215a35151926c8bf90a21f9e941535d358a4`, `IndexExclusionConstraintOperatorProcedureSecurityDefinerObservation` / `Snapshot` / `SourceReceipt`;
- public composition `926019b08c8a96ff0937e9b2876d20516d9f1766`;
- PostgreSQL/APA decision record `cc3a71e4d11bb09c41fb0dd5c8852aeff3f050cb`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-security-definer-integrity.md`;
- predecessor baseline preserved losslessly at `docs/archive/product-technical-gap-baseline-through-cc64bab7.md` by `d3c0f80bfeffae4b38a8dd96184fc15e9b715ff4`.

`IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot` derives the exact `(constraint coordinate, key_position)` inventory from `IndexExclusionConstraintOperatorProcedureKindSnapshot`. Each governed position requires one independently observed raw `prosecdef` value bound to the same stable operator and exact `oprcode` function. Missing/duplicate evidence, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. The successor digest includes the predecessor digest, exact coordinate/key position, operator/function signatures, and raw Boolean. From the same predecessor, `false` and `true` must produce distinct digests.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc` §52.39 for `prosecdef`, PostgreSQL 18.6 `CREATE FUNCTION` for `SECURITY INVOKER` versus `SECURITY DEFINER` privilege semantics, and PostgreSQL 18.6 `ALTER FUNCTION` for independent mutation of the security mode. Focused rationale and APA 7 references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-security-definer-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5230117505`
- structural RED: `4127151673d52b647e40e42f3cf310d958ee409e`
- production: `27b9215a35151926c8bf90a21f9e941535d358a4`
- composition: `926019b08c8a96ff0937e9b2876d20516d9f1766`
- doctoring: `cc3a71e4d11bb09c41fb0dd5c8852aeff3f050cb`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_security_definer.rs`
- test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_security_definer_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureKindSnapshot`
- exact catalog fact: `pg_proc.prosecdef`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-security-definer/procedure-kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `prokind`, `prosecdef`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `prosecdef` must come from the exact joined `pg_proc` row and may not be inferred from routine name, owner, language, configuration, result/cardinality, strictness, volatility, parallel safety, or routine kind.

A real ordinary-EXCLUDE implementation function with its actually observed security mode is the positive control. A synthetic `false`/`true` pair is only a digest-distinguishability unit control; it is not evidence that both modes are accepted by a particular real DDL fixture.

The focused source contract covers invoker provenance, invoker/definer digest separation, operator and implementation-function binding drift, missing evidence, duplicate coordinates, zero position, unknown receipt coordinates, and public composition. It does not replace the still-required exact-head Rust suite or PostgreSQL live differential.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `pg_proc.prosecdef` plus every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.