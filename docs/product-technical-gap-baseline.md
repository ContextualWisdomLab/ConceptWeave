# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete predecessor baseline immediately before the raw EXCLUDE-operator-kind currentization is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-c9777662.md`. Earlier detailed baselines remain in `docs/archive/`, and exact authority/decision records remain in `docs/doctoring/`. Head-specific execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained Source Observation authority

All valid predecessor repairs preserved in `docs/archive/product-technical-gap-baseline-through-c9777662.md` remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

The active representation retains relation/index partition topology; relation rowtype/typmod and key/INCLUDE mapping; operator-family and exclusion semantics; expression/predicate and relation-`Var` equality; collation identity/provider/version/database-encoding semantics; partition declaration/collation coherence; foreign-table partitioned-index behavior; valid-parent/valid-child composition; key-constraint child presence/parentage/inheritance; and ordinary partitioned EXCLUDE constraint catalog identity.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; `conparentid`; `conislocal`; `coninhcount`; raw `connoinherit`; `condeferrable`; `condeferred`; `conenforced`; `convalidated`; `conperiod`; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc` implementation procedure; independently resolved `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` result identity; raw `pg_proc.proretset=false` for scalar EXCLUDE enforcement; independently resolved `pg_constraint.connamespace`; mutually exclusive catalog-family shape; exact constraint-name/backing-index-name coupling; independently observed access-method `can_exclude`; independently resolved backing-index `pg_class.relnamespace`; exact `pg_index.indisunique`/`indisprimary`/`indisexclusion`; explicit ready/valid/live lifecycle integrity; and exact v3 source-content generation binding across the backing-index name/namespace/lifecycle chain.

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

## EXCLUDE raw `pg_proc.proretset` scalar-enforcement integrity

Review `5228163996` on exact predecessor `ef67a083c57f9dca142ce09b691a76fc5e1c03d3` found the next bounded P1. The retained operator-procedure/result chain proves exact `oprcode -> pg_proc`, exact argument identity, and independent `oprresult == prorettype == pg_catalog.bool`, but it does not preserve `pg_proc.proretset`. A `SETOF bool` implementation therefore has the same normalized result type even though its execution cardinality differs.

PostgreSQL 18 ordinary EXCLUDE enforcement in `src/backend/executor/execIndexing.c` invokes each exclusion procedure via scalar `OidFunctionCall2Coll(...)` and immediately applies `DatumGetBool(...)`. `src/backend/parser/parse_oper.c` independently exposes `get_func_retset(opform->oprcode)` when building an operator expression, confirming that return cardinality is a distinct procedure fact rather than a property derivable from `prorettype`.

The ordinary-forward repair lineage is:

- structural source/compile RED `c1fb88d1a29be1ac7503b1eb04f574e5a32a2d8b`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_scalar_contract.rs`; the successor types did not yet exist, so no executed compiler failure is claimed;
- production successor `ed12d481d3ea8981ea7917518d7f917964f189b5`, followed immediately by canonical digest-encoding repair `a77b022acf019e10a106e5daa5586c7c1ac09fff`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_scalar.rs`;
- public composition `8109a8baaebb385428c70c685d47facfc81c5f1d`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- focused authority/decision record `9d3adf977c3bfe578d244e2c4a27202190fa7795`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-scalar-integrity.md`.

`IndexExclusionConstraintOperatorProcedureScalarSnapshot` receives the exact result and operator-kind predecessors. It reconstructs the kind snapshot from the supplied result snapshot before use, so a mismatched predecessor generation fails closed. Every exact constraint/key position then requires one independently observed raw `proretset` fact bound to the same stable operator and exact `oprcode` procedure; only `false` is admitted. Missing/duplicate evidence, operator/procedure binding drift, set-returning state, zero position, and unknown receipt coordinates fail closed. The current operator-kind digest plus exact coordinate/key, stable operator/procedure signatures, and raw Boolean cardinality flag enter a new domain-separated successor digest.

This is intentionally scoped to EXCLUDE enforcement capability. It does not redefine generic PostgreSQL function semantics, ban SRFs elsewhere, or claim that generic `CREATE OPERATOR` syntax itself validates every runtime cardinality property needed by exclusion enforcement.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority for the current decision is PostgreSQL Global Development Group, *PostgreSQL 18 documentation: `pg_proc`*, *CREATE OPERATOR*, and the current `REL_18_STABLE@1ac292cb1436c788fb6ea29551b0fe459e2cb340` source paths `src/backend/executor/execIndexing.c` and `src/backend/parser/parse_oper.c`. The focused APA-style decision record is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-scalar-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5228163996`
- production: `IndexExclusionConstraintOperatorProcedureScalarObservation`, `IndexExclusionConstraintOperatorProcedureScalarSnapshot`, `IndexExclusionConstraintOperatorProcedureScalarSourceReceipt`
- predecessors: `IndexExclusionConstraintOperatorKindSnapshot`, rebound to `IndexExclusionConstraintOperatorResultSnapshot`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_scalar_contract.rs`
- catalog fact: `pg_proc.proretset`
- executor assumption: scalar two-argument `OidFunctionCall2Coll(...)` followed by `DatumGetBool(...)`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head still must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated operator procedure-scalar/kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 differential must resolve every exact `conexclop` OID to one `pg_operator` row and independently read at least `oprkind`, `oprcom`, `oprresult`, and `oprcode`; follow `oprcode` to the exact `pg_proc` row; independently read both `prorettype` and raw `proretset`; resolve result types independently; and retain operator-family/strategy plus exact backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `oprkind='b'` and `proretset=false` must come from their exact catalog rows. Deriving either from normalized signatures or another catalog field is invalid evidence.

Positive control is a real governed scalar binary operator. Negative controls for the current cardinality validation are synthetic/corrupt source observations with `proretset=true`; the differential must not claim that normal PostgreSQL DDL can create an enforcement-valid ordinary EXCLUDE constraint with that state.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_EXACT_SOURCE_GENERATION_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_DIFFERENTIAL_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those acceptance obligations remain open. Publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after an unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `oprkind`, raw `proretset`, and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
