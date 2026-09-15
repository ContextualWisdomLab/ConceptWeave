# PostgreSQL 18 database-default collation provider-shape integrity

## Decision

The database-default effective-collation successor must reject impossible `pg_database` locale/provider shapes before they enter governed identity. This is separate from the `pg_collation` provider-shape repair because PostgreSQL stores different fields and nullability rules in the database catalog.

## Problem

`DatabaseDefaultCollationDefinitionObservation` originally represented `datcollate`, `datctype`, `datlocale`, and `daticurules` as optional values and rejected only embedded NUL bytes. That admitted states PostgreSQL 18 does not accept as database locale definitions, including missing `datcollate`/`datctype`, libc with `datlocale`, non-libc without `datlocale`, and ICU rules under a non-ICU provider.

Those impossible states could then be hashed by `IndexEffectiveCollationDefinitionSnapshot`, weakening the claim that its digest is source-authoritative PostgreSQL 18 evidence.

## PostgreSQL 18 authority

`pg_database.h` declares both `datcollate` and `datctype` with `BKI_FORCE_NOT_NULL`. `dbcommands.c` canonicalizes libc databases with `datlocale = NULL`, requires a locale for non-libc providers, rejects ICU rules unless the provider is ICU, and asserts the libc/non-libc `datlocale` split before forming the catalog tuple. `pg_database_collation_actual_version()` likewise selects `datcollate` for libc and requires `datlocale` for the other providers.

Primary sources:

- PostgreSQL Global Development Group. (2025). *pg_database.h* (`REL_18_STABLE`, `src/include/catalog/pg_database.h`).
- PostgreSQL Global Development Group. (2025). *dbcommands.c* (`REL_18_STABLE`, `src/backend/commands/dbcommands.c`).
- PostgreSQL Global Development Group. (2026). *CREATE DATABASE — PostgreSQL 18 documentation*.

## RED → repair

- Review: `5204999247` on predecessor `9cfc407b476d8525d89fd70d97154aeceb39a3a9`.
- Source RED: `3bdd7b26494dbe3688881d6ffcda94f9ff66c6bd`, `tests/database_default_collation_definition_contract.rs`.
- Minimal causal repair: `7e949f12de0ff959d31ca1e639d86cfdcbec2bd3`, `src/database_default_collation.rs`.

The contract now requires `datcollate` and `datctype` evidence for every provider, requires `datlocale = NULL` for libc and present for built-in/ICU, rejects non-ICU `daticurules`, and retains a positive ICU-rules case. Production rejects violations with `database_default_collation_provider_shape` before the observation can be constructed or hashed.

## Boundary

This value object deliberately does not infer built-in-locale versus database-encoding compatibility. The database encoding is owned by the composed predecessor and the concrete PostgreSQL adapter/live differential. Provider/version strings remain raw source evidence. Stored/current provider-version mismatch remains observable and is not proof that dependent indexes were rebuilt.

No native Rust 1.98 or hosted GREEN is claimed until an unchanged exact head executes the required acceptance gates.
