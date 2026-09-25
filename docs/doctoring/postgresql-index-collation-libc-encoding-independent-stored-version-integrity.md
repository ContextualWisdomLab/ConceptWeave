# PostgreSQL 18 libc encoding-independent stored-version integrity

## Problem

The libc encoding-independent repair restricted raw `pg_collation.collencoding = -1` rows to the PostgreSQL 18 source-reachable locale pairs `C/C` and `POSIX/POSIX`, but `CollationDefinitionObservation` still allowed those rows to carry an arbitrary non-NULL stored `pg_collation.collversion`.

That combination is not reachable through PostgreSQL 18's supported collation paths. It is narrower than the generic libc rule: a database-encoding libc `C`/`POSIX` collation created directly may carry an explicit `VERSION`, while an encoding-independent `C`/`POSIX` row comes only from bootstrap or a `CREATE COLLATION ... FROM` lineage.

## PostgreSQL 18 authority

Authority was rechecked against `postgres/postgres` `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` on 2026-09-16.

- `src/include/catalog/pg_collation.dat` bootstraps `pg_catalog.C` and `pg_catalog.POSIX` as libc `collencoding = -1` rows with no stored `collversion`.
- `src/backend/commands/collationcmds.c::DefineCollation()` forbids combining `FROM` with any other option. Its `FROM` branch copies provider, deterministic mode, raw `collencoding`, `collcollate`, `collctype`, provider locale, and ICU rules, but it does not copy `collversion`.
- When `collversion` is omitted, `DefineCollation()` recomputes it with `get_collation_actual_version()`. For libc `C` and `POSIX`, PostgreSQL 18's libc provider returns no actual version, so the copied encoding-independent row retains SQL `NULL` for `collversion`.
- `ALTER COLLATION ... REFRESH VERSION` rejects NULL-to-non-NULL and non-NULL-to-NULL version transitions as an invalid collation version change. It therefore cannot introduce a stored version on the encoding-independent C/POSIX lineage later.

A directly created libc collation is different: the ordinary libc creation path uses `GetDatabaseEncoding()`, not `-1`, and an explicit `VERSION` is permitted. That concrete-encoding path must remain representable.

## Decision

Keep the existing catalog identity and provider/locale evidence. Add one cross-field invariant in `CollationDefinitionObservation::new`: when provider is libc and raw `collencoding == -1`, stored `collversion` must be absent.

The preceding provider-encoding validator already proves that a libc `-1` row has exact raw locale pair `C/C` or `POSIX/POSIX`; this repair does not duplicate that rule and does not constrain schema/name. It also does not change any digest domain or alter concrete-encoding libc version semantics.

Rejected alternatives:

- Require every libc `C`/`POSIX` row to have `collversion = NULL`: rejected because directly created database-encoding libc collations can carry an explicit `VERSION`.
- Infer the rule from schema/name: rejected because `CREATE COLLATION ... FROM pg_catalog.C/POSIX` creates arbitrary target identities.
- Treat non-NULL stored version as generic drift on an encoding-independent row: rejected because PostgreSQL 18 cannot produce that transition through `ALTER COLLATION ... REFRESH VERSION`.

## RED and repair traceability

- Finding review: `5212404915` on predecessor exact head `666281c7e9970c93ff5b5d18957c6e09d14d3d05`.
- Source RED: `8d10db094b96a39111958344ed67e57488b299cf`, extending `crates/conceptweave-relation-partition/tests/index_collation_libc_encoding_independent_contract.rs`.
  - arbitrary-name `C/C/-1` and `POSIX/POSIX/-1` rows with `collversion = NULL` remain valid;
  - the same encoding-independent rows with fabricated stored versions fail closed with `index_collation_definition_libc_encoding_independent_stored_version`;
  - database-encoding libc `C` with an explicit stored `VERSION` remains a positive control.
- Minimal production repair: `7434f719b68fa2308437f2c10632959d62364290`, `crates/conceptweave-relation-partition/src/collation_definition.rs`.
- Retained-contract correction: `bf62a577eb0da8910b05d82f37c606835521346e`, `crates/conceptweave-relation-partition/tests/index_collation_definition_contract.rs`. The earlier libc actual-version fixture had used one `collencoding=-1` identity while asserting an explicit stored version for `C`, `C.*`, and `POSIX`. That was source-invalid after this invariant and already source-invalid for `C.*` under the preceding provider-encoding rule. The fixture now uses concrete UTF8 encoding ID `6` for the direct libc creation path, preserving its actual-version-NULL purpose and explicit-version positive control without weakening the new encoding-independent invariant.

No executed Rust GREEN is asserted. The available execution host has no `cargo`, `rustc`, or `rustup`; exact-head repository-pinned Rust 1.98 and hosted Product/security acceptance remain required.

## Live differential follow-up

The PostgreSQL transport descendant must validate this invariant against a real PostgreSQL 18 catalog:

1. Confirm bootstrap `pg_catalog.C` and `pg_catalog.POSIX` have raw `collencoding = -1` and `collversion IS NULL`.
2. Execute arbitrary-name `CREATE COLLATION ... FROM pg_catalog.C` and `... FROM pg_catalog.POSIX`; confirm target rows preserve `-1`, the exact locale pair, and `collversion IS NULL`.
3. Execute `ALTER COLLATION ... REFRESH VERSION` on the copied rows and confirm it cannot create a non-NULL stored version.
4. In a UTF8 database, directly create libc `C`/`C.*`/`POSIX` collations with an explicit `VERSION`; confirm their concrete `collencoding = 6`, stored version is preserved, and actual provider version remains NULL where PostgreSQL reports no provider version.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: collation catalog bootstrap (`src/include/catalog/pg_collation.dat`), REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1*. GitHub.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: collation commands and version refresh (`src/backend/commands/collationcmds.c`), REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1*. GitHub.
