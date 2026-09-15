# PostgreSQL 18 libc encoding-independent collation integrity

## Problem

`CollationDefinitionObservation` already bound built-in and ICU providers to PostgreSQL 18 catalog-encoding rules, but the libc branch accepted every raw `pg_collation.collencoding`. That admitted a fabricated tuple such as provider `c`, `collencoding = -1`, `collcollate = 'en_US.UTF-8'`, `collctype = 'en_US.UTF-8'` as governed source evidence.

This is narrower than the earlier, superseded name-based finding. Arbitrary target identities with libc `collencoding = -1` are source-reachable through `CREATE COLLATION target FROM pg_catalog.C` or `... FROM pg_catalog.POSIX`; the target schema/name therefore cannot identify the lineage. The invariant lives in the provider/encoding/locale fields themselves.

## PostgreSQL 18 authority

Authority was rechecked against `postgres/postgres` `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` on 2026-09-15.

- `src/include/catalog/pg_collation.dat` bootstraps the only libc encoding-independent definitions as `C/C/-1` and `POSIX/POSIX/-1`.
- `src/backend/commands/collationcmds.c::DefineCollation()` gives directly created libc collations `GetDatabaseEncoding()` and validates the locale/encoding relationship.
- libc system-collation import derives a concrete backend encoding with `pg_get_encoding_from_locale()` and passes that encoding to `CollationCreate()`.
- `CREATE COLLATION ... FROM existing_collation` copies provider, raw `collencoding`, `collcollate`, and `collctype` from the source. It can therefore copy `C/C/-1` or `POSIX/POSIX/-1` under any target identity, but it cannot turn those rows into an arbitrary non-C/POSIX locale pair while preserving `-1`.

Accordingly, a material libc definition with `collencoding = -1` is source-reachable only when the raw locale pair is exactly `(C, C)` or `(POSIX, POSIX)`. Concrete-encoding libc definitions remain platform/database specific and are not narrowed by this repair.

## Decision

Keep the canonical catalog identity unchanged and tighten the existing provider/catalog-encoding invariant rather than introducing lineage metadata that PostgreSQL does not store.

`validate_provider_encoding()` now receives `collcollate` and `collctype`. For provider `Libc` and raw encoding `-1`, it accepts only exact `C/C` or `POSIX/POSIX`. It deliberately does not constrain schema/name, preserving `CREATE COLLATION ... FROM` copies, and it leaves every concrete-encoding libc row on the existing path.

Rejected alternatives:

- Restrict `collencoding = -1` to the bootstrap identities `pg_catalog.C` and `pg_catalog.POSIX`: rejected because `CREATE COLLATION ... FROM` creates arbitrary target identities while preserving the source fields.
- Treat every libc `-1` row as potentially platform-specific: rejected because the supported PostgreSQL 18 creation/import paths provide a concrete encoding for non-bootstrap libc locales.
- Normalize `C`/`POSIX` or infer lineage from the name: rejected because the observation contract preserves raw catalog evidence and names are not provenance.

## RED and repair traceability

- Finding review: `5211601903` on predecessor exact head `c2d2fe02208be93c851db4332fe70b81f578521c`.
- Source/compile RED: `877f08f9c07f458069f262e430744cd9ba14deb6`, `crates/conceptweave-relation-partition/tests/index_collation_libc_encoding_independent_contract.rs`.
  - arbitrary-name copies with `C/C/-1` and `POSIX/POSIX/-1` stay valid;
  - fabricated `en_US.UTF-8/en_US.UTF-8/-1` fails closed;
  - mixed `C/POSIX/-1` fails closed;
  - ordinary concrete-encoding libc remains valid.
- Minimal production repair: `7fea4ed04afd271119a98773d83ad074809f1cfe`, `crates/conceptweave-relation-partition/src/collation_definition.rs`.
- The production commit is one ordinary-forward commit over the RED and changes only that source file (`+15/-2`), preserving all digest domains.

No executed Rust GREEN is asserted for these commits. The available execution host has no `cargo`, `rustc`, or `rustup`, and the branch must still obtain exact-head repository-pinned Rust 1.98 and hosted Product/security acceptance.

## Live differential follow-up

The PostgreSQL transport descendant must prove the representation against a real PostgreSQL 18 catalog rather than synthesizing rows:

1. Read bootstrap `pg_catalog.C` and `pg_catalog.POSIX` and confirm provider `c`, raw encoding `-1`, and exact locale pairs.
2. Execute `CREATE COLLATION public.c_copy FROM pg_catalog.C` and a POSIX copy; read the target rows and confirm the arbitrary target identities preserve `-1` and the exact source locale pairs.
3. Create/import an available non-C/POSIX libc collation and prove its raw `collencoding` is concrete rather than `-1`.
4. Feed the captured tuples through the representation contract and retain source receipt/provenance without source-copy SQL outside the observation boundary.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: collation catalog bootstrap (`src/include/catalog/pg_collation.dat`), REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1*. GitHub.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: collation commands (`src/backend/commands/collationcmds.c`), REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1*. GitHub.
