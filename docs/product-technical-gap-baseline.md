# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The complete predecessor baseline through exact `3e57508fe488aa059122206a7f21a3576babbe63` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-3e57508f.md`; the older baseline through `08847df6124f8834ed8b8ec33c9451a2a1474d3e` remains at `docs/archive/product-technical-gap-baseline-through-08847df6.md`. Detailed decisions remain in `docs/doctoring/`. Exact-head execution evidence never transfers after head movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a` unless fresh GitHub state says otherwise. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 remains the repository-owned Product workflow prerequisite while protected/default `main` lacks that accepted workflow. Canonical reusable-workflow ownership remains in `ContextualWisdomLab/.github`.

## Retained Source Observation authority

Every valid predecessor repair remains retained without rewriting an issued digest domain. The active representation covers relation/index partition topology, relation rowtype/typmod and key/INCLUDE mapping, operator-family/exclusion semantics, expression/predicate and relation-`Var` equality, collation identity/provider/version/database-encoding semantics, partition declaration/collation coherence, foreign-table partitioned-index behavior, valid-parent/valid-child composition, key-constraint child presence/parentage/inheritance, and ordinary partitioned EXCLUDE constraint catalog identity.

For ordinary `contype='x'` EXCLUDE constraints, retained owner evidence now includes the independent constraint object, exact `conindid` backing index, `conparentid`, `conislocal`, `coninhcount`, raw `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`, ordered raw `conkey`, resolved ordered `conexclop`, independently resolved `connamespace`, the mutually exclusive catalog-family shape (`contype`, relation/domain ownership sentinels, FK-only payload absence, CHECK-expression absence), exact constraint-name/backing-index-name coupling with bounded schema `pg_class` namespace integrity, and the backing index's independently observed `pg_index` state, role vector, and exclusion semantics. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain in their dedicated key/foreign-key owner families rather than being reclassified as ordinary EXCLUDE.

The detailed predecessor decisions, exact review IDs, repair commits, and rejected alternatives are preserved in the archived baseline and focused doctoring records.

## EXCLUDE timing / backing-index immediacy coherence

Review `5220488022` on exact predecessor `3e57508fe488aa059122206a7f21a3576babbe63` found a cross-catalog P1: ConceptWeave already preserved ordinary EXCLUDE `pg_constraint.condeferrable`/`condeferred` and backing `pg_index.indimmediate`, but did not validate their relationship. A source tuple such as `condeferrable=true` with `indimmediate=true` could therefore become governed evidence even though PostgreSQL creates the supporting index with the opposite immediacy state.

Pinned PostgreSQL 18 `REL_18_STABLE` supplies the primary authority:

- `DefineIndex()` maps `stmt->deferrable` to `INDEX_CONSTR_CREATE_DEFERRABLE` before `index_create()`;
- `index_create()` supplies `UpdateIndexRelation()` an `immediate` value that is false when the constraint is deferrable and true otherwise;
- `UpdateIndexRelation()` persists that value as `pg_index.indimmediate`;
- executor arbiter validation explicitly treats `!indimmediate` as a deferrable unique/exclusion index.

The ordinary-forward repair is:

- production coherence successor staged at `4ce488cdaf0e07a2406087867e153c59cc0b2a5d`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_immediacy.rs`;
- source/compile RED contract at `f5b9638cabf3ab18311e7f5dc0893d0aa874ab11`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_immediacy_contract.rs`; at that exact commit the new module was not yet publicly composed, so the contract is a real compile RED rather than a claimed executed failure;
- public composition at `d363119e32a1d33486c0b0812a60b9b41bafd195`, `crates/conceptweave-relation-partition/src/index_partition.rs`;
- focused doctoring at `bda103885561cb69b5f04ebc41830d1b4fb70805`, `docs/doctoring/postgresql-index-exclusion-constraint-immediacy-integrity.md`;
- predecessor active baseline archived losslessly at `79487a83aa010275d74fa66b148d761b73163c74` using the unchanged blob `3fb10672a2858b61742ee52216da00ed1394d7c0`.

