# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-491092b2.md`; the preceding active CHANGELOG is preserved at `docs/archive/CHANGELOG-through-491092b2.md`. Earlier decision surfaces remain under `docs/archive/`, and focused authority/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued predecessor digest domain is rewritten by this repair. For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained source identity includes the independent constraint object and exact backing index; timing/enforcement/validation/period/no-inherit state; ordered `conkey`/`conexclop`; `pg_operator.oprkind`, independently resolved `oprcom`, exact `oprcode -> pg_proc`, and independent Boolean `oprresult`/`prorettype`; `pg_proc.proretset`, `proisstrict`, `provolatile`, `proparallel`, `prokind`, `prosecdef`, `proleakproof`; implementation language/definition material (`prolang`, `prosrc`, `probin`, `prosqlbody`); exact `proowner` plus same-generation role resolution; exact nullable `proconfig`; operator-family/strategy; constraint/index namespace and name coupling; access-method exclusion capability; backing-index role/lifecycle; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain separate owner families and are not reclassified as ordinary EXCLUDE.

## EXCLUDE implementation-function access-control integrity

Review `5231368167` on exact predecessor `491092b20a2ccca4ae3bb8ce6bffcf07eceae838` found the next bounded P1: the governed chain preserved implementation identity, owner and local configuration but omitted exact `pg_proc.proacl` state. PostgreSQL 18 stores routine access privileges independently. Function `EXECUTE` privilege controls direct calls and use of operators implemented by that function; `GRANT`/`REVOKE EXECUTE` can therefore change invocation authority without changing the function signature or the predecessor facts.

The repair adds `IndexExclusionConstraintOperatorProcedureExecuteGrant`, `IndexExclusionConstraintOperatorProcedureAccessControlMaterial`, `IndexExclusionConstraintOperatorProcedureAccessControlObservation`, and `IndexExclusionConstraintOperatorProcedureAccessControlSnapshot` over the exact configuration predecessor. The adapter must read `proacl` from the same joined `pg_proc` row, preserve whether the catalog value was NULL, materialize the function-default ACL when NULL, expand the catalog ACL to its exact `EXECUTE` entries, and resolve grantee/grantor role OIDs in the same source generation. Grantee OID zero is represented as PUBLIC. This source layer preserves ACL-entry semantics only; role-membership/inheritance closure and session authorization remain separate governed facts.

ACL order is not privilege semantics, so the effective ACL-entry set is canonicalized before hashing. Each entry binds PUBLIC-or-role grantee, exact grantor role and grant-option bit. Duplicate effective entries fail closed. Role-bearing ACL material is reduced immediately to a domain-separated SHA-256 digest; receipts expose only NULL-vs-explicit state, grant count and digest. This source layer does not impose a PUBLIC/role admission policy and does not conflate function owner, SECURITY DEFINER, local GUC configuration, or implementation definition with function ACL.

Access-control lineage:

- finding review `5231368167`;
- structural source/compile RED `52a533a6a2979e1ae884830dc5084b2a8dfd963c`; public ACL types did not yet exist, so no executed compiler failure is claimed;
- production successor `edac52866c4a8f8055eb018b4c6782a4b6661feb`;
- public composition `f50040d6fd7e50856a2b3ff032dc0fe7f03d9e59`;
- PostgreSQL/NIST/APA decision record `20de21c339d93286f2066c92db8953e696ec0c3d`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-access-control-integrity.md`;
- pre-access-control CHANGELOG preserved byte-for-byte at `docs/archive/CHANGELOG-through-491092b2.md` by `3ee18990db314dcd0f478560ceefb2aa8eddc1bf`;
- pre-access-control product/technical-gap baseline preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-491092b2.md` by `40fd62627bc17ec101f71b87e11343eb5aca4dae`.

Focused contract coverage includes NULL-default versus explicit-equivalent ACL distinction, grant-order canonicalization, grantee/grantor/grant-option distinguishability, duplicate-grant rejection, blank role rejection, exact operator/function binding, complete unique coordinate coverage, one-based positions, exact receipt coordinates, and public composition.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18 `pg_proc` for `proacl`, PostgreSQL 18 privilege semantics for function `EXECUTE`, and PostgreSQL 18 default-privilege semantics for the initial function ACL. The focused rationale and rejected alternatives are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-access-control-integrity.md`; NIST SP 800-53 Rev. 5 AC-3 and AC-6 provide the access-enforcement/least-privilege governance context without substituting for PostgreSQL catalog truth.

Current access-control traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5231368167`
- structural RED: `52a533a6a2979e1ae884830dc5084b2a8dfd963c`
- production: `edac52866c4a8f8055eb018b4c6782a4b6661feb`
- composition: `f50040d6fd7e50856a2b3ff032dc0fe7f03d9e59`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_access_control.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_access_control_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureConfigurationSnapshot`
- exact catalog fact: `pg_proc.proacl`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new access-control contract plus every retained procedure/operator/Source Observation/relation-partition contract, workspace/doc tests, release build, rustdoc and owned statement/branch/edge coverage, and all applicable hosted quality/security/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; follow `oprcode` to the exact `pg_proc` row and independently read `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, `proconfig`, and `proacl`. It must resolve owner/language in the same generation and resolve ACL grantee/grantor role identities in that same generation, preserving PUBLIC and NULL-default ACL semantics. Operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in the same v3 source-content generation. ACL evidence must come from the exact joined `pg_proc` row, not reconstructed DDL, current-session privilege checks, or a different generation.

A real ordinary-EXCLUDE implementation function with its actual catalog ACL is the positive control. Synthetic ACL variants are digest-distinguishability unit controls only; they do not prove that arbitrary grants are appropriate production policy.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09` on base `a9c6477d326b84411f492ba4cac29f52deebcac3`, OPEN / Draft / mergeable. Its owner-qualified source repair is complete. Fresh exact-current workflow settlement remains nonterminal: CodeQL PR `35177301855` is pending; Security Scan `35177301852`, Python Security `35177301847`, SAST Semgrep `35177301881`, and Agent Review Runtime Quality CI `35177301854` are queued. ConceptWeave does not compete with that owner lane or manufacture wake/no-op evidence.

Product bootstrap #35 remains source-stable at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`; its SAST/Security gates are success but CodeQL PR `34434790860` remains failure, so Ready/mergeable is not normal-landing authority. Fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable semantic publication, version/tag/package/SBOM/provenance, reproducibility and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including function access-control evidence -> bounded PostgreSQL 18 live differential including exact `proacl` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
