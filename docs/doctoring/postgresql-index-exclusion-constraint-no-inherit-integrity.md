# PostgreSQL 18 EXCLUDE constraint no-inherit integrity

## Decision

ConceptWeave must retain `pg_constraint.connoinherit` as explicit governed source evidence for each ordinary index-backed `EXCLUDE` constraint. For the pinned PostgreSQL 18 index-constraint creation path, root constraints require `connoinherit = true`; partition-child constraints require `connoinherit = false`.

This is a domain-separated sibling successor over the exact EXCLUDE identity/parentage snapshot. It does not rewrite the issued EXCLUDE identity, timing, enforcement, or validation digest domains.

## Problem

The EXCLUDE identity predecessor already retains exact `conindid`, `conparentid`, `conislocal`, and `coninhcount`, but it drops `connoinherit`. PostgreSQL exposes that bit independently in `pg_constraint`.

The pinned PostgreSQL 18 source `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/catalog/index.c`, makes the state deterministic in `index_constraint_create()`:

- when `parentConstraintId` is valid: `islocal = false`, `inhcount = 1`, `noinherit = false`;
- otherwise: `islocal = true`, `inhcount = 0`, `noinherit = true`.

The same values are then passed to `CreateConstraintEntry()`. Because `CONSTRAINT_EXCLUSION` uses this index-constraint path, dropping `connoinherit` allows a contradictory catalog tuple to collapse onto the same governed identity despite exact parentage already being known.

## Constraints

- Preserve all issued predecessor digest domains.
- Keep the raw catalog bit explicit; do not infer it during extraction and then discard the observed value.
- Bind expected state to the already-governed EXCLUDE parent coordinate rather than duplicating partition topology.
- Require exactly one observation per predecessor ordinary EXCLUDE constraint.
- Keep temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` in its existing key-constraint family.
- Do not claim live PostgreSQL extraction or exact-head acceptance from source-only changes.

## Alternatives considered

### Derive `connoinherit` from parentage and never observe it

Rejected. Although PostgreSQL 18 currently makes the value deterministic for this path, `pg_constraint` stores it independently. Derivation alone cannot detect extractor omission, catalog contradiction, or source-version drift.

### Add the bit to `IndexExclusionConstraintSnapshot`

Rejected. That would alter an already-issued predecessor digest and break immutable successor reproducibility.

### Domain-separated sibling successor

Selected. `IndexExclusionConstraintNoInheritSnapshot` binds the exact EXCLUDE predecessor digest, exact constraint coordinate, and raw `connoinherit` bit. It validates the bit against predecessor parentage and emits a separate receipt.

## Traceability

- Finding review: `5218606154` on PR #46 at exact predecessor `10c6c8c4827c585a744576700fa04478e20b14aa`.
- Source/compile RED contract: `3552cd383ba54b8f6aae8a8796b492a86a5d4738`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_no_inherit_contract.rs`.
- Production successor: `a605dd29afc48f7b112d6aa8d73bc45e901bb86c`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_no_inherit.rs`.
- Public composition: `cd25d0aaf633ae84caeb7144477027da657ab346`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- PostgreSQL source authority: `postgres/postgres@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/catalog/index.c`, `index_constraint_create()`.
- PostgreSQL catalog authority: PostgreSQL 18 `pg_constraint`.

The RED is an executable source/compile contract, not a claim that it was executed in this environment. Exact-head Rust 1.98 and hosted acceptance remain required after source/docs movement stops.

## Risk and effect

The repair detects a class of impossible or source-drifted EXCLUDE catalog tuples that parentage plus `conislocal`/`coninhcount` alone would otherwise admit. Because the new digest is separate, existing immutable evidence remains reproducible while downstream consumers can require the stronger successor when they need complete PostgreSQL 18 index-constraint inheritance policy.

## Live differential requirement

The PostgreSQL 18 differential must capture ordinary EXCLUDE `conparentid`, `conislocal`, `coninhcount`, and `connoinherit` together. A root EXCLUDE constraint must demonstrate `(conparentid=0, conislocal=true, coninhcount=0, connoinherit=true)` and an attached partition-child EXCLUDE constraint must demonstrate `(parent, false, 1, false)`. ConceptWeave must reject missing, duplicate, or contradictory no-inherit evidence.

The same bounded differential continues to require `contype`, `conindid`, `condeferrable`, `condeferred`, `conenforced`, and `convalidated`, plus a temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` control.

## References

PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: 52.13. pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: src/backend/catalog/index.c, REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1* [Source code]. GitHub. https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/catalog/index.c
