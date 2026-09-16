# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete predecessor baseline through exact `573915a77cd85b66dd5f81bb802594a64c3f5318` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-573915a7.md`. Earlier predecessor baselines remain at `docs/archive/product-technical-gap-baseline-through-3e57508f.md` and `docs/archive/product-technical-gap-baseline-through-08847df6.md`. Detailed source decisions remain in `docs/doctoring/`. Exact-head execution evidence never transfers after head movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a` unless fresh GitHub state says otherwise. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 remains the repository-owned Product workflow prerequisite while protected/default `main` lacks that accepted workflow. Canonical reusable-workflow ownership remains in `ContextualWisdomLab/.github`.

## Retained Source Observation authority

All valid predecessor repairs preserved in `docs/archive/product-technical-gap-baseline-through-573915a7.md` remain authoritative and no issued digest domain is rewritten. The active representation covers relation/index partition topology, relation rowtype/typmod and key/INCLUDE mapping, operator-family/exclusion semantics, expression/predicate and relation-`Var` equality, collation identity/provider/version/database-encoding semantics, partition declaration/collation coherence, foreign-table partitioned-index behavior, valid-parent/valid-child composition, key-constraint child presence/parentage/inheritance, and ordinary partitioned EXCLUDE constraint catalog identity.

For ordinary `contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; `conparentid`; `conislocal`; `coninhcount`; raw `connoinherit`; `condeferrable`; `condeferred`; `conenforced`; `convalidated`; `conperiod`; ordered raw `conkey`; ordered resolved `conexclop`; independently resolved `pg_operator.oprcom` commutator signatures for every exact exclusion-key position; independently resolved `pg_operator.oprcode -> pg_proc` implementation-procedure signatures for every exact exclusion-key position; independently resolved `pg_constraint.connamespace`; mutually exclusive catalog-family shape; exact constraint-name/backing-index-name coupling; independently observed backing-index `pg_index` state/role/exclusion semantics; independently observed access-method `can_exclude`; the exact backing index row's independently resolved `pg_class.relnamespace`; explicit ready/valid/live lifecycle integrity for that exact backing index; and exact v3 source-content generation binding across the name -> namespace -> lifecycle chain.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain in their dedicated key/foreign-key owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE backing-index `pg_class.relnamespace` integrity

Review `5224968197` on exact predecessor `9ee431847d4814d8500d879d8e21134be1d7ec3b` found a cross-catalog P1: ConceptWeave already retained independently resolved `pg_constraint.connamespace`, the exact `conindid` backing-index coordinate, and schema-scoped index-name occupancy, but `IndexPartitionCoordinate.schema_name()` was inherited from the owning relation coordinate. A faulty extractor could therefore normalize a contradictory backing-index `pg_class.relnamespace` back to the relation schema before governed identity was issued.

PostgreSQL 18 stores `pg_class.relnamespace` independently for index relations while index creation places the index in the parent relation's schema. Equality is therefore an invariant to prove after two independent catalog reads, not a value to synthesize from the relation coordinate.

The ordinary-forward repair is:

- structural source/compile RED contract `81a813cedf6c239cccd64e123e85b6a352cee032`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_namespace_contract.rs`; the successor type did not yet exist at that commit, so this is structural compile RED rather than a claimed executed compiler failure;
- production successor `b7783930e17741ff04efcac2ca0e39135f45192e`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_namespace.rs`;
- public composition `7914d15d44b50bf092121d5af2951d94e3926e4`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- focused doctoring `573915a77cd85b66dd5f81bb802594a64c3f5318`, `docs/doctoring/postgresql-index-exclusion-constraint-backing-index-namespace-integrity.md`;
- lossless predecessor baseline archive added immediately after that head as `docs/archive/product-technical-gap-baseline-through-573915a7.md`.

`IndexExclusionConstraintIndexNamespaceSnapshot` rebinds the exact ordinary-EXCLUDE constraint/index inventory proven by `IndexExclusionConstraintIndexNameSnapshot`. Every binding requires exactly one explicit `pg_class.relnamespace -> pg_namespace.nspname` observation for the exact backing index. Missing or duplicate coordinates, a different backing-index binding, blank namespace, or a resolved namespace that differs from the exact backing-index/owning-relation schema fails closed. The raw namespace and both exact source coordinates enter a new domain-separated digest/provenance receipt. The adapter must not copy relation or constraint namespace into the backing-index observation.

Focused contract coverage includes normal provenance, relation/index namespace drift, blank namespace, incomplete inventory, wrong predecessor-index binding, duplicate coordinate, and unknown receipt behavior.

## EXCLUDE backing-index lifecycle and exact source-generation integrity

Review `5226246612` on exact predecessor `3296e5e6e735fa8f1c10dadc6387bd2e6682bf32` found the next cross-catalog P1: the exact `conindid` backing index was already bound and its generic v3 `IndexObservation` retained optional `pg_index.indisready`, `indisvalid`, and `indislive`, but no ordinary-EXCLUDE successor required those independently observed lifecycle bits to be present and healthy.

PostgreSQL 18 defines `indisready=false` as an index ignored by `INSERT`/`UPDATE`, `indisvalid=false` as possibly incomplete and unsafe for queries, and `indislive=false` as an index being dropped and ignored for all purposes. PostgreSQL's documented concurrent reindex phases demonstrate that readiness, validity, constraint rebinding, and retirement are separate generic index-catalog transitions; PostgreSQL 18 also explicitly states that exclusion-constraint indexes cannot themselves be reindexed concurrently, so that sequence is authority for the lifecycle flag semantics rather than a claimed ordinary-EXCLUDE transition path. Constraint existence is not a substitute for the three index lifecycle facts.

The initial ordinary-forward repair is:

- structural source/compile RED contract `07fd2aecd176d4b114781d552e2de82894304ed2`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_lifecycle_contract.rs`; the lifecycle successor type did not yet exist at that commit, so this is structural compile RED rather than a claimed executed compiler failure;
- production successor `a4f5dff99e58ee1be55a2160dcc7fc1ab0d8dea7`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_lifecycle.rs`;
- public composition `203d3fe6a7ed730e9884059b441bee80fe7e2196`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- edge/provenance contract currentization `5a11b4ca161623e231d804b8ee44df1f8cbf5e47`;
- focused doctoring lineage in `docs/doctoring/postgresql-index-exclusion-constraint-backing-index-lifecycle-integrity.md`.

Bounded follow-up review `5226362550` on exact `742bef1a9c358e9bab123d4ec894289caa42b110` found that the first lifecycle constructor still compared only source key, policy binding, extractor revision, and observation time before reading the supplied v3 base. Two distinct v3 snapshots can share those provenance coordinates, so lifecycle evidence from a different content generation could be combined with the old namespace predecessor.

The follow-up repair is:

- behavioral source-RED `19fa58e1ba3355a808953e868150af364000e13c`, adding a same-source/same-policy/same-extractor/same-time/same-index but different-v3-digest rejection contract; no executed Rust failure is claimed because the available execution host lacks the Rust toolchain;
- `7d84bdbdfd4789f425acb1dd78ac97573510378d` retains the exact v3 source-content digest as immutable metadata on `IndexExclusionConstraintIndexNameSnapshot` without changing its issued digest calculation;
- `c1561475e331fae2fe93ae02383547d7610d05a1` propagates that source-content digest through `IndexExclusionConstraintIndexNamespaceSnapshot`, again without changing its issued digest calculation;
- `f7d8dbcf4056c3b1c53cfcffd0bf46d14052f63d` requires exact v3 source-digest equality before lifecycle lookup;
- focused doctoring currentization records both the generic lifecycle semantics and the PostgreSQL exclusion-index concurrent-reindex caveat.

`IndexExclusionConstraintIndexLifecycleSnapshot` now composes the exact current backing-index namespace generation with the exact same v3 source-content generation, resolves the exact backing `IndexObservation`, and requires explicit `ready=true`, `valid=true`, and `live=true`. Missing lifecycle evidence, any false state, a different exact backing index, different source metadata, or a different v3 content digest fails closed. Both the exact v3 base digest and namespace-predecessor digest, the exact constraint/index coordinates, and all three booleans enter the lifecycle successor digest/provenance receipt. Existing predecessor digest calculations remain unchanged.

`pg_index.indcheckxmin` is deliberately not promoted into the same health predicate. PostgreSQL documents it as a planner/HOT-chain visibility horizon, not as the ready/valid/live lifecycle that determines write maintenance, index validity, or whether the index is being dropped. Its raw source value remains retained by `IndexCatalogFlags`.

Focused contract coverage includes normal provenance, each missing lifecycle bit, each false lifecycle bit, wrong exact backing-index binding, mismatched source metadata, same-metadata/different-content source generation, and unknown receipt behavior.

## EXCLUDE operator implementation-function integrity

Review `5226950243` on exact predecessor `84e287842d094755caa75272fae73111125fd3e7` found another cross-catalog P1. `IndexExclusionSemanticsSnapshot` already retained an operator signature plus a backing exclusion procedure signature, but `IndexKeyExclusionSemanticsObservation::new` only proved that the procedure argument types matched the operator operand types. A different same-typed function could therefore be substituted without proving that it was the exact function referenced by `pg_operator.oprcode`.

PostgreSQL 18 `pg_operator.oprcode` identifies the function implementing an operator, and PostgreSQL `IndexInfo::ii_ExclusionProcs` is the underlying function array corresponding to `ii_ExclusionOps`. The two paths must therefore be resolved independently and compared after resolution.

The ordinary-forward repair is:

- structural source/compile RED `2f7e37da0362b59d2875e10dc51ed91935d434ed`, extending the focused commutator contract to require a successor that did not yet exist; no executed compiler failure is claimed;
- `c05f681081412b12daca0e9b88479c0058878f27` retains the already-rebound backing exclusion procedure per exact constraint/key position as immutable metadata on `IndexExclusionConstraintOperatorSnapshot` without changing the existing operator digest calculation;
- production `IndexExclusionConstraintOperatorProcedureSnapshot` `9ff1b49b4188ea98f223d0d45075dbdb1490b813`;
- public composition `b6776db96a677f39aea5fe8f86d5186fb9ee03a8`;
- focused doctoring `13e174a8ea12f71057f5253fd1886dae2600e30f` in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-integrity.md`.

