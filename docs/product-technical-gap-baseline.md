# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active surface through exact `51820de79986f83e8417efd7196159f59f4be1fb` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-51820de7.md`; its matching CHANGELOG is preserved at `docs/archive/CHANGELOG-through-51820de7.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution/review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued digest domain is rewritten. Ordinary `pg_constraint.contype='x'` evidence retains independent constraint/backing-index identity, ordered `conkey`/`conexclop`, operator kind/commutator/result/procedure binding, target-function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, nonzero converter definition identity, converter owner identity, converter object-level `EXECUTE` ACL identity, converter nullable `proconfig` identity, operator-family/strategy, namespace/name/lifecycle/access-method controls, and exact v3 source-content-generation binding. Temporal `WITHOUT OVERLAPS`/`PERIOD` families remain separate.

## EXCLUDE transform-converter security-context integrity

Review `5235250993` on exact predecessor `51820de79986f83e8417efd7196159f59f4be1fb` found the next material P1: every nonzero converter was definition-, owner-, ACL-, and configuration-bound, but raw same-row `pg_proc.prosecdef` remained independently mutable and ungoverned.

PostgreSQL 18 permits `ALTER FUNCTION ... SECURITY INVOKER|SECURITY DEFINER` without changing the function signature or the already-governed converter facts. Invoker mode executes with caller privileges; definer mode executes with the function owner's privileges. PostgreSQL's own security guidance treats `SECURITY DEFINER` as a privileged execution boundary and requires careful `search_path` handling. Converter `prosecdef` therefore cannot be inferred from owner, ACL, `proconfig`, language, definition material, or current caller identity.

The ordinary-forward successor adds `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation`, provenance receipt, and `...SecurityDefinerSnapshot`. Both Boolean states are representable; the snapshot is observational rather than a policy rule. It takes `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot` as its immutable direct predecessor and requires exactly one raw Boolean observation for every exact `(constraint, key_position, transform_type, direction)` converter coordinate with the same converter schema/function binding. Missing/extra directions, duplicate coordinates, zero positions, binding drift, and unknown receipt coordinates fail closed.

### Ordinary-forward lineage

- finding review: `5235250993`;
- structural source/compile RED: `96aeb1b07cf32a225db250c570a5a6ff120221be`; no executed compiler failure is claimed;
- production successor: `6bfa27f7c20e74dfa870515850bcb04424dad4fe`;
- public module composition: `4652d52ad538bf17ba21bd1d8c8ad3b678a5d0d4`;
- pre-security-context gap-baseline archive: `41ab49a8a4cffdd75b404eec36bf496c29ec7f25`;
- pre-security-context CHANGELOG archive: `849fe32b76b670e905a9b53c49aa14f8d29051b3`;
- PostgreSQL/NIST/APA decision record: `8a9361dddd881bb1be38213a84b1b5b627720aa2`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-security-definer-integrity.md`.

Focused contract coverage includes raw `prosecdef` provenance, invoker/definer digest separation, complete converter-direction coverage, exact converter-function binding, duplicate-coordinate rejection, one-based positions, exact receipt lookup, and public composition. Synthetic Boolean states are unit-test distinguishability controls only.

## PostgreSQL 18 and governance authority

Primary authority is PostgreSQL 18 `CREATE FUNCTION`, `ALTER FUNCTION`, and function-security guidance. `SECURITY DEFINER` changes the privilege principal under which function code runs, while `SECURITY INVOKER` uses the caller. NIST least-privilege guidance supports governing elevation boundaries as evidence, but it does not define PostgreSQL catalog semantics; PostgreSQL remains the technical authority for `prosecdef`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_security_definer.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_security_definer_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot`
- exact catalog fact: each nonzero converter function's raw same-row `pg_proc.prosecdef`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 is not available in the current execution host (`cargo`, `rustc`, and `rustup` are absent), so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned statement/branch/edge coverage remain unexecuted. The RED is structural only. One unchanged exact head must obtain native and hosted terminal acceptance before any differential or publication claim.

The bounded PostgreSQL 18 live differential must retain every predecessor ordinary-EXCLUDE fact, resolve each selected `(trftype, target prolang)` `pg_transform` row, resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, and independently capture converter definition, raw `proowner` plus role resolution, raw `proacl` NULL-state plus canonical object-level `EXECUTE` grants, raw nullable `proconfig`, and raw `prosecdef`. Missing converter/security-context resolution, mixed-generation joins, or inferred Boolean state is capture failure rather than an unknown placeholder.

## Residual material gap

Converter definition, owner, ACL, local configuration, and execution security context are now independently source-bound, but complete converter runtime/security identity is not yet claimed. `pg_proc.proleakproof` is the next identified independently mutable converter fact because it can affect evaluation ordering around security-barrier views and row-level security without changing the preceding facts. Strictness, volatility, parallel safety, planner support/cost and other auxiliary fields remain later reviewed successors rather than inferred state.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` is exact `47ccc21c0374e5fdc4deb988fd09b4967adbda23` on protected base `8c77327a52883cbb7a9a698a67c20b3c2bec6700`, OPEN / Draft and currently non-mergeable. Its branch has ordinary/non-force reconciled intervening protected-main truth. Exact-current CodeQL PR `35214196144`, Security Scan `35214196285`, Python Security `35214196142`, SAST Semgrep `35214196258`, and Agent Review Runtime Quality CI `35214196283` remain queued/nonterminal; formal review history contains no qualifying independent approval for this head. No predecessor checks/reviews transfer.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN / Ready / mergeable. SAST Semgrep `34434790777` and Security Scan `34434790791` are success while CodeQL PR `34434790860` remains `completed/failure`; normal landing is therefore not authorized before canonical workflow-owner settlement and fresh compatible acceptance.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SECURITY_DEFINER_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted. Version/tag/package/immutable semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current mergeability/terminal-settlement repair and qualifying independent approval -> fresh compatible #35 acceptance/normal landing -> one unchanged #46 native+hosted terminal GREEN including converter `prosecdef` evidence -> bounded PostgreSQL 18 live differential including converter `prosecdef` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review converter `proleakproof` as the next independent auxiliary fact -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
