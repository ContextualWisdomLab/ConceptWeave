# PostgreSQL 18 database-default libc actual-version integrity

## Problem

`DatabaseDefaultCollationDefinitionObservation` preserves `pg_database.datcollversion` separately from capture-time `pg_database_collation_actual_version(database_oid)`. Before this repair, provider `c` accepted any `actual_version`, including fabricated non-NULL values for libc database locales for which PostgreSQL 18 deterministically reports no actual version.

At PostgreSQL `REL_18_STABLE` commit `6567f3f1f21353b5b0e9538d058998ab76c5a37c`, `src/backend/commands/dbcommands.c::pg_database_collation_actual_version()` reads `pg_database.datcollate` when `datlocprovider = 'c'` and passes it to `get_collation_actual_version()`. The libc implementation in `src/backend/utils/adt/pg_locale_libc.c::get_collation_actual_version_libc()` leaves the actual version NULL when that locale is case-insensitively `C`, begins with `C.`, or is `POSIX`. `AlterDatabaseRefreshColl()` uses the same provider dispatcher, so query-time evidence and refresh semantics share the same C-family/POSIX invariant.

Other libc locales remain platform-dependent. glibc and FreeBSD can expose a version; Windows can return no version for locale-name forms that `GetNLSVersionEx()` cannot resolve. The representation must therefore reject impossible non-NULL evidence for the deterministic unversioned family without requiring a version for every other libc locale.

## Decision

For database-default libc observations, `actual_version` must be NULL when `datcollate` is case-insensitively `C`, any `C.*` spelling, or `POSIX`. Non-C libc database locales keep optional actual-version semantics. `recorded_version` remains a separate field and is not normalized by this rule.

Rejected alternatives:

- require every libc database default to expose an actual version: not portable and contradicts PostgreSQL's Windows fallback;
- allow arbitrary actual-version text and defer validation to the extractor: malformed source evidence would already enter the governed digest;
- infer the current version from `datcollversion`: stored and capture-time provider versions are intentionally distinct evidence;
- normalize fabricated values to NULL: silently mutates the observation instead of failing closed.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review finding: `5209212806`
- Source/compile RED: `af59eef86964d11ccecc56934693411f9b0c9ac4`
- Production repair: `963b6a5a851381c75d89adda7a2a6e8eb08c98db`
- Production seam: `crates/conceptweave-relation-partition/src/database_default_collation.rs::DatabaseDefaultCollationDefinitionObservation::new`
- Focused contract: `crates/conceptweave-relation-partition/tests/database_default_libc_actual_version_contract.rs`
- PostgreSQL authority: `postgres/postgres@6567f3f1f21353b5b0e9538d058998ab76c5a37c`, `src/backend/commands/dbcommands.c::pg_database_collation_actual_version`, `src/backend/commands/dbcommands.c::AlterDatabaseRefreshColl`, `src/backend/utils/adt/pg_locale_libc.c::get_collation_actual_version_libc`

Negative witnesses cover `C`, a case-insensitive `C.*` spelling, and `POSIX`. Positive controls preserve NULL actual versions for those locales and preserve both present and absent actual versions for a non-C libc locale so platform-dependent availability is not overconstrained.

## Acceptance and follow-up

This is source-level RED→repair evidence until an unchanged exact head passes repository-pinned Rust 1.98 and hosted acceptance. The concrete PostgreSQL adapter must read `datlocprovider`, raw `datcollate`, stored `datcollversion`, and `pg_database_collation_actual_version(database_oid)` from the same bounded observation. Live PostgreSQL 18 differential coverage must include libc `C`, an available `C.*` spelling, `POSIX`, and a non-C libc locale on at least one version-reporting platform.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL source code (REL_18_STABLE, commit 6567f3f1f21353b5b0e9538d058998ab76c5a37c)* [Source code]. GitHub.
