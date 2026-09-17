# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline remains preserved at `docs/archive/product-technical-gap-baseline-through-12179434.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, procedure-volatility, procedure-parallel-safety, procedure-kind, procedure-security-definer, procedure-leakproof, procedure-definition, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; raw `pg_proc.provolatile`; raw `pg_proc.proparallel`; raw `pg_proc.prokind='f'`; raw `pg_proc.prosecdef`; raw `pg_proc.proleakproof`; exact implementation language/definition material from `prolang`/`prosrc`/`probin`/`prosqlbody`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE implementation-definition integrity

Review `5230742538` on exact predecessor `12179434ec7b11e8730a6ff4892a6f3a60973c28` found that the retained chain proved the exact ordinary-EXCLUDE implementation function and many independently observed `pg_proc` attributes, but the stable `QualifiedProcedureSignature` did not bind executable definition material.

PostgreSQL 18.6 permits `CREATE OR REPLACE FUNCTION` to replace an existing function definition without changing its name or input argument types and without breaking dependent objects that refer to the function. `pg_proc.prolang` chooses the implementation language/call interface; `prosrc` carries source text or a compiled-function link symbol; `probin` carries additional invocation information and is used for dynamically loaded C functions; and `prosqlbody` carries the pre-parsed body when a SQL function uses SQL-standard body notation. The same governed operator/function signature and retained auxiliary attributes can therefore execute different code if these definition facts are omitted.

The repair consumes the independently resolved `pg_language.lanname` plus exact same-row `prosrc`, optional `probin`, and optional `prosqlbody`, derives a dedicated SHA-256 definition-material digest, and discards the plaintext implementation material. Receipts expose only language identity and the digest. Raw `prosqlbody` `pg_node_tree` text is bound only as source-generation evidence; ConceptWeave does not claim cross-major semantic equivalence for PostgreSQL internal node serialization.

Definition lineage remains:

- finding review `5230742538`;
- structural source/compile contract `f4f652e3f9385b0495ff2ae40a69a7733c971d11`;
- initial production successor `cc7e48e1d823634bcdb74c906f4deea27819a355`;
- public composition `1191dc74d5878825f5db6949bbf5f1f222514cae`;
- production refinement `499e5336609a52145ef85a1d9d63effdaab6bab2`, replacing a temporary high-arity API with a definition-material value object rather than suppressing strict Clippy;
- focused contract refinement `3dd47a11ed652ff3eb2bb14daf00bd4c23bac959`;
- PostgreSQL/APA decision record `0a0889aadb69fdb2fa31c56f46f6c55f3463c374`.

`IndexExclusionConstraintOperatorProcedureDefinitionSnapshot` derives the exact `(constraint coordinate, key_position)` inventory from `IndexExclusionConstraintOperatorProcedureLeakproofSnapshot`. Every governed position requires one definition observation bound to the same stable operator and exact `oprcode` function. Missing/duplicate evidence, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. Changes to `prolang` identity, `prosrc`, optional `probin`, or optional `prosqlbody` change the definition digest and therefore the successor snapshot digest. `NULL` versus an empty optional field remains distinct.

## EXCLUDE implementation-function owner integrity

Review `5231157230` on exact definition predecessor `f43ec0d9ae878b0cad292fc9f927c63350f8c2c3` found the next bounded P1: the governed chain preserved `prosecdef` and executable definition content but not `pg_proc.proowner`. PostgreSQL stores `proowner` independently, and `ALTER FUNCTION ... OWNER TO` can change it without changing the function's input identity. PostgreSQL explicitly states that a `SECURITY DEFINER` function subsequently executes as the new owner, so omitted owner state can collapse materially different execution authority into one governed identity.

