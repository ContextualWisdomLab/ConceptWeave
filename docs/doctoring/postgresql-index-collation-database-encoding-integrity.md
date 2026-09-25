# PostgreSQL index-collation database-encoding integrity

Status: source repaired; native/hosted acceptance pending.

## Problem

Exact #46 predecessor `d769d19cf98aca4967c94494db218fe9c41ab39e` preserves catalog-exact index collation identities as `(namespace, name, collencoding)` for key, expression/predicate, and relation-`Var` evidence. Review `5203629766` found that the representation still did not prove that a supplied `collencoding` is usable in the observed database. A faulty extractor could emit the same incompatible encoding for parent and child and every equality check would pass.

PostgreSQL 18 does not treat `pg_collation.collencoding` as an arbitrary `int4` for usable database objects. `CREATE COLLATION` assigns `-1` for encoding-independent ICU collations, a validated builtin locale encoding for builtin collations, or `GetDatabaseEncoding()` for libc collations. The information-schema collation projection admits only rows with `collencoding IN (-1, current_database.encoding)`. PostgreSQL 18's backend encoding enum runs from `PG_SQL_ASCII = 0` through `PG_KOI8U`, with `PG_ENCODING_BE_LAST == PG_KOI8U`; later enum values are client-only encodings and cannot be a database encoding.

The earlier same-qualified-name `-1` versus UTF8 fixture remains a conservative historical identity test, but it is not sufficient database-compatibility evidence: PostgreSQL also prevents an any-encoding collation and a same-name database-specific collation from shadowing one another in the same namespace. The source-integrity boundary therefore tests a different failure: a UTF8 database bound to a catalog identity carrying a LATIN1-only encoding.

A later review `5204038985` incorrectly projected a PostgreSQL development-line enum change onto PostgreSQL 18 and treated encoding ID `7` as `PG_UNUSED_1`. Fresh authoritative review `5204368970` against `REL_18_STABLE` found the opposite: PostgreSQL 18 still defines ID `7` as `PG_MULE_INTERNAL`, and its `PG_VALID_BE_ENCODING` macro is exactly the inclusive `0..=PG_ENCODING_BE_LAST` range. Rejecting ID `7` was therefore a PostgreSQL-18 false negative.

## Decision

Do not rewrite `CollationCatalogIdentity`, `IndexPartitionCollationIdentitySnapshot`, or `IndexExpressionCollationIdentitySnapshot`; their issued digest domains remain immutable. Keep the domain-separated database-encoding successor.

`PostgresDatabaseEncodingObservation` preserves the exact raw PostgreSQL 18 `pg_database.encoding` backend ID. Its version-specific source-domain predicate mirrors `REL_18_STABLE`: accept every ID from `PG_SQL_ASCII == 0` through `PG_KOI8U == 34`, including `PG_MULE_INTERNAL == 7`; reject client-only, sentinel, and negative values. It validates each modeled catalog identity as either encoding-independent (`collencoding = -1`) or equal to the exact source database encoding.

`IndexCollationDatabaseEncodingSnapshot` composes the exact per-key catalog-identity digest with the exact expression/relation-`Var` catalog-identity digest, verifies source/policy/extractor/time provenance agreement, validates every modeled catalog identity against the database encoding, and frames both predecessor digests plus the exact database encoding under a new digest domain. Its receipt binds the same source provenance and successor digest.

Rejected alternatives:

- mutating the historical collation-identity constructors, because that would redefine already-issued predecessor semantics;
- trusting the future PostgreSQL adapter to supply only compatible encodings, because governed representation must fail closed on impossible extractor output before publication;
- importing `PG_UNUSED_1` from a later PostgreSQL development line, because this contract is explicitly PostgreSQL 18 and `REL_18_STABLE` still assigns ID `7` to `PG_MULE_INTERNAL`;
- reducing identity back to qualified name, because catalog-row identity and database usability are separate invariants.

## Evidence chronology

- Finding: PR #46 review `5203629766` on exact `d769d19cf98aca4967c94494db218fe9c41ab39e`.
- Source RED: `1abd33454fda57f8add1d47b81c4ce3a5305e59f` adds a focused contract that requires UTF8 database evidence to reject a LATIN1 catalog identity and bounds PostgreSQL 18 database encoding IDs.
- Initial causal repair: `33dcb94d71cb1d5fb52b458da507d09941a691a5` adds the database-encoding observation/snapshot successor and export wiring without changing predecessor digests.
- Superseded finding: review `5204038985` on exact `af01eef5d52a1edcf91a1b616b334829f8c958a9` treated ID `7` as an unused PostgreSQL 18 encoding. RED `365d189d14e2525774bb32657ea6b4127a105ee9` and repair `d438f8a77df9c892aa4bc65f62f65ad857bb7ea6` encoded that mistaken version attribution; they remain immutable historical commits but are not current authority.
- Corrective finding: review `5204368970` on exact `72fb56c4826fbd81c620bbcd14cfc67ac00c3cc9` verifies that `REL_18_STABLE` defines ID `7` as `PG_MULE_INTERNAL` and `PG_VALID_BE_ENCODING` as the inclusive backend range.
- Corrective source RED: `a3f877a65105dc7304d524e2fced2f65de83d6d7` requires PostgreSQL 18 encoding ID `7` to be accepted while retaining rejection of client-only `35` and negative sentinel `-1`.
- Minimal causal repair: `15471cf7cb3639f2258b0d39950919fd9f7fad65` removes the future-version enum-hole exclusion and documents the PostgreSQL-18-specific boundary without changing the database-encoding successor digest domain.

No executed Rust RED/GREEN is claimed in this record. The current execution host does not provide the repository-pinned Rust toolchain, and protected ConceptWeave `main` still lacks the repository-owned Product PR workflow. Exact-head native and hosted acceptance remain mandatory.

## Transport boundary

The future PostgreSQL adapter must capture `pg_database.encoding` from the same bounded source/catalog observation as `pg_collation.collencoding`; it must not infer the database encoding from client settings, rendered collation names, locale strings, or process environment. Live differential evidence must include encoding-independent collations, database-specific collations, PostgreSQL 18 `PG_MULE_INTERNAL == 7`, and rejection of client-only/sentinel encoding IDs. If a later PostgreSQL major removes or reserves an encoding ID, that change belongs in a separately versioned adapter/representation contract rather than retroactively changing the PostgreSQL 18 domain.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Collation support*. https://www.postgresql.org/docs/18/collation.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/commands/collationcmds.c` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/collationcmds.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/catalog/information_schema.sql` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/information_schema.sql

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/include/mb/pg_wchar.h` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/mb/pg_wchar.h