Every governed `conexclop` position now requires exactly one independent `pg_operator.oprcode -> pg_proc` observation. The repeated operator must equal the exact predecessor operator and the independently resolved procedure must equal the already-governed backing exclusion procedure. Missing/duplicate positions, operator drift, or a same-typed but different procedure fail closed. OIDs remain adapter-local join coordinates; stable operator/procedure signatures enter the new domain-separated digest and provenance family.

## Current state

**INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_EXACT_SOURCE_GENERATION_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAME_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAME_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_CATALOG_SHAPE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_CATALOG_SHAPE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NAME_INTEGRITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NAME_INTEGRITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_KEY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_EXACT_PREDECESSOR_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_PERIOD_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PERIOD_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_RAW_STATE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_TIMING_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PARTITION_SOURCE_REPAIRED / INDEX_CONSTRAINT_INHERITANCE_STATE_SOURCE_REPAIRED / INDEX_CONSTRAINT_PARENTAGE_SOURCE_REPAIRED / SOURCE_OBSERVATION_RETAINED / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is inferred from source commits. One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the focused backing-index-lifecycle/backing-index-namespace/operator-procedure/operator-commutator/access-method-capability/index-name/catalog-shape/constraint-namespace/relation-local-name/index-role/immediacy/operator/key/period/no-inherit/validation/enforcement/timing contracts, all retained Source Observation/relation-partition contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets acceptance.

