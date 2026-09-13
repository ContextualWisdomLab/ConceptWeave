# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution and review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners; ConceptWeave does not copy foreign truth. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. The PostgreSQL 18 first-class NOT NULL lineage is `5189886354 -> 8ce7fd7c... -> 5c0706da... -> 8912039b... -> e58dd0c5... -> 5ac4cf33... -> 91086655... -> 5190108906 -> 4c1c0640... -> 2f8a4f97... -> e1c62c07... -> c5ac66ff... -> e60cfec3... -> 5190362750 -> 28916758... -> 385bf437... -> 1b16ce65...`. Aggregate admission and the `coninhcount` source-domain bound are source-repaired; exact-head native and hosted acceptance remain pending.
- Product bootstrap #35 remains a separate Product acceptance lane. Central workflow evidence never transfers to #46.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column-collation evidence, source-authoritative column-generation declaration mode, source-authoritative column default/generated-expression evidence, source-authoritative column-identity declaration mode, and PostgreSQL 18 first-class NOT NULL constraint evidence. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

Retained production repairs include:

- literal exclusion-operator inference removal: `5186175514 -> a9065d46... -> e286c352...`;
- PERIOD referenced-key completeness: `5186323924 -> d692773a... -> 1462105f...`;
- temporal backing-index ordered-key/static-shape coherence: `5186585545 -> ffe75edd... -> e258b394...`;
- explicit unusable lifecycle rejection: `5186802339 -> 681e280f... / ed4882e... -> ff842b35...`;
- lifecycle-complete key support: `5187402940 -> 9d9e8d9e... -> 77b12263... -> 384d1305...`;
- column-collation source evidence and FK consistency: `5187855669 -> 8006b24f... -> 3e495eac... -> 66489478... -> 3d8a7fb7... -> 8a4b7a1b... -> 6fb0c2b6...`;
- column-identity declaration mode: `5188215648 -> 85f2eb0c... -> 69a86d2a... -> 95f7f81f...`;
- identity/nullability consistency: `5188419794 -> bd6da911... -> 00b166bf... -> 0bba879d...`;
- column-generation declaration mode: `5188836578 -> be3adfd0... -> 493e56bc... -> 32555632... -> f4e8295b... -> 8b0c6447... -> d67433cc...`;
- column default/generated-expression identity: `5189444945 -> afc9509f... -> 3a76c46b... -> bd63564f... -> 8da6fa31... -> 3b2b6fda...`;
- PostgreSQL 18 first-class NOT NULL identity, aggregate admission, partition-parent validity and `coninhcount` source-domain bound: `5189886354 -> 8ce7fd7c... -> 5c0706da... -> 8912039b... -> e58dd0c5... -> 5ac4cf33... -> 91086655... -> 5190108906 -> 4c1c0640... -> 2f8a4f97... -> e1c62c07... -> c5ac66ff... -> e60cfec3... -> 5190362750 -> 28916758... -> 385bf437... -> 1b16ce65...`.

The shared key-constraint backing-index predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional. Frozen `ColumnObservationV3` remains unchanged.

## Column default / generation expression identity — source repaired

Review `5189444945` on predecessor `fb6b0226fb6c4364f9e241bf01b684e110433386` identified that PostgreSQL 18 column default and generated expressions held by `pg_attrdef` were absent from governed Source Observation identity. Compile/source RED `afc9509f021681e6f6a2d0e6d0c386cce46fad12`, doctoring `3a76c46bc4d12fac2b8c3407a1d6a09005ddd7ea`, production module `bd63564fd721072269a50bf5c8dbecc09a30807f`, aggregate integration `8da6fa3193f9d2191a260372559078bb31f3118a`, and ordering/precondition coverage `3b2b6fda8ccab43ce57488ad5fbb76458502af95` remain retained. Expression evidence binds explicit no/default/generation state and exact server-rendered `pg_get_expr(adbin, adrelid)`, cannot infer missing `attgenerated`, cannot attach after later optional families, and extends the digest exactly once after source-authoritative generation evidence.

## PostgreSQL 18 NOT NULL constraint identity — source repaired, acceptance pending

