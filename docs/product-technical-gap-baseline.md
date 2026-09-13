# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution and review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners; ConceptWeave does not copy foreign truth. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. The PostgreSQL 18 first-class NOT NULL lineage includes aggregate admission, partition-parent validation, the `coninhcount` source-domain bound, and the corrected PRIMARY KEY completeness chronology described below. Exact-head native and hosted acceptance remain pending.
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
- PostgreSQL 18 first-class NOT NULL identity: `5189886354 -> 8ce7fd7c... -> 5c0706da... -> 8912039b... -> e58dd0c5... -> 5ac4cf33... -> 91086655... -> 5190108906 -> 4c1c0640... -> 2f8a4f97... -> e1c62c07... -> c5ac66ff... -> e60cfec3... -> 5190362750 -> 28916758... -> 385bf437... -> 1b16ce65... -> 5190456512 -> 4236309b... -> 5a29f0aa... -> ca8d8e5a... -> 5190470810 -> d4df54d9... -> 0cdd4fd3... -> 6c37275c...`.

The shared key-constraint backing-index predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional. Frozen `ColumnObservationV3` remains unchanged.

## Column default / generation expression identity — source repaired

Review `5189444945` on predecessor `fb6b0226fb6c4364f9e241bf01b684e110433386` identified that PostgreSQL 18 column default and generated expressions held by `pg_attrdef` were absent from governed Source Observation identity. Compile/source RED `afc9509f021681e6f6a2d0e6d0c386cce46fad12`, doctoring `3a76c46bc4d12fac2b8c3407a1d6a09005ddd7ea`, production module `bd63564fd721072269a50bf5c8dbecc09a30807f`, aggregate integration `8da6fa3193f9d2191a260372559078bb31f3118a`, and ordering/precondition coverage `3b2b6fda8ccab43ce57488ad5fbb76458502af95` remain retained. Expression evidence binds explicit no/default/generation state and exact server-rendered `pg_get_expr(adbin, adrelid)`, cannot infer missing `attgenerated`, cannot attach after later optional families, and extends the digest exactly once after source-authoritative generation evidence.

## PostgreSQL 18 NOT NULL constraint identity — source repaired, acceptance pending

Review `5189886354` identified a PostgreSQL 18-specific loss in the successor representation. `ColumnObservationV3::nullable` retains the `pg_attribute.attnotnull` summary, while PostgreSQL 18 stores table NOT NULL specifications as first-class `pg_constraint.contype = 'n'` rows. A first-class constraint can differ in name, validation, enforcement, locality/inheritance and partition-parent linkage while the nullable summary remains equal.

Initial contract `8ce7fd7c7715e33dbb9df799dc903a6adaa079f2` requires materially different names, validation/enforcement and inheritance state to change governed identity; observed-empty to differ from family-unobserved; complete bounded non-nullable-column coverage; contradiction rejection for nullable columns; one explicit NOT NULL constraint per column; and input-order invariance. Parent-linkage RED `e58dd0c56d565b5cc5da6fe21eb89743df1228be` requires nonzero `conparentid` to be resolved into stable source coordinates rather than hashed as a catalog OID. Duplicate-name RED `5ac4cf332a4e1e704e8fb3d6ba9c7c5b73a53ca6` additionally requires relation-local constraint-name uniqueness.

Primary-source doctoring selected a separate domain-separated evidence family rather than mutating the frozen column representation. Production module `8912039b0348b26f2ab925bc4c182011a8f3ea9f` added `NotNullConstraintObservation`, `ParentNotNullConstraintCoordinate`, bounded-column canonicalization, nullable-summary consistency, duplicate-column rejection and `conceptweave.postgres_schema_snapshot.v3.not_null_constraint.v1` digest framing. `91086655ffa2148b35bcccd5a8a1f4ee38136cd9` rejects duplicate exact constraint names within one owning relation.

Review `5190108906` found that resolved `conparentid` accepted arbitrary `RelationKind` values even though PostgreSQL defines it as the corresponding constraint of the parent partitioned table. RED `4c1c0640fc5460bb47ec074c068936e34af22543`, repair `2f8a4f9743f653cfbac092c28376cde7881bd6af`, and doctoring `e1c62c07bd43f6cbfa0bafc7dba708dce32bb886` require `RelationKind::PartitionedTable` / `pg_class.relkind = 'p'` before that coordinate can enter governed hashing.

Aggregate repair `c5ac66ffa623f6b99ba0c08f73eea4e22f06937b` declares and re-exports the family, adds aggregate state and observed-state tracking, `new_with_not_null_constraints`, `with_observed_not_null_constraints`, and `not_null_constraints`, and extends the governed digest exactly once through the module-owned domain. The original v3 digest remains unchanged when the family is unobserved. Observed-empty is distinct from unobserved. Canonical optional-family order is `type/array -> collation -> generation -> expression -> identity -> NOT NULL -> timing -> PERIOD`; earlier family attachment after NOT NULL and later NOT NULL attachment after timing/PERIOD fail closed. Edge contract `e60cfec34fd99df4f7623ec821384064771951c0` pins duplicate-family and ordering protection.

