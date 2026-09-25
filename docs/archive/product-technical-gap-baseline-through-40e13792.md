# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline and CHANGELOG through exact `e8a60dc4a85c8850eacecb3e88ff6b60e6a48700` are preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-e8a60dc4.md` and `docs/archive/CHANGELOG-through-e8a60dc4.md`. Earlier surfaces remain under `docs/archive/`; focused decision/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and its released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner. Consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued predecessor digest domain is rewritten. Ordinary `pg_constraint.contype='x'` EXCLUDE source identity retains the independent constraint/backing index, timing/enforcement/validation/period/no-inherit facts, ordered `conkey`/`conexclop`, `pg_operator.oprkind`, independently resolved `oprcom`, exact `oprcode -> pg_proc`, independent `oprresult`/`prorettype`, target-function `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prokind`, `prosecdef`, `proleakproof`, implementation language/definition (`prolang`, `prosrc`, `probin`, `prosqlbody`), exact `proowner`, nullable `proconfig`, `proacl`, optional `prosupport`, positive-finite `procost`, nullable `protrftypes`, operator-family/strategy, constraint/index namespace/name coupling, access-method exclusion capability, backing-index role/lifecycle, and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain separate owner families and are not reclassified as ordinary EXCLUDE.

## EXCLUDE implementation-function transform-converter integrity

Review `5233258873` on exact predecessor `e8a60dc4a85c8850eacecb3e88ff6b60e6a48700` confirmed the material gap already declared by the transform-type predecessor: `pg_proc.protrftypes` proves transform selection but does not authenticate the mutable `pg_transform` row for the selected `(type, language)` pair.

PostgreSQL 18.6 stores `pg_transform.trftype` and `trflang` separately from optional `trffromsql` and `trftosql` converter-function OIDs. A transform may provide either direction. A nonzero FROM-SQL converter takes one `internal` argument and returns `internal`; a nonzero TO-SQL converter takes one `internal` argument and returns the transform type. Converter function definitions remain independently mutable, so converter name/signature alone is insufficient runtime identity.

The ordinary-forward repair adds `IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition`, `IndexExclusionConstraintOperatorProcedureTransformConverterFunction`, `IndexExclusionConstraintOperatorProcedureTransformConverterBinding`, `IndexExclusionConstraintOperatorProcedureTransformConverterObservation`, `IndexExclusionConstraintOperatorProcedureTransformConverterSourceReceipt`, and `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot`.

For every exact `(constraint, key_position)`, the successor binds the transform-type predecessor to the same source connection, policy binding, extractor revision and observation time; repeats the exact governed operator and target `oprcode` function; cross-checks the target language against retained target-function definition evidence; requires no transform row when `protrftypes` is NULL and exactly one same-generation row per selected type otherwise; preserves FROM-SQL and TO-SQL absence/presence independently; validates each nonzero converter's one-`pg_catalog.internal` call boundary and direction-specific return type; and content-binds converter implementation language plus exact `prosrc`/optional `probin`/optional `prosqlbody` under a separate digest while discarding plaintext implementation material.

Transform rows are canonicalized by qualified transform type. Duplicate selected rows, missing rows, target-language drift, wrong converter signatures, zero positions, duplicate/missing outer coordinates, operator/target-function drift, and unknown receipt coordinates fail closed. This remains source observation rather than transform policy: neither both directions nor a specific procedural language are required.

### Ordinary-forward lineage