The PostgreSQL 18 bounded live differential must read ordinary EXCLUDE `contype`, `conname`, `connamespace`, `conrelid`, `contypid`, `conindid`, `conparentid`, `confrelid`, `confupdtype`, `confdeltype`, `confmatchtype`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`, `conkey`, `confkey`, `conpfeqop`, `conppeqop`, `conffeqop`, `confdelsetcols`, `conexclop`, and `conbin` from the same bounded source observation.

For every `conexclop` position it must independently resolve the operator, its `pg_operator.oprcom`, and its `pg_operator.oprcode`; `oprcode` must then be resolved independently through `pg_proc` and compared with the backing exclusion procedure for the same exact index/key position. For the exact `conindid` backing index it must independently read `pg_class.relname`, `pg_class.relnamespace`, `pg_class.relam`, supporting `pg_index.indkey`, `indisunique`, `indisprimary`, `indisexclusion`, `indimmediate`, `indisready`, `indisvalid`, `indislive`, resolved exclusion semantics, access-method name, and `pg_indexam_has_property(relam, 'can_exclude')`. The index `relnamespace` must be resolved through `pg_namespace` independently of the parent relation and constraint namespace and compared only after those reads. Ready/valid/live must come from the exact same `pg_index` row identified through `conindid`; copying expected `true` values from constraint semantics is invalid differential evidence. The lifecycle evidence must also be carried in the exact same v3 source-content digest generation as the predecessor chain; matching provenance metadata without matching content digest is insufficient.

Constraint-family, naming, timing/immediacy, index-role, access-method-capability, operator-procedure, operator-commutator, backing-index-namespace, backing-index-lifecycle, and exact-source-generation controls remain as documented in the archived predecessor baseline and focused doctoring. CHECK, FK, domain CHECK, temporal key, ordinary EXCLUDE, non-index repeated names on different relations, non-self-commutative operators, same-typed wrong operator procedures, non-capable AMs, independently contradictory namespace observations, missing/false backing-index lifecycle observations, and same-provenance/different-content source generations must remain explicit controls rather than being normalized away.

## Required next order

Canonical `.github` workflow-owner exact-current terminal settlement and owner-local documentation repair -> fresh compatible #35 acceptance/normal landing -> one unchanged #46 native+hosted terminal GREEN -> PostgreSQL 18 bounded live differential including independent `pg_operator.oprcode -> pg_proc`, backing-index `pg_class.relnamespace`, `pg_index.indisready`/`indisvalid`/`indislive`, and exact v3 source-content generation identity plus all retained EXCLUDE catalog integrity -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
