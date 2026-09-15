# PostgreSQL 18 index collation provider/catalog-encoding integrity

## Decision

ConceptWeave must not treat the broad source-database usability rule `pg_collation.collencoding ∈ {-1, pg_database.encoding}` as proof that a material `pg_collation` row has the encoding PostgreSQL 18 assigns to its provider and locale.

`CollationDefinitionObservation` therefore validates the provider-specific catalog encoding before the definition can enter governed hashing:

- built-in `C` requires `collencoding = -1`;
- built-in `C.UTF-8` and `PG_UNICODE_FAST` require PostgreSQL UTF8 encoding ID `6`;
- ICU requires `collencoding = -1`;
- libc keeps the broader predecessor rule because PostgreSQL 18 can expose encoding-specific libc rows and encoding-independent copied/bootstrap rows.

The existing catalog-coordinate, database-encoding, material-definition, and effective-definition digest domains are not reinterpreted or rewritten.

## Problem and evidence

Review `5205986252` on exact predecessor `c604dfff75128dcb312ff55bc3b7a0a69665576c` found that the material-definition constructor checked provider field shape but ignored `CollationCatalogIdentity::encoding()` for built-in and ICU providers. The preceding `IndexCollationDatabaseEncodingSnapshot` proves only that a used collation is encoding-independent (`-1`) or has the exact source database encoding. That admits tuples PostgreSQL 18's canonical creation path cannot mint, including built-in `C` at UTF8 encoding `6`, built-in `C.UTF-8` or `PG_UNICODE_FAST` at `-1`, and ICU `und` at encoding `6`.

PostgreSQL 18 `DefineCollation()` calls `builtin_locale_encoding(colllocale)` for built-in collations, sets ICU `collencoding = -1`, and uses the current database encoding for ordinary libc creation. `builtin_locale_encoding()` returns `-1` for `C` and `PG_UTF8` for both `C.UTF-8` and `PG_UNICODE_FAST`. The bootstrap catalog independently exhibits the same invariants: `ucs_basic`, `pg_c_utf8`, and `pg_unicode_fast` are built-in UTF8 rows; the standard ICU `unicode` row is encoding-independent; `C`/`POSIX` libc rows are encoding-independent.

## RED and repair

Source/compile RED `4bde0a274d237972494298cc95283adcbde981dd` adds `index_collation_provider_encoding_contract.rs`. Positive controls pin built-in `C/-1`, built-in `C.UTF-8/6`, built-in `PG_UNICODE_FAST/6`, and ICU `-1`. Counterexamples require `index_collation_definition_provider_encoding` for built-in `C/6`, the two UTF8-only built-in locales at `-1`, and ICU at `6`.

Production `9e3e42fb1fc7e09e1319f058530d14b60701fc9e` adds provider-specific catalog-encoding validation immediately after the existing provider-shape check. This is a causal constructor-boundary repair: malformed material definition evidence cannot be instantiated and later promoted into `IndexCollationDefinitionSnapshot`.

No executed Rust RED/GREEN is claimed from these commits until one unchanged exact head passes the repository-pinned Rust 1.98 acceptance surface.

## Alternatives considered

Keeping only `IndexCollationDatabaseEncodingSnapshot` was rejected because `-1` and the current database encoding are both broadly usable coordinates but are not interchangeable for built-in and ICU provider rows.

Moving the check only into the future PostgreSQL adapter was rejected because `CollationDefinitionObservation` is a public representation boundary; any caller able to construct an impossible provider/catalog-encoding tuple would still be able to create governed semantic evidence.

Changing an existing digest domain was rejected because the defect is an admission invariant, not a change to the canonical byte representation of already-valid definitions. Ordinary-forward validation preserves predecessor meaning.

## Risk and follow-up

The source-level repair still requires exact-head compilation, formatting, strict Clippy, focused/workspace/doc tests, rustdoc/test/edge-case coverage, release build, and hosted Product/security/dependency/review acceptance. The later concrete PostgreSQL differential must query the raw `pg_collation.collencoding` rather than derive it from provider/locale, and must exercise at least built-in `C/-1`, built-in `C.UTF-8/6`, built-in `PG_UNICODE_FAST/6`, ICU `-1`, libc database-encoding rows, and the encoding-independent libc `C`/`POSIX` cases.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review: `5205986252`
- RED: `4bde0a274d237972494298cc95283adcbde981dd`
- Production repair: `9e3e42fb1fc7e09e1319f058530d14b60701fc9e`
- Production module: `crates/conceptweave-relation-partition/src/collation_definition.rs`
- Focused contract: `crates/conceptweave-relation-partition/tests/index_collation_provider_encoding_contract.rs`
- Upstream predecessor: `IndexCollationDatabaseEncodingSnapshot`
- Governed successor: `IndexCollationDefinitionSnapshot`

## References

PostgreSQL Global Development Group. (2025). *collationcmds.c* (`REL_18_STABLE`) [Source code]. PostgreSQL. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/collationcmds.c

PostgreSQL Global Development Group. (2025). *pg_collation.dat* (`REL_18_STABLE`) [Source code]. PostgreSQL. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_collation.dat

PostgreSQL Global Development Group. (2025). *pg_locale.c* (`REL_18_STABLE`) [Source code]. PostgreSQL. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/adt/pg_locale.c

PostgreSQL Global Development Group. (2025). *CREATE COLLATION*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/sql-createcollation.html