Review `5190362750` found that the public source model accepted the full `u16` range for `pg_constraint.coninhcount`, whose PostgreSQL source type is signed `int2`. RED `2891675822b13282ab57d968cda985d0628920ac`, repair `385bf43794849836ef6871c2abad64c124ddd723`, and doctoring `1b16ce65d674836e88073ba2ea857aa27fb1b2a5` now reject values above 32767 before governed hashing. Negative source values must be rejected by the future adapter before conversion to the public nonnegative count.

### Corrected PRIMARY KEY chronology

Review `5190456512` used an April 2024 development-state discussion to conclude that `attnotnull=true` could be backed directly by a PRIMARY KEY without a separate `contype='n'` row. RED `4236309b5317f54a38d39996affd5b7a950b9f06`, repair `5a29f0aa2d0a07ef1ef3182326efb4e70ecfe5a2`, and doctoring `ca8d8e5a7ddf7971a18cea1e5714a523d94d444e` temporarily removed reverse completeness.

Fresh review `5190470810` found that this was chronologically stale for PostgreSQL 18. Later upstream work explicitly changed PRIMARY KEY handling to create/queue NOT NULL constraints for key columns; PostgreSQL 18 `pg_attribute.attnotnull` documents that the column has a possibly invalid not-null constraint. Corrected RED `d4df54d99bfb50329d9a42c8ce0da61bf77dc536` requires a PRIMARY-KEY-backed non-nullable column with an observed-empty NOT NULL inventory to fail `not_null_constraint_completeness`. Production repair `0cdd4fd3b37642373482cfa9874fe99268ed04d9` restores bounded `attnotnull` ↔ captured `contype='n'` column completeness ordinary-forward. Doctoring `6c37275c49a1dc267d2d7f7ecaca5de6e74886df` records both the superseded intermediate model and PostgreSQL 18 final semantics so the same chronology error is not repeated.

The aggregate source repair is present, but repository-pinned Rust and hosted acceptance have not been established on the moved head. Current Source Observation state is **NOT_NULL_CONSTRAINT_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

The active next causal gate is unchanged-head execution and hosted acceptance, not another semantic family. One exact #46 head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, NOT NULL/expression/generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host has not established repository-pinned Rust 1.98 native execution. These gaps are not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, self-approve, dismiss review, force-push, destructively rebase, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. Product bootstrap/review integration evidence never transfers to #46.

Fresh `.github#2079` authority remains a separate OPEN/Draft owner lane whose finding/probe relation failure is an executable RED. Current owner tests require confirmed probes to carry a valid finding binding at the same changed-side location and falsified probes to carry explicit null; production schema/prompt/validator repair is not claimed GREEN. Provider/model fallback and validator weakening remain invalid repairs.

Central hosted security checks never substitute for that intentional Noema contract RED and are not ConceptWeave acceptance evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

Catalog OIDs are capture-time joins only. Constraint support must bind `pg_constraint.conindid` to the exact same-snapshot `pg_index` row and explicitly capture usable lifecycle, key/static flags, `conexclop`, operator-class/operator-family evidence, timing/action/match state, and temporal type/domain chains. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail closed.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull` and all matching bounded-relation `pg_constraint` rows with `contype = 'n'` in the same catalog snapshot. `conkey` must resolve to exactly one bounded column. It retains exact `conname`, `convalidated`, `conenforced`, `conislocal`, `coninhcount`, and `connoinherit`, and resolves nonzero `conparentid` to an exact parent schema/relation/kind/constraint coordinate only after verifying `pg_class.relkind = 'p'` / `PartitionedTable`. `coninhcount` is read as signed `int2`; negative or out-of-domain values fail closed before conversion. For bounded user relations, the resolved `contype='n'` column set must equal the captured `attnotnull=true` column set. PRIMARY KEY columns are not an exception in PostgreSQL 18 because PRIMARY KEY creation queues first-class NOT NULL constraints. Duplicate names within one relation, duplicate-column, multi-column, unknown, non-partitioned-parent, out-of-domain, missing, or contradictory rows fail closed.

For column collation, generation, identity, and expressions, the existing source-authoritative families and exact optional-family ordering remain unchanged. Expression evidence preserves exact server-rendered `pg_get_expr(adbin, adrelid)` while catalog OIDs/internal node serialization remain capture-time details only.

## Primary authority

- PostgreSQL Global Development Group. (2025). *PostgreSQL 18.0 release notes*. https://www.postgresql.org/docs/18/release-18.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- Herrera, A. (2024, September 25). *Re: not null constraints, again* [PostgreSQL hackers message]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202409252014.74iepgsyuyws%40alvherre.pgsql
- Herrera, A. (2025, April 1). *Re: Support NOT VALID / VALIDATE constraint options for named NOT NULL constraints* [PostgreSQL hackers message]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202504012022.wzrtvfhrltud%40alvherre.pgsql
- Herrera, A. (2024, April 12). *Re: Can't find not null constraint, but \\d+ shows that* [superseded development-state evidence]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202404120752.6ebv4q5zwnfw%40alvherre.pgsql
