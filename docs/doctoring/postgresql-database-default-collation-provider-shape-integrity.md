# PostgreSQL 18 database-default collation provider-shape integrity

## Decision

The database-default effective-collation successor must reject impossible `pg_database` locale/provider shapes before they enter governed identity. This is separate from the `pg_collation` provider-shape repair because PostgreSQL stores different fields and nullability rules in the database catalog.

## Problem

`DatabaseDefaultCollationDefinitionObservation` originally represented `datcollate`, `datctype`, `datlocale`, and `daticurules` as optional values and rejected only embedded NUL bytes. That admitted states PostgreSQL 18 does not accept as database locale definitions, including missing `datcollate`/`datctype`, libc with `datlocale`, non-libc without `datlocale`, and ICU rules under a non-ICU provider.

Those impossible states could then be hashed by `IndexEffectiveCollationDefinitionSnapshot`, weakening the claim that its digest is source-authoritative PostgreSQL 18 evidence.

A later exact-head review found one remaining built-in-provider hole: the repaired constructor required a non-NULL `datlocale` for `datlocprovider = 'b'` but accepted any string. PostgreSQL 18's built-in locale provider admits only `C`, `C.UTF-8`, and `PG_UNICODE_FAST`; values such as `und`, `en-US`, or `ko-KR` therefore could still be minted into governed effective-definition identity despite not being valid PostgreSQL 18 built-in database locales.

## PostgreSQL 18 authority

`pg_database.h` declares both `datcollate` and `datctype` with `BKI_FORCE_NOT_NULL`. `dbcommands.c` canonicalizes libc databases with `datlocale = NULL`, requires a locale for non-libc providers, rejects ICU rules unless the provider is ICU, and asserts the libc/non-libc `datlocale` split before forming the catalog tuple. PostgreSQL 18 `initdb` further constrains `--locale-provider=builtin`: `--locale`/`--builtin-locale` must be one of `C`, `C.UTF-8`, or `PG_UNICODE_FAST`. `pg_database_collation_actual_version()` selects `datcollate` for libc and requires `datlocale` for the other providers.

Primary sources:

- PostgreSQL Global Development Group. (2025). *pg_database.h* (`REL_18_STABLE`, `src/include/catalog/pg_database.h`).
- PostgreSQL Global Development Group. (2025). *dbcommands.c* (`REL_18_STABLE`, `src/backend/commands/dbcommands.c`).
- PostgreSQL Global Development Group. (2026). *CREATE DATABASE — PostgreSQL 18 documentation*.
- PostgreSQL Global Development Group. (2026). *initdb — PostgreSQL 18 documentation*.

## RED → repair

Initial provider-shape repair:

- Review: `5204999247` on predecessor `9cfc407b476d8525d89fd70d97154aeceb39a3a9`.
- Source RED: `3bdd7b26494dbe3688881d6ffcda94f9ff66c6bd`, `tests/database_default_collation_definition_contract.rs`.
- Minimal causal repair: `7e949f12de0ff959d31ca1e639d86cfdcbec2bd3`, `src/database_default_collation.rs`.

Built-in locale-domain follow-up:

- Review: `5205265135` on exact predecessor `03c1e61254f487f51ff6f6a5d46e8141b3300226`.
- Source RED: `fba8e7849ae114df4fdfaf73bd6445a57ebb99a3`, which adds positive controls for `C`, `C.UTF-8`, and `PG_UNICODE_FAST` and rejects `und`, `en-US`, and `ko-KR` under the built-in provider.
- Minimal causal repair: `8efca49731b342e7f65ffb636b4a5a128c596674`, which constrains only the built-in branch of `validate_database_provider_shape()` and does not rewrite any issued predecessor or digest domain.

The contract now requires `datcollate` and `datctype` evidence for every provider, requires `datlocale = NULL` for libc and present for built-in/ICU, rejects non-ICU `daticurules`, constrains built-in `datlocale` to the PostgreSQL 18 built-in vocabulary, and retains a positive ICU-rules case. Production rejects violations with `database_default_collation_provider_shape` before the observation can be constructed or hashed.

## Boundary

This value object deliberately does not infer broader locale-versus-database-encoding compatibility. Database encoding is owned by the composed predecessor and the concrete PostgreSQL adapter/live differential. Provider/version strings remain raw source evidence. Stored/current provider-version mismatch remains observable and is not proof that dependent indexes were rebuilt.

The source RED and repair above are code-level evidence only until one unchanged exact head executes repository-pinned Rust 1.98 and applicable hosted acceptance gates. No predecessor GREEN transfers after head movement.
