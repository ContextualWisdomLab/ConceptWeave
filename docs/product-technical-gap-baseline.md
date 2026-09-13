# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution and review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners; ConceptWeave does not copy foreign truth. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. Column-expression aggregate integration remains retained. PostgreSQL 18 first-class NOT NULL constraint RED `8ce7fd7c7715e33dbb9df799dc903a6adaa079f2`, doctoring `5c0706da778391e39ca94995d5c70c99060f0a17`, staged production module `8912039b0348b26f2ab925bc4c182011a8f3ea9f`, and partition-parent contract `e58dd0c56d565b5cc5da6fe21eb89743df1228be` now define the active causal slice.
- Product bootstrap #35 remains a separate Product acceptance lane. Central workflow evidence never transfers to #46.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column-collation evidence, source-authoritative column-generation declaration mode, source-authoritative column default/generated-expression evidence, and source-authoritative column-identity declaration mode. PostgreSQL 18 first-class NOT NULL constraint state is the active remaining RED: the staged family preserves constraint name, validation/enforcement, inheritance/locality and resolved partition-parent coordinates, but it is not yet admitted by `PostgresSchemaSnapshotV3`. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

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
- column default/generated-expression identity: `5189444945 -> afc9509f... -> 3a76c46b... -> bd63564f... -> 8da6fa31... -> 3b2b6fda...`.

The shared key-constraint backing-index predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional. Frozen `ColumnObservationV3` remains unchanged.

## Column default / generation expression identity — source repaired, acceptance superseded by active NOT NULL RED

Review `5189444945` on predecessor `fb6b0226fb6c4364f9e241bf01b684e110433386` identified that PostgreSQL 18 column default and generated expressions held by `pg_attrdef` were absent from governed Source Observation identity. `pg_attribute.atthasdef` establishes only that an expression exists; `attgenerated` distinguishes ordinary versus generated declaration mode but does not preserve the expression itself. `ALTER TABLE ... SET DEFAULT` and generated-column expression changes can therefore alter source behavior while all previously modeled column facts remain equal.

Compile/source RED `afc9509f021681e6f6a2d0e6d0c386cce46fad12` added `crates/conceptweave-observation/tests/column_expression_contract.rs`. Doctoring `3a76c46bc4d12fac2b8c3407a1d6a09005ddd7ea` selected exact schema/relation/kind/column coordinates plus explicit no/default/generation expression state and exact server-rendered `pg_get_expr(adbin, adrelid)` text as governed evidence. Production module `bd63564fd721072269a50bf5c8dbecc09a30807f`, aggregate integration `8da6fa3193f9d2191a260372559078bb31f3118a`, and ordering/precondition coverage `3b2b6fda8ccab43ce57488ad5fbb76458502af95` remain retained. Expression evidence cannot infer missing `attgenerated`, cannot attach after later optional families, and extends the digest exactly once after source-authoritative generation evidence.

That repair remains part of the child delta but no longer defines the active acceptance head because the later PostgreSQL 18 NOT NULL RED moved #46.

## PostgreSQL 18 NOT NULL constraint identity — module staged, aggregate RED active

Review `5189886354` identified a PostgreSQL 18-specific loss in the successor representation. `ColumnObservationV3::nullable` retains only the `pg_attribute.attnotnull` summary. PostgreSQL 18 now stores explicit column NOT NULL specifications as `pg_constraint.contype = 'n'`; `attnotnull` itself is documented as indicating a possibly invalid not-null constraint. The first-class constraint row can differ in name, validation, enforcement, locality/inheritance and partition-parent linkage while the existing nullable summary remains equal.

Compile/source RED `8ce7fd7c7715e33dbb9df799dc903a6adaa079f2` adds `crates/conceptweave-observation/tests/not_null_constraint_contract.rs`. It requires materially different names, validation/enforcement and inheritance state to change governed identity; observed-empty to differ from family-unobserved; complete coverage of every bounded non-nullable column; contradiction rejection for nullable columns; one explicit NOT NULL constraint per column; and input-order invariance.

Primary-source doctoring `5c0706da778391e39ca94995d5c70c99060f0a17` records the PostgreSQL 18 catalog change and selects a separate domain-separated evidence family rather than mutating the frozen column representation or synthesizing constraints from `attnotnull`.

Production module `8912039b0348b26f2ab925bc4c182011a8f3ea9f` stages `NotNullConstraintObservation`, `ParentNotNullConstraintCoordinate`, complete bounded-column canonicalization, nullable-summary consistency, duplicate-column rejection and `conceptweave.postgres_schema_snapshot.v3.not_null_constraint.v1` digest framing. Catalog OIDs do not enter governed identity. Nonzero `conparentid` is represented by a resolved parent schema/relation/kind/constraint coordinate. Parent-linkage RED `e58dd0c56d565b5cc5da6fe21eb89743df1228be` requires that resolved coordinate to change governed identity and keeps the public contract OID-free.

