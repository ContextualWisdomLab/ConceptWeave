# PostgreSQL database-default libc C-UTF8 encoding integrity

## Problem

`DatabaseDefaultCollationDefinitionObservation::validate_database_encoding()` validated PostgreSQL 18 built-in and ICU database-default encoding constraints but treated every libc definition as encoding-compatible. That admitted governed evidence such as `pg_database.encoding = LATIN1` with libc `datcollate = 'C.UTF-8'` or `datctype = 'C.UTF-8'`.

That tuple is not reachable through PostgreSQL 18 database creation. `CreateDatabase()` calls `check_encoding_locale_matches(encoding, dbcollate, dbctype)` before provider-specific acceptance. The helper resolves each libc locale through `pg_get_encoding_from_locale()` and rejects a known locale encoding that is incompatible with the requested database encoding. `C` and `POSIX` resolve to `PG_SQL_ASCII` and remain compatible with every database encoding; an available `C.UTF-8`/`C.utf8` locale resolves to UTF8 and therefore cannot back a LATIN1 database.

## Constraint

ConceptWeave must not pretend that arbitrary operating-system locale names can be decoded from text alone. PostgreSQL obtains the codeset from the host locale implementation (`newlocale()` + `nl_langinfo_l(CODESET)` on non-Windows systems, Windows locale/code-page APIs on Windows). Mirroring every platform locale alias inside the semantic representation would duplicate mutable OS truth and create false rejection risk.

The repair is therefore deliberately narrower than PostgreSQL's complete runtime check: it closes the source-stable C-UTF8 family whose codeset is explicit while leaving arbitrary libc locale compatibility for the concrete source adapter/live differential.

## RED

Exact predecessor: `b1de4de974ac2b96495d054060ac48cb1cfe7f52`.

Review `5213788571` identified the gap. Source contract commit `9e9379359406787d0e9cf7b964e7800f63055496` adds `database_default_libc_c_utf8_database_encoding_contract.rs` and requires:

- `C.UTF-8` and `C.utf8` in either `datcollate` or `datctype` to fail on LATIN1;
- the same C-UTF8 spellings to remain valid on UTF8;
- libc `C` and `POSIX` to remain valid cross-encoding controls.

The predecessor implementation returned `true` for every libc provider in `validate_database_encoding()`, so the negative witnesses are source-level RED against the reviewed predecessor.

## Repair

Production commit `336c42aaadeb729a65f2343bf3501826c9478343` changes only the libc arm of `validate_database_encoding()` and adds `is_c_utf8_libc_locale()`.

If either raw database locale field is the C-UTF8 family (`C.UTF-8` or `C.utf8`, case-insensitive), the bounded database encoding must be PostgreSQL UTF8 encoding ID `6`. `C`/`POSIX`, built-in, ICU, version evidence, provider-d coherence, predecessor digests, and canonical database-default digest encoding are unchanged.

Rejected alternatives:

- Parsing arbitrary locale suffixes such as `en_US.UTF-8` as authoritative codeset evidence was rejected because PostgreSQL asks the operating system for the locale's actual codeset rather than trusting the spelling.
- Requiring every libc database default to be UTF8 was rejected because PostgreSQL supports libc databases in other compatible encodings.
- Copying PostgreSQL's platform locale tables into ConceptWeave was rejected because that would duplicate mutable source/runtime truth instead of observing it.

## Risk and follow-up

This repair intentionally does not prove arbitrary libc locale/database-encoding compatibility. The concrete PostgreSQL transport must retain a live differential that exercises `CREATE DATABASE`/catalog evidence on the target runtime and must not infer an arbitrary platform locale's codeset from its name. If the transport later exposes authoritative locale-codeset evidence, it should be modeled as a domain-separated successor rather than changing an issued digest domain.

Exact-head Rust execution is still required after this ordinary-forward source repair. Earlier execution evidence does not transfer after the head moves.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code: `src/backend/commands/dbcommands.c` and `src/port/chklocale.c`* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). PostgreSQL Global Development Group source repository.
