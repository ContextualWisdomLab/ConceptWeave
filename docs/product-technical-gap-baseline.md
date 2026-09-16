# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete predecessor baseline through exact `08847df6124f8834ed8b8ec33c9451a2a1474d3e` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-08847df6.md`; detailed decisions remain in `docs/doctoring/`. Exact-head execution evidence never transfers after head movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

## Live stack and single-writer boundary

- #46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a` unless fresh GitHub state says otherwise.
- #45 and #6 must not partially cherry-pick or independently reimplement #46. Adoption is complete ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected/default `main` lacks the required workflow.
- Canonical reusable workflow ownership remains in `ContextualWisdomLab/.github`; this lane does not copy, wake, or weaken that owner contract.

## Retained Source Observation authority

Every valid predecessor repair remains retained without rewriting an issued digest domain. This includes relation/index partition topology, rowtype/`atttypmod`, key/INCLUDE mapping, operator-family and exclusion semantics, expression/predicate and relation-`Var` equality, collation identity/provider/version/database-encoding semantics, direct partition declaration/collation coherence, foreign-table partitioned-index behavior, valid-parent/valid-child composition, key-constraint child presence/parentage/inheritance, and ordinary partitioned EXCLUDE constraint identity/`conindid`/parentage/inheritance/timing/enforcement/validation. The archived baseline and focused doctoring are the detailed authority for those predecessors.

## EXCLUDE enforcement source integrity

Review `5218290892` found that ordinary `EXCLUDE` constraint evidence retained the independent `pg_constraint` object plus `condeferrable`/`condeferred`, but did not retain `pg_constraint.conenforced`. The ordinary-forward domain-separated repair is contract `83bd71cabfba11239a6f9a78b761eaea94b8cce8`, production successor `2538bb4ee2b8d97c33d706c9dbc4875da06d0607`, public composition `fb32d84c39918974f0ad7500f07d907c8b4891ae`, and focused doctoring `09c74ea1b516cb959e390eb71f8ac808513f9190`. `conenforced=false` fails closed for ordinary EXCLUDE evidence; no predecessor digest is rewritten.

## EXCLUDE validation source integrity

Review `5218582084` found the next P1 on predecessor `39a5e57b2da10c1326b57822829f3fa80abc01f9`: the EXCLUDE successor chain retained catalog identity, backing index, partition parentage/inheritance, timing, and enforcement but dropped `pg_constraint.convalidated`.

The pinned PostgreSQL 18 source `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` calls `CreateConstraintEntry()` from `index_constraint_create()` with `true` for `isValidated` when creating index-backed constraints, including `CONSTRAINT_EXCLUSION`. PostgreSQL also exposes `convalidated` independently in `pg_constraint`. A contradictory ordinary `contype='x', convalidated=false` tuple must therefore fail closed instead of collapsing onto validated governed identity.

The repair remains ordinary-forward and domain-separated:

- Source/compile RED contract: `064e3085a56aebfb35bea8cefd143ea4552191d1`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_validation_contract.rs`.
- Production successor: `100f66078fc4568480f7f6d7bdd6060b5d25bd96`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_validation.rs`.
- Public composition: `5cc60c6d2ddc9f6c4cfcbee1aeea0da16d445850`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Focused doctoring: `20191a236ebd59bb9cfc3ed6190c9acf9f2691f2`, `docs/doctoring/postgresql-index-exclusion-constraint-validation-integrity.md`.

`IndexExclusionConstraintValidationSnapshot` composes over the exact enforcement predecessor digest. Every predecessor ordinary EXCLUDE constraint must have exactly one explicit validation observation; duplicates and incomplete inventories fail closed; `convalidated=false` fails closed; and the raw bit is retained in the domain-separated digest and exact provenance receipt.

## EXCLUDE no-inherit source integrity

Review `5218606154` found a second PostgreSQL 18 catalog-state omission on exact predecessor `10c6c8c4827c585a744576700fa04478e20b14aa`: EXCLUDE identity already retained `conparentid`, `conislocal`, and `coninhcount`, but dropped independently stored `pg_constraint.connoinherit`.

Pinned `index_constraint_create()` sets root index constraints to `(conislocal=true, coninhcount=0, connoinherit=true)` and partition-child index constraints to `(false, 1, false)`. Because ordinary EXCLUDE constraints use that path, contradictory `connoinherit` values must be detected instead of inferred away.

The repair is a domain-separated sibling over the exact EXCLUDE identity predecessor:

- Source/compile RED contract: `3552cd383ba54b8f6aae8a8796b492a86a5d4738`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_no_inherit_contract.rs`.
- Production successor: `a605dd29afc48f7b112d6aa8d73bc45e901bb86c`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_no_inherit.rs`.
- Public composition: `cd25d0aaf633ae84caeb7144477027da657ab346`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Focused doctoring: `e4c83d1f7b09b232dec0833b0348f2337584c11d`, `docs/doctoring/postgresql-index-exclusion-constraint-no-inherit-integrity.md`.

`IndexExclusionConstraintNoInheritSnapshot` requires complete one-to-one raw-bit coverage, validates root/partition-child state against already-governed parentage, and hashes the bit into a separate digest/provenance receipt. It does not rewrite the EXCLUDE identity or later timing/enforcement/validation domains.

## Current state

**INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_TIMING_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PARTITION_SOURCE_REPAIRED / INDEX_CONSTRAINT_INHERITANCE_STATE_SOURCE_REPAIRED / INDEX_CONSTRAINT_PARENTAGE_SOURCE_REPAIRED / SOURCE_OBSERVATION_RETAINED / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is inferred from source commits. One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new EXCLUDE validation/enforcement/no-inherit contracts plus all retained Source Observation/relation-partition contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets acceptance.

The concrete PostgreSQL 18 live differential must read ordinary EXCLUDE `contype`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, and `convalidated` in the same bounded source observation. It must demonstrate root `(0,true,0,true)` and partition-child `(parent,false,1,false)` inheritance state, prove supported EXCLUDE rows are enforced and validated, preserve timing distinctions, reject incomplete or contradictory evidence, and keep a temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` control so exclusion behavior is not double-counted.

## Next causal work

1. Converge the canonical `.github` workflow owner and obtain fresh compatible acceptance for Product bootstrap #35; merge #35 normally only when required gates are terminal GREEN.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus hosted Product/security/dependency/review terminal GREEN.
3. Add the concrete PostgreSQL 18 source extractor/live differentials, including EXCLUDE no-inherit/validation/enforcement/timing/catalog state and every retained Source Observation differential, then obtain fresh terminal GREEN on the resulting unchanged head.
4. Continue source-domain review of still-unmodeled material `pg_constraint` fields before claiming EXCLUDE catalog completeness.
5. Only then adopt the complete #46 child ordinary/non-force into #45, obtain fresh #45 acceptance, and propagate through #6.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, or premature publication/release is authorized.