# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active surface through exact `1153c0b6f121b5b99c533d73e3dc9e7b4305c276` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-1153c0b6.md`; its matching CHANGELOG is preserved at `docs/archive/CHANGELOG-through-1153c0b6.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution/review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued digest domain is rewritten. Ordinary `pg_constraint.contype='x'` evidence retains independent constraint/backing-index identity, ordered `conkey`/`conexclop`, operator kind/commutator/result/procedure binding, target-function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, nonzero converter definition identity, converter owner identity, converter object-level `EXECUTE` ACL identity, converter nullable `proconfig` identity, converter raw `prosecdef` identity, operator-family/strategy, namespace/name/lifecycle/access-method controls, and exact v3 source-content-generation binding. Temporal `WITHOUT OVERLAPS`/`PERIOD` families remain separate.

## EXCLUDE transform-converter leakproof integrity

Review `5235830453` on exact predecessor `1153c0b6f121b5b99c533d73e3dc9e7b4305c276` found the next material P1: every nonzero converter was definition-, owner-, ACL-, configuration-, and security-context-bound, but raw same-row `pg_proc.proleakproof` remained independently mutable and ungoverned.

PostgreSQL 18 permits `ALTER FUNCTION ... [NOT] LEAKPROOF` without changing those predecessor facts. PostgreSQL treats leakproofness as a security-sensitive trust assertion: leakproof functions may be evaluated ahead of security-barrier view predicates or row-level-security policy expressions, and the underlying operator function's leakproof status also affects whether otherwise inaccessible planner statistics may be used. The flag therefore cannot be inferred from `prosecdef`, owner, ACL, `proconfig`, language, implementation material, caller identity, or observed plan shape.

The ordinary-forward successor adds `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation`, provenance receipt, and `...LeakproofSnapshot`. Both Boolean states are representable; the snapshot is observational rather than a policy rule. It takes `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot` as its immutable direct predecessor and requires exactly one raw Boolean observation for every exact `(constraint, key_position, transform_type, direction)` converter coordinate with the same converter schema/function binding. Missing/extra directions, duplicate coordinates, zero positions, binding drift, and unknown receipt coordinates fail closed.

### Ordinary-forward lineage

- finding review: `5235830453`;
- structural source/compile RED: `ce86f3c7e84054a0c58e38f81a8b230c3e2653b3`; no executed compiler failure is claimed;
- production successor: `14ebe48407ab448586f68dbce8b2cd55b32ea5ba`;
- public module composition: `5a8229d6d04e3d4498cac61b0603eacc2e4398bc`;
- pre-leakproof gap-baseline archive: `96f7c03b300bea97c6ad8069fe8ec7721e045cb9`;
- pre-leakproof CHANGELOG archive: `cca534cf58069b83aaa6b745e4c74aa576ed5f51`;
- PostgreSQL/NIST/APA decision record: `e5006f422f326f818440ed9a37c033c9dd0534fc`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-leakproof-integrity.md`.

Focused contract coverage includes raw `proleakproof` provenance, false/true digest separation, complete converter-direction coverage, exact converter-function binding, duplicate-coordinate rejection, one-based positions, exact receipt lookup, and public composition. Synthetic Boolean states are unit-test distinguishability controls only.

## PostgreSQL 18 and governance authority

Primary authority is PostgreSQL 18 `CREATE FUNCTION`, `ALTER FUNCTION`, row-security, rules/privileges, and planner-statistics security documentation. PostgreSQL explicitly allows changing leakproof classification and states that leakproof functions/operators may cross security-policy or security-barrier evaluation boundaries. NIST control-assessment guidance supports preserving independently testable security-relevant evidence, but does not define PostgreSQL catalog semantics; PostgreSQL remains the technical authority for `proleakproof`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_leakproof.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_leakproof_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot`
- exact catalog fact: each nonzero selected converter function's raw same-row `pg_proc.proleakproof`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 is not available in the current execution host, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned statement/branch/edge coverage remain unexecuted. The RED is structural only. One unchanged exact head must obtain native and hosted terminal acceptance before any differential or publication claim.

The bounded PostgreSQL 18 live differential must retain every predecessor ordinary-EXCLUDE fact, resolve each selected `(trftype, target prolang)` `pg_transform` row, resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, and independently capture converter definition, raw `proowner` plus role resolution, raw `proacl` NULL-state plus canonical object-level `EXECUTE` grants, raw nullable `proconfig`, raw `prosecdef`, and raw `proleakproof`. Missing converter/leakproof/security-context/configuration/role/ACL resolution, mixed-generation joins, or inferred Boolean state is capture failure rather than an unknown placeholder.

## Residual material gap

Converter definition, owner, ACL, local configuration, execution security context, and leakproof classification are now independently source-bound, but complete converter runtime/planner identity is not claimed. Converter strictness, volatility, parallel safety, planner support/cost and other independently mutable `pg_proc` facts remain later reviewed successors. No next auxiliary field is promoted into source until this successor obtains exact-head native/hosted acceptance and a bounded PostgreSQL 18 differential, avoiding unchecked accumulation of validation debt.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact head `65a71c5965d0e2153464419272c9ccf019b0a9ad`, but protected `.github/main` advanced after that head's last reconciliation from `130ce425f74ca4d21e122bc8fb2f7cbfba2db3a3` to exact `4fda7f504e58f72f0d9120c83b7da2b5cc824f25` via #2165. Raw GitHub state reports the PR mergeable but `mergeable_state=behind`; the owner lane therefore needs another ordinary/non-force protected-tip reconciliation before any current-head acceptance can be authoritative. Exact `65a71c59...` CodeQL PR `35219897360`, Security Scan `35219897339`, Python Security `35219897342`, SAST Semgrep `35219897353`, and Agent Review Runtime Quality CI `35219897310` remain queued/nonterminal, and formal review history has no qualifying independent approval. Even if those predecessor-head runs settle, they do not transfer after the required owner restack.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN / Ready / mergeable. Existing SAST/Security success does not erase the historical exact-head CodeQL failure; normal landing requires fresh compatible acceptance only after canonical workflow-owner reconciliation and terminal settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_RESTACK_OPEN / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted. Version/tag/package/immutable semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` ordinary/non-force reconciliation onto protected `main@4fda7f50...` -> exact-current terminal settlement plus qualifying independent approval -> fresh compatible #35 acceptance/normal landing -> one unchanged #46 native+hosted terminal GREEN including converter `proleakproof` evidence -> bounded PostgreSQL 18 live differential including raw converter `proleakproof` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review remaining converter auxiliary `pg_proc` surfaces -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
