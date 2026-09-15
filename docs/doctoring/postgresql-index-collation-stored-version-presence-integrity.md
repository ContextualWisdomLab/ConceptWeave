# PostgreSQL 18 ordinary collation stored-version presence integrity

## Problem

`CollationDefinitionObservation` already constrained capture-time actual versions by provider, but it still admitted `pg_collation.collversion IS NULL` for ordinary built-in and ICU rows. The generic availability-drift contract also treated ordinary ICU `None(recorded) / Some(actual)` as a valid positive witness.

That state is not reachable through PostgreSQL 18's supported ordinary `pg_collation` creation paths. In `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `DefineCollation()` accepts an explicit `VERSION` when supplied; otherwise, after both direct creation and `CREATE COLLATION ... FROM`, it calls `get_collation_actual_version(provider, locale)` before `CollationCreate()`. The built-in provider returns exactly `"1"`, and a valid ICU collator returns a version string. `initdb` also fills bootstrap `pg_catalog.unicode.collversion` from `pg_collation_actual_version(oid)`, while built-in bootstrap rows already carry version `1`.

Therefore ordinary provider `b` and provider `i` rows require a present stored `collversion`. The stored value need not equal the current value: explicit `VERSION` is intentionally available for `pg_upgrade`, so stale recorded versions remain valid mismatch evidence.

## Boundary

The invariant applies only to ordinary `pg_collation` material definitions:

- built-in: stored `collversion` must be present; capture-time actual version remains exactly `1`;
- ICU: stored `collversion` must be present; capture-time actual version remains present;
- libc: stored and actual versions remain nullable where PostgreSQL/platform semantics permit it;
- provider `d`: bootstrap `pg_catalog.default.collversion` remains structurally NULL and its recorded-version responsibility stays in `pg_database`.

The same presence rule is deliberately **not** transferred to database-default ICU `pg_database.datcollversion`. PostgreSQL `initdb` explicitly clears `template0.datcollversion`, so an ICU-initialized cluster has a source-reachable database-default ICU row with absent recorded version even though its actual version is available.

## Evidence lineage

- Review finding: `5210860831` on #46.
- Source RED contract: `113f63be8f6b028d1e0f98826be9ae6457c15838`, `index_collation_stored_version_presence_contract.rs`.
- Minimal production repair: `4c96f0bc0447f90e357ec868d70398c28136320a`, adding only `Builtin | Icu => version.is_some()` at the material value-object boundary.
- Existing availability-fixture correction: `6a5753bb7cfe13113482d01770896fe2e28bcf1f`, removing the impossible ordinary ICU recorded-version-absence witness while retaining database-default ICU absence as valid.

No digest domain is changed. A newly rejected state was previously representable but was not reachable from the supported PostgreSQL 18 ordinary collation source path.

## Rejected alternatives

Allowing ordinary ICU `collversion=NULL` merely because the catalog column is nullable was rejected: catalog nullability is broader than the supported provider-specific creation domain. Requiring database-default ICU `datcollversion` presence was also rejected because it would incorrectly exclude PostgreSQL's deliberate `template0` state.

## Validation still required

This lane does not claim Rust or hosted GREEN until one unchanged exact #46 head passes repository-pinned Rust 1.98 `fmt`, strict Clippy, focused/workspace/doc tests, release build, rustdoc/coverage, and applicable hosted gates. The later PostgreSQL 18 live differential must verify non-NULL stored versions for ordinary built-in and ICU rows, including direct creation and `CREATE COLLATION ... FROM`, while preserving the `template0` database-default exception.
