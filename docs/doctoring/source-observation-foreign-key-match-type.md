# PostgreSQL foreign-key match-type observation

## Decision

ConceptWeave's governed PostgreSQL v3 Source Observation admits `MATCH SIMPLE` and `MATCH FULL` foreign-key behavior. It rejects `MATCH PARTIAL` as an unsupported PostgreSQL runtime state.

The shared historical vocabulary retains `ForeignKeyMatchType::Partial` so older/frozen representations can remain decodable and their digest vocabulary is not rewritten. The v3 owner-level admission boundary is stricter: vocabulary exposed by a catalog field is not sufficient evidence that PostgreSQL can create or enforce that state.

## Primary evidence

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

The PostgreSQL 18 `CREATE TABLE` reference defines `MATCH SIMPLE`, `MATCH FULL`, and `MATCH PARTIAL`, but explicitly states that `MATCH PARTIAL` is not yet implemented.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

`pg_constraint.confmatchtype` reserves catalog codes for full (`f`), partial (`p`), and simple (`s`). This catalog vocabulary must not be confused with executable feature support.

## Traceability

- Owner: `crates/conceptweave-observation/src/lib.rs`, `PostgresSchemaSnapshotV3` admission.
- Vocabulary retained for compatibility: `crates/conceptweave-observation/src/model.rs`, `ForeignKeyMatchType`.
- Behavioral contract: `crates/conceptweave-observation/tests/foreign_key_match_contract.rs`.
- Required failure: `ObservationError::InvalidObservationField { field: "foreign_key_match_type" }` for governed v3 snapshots containing an observed foreign key whose match type is `Partial`.
- Supported controls: `Simple` and `Full` continue to admit normally.

## Rejected alternatives

Removing `ForeignKeyMatchType::Partial` from the shared model was rejected because that would mutate historical vocabulary and can break reproduction of frozen v2 evidence. Treating `confmatchtype = 'p'` as valid solely because the catalog reserves the code was rejected because PostgreSQL 18 does not implement the corresponding runtime semantics. Adapter-only rejection was rejected as insufficient: governed identity must remain fail closed even if a future or faulty adapter supplies the reserved value.