`IndexExclusionConstraintImmediacySnapshot` rebinds the exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE -> timing chain, resolves each constraint's exact `conindid` backing index, and accepts only `condeferrable=false / indimmediate=true` or `condeferrable=true / indimmediate=false`. It does not synthesize either raw fact from the other. `DEFERRABLE INITIALLY IMMEDIATE` and `DEFERRABLE INITIALLY DEFERRED` both require a non-immediate backing index but remain distinct because `condeferred` remains material in the timing predecessor digest.

## EXCLUDE backing-index role coherence

Review `5221108817` on exact predecessor `a6698c423a15e1d268404588bbecf56d792ca009` found another cross-catalog P1: ordinary EXCLUDE identity already bound the exact `conindid` backing index and retained `pg_index.indisexclusion`, `indisunique`, and `indisprimary`, but no successor rejected an impossible role vector such as an ordinary `contype='x'` EXCLUDE constraint backed by a unique or primary index.

PostgreSQL 18 `transformIndexConstraint()` sets `IndexStmt.unique` for PRIMARY KEY/UNIQUE but not `CONSTR_EXCLUSION`, and sets `IndexStmt.primary` only for PRIMARY KEY. Ordinary EXCLUDE therefore has a non-unique, non-primary supporting index with exclusion behavior. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` is a separate key-constraint control even though PostgreSQL also uses exclusion behavior for that family.

The ordinary-forward repair is:

- source/compile RED contract `b7b364cb7ef05757d3d32c70a4b36907c6ba0247`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_role_contract.rs`; the referenced successor did not yet exist at that commit, so this is a structural compile RED and not a claimed executed failure;
- production successor `86d4e1350b6cff697aec2657359f6f7109e55b6f`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_role.rs`;
- public composition `d81750a9e1d94cc953faae5a32752e5fc16f2660`;
- focused doctoring `bcf0ebf582fcc43ebc53fecda6815908c4f63dad`, `docs/doctoring/postgresql-index-exclusion-constraint-index-role-integrity.md`.

`IndexExclusionConstraintIndexRoleSnapshot` rebinds the exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE -> timing -> timing/index-immediacy chain, resolves each exact `conindid` backing index, retains raw `indisunique`, `indisprimary`, and `indisexclusion`, and admits only `(false, false, true)`. It never manufactures those catalog bits from the constraint kind.

## Relation-wide EXCLUDE constraint-name integrity

Review `5221675071` on exact predecessor `0de651ee57f823638a416677239a9ef4dbac3c84` found that ordinary EXCLUDE coordinates were unique only within the EXCLUDE successor. PostgreSQL 18 `pg_constraint.h` requires `conname` to be unique among the constraints of one relation/domain and backs the relation case with the unique catalog index over `(conrelid, contypid, conname)`. ConceptWeave could nevertheless combine a relation-owned CHECK/PK/UQ/FK row and an ordinary EXCLUDE row with the same exact relation-local name and issue governed evidence for an impossible catalog state.

Bounded follow-up review `5221753670` extended the same finding to the separately observed PostgreSQL 18 first-class NOT NULL family. The repair checks only evidence actually present in `PostgresSchemaSnapshotV3`: base relation constraints plus explicit NOT NULL observations when that family was captured. It does not infer NOT NULL rows from `attnotnull` and does not normalize names. This layer is intentionally relation-local; the narrower schema-wide restriction for index-backed EXCLUDE names is owned by the later backing-index-name successor because it arises from `pg_class`, not from `pg_constraint` alone.

The ordinary-forward lineage is:

- CHECK↔EXCLUDE behavioral contract `0237a677302ad15121555d575dc09252348cc1c0`;
- minimal base-family source repair restored at `a6e53cd459e61efaf61ec94e55cd9128aa5ec3fc` after an over-broad formatting-only intermediate delta was repaired forward;
- expanded NOT NULL behavioral contract, with the temporary duplicate fixture helper removed, at `50b69ea481dd0acdb303c91376976d8f130b942f`;
- complete bounded source repair `a5c0ce6624eb53d767a881d0fee49ba44841c5ef`;
- contract-only formatting churn repaired forward at `864cd3bb003c496435110caeacfcf4bf1bb831de` without changing the behavioral boundary;
- base-layer scope wording currentized at `4cf4f93109876eb528507bbd4669a37eb3ead8ca` so it no longer claims blanket PostgreSQL legality for duplicate index-backed names on different relations;
- focused doctoring first recorded at `01b8795b990d7a9d87f693bd718af8c9e1757aab` and currentized after the contract cleanup.

The check executes before ordinary EXCLUDE digest or source-receipt issuance. Existing digest algorithms are unchanged.

## EXCLUDE constraint namespace integrity

Review `5222226880` on exact predecessor `250ed695fc36f83e97a26b004d1d7728712ef2cc` found a source-integrity P1: the ordinary EXCLUDE coordinate was bound to the owning relation schema but `pg_constraint.connamespace` was never independently retained. An extractor or inconsistent catalog tuple could therefore present a constraint namespace different from the owning relation namespace and still collapse into the same governed identity.

Pinned PostgreSQL 18 `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` is the primary authority. `index_constraint_create()` computes `namespaceId = RelationGetNamespace(heapRelation)` and passes that namespace independently to `CreateConstraintEntry()` along with relation OID, backing index OID, key attributes, exclusion operators, inheritance state, and period state. The repair therefore treats `connamespace` as independent source evidence even though normal PostgreSQL DDL makes it equal to the owning relation namespace.

The ordinary-forward lineage is:

- structural source/compile contract `a4f56f9432bb0d8d368d1b36f89d59661623fd2b`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_namespace_contract.rs`;
- production successor `b71fb962177302377a3bd913e185e2ad587025bb`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_namespace.rs`;
- public composition `c7dcee510671be162908b75a4c7b4723e914adc0`;
- CHANGELOG currentization `2cb49dc3b81ce54d1ec4e04fbe6ab989c9859e2b`;
- edge-contract currentization `13bf58005f8d6bd90cbc9ee5a3234b391e6906c1`, covering blank namespace, duplicate coordinate, incomplete inventory, namespace mismatch, provenance success, and unknown receipt behavior;
- focused doctoring currentized at `4b251612160a3c34ba3fc8c94f4680ca6fddd8a7`, `docs/doctoring/postgresql-index-exclusion-constraint-namespace-integrity.md`.

`IndexExclusionConstraintNamespaceSnapshot` requires exactly one explicit resolved `connamespace -> pg_namespace.nspname` observation for every predecessor ordinary EXCLUDE coordinate, binds the raw resolved name into a new domain-separated digest and provenance receipt, and rejects mismatch with the owning relation schema. It does not synthesize the namespace from relation metadata or rewrite an issued predecessor digest. Its scope is namespace identity only; index-backed name coupling is validated separately.

## EXCLUDE catalog-family shape integrity

Review `5222790284` on exact predecessor `fbb935ea1b7e6c17c81ebd9106cc8d6d20b55b5b` found that ordinary EXCLUDE evidence still did not validate fields that belong exclusively to domain, foreign-key, or CHECK constraint families. A malformed extractor tuple could therefore carry cross-family residue without changing the governed EXCLUDE identity.

Pinned PostgreSQL 18 `pg_constraint.h` is explicit: `contypid` is zero outside domain constraints; non-FK rows use `confrelid=0` and space sentinels for `confupdtype`/`confdeltype`/`confmatchtype`; `confkey`, `conpfeqop`, `conppeqop`, `conffeqop`, and `confdelsetcols` are FK-only; `conbin` is CHECK-only. Ordinary EXCLUDE remains a relation constraint with nonzero `conrelid`. Exact `conkey` and `conexclop` remain owned by their existing dedicated successors.

The ordinary-forward repair is:

- structural source/compile RED contract `531da2eb704806694da2dfe6fdc2003f19d89c56`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_catalog_shape_contract.rs`;
- production successor `7fb3e5ac8cc22c0164aeb6c9bee800b78dd4c84a`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_catalog_shape.rs`;
- public composition `3b8982eba610c97d47936d8e8f63441ed79183bc`;
- contract/API currentization `18e76bfb54b777ec6193b02718f23b3b04c751ee`;
- edge-branch coverage `f6862d7d52eb6bed02a2e06ea0410cc722613256`;
- focused doctoring `6129a871ea0763fd463f96b888be5dcbc8abf602`, `docs/doctoring/postgresql-index-exclusion-constraint-catalog-shape-integrity.md`;
- CHANGELOG currentization `afc20d3cce71961aabfe925cd00a5b52430849c0`.

`IndexExclusionConstraintCatalogShapeSnapshot` requires one explicit family-shape observation for every predecessor ordinary EXCLUDE coordinate, retains the raw discriminator/sentinel/presence facts in a new digest/provenance domain, and rejects any domain/FK/CHECK residue. It does not derive emptiness from `contype`, does not retain capture-time nonzero OIDs as semantic identity, and does not duplicate `conkey`/`conexclop` owner truth.

## EXCLUDE backing-index name / schema relation-namespace integrity

Review `5223551886` on exact predecessor `9995a40ae33367ca0cd778740d02b6e55d937bcc` found a cross-catalog P1: ConceptWeave retained the ordinary EXCLUDE `conname` coordinate and exact resolved `conindid` index coordinate independently but did not prove that their names were equal. Existing fixtures therefore admitted `bookings_no_overlap` backed by `bookings_excl_idx`, and a base-layer test over-generalized relation-local `pg_constraint` naming into a statement that the same index-backed constraint name was legal on another relation.

PostgreSQL 18 `CREATE TABLE` documents that an exclusion constraint is implemented using an index with the same name as the constraint. Its constraint-naming note explains that the usual relation-local freedom does not extend to `UNIQUE`, `PRIMARY KEY`, and `EXCLUDE` constraints because their index names must be unique in the schema. `ALTER INDEX ... RENAME` preserves that coupling by renaming an associated table constraint as well.

The ordinary-forward lineage is:

- structural source/compile contract `41d533d3894a2328df252ce4fe70e2529e92edb2`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_name_contract.rs`;
- production successor `8ac1c1ce762f980a15e6369302761e765ba94b02`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_name.rs`;
- public composition `49f4a60004531819a994aaa1264c34bc53425d9e`;
- base-layer contract wording repair `4cf4f93109876eb528507bbd4669a37eb3ead8ca`;
- focused doctoring `942f411f910f1d65f5e27cc420a55a75db6da464`, `docs/doctoring/postgresql-index-exclusion-constraint-backing-index-name-integrity.md`.

`IndexExclusionConstraintIndexNameSnapshot` rebinds the exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE -> catalog-family-shape chain. It compares the independently observed constraint and exact `conindid` index names, requires exactly one observed index with that name in the bounded schema and requires that occurrence to be the exact backing index, rejects collision with an observed top-level `pg_class` relation name, and binds both coordinates into a new digest/provenance domain. It never derives one name from the other and does not impose schema-wide uniqueness on non-index CHECK/NOT NULL/domain constraint names.

## Current state

**INDEX_EXCLUSION_CONSTRAINT_INDEX_NAME_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAME_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_CATALOG_SHAPE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_CATALOG_SHAPE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NAME_INTEGRITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NAME_INTEGRITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_KEY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_EXACT_PREDECESSOR_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_KEY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_PERIOD_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PERIOD_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_RAW_STATE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_LIFECYCLE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_TIMING_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_PARTITION_SOURCE_REPAIRED / INDEX_CONSTRAINT_INHERITANCE_STATE_SOURCE_REPAIRED / INDEX_CONSTRAINT_PARENTAGE_SOURCE_REPAIRED / SOURCE_OBSERVATION_RETAINED / ACCEPTANCE_PENDING**.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is inferred from source commits. One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the focused EXCLUDE index-name/catalog-shape/namespace/relation-local-name/index-role/immediacy/operator/key/period/no-inherit/validation/enforcement/timing contracts, all retained Source Observation/relation-partition contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets acceptance.

The concrete PostgreSQL 18 live differential must read ordinary EXCLUDE `contype`, `conname`, `connamespace`, `conrelid`, `contypid`, `conindid`, `conparentid`, `confrelid`, `confupdtype`, `confdeltype`, `confmatchtype`, `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`, `conkey`, `confkey`, `conpfeqop`, `conppeqop`, `conffeqop`, `confdelsetcols`, `conexclop`, and `conbin` plus the independently resolved backing `pg_class.relname`/schema, other bounded schema `pg_class` relation names, supporting `pg_index.indkey`, `indisunique`, `indisprimary`, `indisexclusion`, `indimmediate`, and resolved backing-index exclusion semantics in the same bounded source observation.

Catalog-family shape coverage must prove ordinary EXCLUDE rows have `contype='x'`, nonzero `conrelid`, zero `contypid`/`confrelid`, space FK action sentinels, NULL FK-only arrays, and NULL `conbin` from independently selected fields. CHECK, FK, domain CHECK, temporal key, and ordinary EXCLUDE rows are controls; ConceptWeave must not manufacture an empty shape from the constraint discriminator.

Constraint namespace coverage must resolve `pg_constraint.connamespace` through `pg_namespace` independently of the `conrelid` relation namespace and prove equality only after both are observed. Relation-local constraint-name coverage must verify that all captured relation-scoped `pg_constraint` families are unique by exact relation identity and `conname`, including explicitly observed first-class NOT NULL evidence. An unobserved constraint family must not be synthesized merely to satisfy either invariant.

Backing-index name coverage must independently read ordinary EXCLUDE `conname` and the exact resolved `conindid` index `pg_class.relname`, prove exact equality, and then prove that index name occupies exactly one slot in the bounded schema relation namespace. A repeated CHECK/NOT NULL name on another relation remains a non-index control; two index-backed EXCLUDE names or an index/table relation-name collision in the same schema must be rejected.

Role coverage must prove an ordinary EXCLUDE backing index is `(indisunique=false, indisprimary=false, indisexclusion=true)` from independently read catalog fields. PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` must remain a separate temporal-key control rather than being absorbed into the ordinary EXCLUDE family.

