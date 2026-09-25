# PostgreSQL 18 ICU actual-version presence integrity

## Finding

At Source Observation head `17c1f31713e92116150cf3ae9c868516c789148f`, both ordinary `CollationDefinitionObservation` and `DatabaseDefaultCollationDefinitionObservation` admitted ICU provider evidence with a missing capture-time actual version. That state is not source-reachable from a successful PostgreSQL 18 ICU version observation.

PostgreSQL `REL_18_STABLE@937a0e68cb55ac38341f242cf3302906e6e2ec42` implements `get_collation_actual_version_icu()` by opening the ICU collator, reading `ucol_getVersion`, converting the version with `u_versionToString`, and returning `pstrdup(buf)`. It does not have a successful NULL-return path. `pg_collation_actual_version()` only returns SQL NULL when the provider dispatcher returns NULL. Database-default ICU version lookup uses the same provider/version mechanism.

This differs from libc, where PostgreSQL deliberately returns NULL for `C`, every case-insensitive `C.*` locale, and `POSIX`, and can also return NULL on supported platform-specific lookup failures. The repair must therefore be provider-specific rather than a blanket non-NULL rule.

## Decision

ICU capture-time actual-version evidence is mandatory for both material `pg_collation` observations and effective `pg_database` default-collation observations. Recorded `collversion` / `datcollversion` remains independent and nullable so recorded-versus-current drift is preserved. Existing libc and provider-`d` semantics are unchanged.

## Traceability

- Review finding: PR #46 review `5208333588`, anchored to `17c1f31713e92116150cf3ae9c868516c789148f`.
- RED contract: `b47f1862cd0b2b4ee39303be6502902c30f76872`, `crates/conceptweave-relation-partition/tests/icu_actual_version_presence_contract.rs`.
- Material-collation repair: `788438822842d9d4756d1fb8cfdc140c4a60fe8b`, `crates/conceptweave-relation-partition/src/collation_definition.rs`.
- Database-default repair: `9da54d2121f0310ccdc56c66811d11424e450b25`, `crates/conceptweave-relation-partition/src/database_default_collation.rs`.
- Existing availability contract correction: `14f93f16f4db13c9909b335f3c881b93093d971e`, `crates/conceptweave-relation-partition/tests/collation_version_availability_contract.rs`.
- PostgreSQL authority: `postgres/postgres REL_18_STABLE@937a0e68cb55ac38341f242cf3302906e6e2ec42`, `src/backend/utils/adt/pg_locale_icu.c::get_collation_actual_version_icu`, `src/backend/commands/collationcmds.c::pg_collation_actual_version`.

## Acceptance

The local execution host for this writer does not provide `cargo`, `rustc`, or `rustup`, and this PR head currently has no repository-owned PR workflow run. The commits above therefore establish source/compile RED-to-repair evidence only; they are not claimed as executed Rust GREEN. Exact-head native and hosted acceptance remains required before adoption or publication.