Review `5189886354` identified a PostgreSQL 18-specific loss in the successor representation. `ColumnObservationV3::nullable` retains the `pg_attribute.attnotnull` summary, while PostgreSQL 18 stores table NOT NULL constraints as first-class `pg_constraint.contype = 'n'` rows. A first-class constraint can differ in name, validation, enforcement, locality/inheritance and partition-parent linkage while the nullable summary remains equal.

Compile/source RED `8ce7fd7c7715e33dbb9df799dc903a6adaa079f2` requires materially different names, validation/enforcement and inheritance state to change governed identity; observed-empty to differ from family-unobserved; complete coverage of every bounded non-nullable column; contradiction rejection for nullable columns; one explicit NOT NULL constraint per column; and input-order invariance. Parent-linkage RED `e58dd0c56d565b5cc5da6fe21eb89743df1228be` requires nonzero `conparentid` to be resolved into stable source coordinates rather than hashed as a catalog OID. Duplicate-name RED `5ac4cf332a4e1e704e8fb3d6ba9c7c5b73a53ca6` additionally requires relation-local constraint-name uniqueness.

Primary-source doctoring `5c0706da778391e39ca94995d5c70c99060f0a17` records the PostgreSQL 18 catalog change and selects a separate domain-separated evidence family rather than mutating the frozen column representation or synthesizing constraints from `attnotnull`. Production module `8912039b0348b26f2ab925bc4c182011a8f3ea9f` added `NotNullConstraintObservation`, `ParentNotNullConstraintCoordinate`, complete bounded-column canonicalization, nullable-summary consistency, duplicate-column rejection and `conceptweave.postgres_schema_snapshot.v3.not_null_constraint.v1` digest framing. Follow-up `91086655ffa2148b35bcccd5a8a1f4ee38136cd9` rejects duplicate exact constraint names within one owning relation.

Review `5190108906` found that resolved `conparentid` accepted arbitrary `RelationKind` values even though PostgreSQL defines it as the corresponding constraint of the parent partitioned table. RED `4c1c0640fc5460bb47ec074c068936e34af22543`, repair `2f8a4f9743f653cfbac092c28376cde7881bd6af`, and doctoring `e1c62c07bd43f6cbfa0bafc7dba708dce32bb886` now require `RelationKind::PartitionedTable` / `pg_class.relkind = 'p'` before that coordinate can enter governed hashing.

Aggregate repair `c5ac66ffa623f6b99ba0c08f73eea4e22f06937b` declares and re-exports the family, adds aggregate state and observed-state tracking, `new_with_not_null_constraints`, `with_observed_not_null_constraints`, and `not_null_constraints`, and extends the governed digest exactly once through the module-owned domain. The original v3 digest remains unchanged when the family is unobserved. Observed-empty is distinct from unobserved. Canonical optional-family order is now `type/array -> collation -> generation -> expression -> identity -> NOT NULL -> timing -> PERIOD`; earlier family attachment after NOT NULL and later NOT NULL attachment after timing/PERIOD fail closed.

Edge contract `e60cfec34fd99df4f7623ec821384064771951c0` makes duplicate NOT NULL-family attachment fail with `not_null_constraint_already_observed` and verifies that an earlier type-kind family cannot be attached after NOT NULL evidence.

Exact-source review `5190362750` found a remaining catalog-domain defect on predecessor `987c83333b36807f895a491e6e2ebe1611ec3704`: the production API modeled `coninhcount` as `u16` without bounding it to PostgreSQL's signed `int2` source domain. RED `2891675822b13282ab57d968cda985d0628920ac` proves that 32768 must be rejected, repair `385bf43794849836ef6871c2abad64c124ddd723` fails closed with `not_null_constraint_inheritance_ancestor_count` before governed hashing, and doctoring `1b16ce65d674836e88073ba2ea857aa27fb1b2a5` records the source-domain and adapter contract. Values 0..32767 remain representable; negative source values never enter the unsigned domain and must be rejected by the future adapter before conversion.

The aggregate source repair is present, but repository-pinned Rust and hosted acceptance have not been established on the moved head. Current Source Observation state is **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. Ready, merge authorization, publication, and release are not claimed.

PostgreSQL 18 documentation states that `pg_constraint` stores not-null constraints, defines `coninhcount` as `int2`, and says the field is the number of direct inheritance ancestors. PostgreSQL also forces primary-key columns NOT NULL. Adapter completeness must compare the captured first-class table NOT NULL inventory with the bounded `attnotnull` summary rather than infer rows from names or DDL text.

