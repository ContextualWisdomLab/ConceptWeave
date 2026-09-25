# PostgreSQL 18 collation version-availability integrity

## Problem

`CollationDefinitionObservation` and `DatabaseDefaultCollationDefinitionObservation` already preserve the catalog-recorded provider version and the capture-time provider version as separate optional values. Their `has_version_mismatch()` helpers, however, only reported drift when both values were present and textually different. A transition between a present version and SQL `NULL` therefore remained visible in the governed digest but was exposed to callers as `false` for version mismatch.

That is not PostgreSQL 18's refresh semantics. In `REL_18_STABLE`, both `ALTER COLLATION ... REFRESH VERSION` and `ALTER DATABASE ... REFRESH COLLATION VERSION` explicitly reject a NULL-to-non-NULL or non-NULL-to-NULL transition as an invalid collation-version change. A consumer deciding whether collation-dependent semantic evidence is clean must therefore not collapse version availability loss or appearance into the no-mismatch state.

## Constraints

The repair must preserve the exact raw `Option<String>` evidence, existing digest domains, catalog-coordinate predecessors, source provenance, and downstream publication boundary. It must not synthesize a version, coerce `NULL` to an empty string, rewrite historical receipts, or claim that a version mismatch itself proves whether dependent indexes have been rebuilt.

## Decision

Review `5206594819` records the P1 finding at predecessor `2dbb0332699afd13ae003de90e61d8145d84be46`.

Executable contract `281c6bbd824b0924552f40581f78c690016ecb78` requires both ordinary `pg_collation` evidence and database-default `pg_database` evidence to report:

- `None` versus `None` as no mismatch;
- equal `Some` versus `Some` as no mismatch;
- `Some` versus `None` as mismatch;
- `None` versus `Some` as mismatch;
- unequal `Some` versus `Some` as mismatch.

Production repair `79d6faa0b4cb0800023406bb9e9c38c6527bb199` changes ordinary collation mismatch evaluation to direct optional-value inequality. Production repair `72d5390a1472c4d8d1cb491cae488abe1b9ef70a` applies the same rule to database-default collation evidence. The encoded evidence and digest construction remain unchanged; only the risk/query predicate is corrected.

## Alternatives rejected

Treating `NULL` as “unknown but clean” was rejected because it hides the exact availability state PostgreSQL itself treats as an invalid refresh transition. Rejecting all `NULL` versions at construction was also rejected: PostgreSQL providers can legitimately have no version, and ConceptWeave must preserve source truth rather than manufacture provider semantics. Normalizing `NULL` to a sentinel string was rejected because it would alter the governed representation and historical digest meaning without adding information.

## Traceability and acceptance

- Owner PR: ContextualWisdomLab/ConceptWeave#46
- Review: `5206594819`
- RED contract: `crates/conceptweave-relation-partition/tests/collation_version_availability_contract.rs` at `281c6bbd824b0924552f40581f78c690016ecb78`
- Ordinary collation repair: `crates/conceptweave-relation-partition/src/collation_definition.rs` at `79d6faa0b4cb0800023406bb9e9c38c6527bb199`
- Database-default repair: `crates/conceptweave-relation-partition/src/database_default_collation.rs` at `72d5390a1472c4d8d1cb491cae488abe1b9ef70a`
- PostgreSQL primary source: `postgres/postgres`, `REL_18_STABLE`, `src/backend/commands/collationcmds.c` and `src/backend/commands/dbcommands.c`.

This is source-level RED → repair evidence only until one unchanged exact head passes repository-pinned Rust 1.98 tests, formatting, strict Clippy, rustdoc/release/coverage and applicable hosted Product/security/review gates. Concrete PostgreSQL transport must later verify present/present-equal, present/present-different, and version-unavailable cases against live PostgreSQL 18 without deriving the actual version from the stored catalog value.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: collation commands (`REL_18_STABLE`)* [Source code]. GitHub. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/collationcmds.c

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: database commands (`REL_18_STABLE`)* [Source code]. GitHub. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/dbcommands.c