The production module is intentionally not yet declared/re-exported by the crate root and `PostgresSchemaSnapshotV3` does not yet retain an observed flag, state, constructor, consuming attachment seam, accessor or digest extension. The tests therefore remain a real compile-level RED. Current Source Observation state is **NOT_NULL_CONSTRAINT_MODULE_STAGED / AGGREGATE_RED_ACTIVE**. Native/Product GREEN, Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

The active next causal change is aggregate admission for the PostgreSQL 18 NOT NULL family. It must preserve the original v3 digest when the family is unobserved, distinguish observed-empty state, extend the digest exactly once, and enforce a single canonical optional-family order without allowing late attachment to create an alternate semantic identity. Once the RED is repaired, one unchanged exact #46 head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, NOT NULL/expression/generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host has not established repository-pinned Rust 1.98 native execution for this head. Hosted checks must be evaluated only against the final unchanged exact #46 head; predecessor evidence is not transferred. This is not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, self-approve, dismiss review, force-push, destructively rebase, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. Product bootstrap/review integration evidence never transfers to #46.

The Required Noema Review finding/probe relation failure remains an executable owner RED in `.github#2079@35d1b39ad268e5cf7157716296c926df2bded3e6`, OPEN / Draft / mechanically mergeable at the last verified owner coordinate. This head is an ordinary non-force merge-forward of intentional RED `6ca329896a846110ade7182ed6fa0fa7b0fbba7d` and then-current protected `.github/main`; it does not by itself prove the production schema/prompt/validator repair. The advertised structured-output schema still represented `findings[]` and `adversarial_validation.probes[]` independently while the deterministic validator required a confirmed probe anchored to a published finding location. Owner review `5188653693`, RED `6ca329896a846110ade7182ed6fa0fa7b0fbba7d`, and review `5188972300` define the owner-local repair: a schema-representable finding/probe binding, prompt population rules, and deterministic validation of a valid referenced finding plus the same changed-side location. Provider/model fallback and validator weakening are invalid repairs.

Central hosted security checks never substitute for that intentional Noema contract RED and are not ConceptWeave acceptance evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

Catalog OIDs are capture-time joins only. Constraint support must bind `pg_constraint.conindid` to the exact same-snapshot `pg_index` row and explicitly capture usable lifecycle, key/static flags, `conexclop`, operator-class/operator-family evidence, timing/action/match state, and temporal type/domain chains. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail closed.

For column collation, the adapter captures `attcollation` for every bounded column when claiming that family; zero is explicit uncollatable evidence and nonzero OIDs resolve inside the same catalog snapshot to exact `pg_collation` namespace/name plus `collisdeterministic`.

For PostgreSQL 18 NOT NULL constraints, the adapter captures `pg_attribute.attnotnull` and all matching relation `pg_constraint` rows with `contype = 'n'` in the same bounded catalog snapshot. `conkey` must resolve to exactly one bounded column. The adapter retains exact `conname`, `convalidated`, `conenforced`, `conislocal`, `coninhcount`, and `connoinherit`, and resolves nonzero `conparentid` to the exact parent schema/relation/kind/constraint coordinate. OIDs remain joins only. When the family is claimed, its resolved column set must exactly match columns whose `attnotnull` summary is true; missing, duplicate, multi-column, unknown, or contradictory rows fail closed.

For column identity, the adapter captures `attidentity` and `attnotnull` for every bounded column when claiming that family and maps only documented empty/`a`/`d` states. Unexpected values fail closed; sequence options/ownership are separate evidence.

For column generation, the adapter captures `attgenerated` for every bounded column when claiming that family and maps only documented empty/`s`/`v` states. Unexpected values fail closed. `atthasdef` does not establish expression kind or contents.

For column expressions, the adapter joins `pg_attribute` to the matching `pg_attrdef` row within the same bounded catalog snapshot. When the family is claimed, every bounded column records explicit no-expression, default-expression, or generation-expression state. Present expressions preserve exact server-rendered `pg_get_expr(adbin, adrelid)` text and must agree with already-observed generation mode. `pg_attrdef` OIDs and internal `adbin` serialization do not become governed consumer coordinates.

## Primary authority

- PostgreSQL Global Development Group. (2025). *PostgreSQL 18.0 release notes*. https://www.postgresql.org/docs/release/18.0/
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attrdef*. https://www.postgresql.org/docs/18/catalog-pg-attrdef.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_collation*. https://www.postgresql.org/docs/18/catalog-pg-collation.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Identity columns*. https://www.postgresql.org/docs/18/ddl-identity-columns.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Generated columns*. https://www.postgresql.org/docs/18/ddl-generated-columns.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
