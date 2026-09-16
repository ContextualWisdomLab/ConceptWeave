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

Every valid predecessor repair remains retained without rewriting an issued digest domain. This includes relation/index partition topology, rowtype/`atttypmod`, key/INCLUDE mapping, operator-family and exclusion semantics, expression/predicate and relation-`Var` equality, collation identity/provider/version/database-encoding semantics, direct partition declaration/collation coherence, foreign-table partitioned-index behavior, valid-parent/valid-child composition, key-constraint child presence/parentage/inheritance, and ordinary partitioned EXCLUDE constraint identity/`conindid`/parentage/inheritance/timing/enforcement/validation/raw `connoinherit`. The archived baseline and focused doctoring are the detailed authority for those predecessors.

## EXCLUDE enforcement source integrity

Review `5218290892` found that ordinary `EXCLUDE` constraint evidence retained the independent `pg_constraint` object plus `condeferrable`/`condeferred`, but did not retain `pg_constraint.conenforced`. The ordinary-forward domain-separated repair is contract `83bd71cabfba11239a6f9a78b761eaea94b8cce8`, production successor `2538bb4ee2b8d97c33d706c9dbc4875da06d0607`, public composition `fb32d84c39918974f0ad7500f07d907c8b4891ae`, and focused doctoring `09c74ea1b516cb959e390eb71f8ac808513f9190`. `conenforced=false` fails closed for ordinary EXCLUDE evidence; no predecessor digest is rewritten.

## EXCLUDE validation source integrity

Review `5218582084` found the next P1 on predecessor `39a5e57b2da10c1326b57822829f3fa80abc01f9`: the EXCLUDE successor chain retained catalog identity, backing index, partition parentage/inheritance, timing, and enforcement but dropped `pg_constraint.convalidated`.

The pinned PostgreSQL 18 source `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` calls `CreateConstraintEntry()` from `index_constraint_create()` with `true` for `isValidated` when creating index-backed constraints, including `CONSTRAINT_EXCLUSION`. A contradictory ordinary `contype='x', convalidated=false` tuple therefore fails closed in the validation successor.

- Source/compile RED contract: `064e3085a56aebfb35bea8cefd143ea4552191d1`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_validation_contract.rs`.
- Production successor: `100f66078fc4568480f7f6d7bdd6060b5d25bd96`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_validation.rs`.
- Public composition: `5cc60c6d2ddc9f6c4cfcbee1aeea0da16d445850`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Focused doctoring: `20191a236ebd59bb9cfc3ed6190c9acf9f2691f2`, `docs/doctoring/postgresql-index-exclusion-constraint-validation-integrity.md`.

Every predecessor ordinary EXCLUDE constraint requires exactly one explicit validation observation; duplicates and incomplete inventories fail closed; the raw bit remains in the domain-separated digest and exact provenance receipt.

## EXCLUDE no-inherit source integrity

Review `5218606154` found that the EXCLUDE identity predecessor dropped independently stored `pg_constraint.connoinherit`. The field must remain explicit governed evidence, but current parentage is not a valid normalization rule.

The initial repair incorrectly enforced root `true` / partition-child `false`. Review `5218646331` caught the over-constraint before acceptance. Pinned PostgreSQL 18 shows why:

- `index_constraint_create()` initializes a constraint created with `parentConstraintId` as `connoinherit=false`, while standalone creation initializes `true`;
- `ATExecAttachPartition()` can reuse an already-existing valid child index/constraint and call `ConstraintSetParentConstraint()`;
- `ConstraintSetParentConstraint()` updates `conparentid`, `conislocal`, and `coninhcount` but does not rewrite `connoinherit`, in either attach or detach direction.

An attached preexisting child can therefore retain `connoinherit=true`, while a child cloned with its parent can carry `false`. The corrected owner rule is raw-state preservation, not derivation from current parentage.

The ordinary-forward correction retains the domain-separated successor:

- Original omission contract/successor: `3552cd383ba54b8f6aae8a8796b492a86a5d4738` / `a605dd29afc48f7b112d6aa8d73bc45e901bb86c`.
- Repair finding: review `5218646331` at predecessor `25508f53ed4ca8f4bb17b637dc82801ffcdbd7fc`.
- Corrected source/compile contract: `e8aa445679a56ba950e193fb4f9825614522d1ab`.
- Corrected production successor: `ed7362c110992e0d9914e971a2e63d2bf86ea589`.
- Public composition remains `cd25d0aaf633ae84caeb7144477027da657ab346`.
- Corrected doctoring: `4f2c1c852d76b2f846220be4287ebfe9a606fbeb`.

