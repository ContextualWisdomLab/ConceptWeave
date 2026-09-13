# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only. Execution and review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, and `contextual-orchestrator` production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners; ConceptWeave does not copy foreign truth. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft, is the active Source Observation writer. Its current source lineage is `ac590894bfd93297aed1b83859d4745bb5235d40 -> bd63564fd721072269a50bf5c8dbecc09a30807f`.
- Product bootstrap #35 remains a separate Product acceptance lane. Central workflow evidence never transfers to #46.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal-constraint evidence, source-authoritative column-collation evidence, source-authoritative column-generation declaration mode, and source-authoritative column-identity declaration mode. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

Retained production repairs include:

- literal exclusion-operator inference removal: `5186175514 -> a9065d46... -> e286c352...`;
- PERIOD referenced-key completeness: `5186323924 -> d692773a... -> 1462105f...`;
- temporal backing-index ordered-key/static-shape coherence: `5186585545 -> ffe75edd... -> e258b394...`;
- explicit unusable lifecycle rejection: `5186802339 -> 681e280f... / ed4882e... -> ff842b35...`;
- lifecycle-complete key support: `5187402940 -> 9d9e8d9e... -> 77b12263... -> 384d1305...`;
- column-collation source evidence and FK consistency: `5187855669 -> 8006b24f... -> 3e495eac... -> 66489478... -> 3d8a7fb7... -> 8a4b7a1b... -> 6fb0c2b6...`;
- column-identity declaration mode: `5188215648 -> 85f2eb0c... -> 69a86d2a... -> 95f7f81f...`;
- identity/nullability consistency: `5188419794 -> bd6da911... -> 00b166bf... -> 0bba879d...`;
- column-generation declaration mode: `5188836578 -> be3adfd0... -> 493e56bc... -> 32555632... -> f4e8295b... -> 8b0c6447... -> d67433cc...`.

The shared key-constraint backing-index predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional. Frozen `ColumnObservationV3` remains unchanged.

## Column default / generation expression identity — module staged, aggregate RED active

Review `5189444945` on predecessor `fb6b0226fb6c4364f9e241bf01b684e110433386` identified that PostgreSQL 18 column default and generated expressions held by `pg_attrdef` were absent from governed Source Observation identity. `pg_attribute.atthasdef` establishes only that an expression exists; `attgenerated` distinguishes ordinary versus generated declaration mode but does not preserve the expression itself. `ALTER TABLE ... SET DEFAULT` and generated-column expression changes can therefore alter source behavior while all previously modeled column facts remain equal.

Compile/source RED `afc9509f021681e6f6a2d0e6d0c386cce46fad12` adds `crates/conceptweave-observation/tests/column_expression_contract.rs`. The contract requires:

- materially different ordinary defaults to produce different governed digests;
- materially different generation expressions to produce different governed digests;
- explicit no-expression evidence to remain distinct from family-unobserved state;
- complete bounded relation-column coverage;
- duplicate-coordinate rejection;
- input-order invariance;
- consistency with the already-authoritative column-generation family.

Doctoring `3a76c46bc4d12fac2b8c3407a1d6a09005ddd7ea` selects exact schema/relation/kind/column coordinates plus explicit no/default/generation expression state and exact server-rendered `pg_get_expr(adbin, adrelid)` text as governed evidence. `pg_attrdef` OIDs and `adbin`'s internal `pg_node_tree` serialization remain capture-time/internal details rather than consumer identity. Dependency extraction, function volatility/leakproof/security, semantic equivalence, and application-domain meaning remain separate later contracts.

Production commit `bd63564fd721072269a50bf5c8dbecc09a30807f` now stages `crates/conceptweave-observation/src/column_expression.rs`. It implements `ColumnExpressionObservation` with exact bounded coordinates, explicit no/default/generation states, nonblank exact expression preservation, complete coverage, duplicate rejection, deterministic ordering, generation-mode consistency, and domain-separated framing under `conceptweave.postgres_schema_snapshot.v3.column_expression.v1`.

