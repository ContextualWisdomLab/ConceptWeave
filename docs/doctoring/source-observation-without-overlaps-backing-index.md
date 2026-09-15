# Source Observation: `WITHOUT OVERLAPS` backing-index evidence

## Decision

ConceptWeave treats `pg_constraint.conperiod` as the source-authoritative fact that a PostgreSQL PRIMARY KEY or UNIQUE constraint was declared with `WITHOUT OVERLAPS`. Index shape must never be used to invent that fact.

Once `conperiod=true` has been explicitly observed, however, governed evidence must remain coherent with PostgreSQL's implementation contract. PostgreSQL 18 specifies that `WITHOUT OVERLAPS` is enforced with exclusion semantics, that a UNIQUE constraint containing `WITHOUT OVERLAPS` automatically creates a GiST index with the same name as the constraint, and that PRIMARY KEY likewise uses GiST when `WITHOUT OVERLAPS` is specified. The generated index covers the constraint key, while `pg_index` separately exposes uniqueness, primary-key, exclusion, key-column, predicate, and null-treatment state. `pg_constraint.conexclop` records the per-column exclusion operators for `WITHOUT OVERLAPS` PRIMARY KEY/UNIQUE rows.

ConceptWeave does not yet retain `pg_constraint.conindid` as a durable relationship. Therefore same-name lookup is only a bounded lookup seam, not sufficient proof by itself. A positive temporal key must also prove that the resolved backing index has the exact ordered key-column shape of the constraint and coherent material catalog state. This is a one-way coherence rule: `conperiod=true` requires backing evidence; GiST/exclusion/index shape never implies `conperiod=true`.

## Retained presence repair

Review `5185780899` found that the earlier implementation checked backing evidence only inside an optional branch. Behavioral RED `c80d07816863f814e9b8fbb716661310d8b2b150` required missing same-name index or missing material `pg_index` flags to fail. The retained production repair requires same-name backing-index evidence, material catalog flags, `indisexclusion=true`, and access method `gist` for every positive `WITHOUT OVERLAPS` key while leaving ordinary `conperiod=false` keys independent of unrelated optional index evidence.

## Key-shape finding, RED, and repair

Review `5186585545` on PR #46 exact `c0c123178794fbd28b4d93008c071aea21dac6fb` found a second fail-open seam. `canonicalize_constraint_periods()` required the same-name GiST/exclusion index but did not validate that its key attributes actually matched the PRIMARY KEY/UNIQUE constrained columns unless the separate optional constraint-timing family had been observed first. A same-name GiST exclusion index over a different column order could therefore be admitted as governed temporal evidence.

Behavioral RED `ffe75eddf530ad3963c087afdfe1109da15bad14` extends `constraint_period_backing_index_presence_contract.rs` with a hostile witness whose constraint is `(document_id, valid_during WITHOUT OVERLAPS)` while the same-name GiST/exclusion index key order is `(valid_during, document_id)`. The snapshot must reject this as `constraint_period_backing_index`; the coherent positive control remains admissible.

Production repair `e258b39474454b4c84f51f23dcfb6b00285f4db3` introduces one shared static backing-index predicate used by both `canonicalize_constraint_timings()` and `canonicalize_constraint_periods()`. It verifies exact ordered key-column equality, uniqueness, absence of a partial predicate, PK versus UNIQUE catalog role, and UNIQUE null treatment when that constraint fact was observed. Positive period admission then adds its already-required `indisexclusion=true` and `gist` checks. Timing admission separately retains `indimmediate` validation and its exclusion-access-method coherence. The timing family therefore remains optional rather than becoming a prerequisite for static temporal-key coherence.

The repair is deliberately one-way. It does not derive `conperiod` from index shape, does not use OIDs as durable identity, and does not require ordinary `conperiod=false` keys to provide unrelated temporal backing evidence.

## Alternatives rejected

Inferring `conperiod` from a GiST index or `indisexclusion` is rejected because `pg_constraint.conperiod` is the direct declaration fact. Requiring the timing family merely to obtain static key/index coherence is rejected because optional catalog families should remain independently composable. Accepting same-name lookup as complete authority is rejected because name equality does not prove the observed index key, flags, predicate state, or null treatment match the constraint. Source catalog OIDs such as `conindid` may be used as capture-time joins by the future adapter, but OIDs are not durable governed identity.

## Traceability

Owner: Source Observation bounded context, PR #46.

Production seam: `crates/conceptweave-observation/src/lib.rs` → `key_constraint_backing_index_static_shape_matches()`, `canonicalize_constraint_periods()`, and `canonicalize_constraint_timings()`.

Behavioral contract: `crates/conceptweave-observation/tests/constraint_period_backing_index_presence_contract.rs`.

Primary sources:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

Status: source repaired through `e258b39474454b4c84f51f23dcfb6b00285f4db3`; unchanged-head native/Product acceptance is pending.