- finding review `5233258873`;
- structural source/compile RED `606916ee910ac611b208aaa587bdf933159459da`; public converter-evidence types did not yet exist, so no executed compiler failure is claimed;
- initial production successor `6ea573c5540236fba13e993cdfa295a42977ad54`;
- public composition `433c416ada9151b9db839d130fce444384e5519c`;
- production API refactor `fa4802d71a9ea282a7ceaaf97a10193ca2f77045`, replacing an eight-argument constructor boundary with explicit converter-definition material instead of adding a Clippy waiver;
- focused contract adaptation `a82c50dd9af22d36096b2ee5984e35a784787934`;
- PostgreSQL/APA decision record `c254d7365aee5d99d354916827e6634b65871e6f`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-integrity.md`;
- lossless pre-converter CHANGELOG archive `0da5ec16ea16064c516e2024cb0a2460922cdcaa`;
- lossless pre-converter gap-baseline archive `fa955b5bdd1a588c37f036cd2551311510168c7e`.

Focused contract coverage currently includes exact converter-row provenance, converter implementation replacement digest separation, one-direction-only transforms, missing-row rejection, NULL-`protrftypes` isolation, target-language drift, empty-direction rejection, FROM-SQL/TO-SQL return-type checks, required `internal` argument, operator binding, one-based positions, receipt coordinates, and public composition. Because Rust 1.98 execution is unavailable in this runtime, executed coverage and the 100% owned edge/branch target are not established for the current head.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_transform`, PostgreSQL 18.6 `CREATE TRANSFORM`, and PostgreSQL 18.6 `pg_proc`. The focused rationale, alternatives, rejected claims, privacy boundary, residual converter-function auxiliary-fact risk, and exact commit lineage are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-integrity.md`.

Current transform-converter traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5233258873`
- structural RED: `606916ee910ac611b208aaa587bdf933159459da`
- production: `6ea573c5540236fba13e993cdfa295a42977ad54`
- composition: `433c416ada9151b9db839d130fce444384e5519c`
- production refactor: `fa4802d71a9ea282a7ceaaf97a10193ca2f77045`
- focused contract: `a82c50dd9af22d36096b2ee5984e35a784787934`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot`
- supporting target-language evidence: `IndexExclusionConstraintOperatorProcedureDefinitionSnapshot`
- exact catalog facts: `pg_transform.trftype`, `trflang`, `trffromsql`, `trftosql`, and each nonzero converter's same-generation `pg_proc` signature/definition material.

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the transform-converter contract plus every retained procedure/operator/Source Observation/relation-partition contract, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and all applicable hosted quality/security/review gates. Any source movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve every exact `conexclop` to one `pg_operator` row and every target `oprcode` to the exact target `pg_proc`; independently read all retained target-function facts through `protrftypes`; preserve NULL `protrftypes`; and, for each selected transform type, resolve the same-generation `(trftype, target prolang)` `pg_transform` row. `trffromsql=0` and `trftosql=0` remain explicit language-default absence, not unresolved identity. Every nonzero converter OID must resolve to the exact converter `pg_proc`, its one-`internal` argument, direction-specific return type, implementation language, `prosrc`, `probin`, and `prosqlbody`. Missing selected transform rows, unresolved nonzero converter OIDs, or generation disagreement are capture failures. Operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in that same v3 generation.

A real ordinary-EXCLUDE implementation function and actual source transform state are the positive control. Synthetic transform/converter fixtures are unit-test distinguishability controls only and never evidence that those rows exist in PostgreSQL.

## Residual material gap

Converter implementation identity is now content-bound, but complete converter-function runtime/security identity is not yet claimed. Independent auxiliary converter `pg_proc` facts — including owner, ACL, configuration, security mode, leakproofness, strictness, volatility, parallel safety, planner support/cost and other separately mutable fields — must not be inferred from target-function evidence or from converter source definition. If the released semantic contract requires that stronger claim, it is the next separately reviewed successor rather than an expansion of this frozen digest domain.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09`, OPEN / Draft / mergeable, with its owner-qualified source repair complete. Fresh exact-current evidence has advanced: SAST Semgrep `35177301881` and Agent Review Runtime Quality CI `35177301854` are terminal-success; CodeQL PR `35177301855`, Security Scan `35177301852`, and Python Security `35177301847` remain queued. Terminal settlement and qualifying independent current-head review therefore remain open.

Product bootstrap #35 remains a separate prerequisite. Ready/mergeable alone is not normal-landing authority while its required CodeQL acceptance debt remains unresolved; a fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_TYPES_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_AUXILIARY_PROC_IDENTITY_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_COST_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable semantic publication, version/tag/package/SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including transform-converter evidence -> bounded PostgreSQL 18 live differential including exact selected `pg_transform` converter rows and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review converter auxiliary `pg_proc` identity as the next material catalog gap if complete runtime/security identity is required -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
