# PostgreSQL copied `ucs_basic` stored-version integrity

## Problem

The Source Observation representation already preserves PostgreSQL 18's source-reachable UTF8 built-in `C` rows. Bootstrap `pg_catalog.ucs_basic` is provider `b`, locale `C`, `collencoding = 6`, and `collversion = '1'`; `CREATE COLLATION ... FROM pg_catalog.ucs_basic` can copy that provider/locale/encoding tuple under another catalog identity.

The previous guard correctly admitted those copied rows but still allowed their stored `pg_collation.collversion` to be absent or arbitrary. That state is not produced by PostgreSQL 18's supported creation path. The `FROM` form must be the only collation option, copies provider/encoding/locale fields, does not copy `collversion`, and then recomputes a missing `collversion` with `get_collation_actual_version()`. For built-in locale `C`, PostgreSQL 18 returns exactly `"1"`.

Direct `provider = builtin, locale = 'C'` creation is a different source path: it derives `collencoding = -1`, and an operator-supplied `VERSION` remains representable there. The repair therefore must not globally force every built-in `C` row's stored version to `1`.

## Decision

`CollationDefinitionObservation` fails closed only when all of the following are true:

- provider is PostgreSQL built-in (`b`),
- provider locale is `C`,
- raw `pg_collation.collencoding` is PostgreSQL UTF8 encoding ID `6`, and
- stored `pg_collation.collversion` is not exactly `"1"`.

This tuple identifies the source-reachable `ucs_basic` copy lineage without using schema or collation name as semantic authority. Direct built-in `C/-1` keeps independently observed stored-version evidence. The existing capture-time built-in actual-version rule remains exactly `"1"`; no digest domain changes.

Rejected alternatives:

- Restricting the rule to exact `pg_catalog.ucs_basic` would regress the already-repaired `CREATE COLLATION ... FROM` semantics because copied descendants have independent catalog identities.
- Requiring stored version `1` for every built-in provider row would incorrectly reject direct collations created with an explicit `VERSION` option.
- Normalizing copied rows back to `pg_catalog.ucs_basic` would destroy catalog identity and violate source-observation ownership.

## Traceability

- Finding review: PR #46 review `5210108597`, predecessor exact head `35ac4b75f6c22f174550a52250def33426f35bf0`.
- Source/compile RED: `88e13524f4fca9f27d496105c2cf5aae9acd9cbd`, `crates/conceptweave-relation-partition/tests/index_collation_copied_ucs_basic_version_contract.rs`.
- Minimal production repair: `3f2118970f119e14090694f2a6d3581159d54172`, `CollationDefinitionObservation::new`.
- Error contract: `index_collation_definition_copied_ucs_basic_version`.
- PostgreSQL authority read against `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`; the latest branch commit is documentation-only relative to the collation implementation parent, so the cited collation semantics are unchanged from its parent `6567f3f1f21353b5b0e9538d058998ab76c5a37c`.

## Acceptance boundary

The new test is a source/compile RED followed by a source repair, not executed GREEN. This execution environment does not currently provide the repository-pinned Rust 1.98 toolchain, and the protected branch has not yet supplied the repository-owned Product pull-request workflow required by the stack. Exact-head acceptance still requires native and hosted `fmt`, strict workspace/all-target Clippy, the focused contract, workspace/doc tests, release build, rustdoc/test/edge-case coverage, and applicable security/review evidence.

The later PostgreSQL live differential must create a target with `CREATE COLLATION public.ucs_basic_copy FROM pg_catalog.ucs_basic` and prove that the target retains provider `b`, locale `C`, `collencoding = 6`, and stored/current version `1` under its own target catalog identity.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code: `src/backend/commands/collationcmds.c`, `src/backend/utils/adt/pg_locale_builtin.c`, and `src/include/catalog/pg_collation.dat` (`REL_18_STABLE`)*. PostgreSQL Global Development Group.
