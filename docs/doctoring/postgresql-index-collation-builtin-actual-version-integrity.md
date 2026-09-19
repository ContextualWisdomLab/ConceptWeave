# PostgreSQL 18 built-in collation actual-version integrity

## Problem

`CollationDefinitionObservation` preserves both stored `pg_collation.collversion` and capture-time `pg_collation_actual_version(oid)` so provider drift remains visible. Before this repair, the constructor accepted any capture-time `actual_version` for built-in collations. That can mint impossible provider evidence and distort the governed mismatch predicate even when the catalog coordinate, provider, locale, and stored version are otherwise valid.

PostgreSQL 18 `src/backend/utils/adt/pg_locale_builtin.c::get_collation_actual_version_builtin()` returns the exact string `"1"` for every supported built-in locale: `C`, `C.UTF-8`, and `PG_UNICODE_FAST`. The stored version remains separate evidence and may be stale; validating the actual value must not normalize the recorded value.

## Decision

Built-in material collation observations now require capture-time `actual_version == Some("1")`. Recorded `collversion` remains untouched, so a stale recorded value still produces visible recorded-versus-actual drift. Other providers retain the existing availability/drift semantics because their actual-version availability depends on provider/platform state.

Rejected alternatives:

- normalize arbitrary built-in actual versions to `1`: destroys observed evidence rather than failing closed;
- force recorded built-in version to `1`: erases exactly the stale-state evidence the successor was designed to retain;
- apply the rule to libc/ICU: PostgreSQL 18 does not give those providers the same fixed-version contract.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review finding: `5207079626`
- Source/compile RED: `d29b230f3ce4d6b24eed035f8e532e18e6d4de70`
- Production repair: `dfe67ec778e31c7d39c1958095b43cbee6280212`
- Production seam: `crates/conceptweave-relation-partition/src/collation_definition.rs::validate_provider_actual_version`
- Focused contract: `crates/conceptweave-relation-partition/tests/index_collation_provider_encoding_contract.rs::builtin_actual_provider_version_is_exactly_postgresql18_version_one`

The focused positive control deliberately supplies a stale recorded value together with actual version `1`; this proves that the repair constrains only capture-time provider truth and does not erase drift evidence.

## Acceptance and follow-up

This is source-level repair, not executed Rust or hosted acceptance. The concrete PostgreSQL adapter must obtain `pg_collation_actual_version(oid)` from the same bounded capture and must not substitute package, server, PostgreSQL-major, or ICU version text for the built-in provider's actual collation version. Live differential coverage must include all PostgreSQL 18 built-in locales and the `pg_catalog.ucs_basic` bootstrap exception.
