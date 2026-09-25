# PostgreSQL PRIMARY KEY / NULLS NOT DISTINCT integrity

## Decision

ConceptWeave must reject a governed Source Observation in which a PostgreSQL index is observed as the backing primary index (`pg_index.indisprimary = true`) while that same index is observed with `NULLS NOT DISTINCT` semantics.

This is a PostgreSQL source invariant. It is not a preference about SQL rendering, and it must not be weakened merely because all PRIMARY KEY columns are separately observed as NOT NULL.

## Finding

Exact reviewed ConceptWeave head `6331d11b8a91985925575adac7686af76c4046ae` has two individually reasonable checks that leave a gap when composed:

- `IndexObservation::new()` admits `is_unique = true` with `nulls_not_distinct = Some(true)` because PostgreSQL permits `NULLS NOT DISTINCT` for ordinary unique indexes.
- `IndexObservation::with_catalog_flags()` enforces `indisprimary => indisunique`, but does not couple the primary flag to null treatment.
- `validate_schema_relation_invariants()` then requires an observed primary catalog index to match a same-name, same-key PRIMARY KEY constraint, but does not inspect `primary_index.nulls_not_distinct()`.

A caller can therefore construct a unique `NULLS NOT DISTINCT` index, mark it `indisprimary = true`, attach the matching PRIMARY KEY constraint, and pass the base aggregate. That source state cannot be produced by PostgreSQL 18's primary-key path.

COMMENT review `5276003307` records the finding. Focused structural/behavioral RED `4962672ad64dbaba35088f87b15ca305200b1c1b` adds `crates/conceptweave-observation/tests/index_primary_nulls_not_distinct_contract.rs` with two boundaries: primary `NULLS NOT DISTINCT` must fail closed, while a standalone unique `NULLS NOT DISTINCT` index remains admissible.

## PostgreSQL 18 authority

At exact PostgreSQL 18 source commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc`, `src/backend/catalog/index.c::index_check_primary_key()` explicitly checks `IndexInfo::ii_NullsNotDistinct` and raises `ERRCODE_INVALID_TABLE_DEFINITION` with `primary keys cannot use NULLS NOT DISTINCT indexes` before accepting a PRIMARY KEY index. The same function separately enforces simple key columns and NOT NULL columns.

This distinction matters. `NULLS NOT DISTINCT` is legal for unique indexes in general, but PostgreSQL does not permit that index semantics for a PRIMARY KEY backing index. The causal ConceptWeave rule is therefore conditional on primary identity; globally rejecting `NULLS NOT DISTINCT` from unique indexes would be incorrect.

## Owner seam and minimal repair

The narrow owner is the already-existing primary index/constraint invariant. A valid repair may be placed either at `IndexObservation::with_catalog_flags()` when `primary=true` is attached, or at `validate_schema_relation_invariants()` beside the primary-index reciprocity check. The acceptance semantics are the same:

- `indisprimary=true` plus `nulls_not_distinct=Some(true)` fails closed;
- `indisprimary=true` plus `Some(false)` remains admissible subject to the existing reciprocity and PRIMARY KEY checks;
- standalone non-primary unique indexes retain `NULLS NOT DISTINCT` support;
- optional lifecycle/timing/PERIOD evidence is not made mandatory at the base snapshot boundary.

The existing `key_constraint_backing_index_static_shape_matches()` helper does not currently close this gap for PRIMARY KEY. Its PRIMARY KEY branch sets expected null treatment to `None`, and the subsequent `is_none_or(...)` predicate therefore imposes no null-treatment requirement. Earlier doctoring language that delegated PRIMARY KEY null treatment to that stronger helper is superseded by this finding and must be corrected when the production repair is landed.

## TRACEABILITY

| Authority | Exact evidence | ConceptWeave seam | Required evidence |
| --- | --- | --- | --- |
| PostgreSQL 18 primary-key creation/attachment | `postgres/postgres@051db7737c18b1c5d25cdc4ad508608c4b53fafc`, `src/backend/catalog/index.c::index_check_primary_key()` | `IndexObservation::with_catalog_flags()` and `validate_schema_relation_invariants()` | focused primary rejection + standalone-unique control |
| ConceptWeave reviewed source | `ContextualWisdomLab/ConceptWeave@6331d11b8a91985925575adac7686af76c4046ae` | primary catalog-index reciprocity block | review `5276003307` |
| RED contract | `4962672ad64dbaba35088f87b15ca305200b1c1b` | `index_primary_nulls_not_distinct_contract.rs` | exact-head execution after causal repair |

## Alternatives rejected

Globally rejecting `NULLS NOT DISTINCT` from `IndexObservation::new()` is rejected because PostgreSQL permits that semantic for ordinary unique indexes.

Requiring the optional constraint-timing family before detecting this state is rejected because `indisprimary`, index null treatment, and the PRIMARY KEY constraint are already represented in the base relation/index inventory.

Treating NOT NULL column evidence as sufficient is rejected because PostgreSQL's `index_check_primary_key()` contains a separate explicit prohibition on `ii_NullsNotDistinct`; the two rules are not substitutes.

## References

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 source: index creation and primary-key validation (`src/backend/catalog/index.c`)* [Source code, `051db7737c18b1c5d25cdc4ad508608c4b53fafc`]. https://github.com/postgres/postgres/blob/051db7737c18b1c5d25cdc4ad508608c4b53fafc/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: 52.26. pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

## Acceptance

The current state is RED-recorded, not source-repaired. Acceptance requires a minimal causal production change, exact-head GREEN for `index_primary_nulls_not_distinct_contract.rs`, retained primary reciprocity / unique-index / timing / PERIOD contracts, and the normal Rust 1.98, strict Clippy, workspace/doc, rustdoc/coverage, and PostgreSQL 18 same-generation differential gates. No predecessor execution evidence transfers after the source or documentation head moves.
