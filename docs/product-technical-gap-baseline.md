# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline and CHANGELOG through exact `40e137925e46c5f4c758fc671ba699b59c78515e` are preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-40e13792.md` and `docs/archive/CHANGELOG-through-40e13792.md`. Earlier surfaces remain under `docs/archive/`; focused decision/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and its released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner. Consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued predecessor digest domain is rewritten. Ordinary `pg_constraint.contype='x'` EXCLUDE source identity retains the independent constraint/backing index, timing/enforcement/validation/period/no-inherit facts, ordered `conkey`/`conexclop`, `pg_operator.oprkind`, independently resolved `oprcom`, exact `oprcode -> pg_proc`, independent `oprresult`/`prorettype`, target-function `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prokind`, `prosecdef`, `proleakproof`, implementation language/definition (`prolang`, `prosrc`, `probin`, `prosqlbody`), exact target-function `proowner`, nullable `proconfig`, `proacl`, optional `prosupport`, positive-finite `procost`, nullable `protrftypes`, exact selected `pg_transform` rows and nonzero converter definition identity, operator-family/strategy, constraint/index namespace/name coupling, access-method exclusion capability, backing-index role/lifecycle, and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain separate owner families and are not reclassified as ordinary EXCLUDE.

## EXCLUDE transform-converter owner integrity

Review `5233862979` on exact predecessor `40e137925e46c5f4c758fc671ba699b59c78515e` confirmed a material residual gap in the converter predecessor: converter implementation identity was content-bound, but each nonzero converter function's independently mutable `pg_proc.proowner` was not.

PostgreSQL 18 stores function ownership in the dedicated `pg_proc.proowner` catalog field. `ALTER FUNCTION ... OWNER TO` can change that owner without redefining the converter function's input identity or source body. `CREATE TRANSFORM` separately requires ownership and `EXECUTE` privilege on specified converter functions. Owner, ACL, transform row, function body, and security mode therefore cannot be collapsed into one inferred fact.

The ordinary-forward repair adds `IndexExclusionConstraintOperatorProcedureTransformConverterDirection`, `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation`, `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSourceReceipt`, and `IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot`.

For every exact `(constraint, key_position, transform_type, direction)` represented by a nonzero predecessor converter, the successor requires exactly one owner observation. It repeats the exact converter schema/function coordinate from the predecessor, retains raw nonzero `proowner`, independently resolves the owner role name in the same source generation, and domain-separates the new digest from the frozen converter predecessor. Missing/extra directions, duplicate coordinates, zero positions, blank converter/role identifiers, zero owner OID, binding drift, and unknown receipt coordinates fail closed.

The contract remains observational. It does not require a particular owner and does not infer owner from transform creator, schema owner, session principal, ACL, `SECURITY DEFINER`, or any target-function fact.

### Ordinary-forward lineage

- finding review `5233862979`;
- structural source/compile RED `6b8f1daf317ccae54a9d8ebbd7a1144bf9881b5c`; the new public owner-evidence types did not yet exist and no executed compiler failure is claimed;
- production successor `11c9cbebab54685b8a06516d640f0b518a698a5b`;
- public composition `0ccc3415f999bce9353330affaff2b71335f4d08`;
- focused receipt-edge correction `fdc9c792db13148af66dcd4bb0f1fe414fca582a`;
- canonical receipt-location correction `fd3ced1bb5325ceabdd0c0c166410eda4a260035`;
- lossless pre-owner gap-baseline archive `22bc5cf3e0806e07d156f1ad9cabd66c2f8edf64`;
- lossless pre-owner CHANGELOG archive `20f7eabf5274b9a394332a41260c80fb62cc16e6`;
- PostgreSQL/NIST/APA decision record `8ab5ac9b63333df0e6111440a50371910b59a573`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-owner-integrity.md`.

Focused contract coverage includes exact owner provenance, owner-reassignment digest separation, complete nonzero converter coverage, converter-function binding drift, zero owner OID, duplicate owner coordinates, exact receipt lookup, and public composition. Because Rust 1.98 execution is unavailable in this runtime, executed coverage and the 100% owned edge/branch target are not established for the current head.

## PostgreSQL 18 authority and TRACEABILITY

Primary technical authority is PostgreSQL 18 `ALTER FUNCTION`, `CREATE TRANSFORM`, and `pg_proc`; NIST SP 800-53 Rev. 5 AC-3/AC-6 is used only as governance context for preserving controlling-principal evidence. The focused rationale, alternatives, rejected claims, privacy boundary, residual converter-function auxiliary-fact risk, and exact lineage are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-owner-integrity.md`.

Current converter-owner traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5233862979`
- structural RED: `6b8f1daf317ccae54a9d8ebbd7a1144bf9881b5c`
- production: `11c9cbebab54685b8a06516d640f0b518a698a5b`
- composition: `0ccc3415f999bce9353330affaff2b71335f4d08`
- focused correction: `fdc9c792db13148af66dcd4bb0f1fe414fca582a`
- location correction: `fd3ced1bb5325ceabdd0c0c166410eda4a260035`
- doctoring: `8ab5ac9b63333df0e6111440a50371910b59a573`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_owner.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_owner_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot`
- exact catalog fact: every nonzero transform converter function's `pg_proc.proowner` plus same-generation role-name resolution.

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the converter-owner contract plus every retained procedure/operator/Source Observation/relation-partition contract, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and all applicable hosted quality/security/review gates. Any source movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve every exact `conexclop` to one `pg_operator` row and every target `oprcode` to the exact target `pg_proc`; independently read all retained target-function facts through `protrftypes`; resolve every selected `(trftype, target prolang)` `pg_transform` row; preserve zero/nonzero FROM-SQL and TO-SQL directions; and resolve every nonzero converter OID to its exact converter `pg_proc`. For each nonzero converter, the same capture generation must now retain its exact signature/definition material plus raw nonzero `proowner` and resolved role name. An unresolved owner OID, missing role resolution, or generation mismatch is capture failure. Operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in that same v3 generation.

A real ordinary-EXCLUDE implementation function and actual source transform state are the positive control. Synthetic transform/converter/owner fixtures are unit-test distinguishability controls only and never evidence that those rows or roles exist in PostgreSQL.

## Residual material gap

Converter owner identity is now separately bound, but complete converter-function runtime/security identity is still not claimed. Independent auxiliary converter `pg_proc` facts — including ACL, nullable `proconfig`, `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, planner support/cost and other separately mutable fields — must not be inferred from target-function evidence, converter definition, or converter owner. The next material converter successor should continue this one-fact-at-a-time ordinary-forward discipline rather than widen the frozen converter-owner digest.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09`, OPEN / Draft / mergeable, with owner-qualified source repair complete. Fresh exact-current workflow state remains: SAST Semgrep `35177301881` and Agent Review Runtime Quality CI `35177301854` terminal-success; CodeQL PR `35177301855`, Security Scan `35177301852`, and Python Security `35177301847` queued/nonterminal. Terminal settlement and qualifying independent current-head review remain open.

Product bootstrap #35 remains a separate prerequisite. Ready/mergeable alone is not normal-landing authority while its CodeQL acceptance debt remains unresolved; a fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_AUXILIARY_ACL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_TYPES_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_COST_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable semantic publication, version/tag/package/SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including transform-converter owner evidence -> bounded PostgreSQL 18 live differential including exact converter `proowner`/role resolution and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review converter ACL as the next independently mutable auxiliary `pg_proc` surface -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