Timing/immediacy coverage must create NOT DEFERRABLE, DEFERRABLE INITIALLY IMMEDIATE, and DEFERRABLE INITIALLY DEFERRED ordinary EXCLUDE constraints and prove backing `indimmediate` values `true`, `false`, and `false` respectively without deriving either catalog fact from the other. Parent/child partition examples must satisfy the same invariant per local constraint/backing-index coordinate.

`conkey` coverage must include a simple column, an expression zero slot, INCLUDE omission, and partition parent/child local attnums. `conexclop` must be independently read from the constraint and compared with the exact backing-index operator vector. `connoinherit` coverage must include clone-with-parent, standalone-then-attach, and detach lifecycle states. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` controls must remain separate.

## Next causal work

1. Converge the canonical `.github` workflow owner and obtain fresh compatible acceptance for Product bootstrap #35; merge #35 normally only when required gates are terminal GREEN.
2. Obtain one unchanged #46 head with repository-pinned Rust 1.98 native GREEN plus hosted Product/security/dependency/review terminal GREEN.
3. Add the PostgreSQL 18 bounded live differentials for retained ordinary EXCLUDE catalog state, including backing-index name/schema relation-namespace integrity, catalog-family shape, independent constraint namespace, relation-local constraint-name integrity, backing-index role, and timing/`indimmediate` invariants, and obtain fresh terminal GREEN on the resulting unchanged head.
4. Continue bounded source-domain review for materially relevant catalog state rather than claiming catalog completeness from field enumeration alone.
5. Only then adopt the complete #46 child ordinary/non-force into #45, obtain fresh #45 acceptance, and propagate through #6.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, or premature publication/release is authorized.