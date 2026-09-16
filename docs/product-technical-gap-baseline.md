# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete predecessor baseline immediately before the raw EXCLUDE-operator-kind currentization is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-c9777662.md`. Earlier detailed baselines remain in `docs/archive/`, and exact authority/decision records remain in `docs/doctoring/`. Head-specific execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained Source Observation authority

All valid predecessor repairs preserved in `docs/archive/product-technical-gap-baseline-through-c9777662.md` remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

The active representation retains relation/index partition topology; relation rowtype/typmod and key/INCLUDE mapping; operator-family and exclusion semantics; expression/predicate and relation-`Var` equality; collation identity/provider/version/database-encoding semantics; partition declaration/collation coherence; foreign-table partitioned-index behavior; valid-parent/valid-child composition; key-constraint child presence/parentage/inheritance; and ordinary partitioned EXCLUDE constraint catalog identity.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; `conparentid`; `conislocal`; `coninhcount`; raw `connoinherit`; `condeferrable`; `condeferred`; `conenforced`; `convalidated`; `conperiod`; ordered raw `conkey`; ordered resolved `conexclop`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc` implementation procedure; independently resolved `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` result identity; independently resolved `pg_constraint.connamespace`; mutually exclusive catalog-family shape; exact constraint-name/backing-index-name coupling; independently observed access-method `can_exclude`; independently resolved backing-index `pg_class.relnamespace`; exact `pg_index.indisunique`/`indisprimary`/`indisexclusion`; explicit ready/valid/live lifecycle integrity; and exact v3 source-content generation binding across the backing-index name/namespace/lifecycle chain.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their key/foreign-key owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_operator.oprkind` integrity

Review `5227579925` on exact predecessor `17a697f8b658d90f5c361a1abc489e4174df1429` found a catalog-source P1. Stable `QualifiedOperatorSignature` preserves schema, operator name, and resolved left/right operand types, and retained successors already prove self-commutator, implementation procedure, and Boolean result contract. None of those facts proves that the exact source `pg_operator` row was itself observed with binary `oprkind`.

PostgreSQL 18 defines `pg_operator.oprkind` as independent catalog state: `b` is infix/binary (“both”) and `l` is prefix (“left”); `oprleft` is zero for prefix operators. `CREATE OPERATOR` requires both `LEFTARG` and `RIGHTARG` for a binary operator and only `RIGHTARG` for a prefix operator. An ordinary EXCLUDE element is a commutative binary comparison. The adapter must therefore read raw `oprkind` from the exact `conexclop` row instead of deriving binary state from its own normalized two-operand signature.

The ordinary-forward repair lineage is:

- structural source/compile RED `3ddf0b09c50b8f1a60a677a4fe4be268ae679120`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_kind_contract.rs`; the successor types did not yet exist, so no executed compiler failure is claimed;
- production successor `69c180a37ce2ec0d34070733806ab5419c081375`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_kind.rs`;
- public composition `da7567871d61f7bb2f4ba284a62660c4d5cd2928`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- focused authority/decision record `c9777662921be7c8d19c3c7844178105d6356d4e`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-kind-integrity.md`;
- predecessor decision surfaces preserved losslessly in `docs/archive/product-technical-gap-baseline-through-c9777662.md` and `docs/archive/CHANGELOG-through-c9777662.md`.

`IndexExclusionConstraintOperatorKindSnapshot` derives the expected exact constraint/key inventory from `IndexExclusionConstraintOperatorResultSnapshot`. Every position requires exactly one raw-kind observation, the repeated operator must equal the exact predecessor operator, and only `oprkind='b'` is admitted. Missing evidence, duplicate coordinate, operator-binding drift, prefix `l`, unknown raw kind, zero key position, or unknown receipt coordinate fails closed. The raw discriminator, exact coordinate/key position, stable operator signature, and predecessor digest enter a new domain-separated digest/provenance family.

The contract deliberately does not add `oprkind` to `QualifiedOperatorSignature` or rewrite existing operator identities. Stable semantic identity and raw catalog-row shape are related but separate concerns.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority for the current decision is PostgreSQL Global Development Group, *PostgreSQL 18 documentation: `pg_operator`*, *CREATE OPERATOR*, and *CREATE TABLE*. The focused APA-style decision record is `docs/doctoring/postgresql-index-exclusion-constraint-operator-kind-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5227579925`
- production: `IndexExclusionConstraintOperatorKindObservation`, `IndexExclusionConstraintOperatorKindSnapshot`, `IndexExclusionConstraintOperatorKindSourceReceipt`
- predecessor: `IndexExclusionConstraintOperatorResultSnapshot`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_kind_contract.rs`
- catalog fact: `pg_operator.oprkind`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head still must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated operator-kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 differential must resolve every exact `conexclop` OID to one `pg_operator` row and independently read at least `oprkind`, `oprcom`, `oprresult`, and `oprcode`; follow `oprcode` to exact `pg_proc.prorettype`; resolve result types independently; and retain operator-family/strategy plus exact backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `oprkind='b'` must come from the row itself. Deriving it from two normalized operand types, operator name, commutator state, operator-family membership, strategy, or procedure signature is invalid evidence.

Positive control is a real governed binary operator. Negative controls for raw-kind validation are synthetic/corrupt source observations with `l` or another invalid discriminator; the differential must not claim that normal PostgreSQL DDL can create such an ordinary EXCLUDE constraint.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_EXACT_SOURCE_GENERATION_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_DIFFERENTIAL_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those acceptance obligations remain open. Publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after an unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `oprkind` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
