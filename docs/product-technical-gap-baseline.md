# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active surface through exact `b5904f6ede97bf433665a45e7da910fc7433b7f8` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-b5904f6e.md`; its matching CHANGELOG is preserved at `docs/archive/CHANGELOG-through-b5904f6e.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution/review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued digest domain is rewritten. Ordinary `pg_constraint.contype='x'` evidence retains independent constraint/backing-index identity, ordered `conkey`/`conexclop`, operator kind/commutator/result/procedure binding, target-function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, nonzero converter definition identity, converter owner identity, converter object-level EXECUTE ACL identity, operator-family/strategy, namespace/name/lifecycle/access-method controls, and exact v3 source-content-generation binding. Temporal `WITHOUT OVERLAPS`/`PERIOD` families remain separate.

## EXCLUDE transform-converter configuration integrity

Review `5235145397` on exact predecessor `b5904f6ede97bf433665a45e7da910fc7433b7f8` found the next material P1: each nonzero transform converter had exact definition, owner, and ACL identity, but nullable `pg_proc.proconfig` remained independently mutable and ungoverned.

PostgreSQL 18 allows function-local configuration through `SET configuration_parameter` and permits `ALTER FUNCTION ... SET`, `SET FROM CURRENT`, `RESET`, and `RESET ALL` without changing the function's input identity. PostgreSQL's own `SECURITY DEFINER` guidance uses a controlled `search_path` because function-local settings can materially change runtime and security semantics. Converter `proconfig` therefore cannot be inferred from converter definition, owner, ACL, current/session GUCs, or reconstructed DDL.

The ordinary-forward successor adds `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial`, `...ConfigurationObservation`, provenance receipt, and `...ConfigurationSnapshot`. `ConfigurationMaterial` preserves raw `proconfig IS NULL` versus explicit array state, exact array length, entry order, and entry bytes in a domain-separated SHA-256 digest while keeping raw GUC values out of receipts.

For every exact `(constraint, key_position, transform_type, direction)` in `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot`, the new snapshot requires exactly one configuration observation with the same converter schema/function binding. Missing/extra directions, duplicate coordinates, zero positions, binding drift, and unknown receipt coordinates fail closed. Effective session configuration and product policy remain outside Source Observation.

### Ordinary-forward lineage

- finding review: `5235145397`;
- structural source/compile RED: `b99338232954258f93fd43253e61cd58c76e295b`; no executed compiler failure is claimed;
- production successor: `f863ec28ac0d910befbd3c416e9a0ab4797d300c`;
- public module composition: `07053742f96f253cf3b76d1563d500a0e17be232`;
- pre-configuration gap-baseline archive: `fa2770ac67ec527472907febcacdf682b7aabcdc`;
- pre-configuration CHANGELOG archive: `a81b10183bfbf431588b96f4a92b64e29922a218`;
- PostgreSQL/NIST/APA decision record: `ffcd5aaab694eae9cac1c39df8ad5ae9c5d75c4b`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-configuration-integrity.md`.

Focused contract coverage includes raw configuration provenance, NULL-vs-empty-array separation, setting-value separation, catalog-array order preservation, complete converter-direction coverage, exact converter-function binding, duplicate-coordinate rejection, one-based positions, exact receipt lookup, and public composition. Synthetic GUC values are unit-test distinguishability controls only.

## PostgreSQL 18 and governance authority

Primary authority is PostgreSQL 18 `pg_proc`, `CREATE FUNCTION`, and `ALTER FUNCTION`. Function-local settings are independently mutable, and `RESET` returns execution to the environment-provided value. PostgreSQL specifically documents secure `search_path` handling for `SECURITY DEFINER` functions. NIST SP 800-53 Rev. 5 CM-6 treats configuration settings as security-relevant parameters that require establishment, implementation, documentation, and control; this is governance mapping, not a claim that NIST defines PostgreSQL catalog semantics.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_configuration.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_configuration_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot`
- exact catalog fact: each nonzero converter function's nullable same-row `pg_proc.proconfig` array.

## Acceptance boundary

**Source repaired is not GREEN.** The available execution environment has not established repository-pinned Rust 1.98 execution, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned statement/branch/edge coverage remain unexecuted for the moving head. The RED is structural only. One unchanged exact head must obtain native and hosted terminal acceptance before any differential or publication claim.

The bounded PostgreSQL 18 live differential must retain every predecessor ordinary-EXCLUDE fact, resolve each selected `(trftype, target prolang)` `pg_transform` row, resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, and independently capture converter definition, raw `proowner` plus role resolution, raw `proacl` NULL-state plus canonical object-level `EXECUTE` grants, and now raw nullable `proconfig`. Missing converter/configuration resolution, mixed-generation joins, or substitution with effective/session settings is capture failure rather than an unknown placeholder.

## Residual material gap

Converter definition, owner, ACL, and local configuration are now independently source-bound, but complete converter runtime/security identity is not yet claimed. `pg_proc.prosecdef` is the next identified independently mutable converter fact because execution privilege context can change without redefining the converter signature/body, owner, ACL, or `proconfig`. `proleakproof`, strictness, volatility, parallel safety, planner support/cost and other auxiliary fields remain later reviewed successors rather than inferred state.

## Canonical prerequisite state

Fresh canonical `ContextualWisdomLab/.github#2106` has advanced materially from the predecessor authority. It is now exact head `47ccc21c0374e5fdc4deb988fd09b4967adbda23` on protected base `8c77327a52883cbb7a9a698a67c20b3c2bec6700`, OPEN / Draft, with GitHub currently reporting non-mergeable. Its branch already ordinary/non-force reconciled intervening protected-main truth and preserved the owner-qualified evidence repair. Exact-current CodeQL PR `35214196144`, Security Scan `35214196285`, Python Security `35214196142`, SAST Semgrep `35214196258`, and Agent Review Runtime Quality CI `35214196283` are all queued/nonterminal. Formal review history contains no qualifying independent approval for exact `47ccc21c...`. No predecessor checks/reviews transfer.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN / Ready / mergeable. SAST Semgrep `34434790777` and Security Scan `34434790791` are success, while CodeQL PR `34434790860` remains `completed/failure`; normal landing is therefore not authorized before canonical workflow-owner settlement and fresh compatible acceptance.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_CONFIGURATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted. Version/tag/package/immutable semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current mergeability/terminal-settlement repair and qualifying independent approval -> fresh compatible #35 acceptance/normal landing -> one unchanged #46 native+hosted terminal GREEN including converter `proconfig` evidence -> bounded PostgreSQL 18 live differential including converter `proconfig` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review converter `prosecdef` as the next independent auxiliary fact -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
