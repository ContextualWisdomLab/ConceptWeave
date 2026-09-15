# PostgreSQL 18 temporal foreign-key action semantics

Status: **Proposed / behavioral RED active** on Source Observation successor #46.

## Problem

ConceptWeave now preserves PostgreSQL 18 `pg_constraint.conperiod` as explicit observed evidence. For a foreign key, `conperiod=true` means the constraint was declared with `PERIOD`. The current period-family admission verifies basic PERIOD shape and same-snapshot referenced `WITHOUT OVERLAPS` key coherence, but it does not inspect the foreign key's observed referential actions.

That omission permits source states PostgreSQL 18 cannot create to become governed immutable evidence. PostgreSQL documents `RESTRICT`, `CASCADE`, `SET NULL`, and `SET DEFAULT` as unsupported for temporal foreign keys. `NO ACTION` remains supported. Because `pg_constraint` exposes the foreign-key update/delete action codes alongside `conperiod`, a snapshot that claims explicit PERIOD state but leaves `ForeignKeyReferenceBehavior` unobserved also lacks the evidence required to validate this invariant.

## Decision

When `ConstraintPeriodObservation::has_period_semantics()` is true for a `TableConstraintObservation::ForeignKey`:

- `ForeignKeyReferenceBehavior` must be observed;
- `update_action()` must be `ForeignKeyAction::NoAction`;
- `delete_action()` must be `ForeignKeyAction::NoAction`;
- any missing action evidence or observed `Restrict`, `Cascade`, `SetNull`, or `SetDefault` fails closed with `ObservationError::InvalidObservationField { field: "constraint_period_action" }`.

This check belongs only to the explicit PERIOD-family admission. Ordinary non-temporal foreign keys retain their existing action semantics and representation.

## Traceability

| Authority / artifact | Required meaning | Exact implementation or verification point |
| --- | --- | --- |
| PostgreSQL 18 `CREATE TABLE` | Temporal foreign keys do not support `RESTRICT`, `CASCADE`, `SET NULL`, or `SET DEFAULT`; `NO ACTION` is the supported referential action | `PostgresSchemaSnapshotV3::with_observed_constraint_periods` -> `canonicalize_constraint_periods` in `crates/conceptweave-observation/src/lib.rs` |
| PostgreSQL 18 `pg_constraint` | `conperiod=true` denotes `PERIOD` for foreign keys; FK action codes are catalog facts | `ConstraintPeriodObservation` in `crates/conceptweave-observation/src/constraint_period.rs` plus `ForeignKeyReferenceBehavior` in `src/model.rs` |
| Review `5184007447` | Existing period admission ignores action evidence | Behavioral finding on #46 predecessor `d94644e9ee542a0cec5c7902915b47dee13e209e` |
| RED `03e4443b5834383f4d25a8e83786cccb62e003be` | Unsupported update/delete actions must fail closed | `tests/constraint_period_action_contract.rs` |
| RED strengthening `60b59db961ee35a0a0d5de91422ca68612afb8eb` | Explicit PERIOD governance also requires action evidence to have been observed | `temporal_foreign_key_requires_observed_reference_behavior` plus `NO ACTION` positive control |
| Review `5184025102` | Exact-current static review confirms production remains RED | #46 exact-current review |

The repair is complete only when the minimal production admission change turns this contract GREEN on one unchanged exact head together with the retained period, timing, backing-index, array/type, and frozen-v2 contracts.

## Rejected alternatives

Inferring `NO ACTION` from missing `ForeignKeyReferenceBehavior` is rejected because absence means unobserved, not a provider default. Rejecting these actions in `ForeignKeyObservation` construction is also rejected because they remain valid for ordinary non-temporal foreign keys. Temporal status is not inferred from GiST, `indisexclusion`, constraint names, reconstructed DDL, or any other neighboring fact; `pg_constraint.conperiod` remains the authority.

## Sources

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
