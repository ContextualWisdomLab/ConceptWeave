# PostgreSQL 18 foreign-table NOT NULL enforcement integrity

## Correction

The earlier interpretation of the `CREATE FOREIGN TABLE` synopsis was too broad. PostgreSQL 18 REL_18_STABLE does **not** admit `pg_constraint.contype = 'n'` with `conenforced = false`, including for foreign tables.

The grammar places `[ ENFORCED | NOT ENFORCED ]` after the foreign-table constraint alternatives, but source representability is narrower than that synopsis grouping suggests. The authoritative storage path in `StoreRelNotNull()` sets the cooked NOT NULL constraint to `is_enforced = true`, and `CreateConstraintEntry()` asserts that only CHECK and FOREIGN KEY constraints may be not enforced. ConceptWeave therefore must not mint governed identity for a foreign-table NOT NULL row with `conenforced = false`.

## Problem

The previous repair exempted `RelationKind::ForeignTable` from the NOT NULL enforcement guard and added a test that treated enforced and not-enforced foreign-table NOT NULL rows as two valid governed identities. That state space contradicts the PostgreSQL 18 implementation.

This is material because `conenforced` is hashed. Leaving the exemption in place would make a caller-fabricated catalog tuple immutable and authoritative even though PostgreSQL's NOT NULL storage path cannot produce it.

## Decision

`NotNullConstraintObservation::new()` requires `enforced = true` for every admitted NOT NULL owner: ordinary table, partitioned table, and foreign table.

`RelationKind::ForeignTable` remains a valid NOT NULL owner, but it does not receive a broader `conenforced` domain. The foreign-table documentation's statement that PostgreSQL core does not enforce foreign-table CHECK/NOT NULL declarations describes runtime trust in the remote system; it does not change the `pg_constraint.conenforced` state that PostgreSQL records for a NOT NULL row.

The separate foreign-table validation rule remains: PostgreSQL 18 `ALTER FOREIGN TABLE` permits `NOT VALID` only for CHECK, so foreign-table NOT NULL also requires `convalidated = true`.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: heap.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/heap.c

`StoreRelNotNull()` emits a NOT NULL `CookedConstraint` with `is_enforced = true`.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c

`CreateConstraintEntry()` asserts that only CHECK or FOREIGN KEY constraints may be not enforced and that a not-enforced constraint must be not valid.

PostgreSQL Global Development Group. (2026). *CREATE FOREIGN TABLE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-createforeigntable.html

The synopsis is retained as syntax documentation, while the notes explain that PostgreSQL core does not enforce foreign-table CHECK/NOT NULL declarations. That operational note is not evidence for a false NOT NULL `conenforced` catalog bit.

PostgreSQL Global Development Group. (2026). *CREATE TABLE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-createtable.html

The constraint-characteristics documentation states that NOT ENFORCED is currently supported only for foreign key and CHECK constraints.

## Rejected alternatives

Keeping the foreign-table exemption based only on synopsis placement is rejected because it conflicts with REL_18_STABLE production code. Dropping `conenforced` from NOT NULL identity is rejected because the adapter must still capture and verify source state; an impossible false value must fail closed rather than disappear through normalization. Treating “PostgreSQL core does not enforce foreign-table constraints” as equivalent to `conenforced=false` is rejected because runtime responsibility and catalog state are different facts.

## Executable traceability

The superseded interpretation remains traceable rather than being silently erased: review `5196207955`, RED `6d2b22891aecdc6be6375a0ca56c648fc853b64b`, repair `41de3e390905299c3b77e7ba9dbd950c2fa90d46`, regression `5f786846a03974dbe8a6eb085fb9e151d20202d2`, and the prior version of this doctoring document describe the interpretation now rejected by stronger source authority.

The correction is:

- review finding `5196804677` on exact `6696dc8deb0c216b31eaa7ec9ec690401e631e58`;
- RED contract `4a611cc3dc15f4200f65bf08945c9cfa759daa7c`, replacing the invalid preservation/digest tests with `foreign_table_rejects_not_enforced_not_null`;
- minimal production repair `3e9828311a55187d758b9bf729f69870c16f7be9`, making the enforcement guard unconditional across admitted NOT NULL relation kinds;
- production delta from RED: one production file, seven additions and six deletions; parent topology, validation, completeness, name integrity, and digest framing remain unchanged.

## Adapter obligation

The PostgreSQL adapter must capture `pg_constraint.conenforced` rather than synthesize it, but a first-class NOT NULL observation is admissible only when the captured value is `true`. This applies to ordinary, partitioned, and foreign tables. A false value is contradictory source evidence and fails before immutable snapshot construction.

The adapter must not derive the bit from the remote foreign server's behavior or from the general warning that PostgreSQL does not verify foreign-table declarations. Those facts belong to runtime/remote truth, not to the local NOT NULL catalog tuple.

## Acceptance boundary

This correction is source-level until one unchanged exact #46 head executes the repository-pinned Rust 1.98 suite and obtains hosted Product/security/dependency/review acceptance. Prior GREEN, third-party status, syntax documentation alone, or a mechanically mergeable Draft do not satisfy that gate.
