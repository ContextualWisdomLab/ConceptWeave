# PostgreSQL ordinary EXCLUDE backing-index role integrity

## Decision

ConceptWeave must preserve the exact `pg_constraint.conindid` edge and the supporting index's independently observed `pg_index` role state, then fail closed unless an ordinary `contype='x'` EXCLUDE constraint is backed by an index with:

- `indisexclusion = true`;
- `indisunique = false`;
- `indisprimary = false`.

The rule applies to ordinary EXCLUDE constraints only. PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` remains in the key-constraint owner family even though PostgreSQL uses exclusion behavior for temporal keys.

## Problem

The predecessor through exact ConceptWeave `a6698c423a15e1d268404588bbecf56d792ca009` retained the ordinary EXCLUDE constraint object, its exact `conindid` backing-index coordinate, and the backing index's raw `pg_index` state. It did not, however, validate the role vector across those two independently observed catalog objects.

`IndexExclusionConstraintSnapshot` selected ordinary EXCLUDE backing indexes from `indisexclusion`, while `IndexObservation` separately retained `indisunique` and `indisprimary`. A fabricated or corrupted source tuple could therefore pair `contype='x'` with a backing index reported as unique or primary and still become governed evidence. That is not merely a display discrepancy: unique/primary role changes enforcement semantics and can collapse the boundary between ordinary EXCLUDE and temporal key constraints.

## PostgreSQL 18 authority

Primary implementation authority is PostgreSQL `REL_18_STABLE` exact commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

In `src/backend/parser/parse_utilcmd.c`, `transformIndexConstraint()` maps an index-creating table constraint to `IndexStmt` state. It sets `unique` for every PRIMARY KEY/UNIQUE constraint but explicitly not for `CONSTR_EXCLUSION`, and sets `primary` only for `CONSTR_PRIMARY`. The EXCLUDE operator list is carried separately. This makes ordinary EXCLUDE a non-unique, non-primary supporting index by construction rather than a special spelling of UNIQUE.

PostgreSQL's `pg_index` catalog documentation defines `indisunique`, `indisprimary`, and `indisexclusion` as separate stored facts. They therefore remain independent source evidence in ConceptWeave and are not inferred from `contype`.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` is a separate control. PostgreSQL's index command path treats `iswithoutoverlaps` as exclusion behavior while the constraint remains PRIMARY KEY/UNIQUE. ConceptWeave must not reclassify that family as ordinary `contype='x'` merely because its backing index has exclusion semantics.

## Alternatives rejected

**Rewrite `IndexExclusionConstraintSnapshot`.** Rejected because that snapshot already has an issued digest domain. Tightening its accepted state in place would make historical receipts depend on new semantics without a new domain separator.

**Derive index flags from constraint type.** Rejected because the catalog fields are independently observable and contradictory source state is precisely what the semantic evidence layer must detect rather than normalize away.

**Treat every `indisexclusion=true` index as ordinary EXCLUDE.** Rejected because temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` also uses exclusion behavior and remains owned by the key-constraint family.

## Repair

Review `5221108817` recorded the finding on exact predecessor `a6698c423a15e1d268404588bbecf56d792ca009`.

The source/compile contract was introduced first at `b7b364cb7ef05757d3d32c70a4b36907c6ba0247` in `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_role_contract.rs`. At that commit the referenced successor type did not yet exist, so it is a structural compile-RED contract; no Rust compiler was available in the execution host and no executed RED is claimed.

Production implementation followed at `86d4e1350b6cff697aec2657359f6f7109e55b6f` in `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_role.rs`, with public composition at `d81750a9e1d94cc953faae5a32752e5fc16f2660`.

`IndexExclusionConstraintIndexRoleSnapshot` rebinds the exact v3 → relation-partition → index-partition → ordinary-EXCLUDE → timing → timing/index-immediacy chain. For each exact constraint coordinate it resolves the already-observed `conindid` backing index, retains `indisunique`, `indisprimary`, and `indisexclusion`, and rejects every role vector except `(false, false, true)`. The new digest is domain-separated from every predecessor digest and includes both the exact constraint/index coordinates and all three raw role bits.

## Verification contract

The focused contract covers:

- `indisunique=true, indisprimary=false, indisexclusion=true` → reject;
- `indisunique=true, indisprimary=true, indisexclusion=true` → reject;
- `indisunique=false, indisprimary=false, indisexclusion=true` → admit and issue provenance for the exact child `conindid` coordinate.

A PostgreSQL 18 live differential remains required. It must independently read ordinary EXCLUDE `contype`/`conindid` and backing `pg_index.indisunique`/`indisprimary`/`indisexclusion`, and it must include PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` as a control proving that temporal key exclusion behavior is not absorbed into the ordinary EXCLUDE family.

## Acceptance status

Source repair is present, but acceptance is open. One unchanged exact head must pass repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, this focused contract and all retained contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Head movement invalidates predecessor execution evidence.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source code: `src/backend/parser/parse_utilcmd.c`* (REL_18_STABLE, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 documentation: `pg_index`*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 documentation: constraints and exclusion constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html
