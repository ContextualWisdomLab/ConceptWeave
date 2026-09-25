# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline through exact `d35374cba690503d45b96079df84b5d945d4df42` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-d35374cb.md`; its matching CHANGELOG is preserved at `docs/archive/CHANGELOG-through-d35374cb.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution/review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued digest domain is rewritten. Ordinary `pg_constraint.contype='x'` evidence retains independent constraint/backing-index identity, ordered `conkey`/`conexclop`, operator kind/commutator/result/procedure binding, target-function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, nonzero converter definition identity, converter owner identity, operator-family/strategy, namespace/name/lifecycle/access-method controls, and exact v3 source-content-generation binding. Temporal `WITHOUT OVERLAPS`/`PERIOD` families remain separate.

## EXCLUDE transform-converter access-control integrity

Review `5234509421` on exact predecessor `d35374cba690503d45b96079df84b5d945d4df42` found the next material P1: every nonzero transform converter was content-bound and owner-bound, but its independently mutable `pg_proc.proacl` was not governed.

PostgreSQL 18 stores routine access privileges in `pg_proc.proacl`. Function `EXECUTE` grants can be changed with `GRANT`/`REVOKE` without redefining function identity, body, or owner. `CREATE TRANSFORM` separately requires ownership and `EXECUTE` privilege on specified FROM-SQL/TO-SQL converter functions. Converter ACL therefore cannot be inferred from converter definition, target-function ACL, or owner evidence.

The ordinary-forward successor adds converter-specific `ExecuteGrant`, `AccessControlMaterial`, `AccessControlObservation`, provenance receipt, and `AccessControlSnapshot` types. `AccessControlMaterial` preserves raw `proacl IS NULL` versus explicit ACL state and domain-hashes a canonical object-level `EXECUTE` grant set. `PUBLIC`, named grantees, grantor role, and grant option are distinct facts. Duplicate grant evidence fails closed.

For every exact `(constraint, key_position, transform_type, direction)` in `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot`, the new snapshot requires exactly one ACL observation with the same converter schema/function binding. Missing/extra directions, duplicate coordinates, zero positions, binding drift, malformed role evidence, and unknown receipt coordinates fail closed. Transitive role-membership closure, superuser semantics, current-session role selection, and product allow/deny policy remain outside Source Observation.

### Ordinary-forward lineage

- finding review: `5234509421`;
- structural source/compile RED: `8b65892785b5c4b1db0f30812d1a010fff6ac71f`; no executed compiler failure is claimed;
- production successor: `c6d8f4ca6c49ec3be44fd797df5f6ce9f8d83a0b`;
- public module composition: `816621e01667b76e22fe7633a56bf50428d6c2f8`;
- pre-ACL CHANGELOG archive: `8a4ca9e14b66046ec81c1f4ba46dd07a873612a0`;
- pre-ACL gap-baseline archive: `f81f2d73b4d82eac5085e8f998e0147a51d3f416`;
- PostgreSQL/APA decision record: `4032991d0bd6242aae55890bd282325c8408639a`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-access-control-integrity.md`.

Focused contract coverage includes raw NULL-state preservation, NULL-vs-explicit digest separation, grant-set digest separation, complete nonzero converter-direction coverage, exact converter binding, duplicate-grant rejection, exact receipt lookup through inherited coverage, and public composition. Synthetic ACL fixtures are unit-test distinguishability controls only.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18 `pg_proc`, `GRANT`, `CREATE TRANSFORM`, and `CREATE FUNCTION`. `pg_proc.proacl` is the routine privilege catalog field; `GRANT` defines `EXECUTE`, `PUBLIC`, grant option and role-derived runtime privilege semantics; `CREATE TRANSFORM` requires ownership and `EXECUTE` on selected converters; and `CREATE OR REPLACE FUNCTION` preserves permissions while replacing a definition, demonstrating that function definition and privilege identity are separate dimensions.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_access_control.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_access_control_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot`
- exact catalog fact: each nonzero converter function's `pg_proc.proacl`, raw NULL-state and same-generation canonical object-level `EXECUTE` grants.

## Acceptance boundary

**Source repaired is not GREEN.** This runtime still does not provide repository-pinned Rust 1.98 `cargo`/`rustc`/`rustup`; `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned statement/branch/edge coverage are therefore unexecuted. The RED is structural only. One unchanged exact head must obtain native and hosted terminal acceptance before any differential or publication claim.

The bounded PostgreSQL 18 live differential must retain every predecessor ordinary-EXCLUDE fact, resolve each selected `(trftype, target prolang)` `pg_transform` row, resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, and independently capture converter definition, raw `proowner` plus role resolution, and now raw `proacl` NULL-state plus object-level `EXECUTE` grant evidence. Missing role/ACL resolution or generation mismatch is capture failure rather than an unknown placeholder.

## Residual material gap

Converter definition, owner, and ACL are now independently source-bound, but complete converter runtime/security identity is not yet claimed. Nullable converter `pg_proc.proconfig` is the next identified material auxiliary fact because function-local execution settings can change independently of definition/owner/ACL. `prosecdef`, `proleakproof`, strictness, volatility, parallel safety, planner support/cost and other independently mutable fields remain later reviewed successors rather than inferred state.

## Canonical prerequisite state

Fresh `ContextualWisdomLab/.github#2106` state remains exact `653466de1520c966addbeec985b9db2d21421d09`, OPEN / Draft / mergeable. Owner-qualified source repair is complete. Exact-current SAST Semgrep `35177301881` and Agent Review Runtime Quality CI `35177301854` are `completed/success`; CodeQL PR `35177301855`, Security Scan `35177301852`, and Python Security `35177301847` remain queued/nonterminal. Qualifying independent current-head approval is still absent according to the canonical PR authority, so normal landing remains blocked without synthetic wake/rerun evidence.

Product bootstrap #35 remains a separate prerequisite. Its historical CodeQL acceptance debt cannot be overridden by Ready/mergeable state; fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_CONFIGURATION_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted. Version/tag/package/immutable semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement and qualifying independent approval -> fresh compatible #35 acceptance/normal landing -> one unchanged #46 native+hosted terminal GREEN including converter ACL evidence -> bounded PostgreSQL 18 live differential including converter `proacl` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review nullable converter `proconfig` as the next independent auxiliary fact -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