## Exact-head acceptance

The active next causal gate is unchanged-head execution and hosted acceptance, not another semantic family. One exact #46 head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, NOT NULL/expression/generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The latest source/doc movement invalidates predecessor execution evidence. Pull-request workflow and combined-status evidence must be read fresh for the exact current head. The current execution host has not established repository-pinned Rust 1.98 native execution. These gaps are not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, self-approve, dismiss review, force-push, destructively rebase, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. Product bootstrap/review integration evidence never transfers to #46.

Fresh `.github#2079` metadata reports exact `20972a57ce768751dbb4b29e7dbed76271bfed14`, OPEN / Draft / mechanically mergeable on protected `.github/main@64f483db9d052322c65bcdf1675d66138156f306`. The finding/probe relation failure remains an executable owner RED. Current owner tests require confirmed probes to carry a valid finding binding at the same changed-side location and falsified probes to carry explicit null; production schema/prompt/validator repair is still not claimed GREEN. Provider/model fallback and validator weakening remain invalid repairs.

Central hosted security checks never substitute for that intentional Noema contract RED and are not ConceptWeave acceptance evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

Catalog OIDs are capture-time joins only. Constraint support must bind `pg_constraint.conindid` to the exact same-snapshot `pg_index` row and explicitly capture usable lifecycle, key/static flags, `conexclop`, operator-class/operator-family evidence, timing/action/match state, and temporal type/domain chains. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail closed.

For column collation, the adapter captures `attcollation` for every bounded column when claiming that family; zero is explicit uncollatable evidence and nonzero OIDs resolve inside the same catalog snapshot to exact `pg_collation` namespace/name plus `collisdeterministic`.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull` and all matching relation `pg_constraint` rows with `contype = 'n'` in the same bounded catalog snapshot. `conkey` must resolve to exactly one bounded column. The adapter retains exact `conname`, `convalidated`, `conenforced`, `conislocal`, `coninhcount`, and `connoinherit`, and resolves nonzero `conparentid` to the exact parent schema/relation/kind/constraint coordinate only after joining the parent relation and verifying `pg_class.relkind = 'p'` / `PartitionedTable`. `coninhcount` is read as PostgreSQL signed `int2`; a negative value or any value outside the documented nonnegative source domain fails closed before conversion to the public nonnegative count. OIDs remain joins only. When the family is claimed, its resolved column set must exactly match columns whose `attnotnull` summary is true; duplicate constraint names within one relation and missing, duplicate-column, multi-column, unknown, non-partitioned-parent, out-of-domain, or contradictory rows fail closed.

For column identity, the adapter captures `attidentity` and `attnotnull` for every bounded column when claiming that family and maps only documented empty/`a`/`d` states. Unexpected values fail closed; sequence options/ownership are separate evidence.

For column generation, the adapter captures `attgenerated` for every bounded column when claiming that family and maps only documented empty/`s`/`v` states. Unexpected values fail closed. `atthasdef` does not establish expression kind or contents.

For column expressions, the adapter joins `pg_attribute` to the matching `pg_attrdef` row within the same bounded catalog snapshot. When the family is claimed, every bounded column records explicit no-expression, default-expression, or generation-expression state. Present expressions preserve exact server-rendered `pg_get_expr(adbin, adrelid)` text and must agree with already-observed generation mode. `pg_attrdef` OIDs and internal `adbin` serialization do not become governed consumer coordinates.

## Primary authority

- PostgreSQL Global Development Group. (2025). *PostgreSQL 18.0 release notes*. https://www.postgresql.org/docs/release/18.0/
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attrdef*. https://www.postgresql.org/docs/18/catalog-pg-attrdef.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_collation*. https://www.postgresql.org/docs/18/catalog-pg-collation.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Identity columns*. https://www.postgresql.org/docs/18/ddl-identity-columns.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Generated columns*. https://www.postgresql.org/docs/18/ddl-generated-columns.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- Herrera, A. (2023). *Catalog not-null constraints* [PostgreSQL committers message]. PostgreSQL. https://www.postgresql.org/message-id/E1qZV2v-000p1M-8n%40gemulon.postgresql.org
