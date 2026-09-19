# PostgreSQL 18 libc collation actual-version integrity

## Problem

`CollationDefinitionObservation` preserves stored `pg_collation.collversion` separately from capture-time `pg_collation_actual_version(oid)`. Before this repair, the constructor accepted any `actual_version` for libc collations. That admits source states PostgreSQL 18 cannot produce for the libc `C` family and `POSIX`, allowing fabricated provider-version text to enter governed semantic evidence.

At PostgreSQL `REL_18_STABLE` commit `937a0e68cb55ac38341f242cf3302906e6e2ec42`, `src/backend/utils/adt/pg_locale_libc.c::get_collation_actual_version_libc()` leaves the actual version NULL when `collcollate` is case-insensitively `C`, begins with `C.`, or is `POSIX`. `src/backend/commands/collationcmds.c::DefineCollation()` uses `get_collation_actual_version()` when no explicit stored `VERSION` is supplied, and `pg_collation_actual_version(oid)` uses the same provider-specific source. Therefore a non-NULL capture-time actual version for those libc locales is not PostgreSQL 18 source evidence.

The stored version remains a different fact. PostgreSQL accepts an explicit `VERSION` attribute, so a stored `collversion` can be present while the provider reports no actual version. That availability mismatch must remain representable rather than being normalized away.

## Decision

For libc observations, capture-time `actual_version` must be NULL when `collcollate` is case-insensitively `C`, any `C.*` locale, or `POSIX`. The check is limited to `actual_version`; an explicitly stored `collversion` is preserved. Other libc locales retain platform-dependent optional actual-version semantics because PostgreSQL may report a libc, FreeBSD, or Windows collation version, and Windows can still report no version for an unsupported locale-name form.

Rejected alternatives:

- require all libc actual versions to be present: incorrect on `C`/`C.*`/`POSIX` and on supported Windows failure-to-identify cases;
- force stored `collversion` to NULL for the unversioned libc family: would reject operator-supplied stored evidence that PostgreSQL permits;
- normalize fabricated actual-version text to NULL: destroys the observed input instead of failing closed;
- treat this as transport-only validation: malformed provider-version evidence would already have entered the immutable semantic-definition digest.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review finding: `5207719642`
- Source/compile RED: `0957ec241d0e88ce1d0e0e14347a7eefd1b42fba`
- Production repair: `3064547898572c93353b789cef80fdf67b9c53b8`
- Production seam: `crates/conceptweave-relation-partition/src/collation_definition.rs::validate_provider_actual_version`
- Focused contract: `crates/conceptweave-relation-partition/tests/index_collation_definition_contract.rs::postgres18_libc_c_family_has_no_capture_time_actual_version`
- PostgreSQL authority: `postgres/postgres@937a0e68cb55ac38341f242cf3302906e6e2ec42`, `src/backend/utils/adt/pg_locale_libc.c::get_collation_actual_version_libc`, `src/backend/commands/collationcmds.c::DefineCollation`

The negative witnesses cover case-insensitive `C`, `C.*`, and `POSIX`. Each has a positive control that retains an operator-supplied recorded version with `actual_version = NULL`, proving the repair preserves version-availability drift rather than erasing it.

## Acceptance and follow-up

This is source-level RED→repair evidence, not executed Rust or hosted acceptance. The concrete PostgreSQL adapter must read both stored `pg_collation.collversion` and `pg_collation_actual_version(oid)` from the same bounded observation. Live PostgreSQL 18 differential coverage must include libc `C`, at least one `C.*` spelling available on the test host, `POSIX`, a version-reporting non-C libc locale where available, and the Windows no-version path where that platform is supported.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL source code (REL_18_STABLE, commit 937a0e68cb55ac38341f242cf3302906e6e2ec42)* [Source code]. GitHub.