The repair layers `IndexExclusionConstraintOperatorProcedureOwnerSnapshot` over the exact definition predecessor. Every exact ordinary-EXCLUDE coordinate/key position receives one owner observation bound to the same operator and exact `oprcode` function. The observation preserves nonzero raw `pg_proc.proowner` plus the exact role name resolved through the publicly readable `pg_roles.oid -> rolname` view. Both raw OID and resolved role name enter the new domain-separated successor digest. This distinguishes owner transfer, role rename, and dropped/recreated same-name role states without reading `pg_authid` password material.

Owner lineage:

- finding review `5231157230`;
- structural source/compile RED `0ebd61cb44957d5d5278c9d70c7bcf63ec7bd94c`; the contract referenced the new public owner types before production existed, so no executed compiler failure is claimed;
- production successor `39382ad29a35b428c8b431a46b4071aa202603ec`;
- public composition `51d8d43af1deb230fa475f211c87ccd890bdbe68`;
- PostgreSQL/APA decision record `73bd9c86b16c77f2b53c78bf19438d486ee6e8e`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-owner-integrity.md`;
- active CHANGELOG currentization `d470cf19072207c702abfec59de4d6b2be8bc40a`.

Owner invariants are exact completeness, unique coordinates, exact operator/function predecessor binding, nonzero owner OID, nonblank independently resolved role name, one-based key positions, and exact receipt coordinates. Ownership remains observational for both SECURITY INVOKER and SECURITY DEFINER functions. Role privilege closure, role membership, and role-local configuration are separate concerns and are not inferred by this layer.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `CREATE FUNCTION`/`ALTER FUNCTION`, PostgreSQL 18 `pg_proc`, and PostgreSQL 18 `pg_roles`. `ALTER FUNCTION` establishes that function ownership can change without changing input identity and states the execution-principal consequence for SECURITY DEFINER functions. `pg_proc` identifies `proowner` as the owner OID; `pg_roles` exposes public `oid`/`rolname` resolution without password disclosure. Focused rationale and rejected alternatives are in the two current doctoring records for definition and owner integrity.

Current owner traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5231157230`
- structural RED: `0ebd61cb44957d5d5278c9d70c7bcf63ec7bd94c`
- production: `39382ad29a35b428c8b431a46b4071aa202603ec`
- composition: `51d8d43af1deb230fa475f211c87ccd890bdbe68`
- doctoring: `73bd9c86b16c77f2b53c78bf19438d486ee6e8e`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_owner.rs`
- test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_owner_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureDefinitionSnapshot`
- exact catalog facts: `pg_proc.proowner`, resolved `pg_roles.oid`, `pg_roles.rolname`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-owner/definition/leakproof/security-definer/kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, and `prosqlbody`; resolve the exact same-generation `proowner` through `pg_roles.oid -> rolname`; resolve `prolang` to exact `pg_language.lanname`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. Owner and definition material must come from the exact joined `pg_proc` row and may not be reconstructed from routine name, schema ownership, session identity, decompiled DDL, or another generation.

A real ordinary-EXCLUDE implementation function with its actual owner/language/definition tuple is the positive control. Synthetic owner OID/name and source/link/body variants are digest-distinguishability unit controls only; they are not evidence that arbitrary principals or replacement bodies are valid PostgreSQL exclusion-operator configurations.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains exact `653466de1520c966addbeec985b9db2d21421d09` on base `a9c6477d326b84411f492ba4cac29f52deebcac3`, OPEN / Draft / mergeable. Commit `653466de...` repaired the two prior owner-qualified durable repository identity REDs. Fresh exact-current workflow inventory remains nonterminal: CodeQL PR `35177301855` is pending; Security Scan `35177301852`, Python Security `35177301847`, SAST Semgrep `35177301881`, and Agent Review Runtime Quality CI `35177301854` are queued. ConceptWeave does not compete with that owner lane or manufacture wake/no-op evidence.

Product bootstrap #35 remains source-stable at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` until the canonical workflow owner reaches compatible terminal acceptance and #35 receives fresh current-generation acceptance.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including the new owner successor -> bounded PostgreSQL 18 live differential including exact owner plus implementation-definition material and every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
