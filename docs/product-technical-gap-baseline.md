# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete predecessor baseline through exact `3e57508fe488aa059122206a7f21a3576babbe63` is preserved at `docs/archive/product-technical-gap-baseline-through-3e57508f.md`; older archived checkpoints remain under `docs/archive/`. Decisions after that checkpoint are retained in focused `docs/doctoring/postgresql-index-exclusion-constraint-*.md` records and Git history. Exact-head execution evidence never transfers after head movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a` unless fresh GitHub state says otherwise. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 remains the repository-owned Product workflow prerequisite while protected/default `main` lacks that accepted workflow. Canonical reusable-workflow ownership remains in `ContextualWisdomLab/.github`.

## Retained Source Observation authority

Every valid predecessor repair remains retained without rewriting an issued digest domain. The active representation covers relation/index partition topology, rowtype/typmod and key/INCLUDE mapping, operator-family/exclusion semantics, expression/predicate and relation-`Var` equality, collation identity/provider/version/database-encoding semantics, partition declaration/collation coherence, foreign-table partitioned-index behavior, valid-parent/valid-child composition, key-constraint child presence/parentage/inheritance, and ordinary partitioned EXCLUDE constraint catalog identity.

For ordinary `contype='x'` EXCLUDE constraints, governed evidence includes the independent constraint object; exact `conindid`; `conparentid`; `conislocal`; `coninhcount`; raw `connoinherit`; `condeferrable`; `condeferred`; `conenforced`; `convalidated`; `conperiod`; ordered raw `conkey`; resolved ordered `conexclop`; independently observed resolved `connamespace`; relation-wide captured-constraint `conname` integrity; and the exact backing index's independently observed role, immediacy, and exclusion semantics. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain in their dedicated key/foreign-key owner families rather than being reclassified as ordinary EXCLUDE.

Focused doctoring records preserve the exact review IDs, repair commits, primary-source evidence, rejected alternatives, and required differential controls for timing/immediacy, backing-index role, enforcement, validation, no-inherit lifecycle, period, key vector, exclusion operators, name integrity, and namespace integrity.

## EXCLUDE constraint namespace integrity

Review `5222226880` on exact predecessor `250ed695fc36f83e97a26b004d1d7728712ef2cc` found a source-integrity P1: the ordinary EXCLUDE coordinate used the owning relation schema but did not independently retain `pg_constraint.connamespace`. An extractor or inconsistent catalog tuple could therefore present a constraint namespace different from the owning relation namespace and still collapse into the same governed identity.

Pinned PostgreSQL 18 `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` is the primary authority. `index_constraint_create()` obtains `namespaceId = RelationGetNamespace(heapRelation)` and passes that namespace independently to `CreateConstraintEntry()` along with relation OID, backing index OID, key attributes, exclusion operators, inheritance state, and period state. `pg_constraint.connamespace` is therefore a material source fact even though normal DDL makes it equal to the owning relation namespace.

The ordinary-forward lineage is:

- source/compile contract `a4f56f9432bb0d8d368d1b36f89d59661623fd2b`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_namespace_contract.rs`;
- production successor `b71fb962177302377a3bd913e185e2ad587025bb`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_namespace.rs`;
- public composition `c7dcee510671be162908b75a4c7b4723e914adc0`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- focused doctoring `806b372f3514e3cb29918363990f26aba8f5f1ee`, `docs/doctoring/postgresql-index-exclusion-constraint-namespace-integrity.md`;
- CHANGELOG currentization `2cb49dc3b81ce54d1ec4e04fbe6ab989c9859e2b`.

`IndexExclusionConstraintNamespaceSnapshot` requires exactly one explicit resolved namespace observation for every predecessor ordinary EXCLUDE constraint, keeps the raw resolved schema name in a new domain-separated digest and provenance receipt, and fails closed when it differs from the owning relation schema. It does not derive the namespace from relation metadata, rewrite an issued predecessor digest, or introduce schema-wide constraint-name uniqueness; same-name constraints on different relations remain legal.

## Current state

**INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NAME_INTEGRITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NAME_INTEGRITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_KEY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_EXACT_PREDECESSOR_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_PERIOD_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PERIOD_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_RAW_STATE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_TIMING_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PARTITION_SOURCE_REPAIRED / INDEX_CONSTRAINT_INHERITANCE_STATE_SOURCE_REPAIRED / INDEX_CONSTRAINT_PARENTAGE_SOURCE_REPAIRED / SOURCE_OBSERVATION_RETAINED / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is inferred from source commits. One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused EXCLUDE namespace/name-integrity/index-role/immediacy/operator/key/period/no-inherit/validation/enforcement/timing contracts, all retained Source Observation/relation-partition contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets acceptance.

The PostgreSQL 18 bounded live differential must independently read ordinary EXCLUDE `contype`, `connamespace`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`, `conkey`, and `conexclop`, plus supporting `pg_index.indkey`, `indisunique`, `indisprimary`, `indisexclusion`, `indimmediate`, and resolved backing-index exclusion semantics in the same bounded source observation. `connamespace` must be resolved through `pg_namespace` independently of `conrelid` relation namespace before equality is checked. Same-name constraints on different relations are a required positive control; no unobserved constraint family may be synthesized.

## Next causal work

1. Converge the canonical `.github` workflow owner and obtain fresh compatible acceptance for Product bootstrap #35; merge #35 normally only when required gates are terminal GREEN.
2. Obtain one unchanged #46 head with repository-pinned Rust 1.98 native GREEN plus hosted Product/security/dependency/review terminal GREEN.
3. Run the PostgreSQL 18 bounded live differential for the retained ordinary EXCLUDE catalog bundle, including independent `connamespace` resolution, relation-wide captured-name integrity, backing-index role, timing/immediacy, key/operator, and partition lifecycle controls; then obtain fresh terminal GREEN on the resulting unchanged head.
4. Continue bounded review for materially relevant source-catalog state rather than claiming catalog completeness by field enumeration.
5. Only then adopt the complete #46 child ordinary/non-force into #45, obtain fresh #45 acceptance, and propagate through #6.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, or premature publication/release is authorized.
