# PostgreSQL index-collation database-encoding integrity

Status: source repaired; native/hosted acceptance pending.

## Problem

Exact #46 predecessor `d769d19cf98aca4967c94494db218fe9c41ab39e` preserves catalog-exact index collation identities as `(namespace, name, collencoding)` for key, expression/predicate, and relation-`Var` evidence. Review `5203629766` found that the representation still did not prove that a supplied `collencoding` is usable in the observed database. A faulty extractor could emit the same incompatible encoding for parent and child and every equality check would pass.

PostgreSQL 18 does not treat `pg_collation.collencoding` as an arbitrary `int4` for usable database objects. `CREATE COLLATION` assigns `-1` for encoding-independent ICU collations, a validated builtin locale encoding for builtin collations, or `GetDatabaseEncoding()` for libc collations. The information-schema collation projection admits only rows with `collencoding IN (-1, current_database.encoding)`. PostgreSQL's backend encoding enum runs from `PG_SQL_ASCII = 0` through `PG_KOI8U`, with `PG_ENCODING_BE_LAST == PG_KOI8U`; later enum values are client-only encodings and cannot be a database encoding.

The earlier same-qualified-name `-1` versus UTF8 fixture remains a conservative historical identity test, but it is not sufficient database-compatibility evidence: PostgreSQL also prevents an any-encoding collation and a same-name database-specific collation from shadowing one another in the same namespace. The new source-integrity boundary therefore tests a different failure: a UTF8 database bound to a catalog identity carrying a LATIN1-only encoding.

A follow-up review `5204038985` on exact `af01eef5d52a1edcf91a1b616b334829f8c958a9` found that the first database-encoding constructor mirrored only the upper bound, not PostgreSQL's complete backend-valid predicate. PostgreSQL 18 retains `PG_UNUSED_1 == 7` inside the enum range, and `PG_VALID_BE_ENCODING` explicitly excludes `PG_UNUSED_ENCODING`. The original `0..=34` range therefore admitted raw ID `7`, allowing an impossible `pg_database.encoding` value to receive governed successor identity.

## Decision

Do not rewrite `CollationCatalogIdentity`, `IndexPartitionCollationIdentitySnapshot`, or `IndexExpressionCollationIdentitySnapshot`; their issued digest domains remain immutable. Add a domain-separated successor instead.

`PostgresDatabaseEncodingObservation` preserves the exact raw PostgreSQL 18 `pg_database.encoding` backend ID. Its source-domain predicate mirrors PostgreSQL 18 `PG_VALID_BE_ENCODING`: accept only IDs from `PG_SQL_ASCII` through `PG_KOI8U`, exclude the historical `PG_UNUSED_1` hole, and reject client-only, sentinel, and negative values. It validates each modeled catalog identity as either encoding-independent (`collencoding = -1`) or equal to the exact source database encoding.

`IndexCollationDatabaseEncodingSnapshot` composes the exact per-key catalog-identity digest with the exact expression/relation-`Var` catalog-identity digest, verifies source/policy/extractor/time provenance agreement, validates every modeled catalog identity against the database encoding, and frames both predecessor digests plus the exact database encoding under a new digest domain. Its receipt binds the same source provenance and successor digest.

Rejected alternatives:

- mutating the historical collation-identity constructors, because that would redefine already-issued predecessor semantics;
- trusting the future PostgreSQL adapter to supply only compatible encodings, because governed representation must fail closed on impossible extractor output before publication;
- using only `0 <= encoding <= PG_ENCODING_BE_LAST`, because PostgreSQL's own backend-valid macro separately excludes the `PG_UNUSED_1` enum hole;
- reducing identity back to qualified name, because catalog-row identity and database usability are separate invariants.

## Evidence chronology

- Finding: PR #46 review `5203629766` on exact `d769d19cf98aca4967c94494db218fe9c41ab39e`.
- Source RED: `1abd33454fda57f8add1d47b81c4ce3a5305e59f` adds a focused contract that requires UTF8 database evidence to reject a LATIN1 catalog identity and bounds PostgreSQL 18 database encoding IDs.
- Initial causal repair: `33dcb94d71cb1d5fb52b458da507d09941a691a5` adds the database-encoding observation/snapshot successor and export wiring without changing predecessor digests.
- Follow-up finding: PR #46 review `5204038985` on exact `af01eef5d52a1edcf91a1b616b334829f8c958a9` identifies `PG_UNUSED_1 == 7` as an invalid backend encoding admitted by the first range-only constructor.
- Follow-up source RED: `365d189d14e2525774bb32657ea6b4127a105ee9` requires ID `7` to fail while preserving valid boundary IDs `0` and `34`.
- Minimal causal repair: `d438f8a77df9c892aa4bc65f62f65ad857bb7ea6` excludes the PostgreSQL 18 unused encoding hole while retaining the exact backend upper bound.

No executed Rust RED/GREEN is claimed in this record. The current execution host does not provide the repository-pinned Rust toolchain, and protected ConceptWeave `main` still lacks the repository-owned Product PR workflow. Exact-head native and hosted acceptance remain mandatory.

## Transport boundary

The future PostgreSQL adapter must capture `pg_database.encoding` from the same bounded source/catalog observation as `pg_collation.collencoding`; it must not infer the database encoding from client settings, rendered collation names, locale strings, or process environment. Live differential evidence must include encoding-independent collations, database-specific collations, and rejection of impossible backend IDs such as `PG_UNUSED_1`; it must prove that incompatible or impossible catalog/database encoding evidence cannot be elevated into the governed successor.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Collation support*. https://www.postgresql.org/docs/18/collation.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/commands/collationcmds.c` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/collationcmds.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/catalog/information_schema.sql` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/information_schema.sql

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/include/mb/pg_wchar.h` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/mb/pg_wchar.h