`IndexExclusionConstraintNoInheritSnapshot` now requires complete one-to-one raw-bit coverage, rejects duplicates/incomplete coordinates, hashes the raw bit into a separate digest/provenance receipt, and deliberately does not reject either boolean value based solely on current parentage. Distinct source-reachable raw states must produce distinct governed digests.

## EXCLUDE period-state source integrity

Review `5218939490` found that ordinary `contype='x'` EXCLUDE identity still dropped `pg_constraint.conperiod`. PostgreSQL 18 defines `conperiod=true` only for `WITHOUT OVERLAPS` primary/unique constraints and `PERIOD` foreign keys. Ordinary EXCLUDE therefore requires explicit `conperiod=false`; otherwise a contradictory `x + conperiod=true` tuple can collapse into the same governed ordinary-EXCLUDE identity.

Temporal p/u/f `conperiod` semantics remain owned by `conceptweave-observation::ConstraintPeriodObservation`; this repair does not duplicate that family or derive temporal authority from `pg_index.indisexclusion`.

- Source/compile RED contract: `47b543b5618ceb30d1df285f7880fcbb1d313118`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_period_contract.rs`.
- Production successor: `7840356c3a2ffd60b690e6b2a7b1b14535e6f985`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_period.rs`.
- Public composition: `13846fa2ac40a144ba454fa6c56fd924045662bc`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Focused doctoring: `121b4f0a15532a28869d854deb5b49b0cb1487db`, `docs/doctoring/postgresql-index-exclusion-constraint-period-integrity.md`.

`IndexExclusionConstraintPeriodSnapshot` requires complete one-to-one period-state coverage over the exact ordinary-EXCLUDE identity predecessor, rejects duplicates/incomplete coordinates, rejects `conperiod=true`, and binds the predecessor digest, exact constraint coordinate, explicit false bit, and `/period` provenance in a new digest domain without rewriting issued predecessors.

## EXCLUDE constraint-key source integrity

Review `5219346562` found the next P1 on exact predecessor `658b46604edf597cac700175fc47b91200eaeed4`: the ordinary EXCLUDE successor chain still dropped the independently stored `pg_constraint.conkey` array. PostgreSQL 18 persists the first `ii_NumIndexKeyAttrs` values of `IndexInfo::ii_IndexAttrNumbers` into `conkey`; simple column elements carry exact relation attribute numbers and expression elements carry zero. INCLUDE payload positions are not part of the stored constraint-key array.

The first source/compile contract `edc700bf98bfb3fce9848ab9693624c18c1d97ce` and first production successor `5a4d55a2e652694f259a50632012c0e3e563f6b7` established raw `conkey` preservation. Exact-head review `5219378111` then found that the first successor accepted a separately supplied v3 base for deriving expected attribute numbers without proving that base was the one transitively bound by the EXCLUDE predecessor.

The corrected ordinary-forward lineage is:

- tightened source/compile contract: `74aa937f91514364327b11a8db12fb44621e9115`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_key_contract.rs`;
- corrected production successor: `08f96237c1301704af2c0be6c8312464a175b987`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_key.rs`;
- public composition remains `e856d8f148fb2826503a7928d3fa1da1d38bdea4`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- focused doctoring currentized by `001faf8f1bfc13d90bdb7e6c3ceb6b1c3cbe7b4`, `docs/doctoring/postgresql-index-exclusion-constraint-key-integrity.md`.

`IndexExclusionConstraintKeySnapshot` now rebuilds the ordinary EXCLUDE identity from the supplied exact v3 + relation-partition + index-partition predecessors and requires a digest match, then rebound-validates the exact period successor. It requires one raw ordered `conkey` vector for every ordinary EXCLUDE coordinate, verifies that vector against the exact `conindid` backing index and owning relation (`column -> attnum`, expression -> `0`, INCLUDE omitted), and binds the raw vector into a new domain-separated digest and `/key-attributes` receipt. Issued predecessor digests remain unchanged.

## EXCLUDE constraint-operator source integrity

