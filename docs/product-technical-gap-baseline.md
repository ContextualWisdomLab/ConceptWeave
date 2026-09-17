# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active surface through exact `4256f8dc361ae8ed00980799d524a261b161b1a8` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-4256f8dc.md`; its matching CHANGELOG is preserved at `docs/archive/CHANGELOG-through-4256f8dc.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution/review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued digest domain is rewritten. Ordinary `pg_constraint.contype='x'` evidence retains independent constraint/backing-index identity, ordered `conkey`/`conexclop`, operator kind/commutator/result/procedure binding, target-function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, nonzero converter definition identity, converter owner identity, converter object-level `EXECUTE` ACL identity, converter nullable `proconfig` identity, converter raw `prosecdef` identity, converter raw `proleakproof` identity, operator-family/strategy, namespace/name/lifecycle/access-method controls, and exact v3 source-content-generation binding. Temporal `WITHOUT OVERLAPS`/`PERIOD` families remain separate.

## EXCLUDE transform-converter leakproof integrity

Review `5235830453` on exact predecessor `1153c0b6f121b5b99c533d73e3dc9e7b4305c276` found that every nonzero converter was definition-, owner-, ACL-, configuration-, and security-context-bound while raw same-row `pg_proc.proleakproof` remained independently mutable and ungoverned. PostgreSQL 18 permits `ALTER FUNCTION ... [NOT] LEAKPROOF` without changing those predecessor facts, and leakproof classification affects security-barrier/RLS evaluation ordering and planner-statistics access.

The ordinary-forward successor adds `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation`, provenance receipt, and `...LeakproofSnapshot`. Both Boolean states are representable; the snapshot is observational rather than a policy rule. It takes `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot` as its immutable direct predecessor and requires exactly one raw Boolean observation for every exact `(constraint, key_position, transform_type, direction)` converter coordinate with the same converter schema/function binding. Missing/extra directions, duplicate coordinates, zero positions, binding drift, and unknown receipt coordinates fail closed.

### Provenance-location P1 repair

Review `5236499737` on exact `4256f8dc361ae8ed00980799d524a261b161b1a8` found that the leakproof digest was collision-safe but its claimed collision-safe `canonical_location()` was not. `QualifiedTypeName` preserves exact quoted PostgreSQL identifiers, while raw `schema.type` serialization collapsed valid pairs such as `(payload.domain, json)` and `(payload, domain.json)` into the same path text. PostgreSQL 18 permits quoted identifiers to contain arbitrary characters except zero, so dots, slashes, percent signs, whitespace, and Unicode cannot be assumed absent.

Realistic RED `008c99a2e82105046df0b9ed7061bc6cb931ebb9` adds a focused collision regression. Minimum causal fix `3ce7e0bcf6d6a3a0935465147e2eaf5fd3a63e81` percent-encodes the transform schema and type components independently before the dot separator; ordinary ASCII identifier characters stay readable while every separator-capable/special UTF-8 byte is `%HH` encoded. This changes only provenance/error string identity, not the leakproof digest domain or typed lookup key. The pre-repair decision surface is preserved at `docs/archive/*through-4256f8dc.md`.

## PostgreSQL 18 and governance authority

Primary authority is PostgreSQL 18 `CREATE FUNCTION`, `ALTER FUNCTION`, lexical structure, row-security, rules/privileges, and planner-statistics security documentation. PostgreSQL explicitly allows changing leakproof classification and permits quoted identifiers containing arbitrary characters except code zero. NIST control-assessment guidance supports preserving independently testable security-relevant evidence, but PostgreSQL remains the technical authority for catalog and identifier semantics.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- leakproof finding review: `5235830453`
- location finding review: `5236499737`
- location RED: `008c99a2e82105046df0b9ed7061bc6cb931ebb9`
- location fix: `3ce7e0bcf6d6a3a0935465147e2eaf5fd3a63e81`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_leakproof.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_leakproof_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot`
- exact catalog fact: each nonzero selected converter function's raw same-row `pg_proc.proleakproof`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 is not available in the current execution host, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned statement/branch/edge coverage remain unexecuted. The location RED is logically executable against the predecessor because both valid quoted type coordinates serialized to the same string, but no native test execution is claimed. One unchanged exact head must obtain native and hosted terminal acceptance before any differential or publication claim.

The bounded PostgreSQL 18 live differential must retain every predecessor ordinary-EXCLUDE fact, resolve each selected `(trftype, target prolang)` `pg_transform` row, resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, and independently capture converter definition, raw `proowner` plus role resolution, raw `proacl` NULL-state plus canonical object-level `EXECUTE` grants, raw nullable `proconfig`, raw `prosecdef`, and raw `proleakproof`. Missing converter/leakproof/security-context/configuration/role/ACL resolution, mixed-generation joins, or inferred Boolean state is capture failure rather than an unknown placeholder.

## Residual material gap

Converter definition, owner, ACL, local configuration, execution security context, leakproof classification, and collision-safe leakproof provenance location are source-bound, but complete converter runtime/planner identity is not claimed. Converter strictness, volatility, parallel safety, planner support/cost and other independently mutable `pg_proc` facts remain later reviewed successors. No next auxiliary field is promoted into source until this successor obtains exact-head native/hosted acceptance and a bounded PostgreSQL 18 differential, avoiding unchecked accumulation of validation debt.

## Canonical prerequisite state

Review `5237380716` found that canonical `ContextualWisdomLab/.github#2106` predecessor `6be76f6594b784d93d7be33fa1677b2419bf07b2` had retained a stale delta in `tests/test_opencode_agent_contract.py`: the test restored a separate `coverage-source-tree` job even though protected `.github/main@4e56ff0fd10e8d56e9af273881a7eb65d7e4f513` had already folded those reads into `validate-pr-metadata`, and #2106 does not modify the inherited OpenCode dispatch workflow. The reconciled branch therefore tested a job absent from its actual workflow tree.

Ordinary-forward owner repair `0e9412f93bb9d4a08a689f62a563dc666ae91f88` adopts protected main's contract-test blob unchanged. Fresh compare is 57 ahead / 0 behind with the effective #2106 delta reduced from nine paths to eight; GitHub reports the Draft mergeable. This repair changes no CodeQL/bootstrap behavior and does not weaken any gate or permission boundary.

Exact `0e9412f9...` naturally generated CodeQL PR `35236110056`, SAST Semgrep `35236109832`, Security Scan `35236109955`, Python Security `35236109883`, and Agent Review Runtime Quality CI `35236110143`; all are queued/nonterminal at this decision surface. No qualifying independent current-head `APPROVED` exists. Predecessor checks/reviews remain historical only.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN / Ready / mergeable. Existing SAST/Security success does not erase the historical exact-head CodeQL failure; normal landing requires fresh compatible acceptance only after canonical workflow-owner terminal settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_LOCATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_DIFFERENTIAL_OPEN / CANONICAL_WORKFLOW_OWNER_CURRENT_BASE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted. Version/tag/package/immutable semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106@0e9412f9...` exact-current terminal settlement plus qualifying independent approval -> fresh compatible #35 acceptance/normal landing -> one unchanged #46 native+hosted terminal GREEN including converter `proleakproof` and quoted-identifier location regression -> bounded PostgreSQL 18 live differential including raw converter `proleakproof` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review remaining converter auxiliary `pg_proc` surfaces -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.