This is deliberately not a source-repaired claim. `PostgresSchemaSnapshotV3` still lacks root integration: `mod column_expression`, public re-export, stored expression-family state/observed flag, `new_with_column_expressions`, `with_observed_column_expressions`, `column_expressions`, digest attachment, and canonical optional-family ordering. Until those are integrated, `column_expression_contract.rs` remains expected compile/source RED.

The canonical optional-family order is type/array -> column collation -> column generation -> column expression -> column identity -> constraint timing -> PERIOD. Expression attachment must require an already-observed generation family; reverse-order attachment after identity/timing/PERIOD must fail closed rather than create a second identity for the same source facts.

Current Source Observation state is **COLUMN_EXPRESSION_MODULE_STAGED / AGGREGATE_RED_ACTIVE**. Native/Product GREEN, source-repaired status, Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

After aggregate repair, one unchanged exact #46 head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, `column_expression_contract`, generation/identity/collation contracts, retained lifecycle/temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable hosted Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host has not established repository-pinned Rust 1.98 native execution for this head. #46 also has no pull-request-triggered workflow run at the pre-module `ac590894...` checkpoint; no predecessor evidence is transferred to the staged module successor. This is not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, self-approve, dismiss review, force-push, destructively rebase, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. Product bootstrap/review integration evidence never transfers to #46.

The Required Noema Review finding/probe relation failure remains an executable owner RED in `.github#2079@6ca329896a846110ade7182ed6fa0fa7b0fbba7d`. Fresh metadata reports OPEN / Draft with mechanical mergeability currently false. The advertised structured-output schema still represents `findings[]` and `adversarial_validation.probes[]` independently while the deterministic validator requires a confirmed probe anchored to a published finding location. Owner review `5188653693`, RED `6ca329896a846110ade7182ed6fa0fa7b0fbba7d`, and review `5188972300` define the owner-local repair: a schema-representable finding/probe binding, prompt population rules, and deterministic validation of a valid referenced finding plus the same changed-side location. Provider/model fallback and validator weakening are invalid repairs.

On exact `.github#2079@6ca329...`, Security Scan run `34731494426`, SAST Semgrep `34731494413`, Python Security `34731494455`, and CodeQL PR `34731494433` are terminal success. Those security checks do not satisfy the intentional Noema RED and are not ConceptWeave acceptance evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, and use bounded `REPEATABLE READ READ ONLY` catalog capture. It must never keep an explicit database transaction or lock open while waiting on an LLM or long external computation.

Catalog OIDs are capture-time joins only. Constraint support must bind `pg_constraint.conindid` to the exact same-snapshot `pg_index` row and explicitly capture usable lifecycle, key/static flags, `conexclop`, operator-class/operator-family evidence, timing/action/match state, and temporal type/domain chains. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail closed.

For column collation, the adapter captures `attcollation` for every bounded column when claiming that family; zero is explicit uncollatable evidence and nonzero OIDs resolve inside the same catalog snapshot to exact `pg_collation` namespace/name plus `collisdeterministic`.

For column identity, the adapter captures `attidentity` and `attnotnull` for every bounded column when claiming that family and maps only documented empty/`a`/`d` states. Unexpected values fail closed; sequence options/ownership are separate evidence.

For column generation, the adapter captures `attgenerated` for every bounded column when claiming that family and maps only documented empty/`s`/`v` states. Unexpected values fail closed. `atthasdef` does not establish expression kind or contents.

For column expressions, the adapter joins `pg_attribute` to the matching `pg_attrdef` row within the same bounded catalog snapshot. When the family is claimed, every bounded column records explicit no-expression, default-expression, or generation-expression state. Present expressions preserve exact server-rendered `pg_get_expr(adbin, adrelid)` text and must agree with already-observed generation mode. `pg_attrdef` OIDs and internal `adbin` serialization do not become governed consumer coordinates.

## Primary authority

- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_attrdef*. https://www.postgresql.org/docs/18/catalog-pg-attrdef.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_collation*. https://www.postgresql.org/docs/18/catalog-pg-collation.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: Identity columns*. https://www.postgresql.org/docs/18/ddl-identity-columns.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: Generated columns*. https://www.postgresql.org/docs/18/ddl-generated-columns.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