Review `5219874038` found the next P1 on exact predecessor `1051c8cddcd16bdb6d47bc5c2c13a7c736b2b859`: ordinary `contype='x'` evidence governed backing-index exclusion semantics but still dropped the independently stored `pg_constraint.conexclop` array. PostgreSQL 18 defines `conexclop` as the exclusion-operator OID array on the constraint row, and also uses the field for temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS`; those temporal p/u rows remain in their key-constraint owner family rather than being reclassified as ordinary EXCLUDE.

The ordinary-forward lineage is:

- source/compile regression contract: `1be6077cc9b888c013acfface68afaa739ccc80d`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_contract.rs`;
- initial causal successor: `20b9184cb7bef9a4c6baa4379055c34968ca030a`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator.rs`;
- public composition/export: `02805d76f63cc5e3a704b415c0161bf44b6955bd`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- API/lint refinement without warning suppression: `81788a5e108bd4198a30faed129861c55cc5bd7a`, which introduced named source and semantics predecessor lineages instead of retaining a local `clippy::too_many_arguments` allow;
- aligned regression contract: `286bab3dffe8eab2538e65e1843b9aa406c3ea96`;
- focused doctoring currentized at `edab86a8f8c07a3afea22ae6df7ede4b31af2481`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-integrity.md`.

`IndexExclusionConstraintOperatorSnapshot` requires one complete ordered constraint-side operator vector for every ordinary EXCLUDE coordinate. Capture-time OIDs are resolved to stable operator signatures, the exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE -> period -> `conkey` branch and operator-family -> backing-index exclusion-semantics branch are rebound, and the independently observed `conexclop` vector must match the exact `conindid` backing index. Missing/duplicate evidence or operator drift fails closed. The successor hashes both predecessor digests plus exact constraint coordinates and resolved operator signatures under a new domain separator; issued predecessor digests remain unchanged.

## Current state

**INDEX_EXCLUSION_CONSTRAINT_OPERATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_KEY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_EXACT_PREDECESSOR_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_PERIOD_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PERIOD_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_RAW_STATE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_TIMING_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PARTITION_SOURCE_REPAIRED / INDEX_CONSTRAINT_INHERITANCE_STATE_SOURCE_REPAIRED / INDEX_CONSTRAINT_PARENTAGE_SOURCE_REPAIRED / SOURCE_OBSERVATION_RETAINED / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is inferred from source commits. One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the ordinary EXCLUDE operator/key/period/no-inherit/validation/enforcement contracts, all retained Source Observation/relation-partition contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets acceptance.

The concrete PostgreSQL 18 live differential must read ordinary EXCLUDE `contype`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`, `conkey`, and `conexclop` in one bounded source observation and read the supporting `pg_index.indkey` plus resolved backing-index exclusion semantics in the same bounded read. `conkey` coverage must include a simple column, an expression slot proving the zero marker, an INCLUDE payload proving it is excluded from `conkey`, and a partitioned parent/child pair whose local attribute numbers remain correct for each owning relation. `conexclop` coverage must prove exact equality with the backing index operator vector while preserving independent constraint-side evidence. For `connoinherit`, the differential must cover at least both child lifecycle paths: a child cloned with its parent (`false`) and a compatible standalone child attached later (which can retain `true`), plus detach behavior. ConceptWeave must preserve the raw values, reject missing/duplicate evidence, and distinguish resulting digests rather than infer source state from current parentage.

The differential must also prove ordinary EXCLUDE `conperiod=false`, enforcement/validation and timing distinctions, and retain temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` plus FOREIGN KEY `PERIOD` controls so temporal p/u/f semantics are not double-counted as ordinary EXCLUDE.

## Next causal work

1. Converge the canonical `.github` workflow owner and obtain fresh compatible acceptance for Product bootstrap #35; merge #35 normally only when required gates are terminal GREEN.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus hosted Product/security/dependency/review terminal GREEN.
3. Add the concrete PostgreSQL 18 source extractor/live differentials, including ordinary EXCLUDE constraint-operator/key/period/no-inherit lifecycle/validation/enforcement/timing/catalog state and every retained Source Observation differential, then obtain fresh terminal GREEN on the resulting unchanged head.
4. Continue source-domain review of still-unmodeled material `pg_constraint` fields before claiming EXCLUDE catalog completeness.
5. Only then adopt the complete #46 child ordinary/non-force into #45, obtain fresh #45 acceptance, and propagate through #6.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, or premature publication/release is authorized.