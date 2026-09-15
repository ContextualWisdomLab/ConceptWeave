# PostgreSQL 18 database-default built-in actual-version integrity

## Problem

`DatabaseDefaultCollationDefinitionObservation` already restricted PostgreSQL 18 built-in database locale names to `C`, `C.UTF-8`, and `PG_UNICODE_FAST`, but it did not bind capture-time `pg_database_collation_actual_version(database_oid)` to the built-in provider's actual version domain. The constructor therefore accepted `None` or arbitrary strings such as `"18"` and could hash source states PostgreSQL 18 cannot produce for a valid built-in database locale.

This is distinct from recorded `pg_database.datcollversion`. The recorded value may be absent or stale and must remain independent drift evidence.

## PostgreSQL 18 authority

Authority was re-read on `REL_18_STABLE@6567f3f1f21353b5b0e9538d058998ab76c5a37c`.

`src/backend/commands/dbcommands.c::pg_database_collation_actual_version()` selects the database locale text and calls `get_collation_actual_version(datlocprovider, locale)`. `src/backend/utils/adt/pg_locale.c::get_collation_actual_version()` dispatches built-in provider state to `get_collation_actual_version_builtin()`. `src/backend/utils/adt/pg_locale_builtin.c::get_collation_actual_version_builtin()` returns exactly `"1"` for every PostgreSQL 18 built-in locale (`C`, `C.UTF-8`, `PG_UNICODE_FAST`) and errors for any other locale.

Therefore a valid built-in database-default observation has capture-time actual version `Some("1")`. Missing or different actual-version text is not ordinary availability drift. Libc retains its provider-specific nullable actual-version behavior, and ICU retains the separate non-NULL actual-version requirement.

## Decision

Review `5208993679` records the P1 finding on exact predecessor `c255967a4efc09d1ab83cf8beec724fa5972ab91`.

Source/compile RED `b9305ce9f91153912dd657b6e9fcb5817d9e67b6` adds a focused contract that:

- accepts all three built-in database locales only with actual version `1`;
- rejects missing, `18`, and `2` capture-time actual versions with `database_default_collation_actual_version`;
- preserves missing or stale recorded `datcollversion` as mismatch evidence against actual version `1`.

Production repair `8c6cde815524437d76be8e5549473d7a298afbb9` validates actual-version evidence by provider at construction: built-in requires exactly `Some("1")`, ICU requires presence, and libc remains unconstrained at this layer. Existing fixture repair `8011132558d20bbf89bf88f6436ce5d322cc1204` replaces fabricated built-in version `18` positive controls with PostgreSQL 18's fixed version `1` without weakening provider-shape or encoding tests.

Follow-up review `5209094918` found a second acceptance-fixture defect after the stricter provider rules were composed: `database_default_collation_version_coherence_contract.rs` still used an ICU-only helper to construct `actual_version = None` and expected success. That state is impossible for ICU, although matching NULL actual-version evidence remains valid for providers such as libc `C`. Fixture repair `0fcf97761c5674f8ded811eea489977cb3b5c577` moves the nullable coherence witness to an explicit libc `C` database default rather than weakening ICU validation.

## Rejected alternatives

Treating built-in `None` as generic version-availability drift was rejected because it admits a source state the PostgreSQL 18 built-in provider does not return. Constraining recorded `datcollversion` to `1` was also rejected because it would erase legitimate stale/missing recorded-version evidence and confuse capture-time provider truth with catalog maintenance state. Restoring ICU `actual_version = None` merely to keep an old fixture passing was rejected for the same reason; the fixture must model a provider that PostgreSQL 18 can actually report as versionless.

## Risk and follow-up

The current execution host does not provide repository-pinned Rust 1.98 tooling, so these commits are source/compile-contract evidence rather than executed RED/GREEN. One unchanged descendant head still needs native and hosted acceptance.

The concrete PostgreSQL adapter/live differential must read `pg_database_collation_actual_version(database_oid)` rather than synthesize it from the locale name, and must prove built-in actual version `1` for `C`, `C.UTF-8`, and `PG_UNICODE_FAST` alongside libc nullable-version cases and non-NULL ICU cases. `ALTER DATABASE ... REFRESH COLLATION VERSION` remains catalog maintenance and is not proof that dependent indexes were rebuilt.
