# PostgreSQL transform-converter initial-privilege recovery validation

## Decision

`pg_init_privs` remains Source Observation truth. ConceptWeave does not repair a dangling PostgreSQL role reference, discard the affected ACL entry, or reinterpret a raw role OID as a role name. The immutable converter initial-privilege material preserves that source state and exposes only aggregate unresolved grantee/grantor counts for governance.

The missing boundary was deterministic validation. Before this repair, an immutable material could report nonzero unresolved-reference counts but no ConceptWeave domain service turned that condition into a recovery/publication verdict. A caller could therefore move from observation toward publication without a canonical interpretation of the diagnostic. That contradicted the product boundary `observe -> discover -> propose -> align -> validate -> review -> publish` even though the source digest itself was lossless.

Finding review `5254918439` at exact head `b88836369e5481f8f91eda86003af979f28bc7c9` records the defect. Structural RED `21817d01f0aaa69902699eb21109c2a057401a8c` referenced the missing public validator and required row absence plus fully resolved material to validate `Ready`, while any unresolved grantee or grantor count validates `Blocked`. It also requires public diagnostics to expose only aggregate counts and never the raw OID values carried by source identity.

Production commit `97ff3903278bafbd2133a88ded475ca481e8b1b7` adds the pure Rust recovery-validation verdict and service. Composition commit `da84d2afc99d41f7af9273cd192047704d779a47` exposes the service through the existing relation/index-partition public surface. The validator consumes only `unresolved_grantee_count()` and `unresolved_grantor_count()` from the already-admitted material. It does not change the source digest, receipt, converter coordinate, PostgreSQL capture contract, or raw-OID privacy boundary.

## Invariants

- `pg_init_privs` row absence is a legitimate source state and validates `Ready`.
- A present material with zero unresolved grantee and grantor references validates `Ready`.
- Any positive unresolved grantee or grantor count validates `Blocked`.
- `Blocked` exposes only aggregate counts; raw dangling OIDs stay inside immutable source identity.
- Validation is derived governance state and therefore does not participate in Source Observation digest identity.
- ConceptWeave observes and validates; PostgreSQL role/catalog remediation remains outside this bounded context.

## Alternatives rejected

Treating every `pg_init_privs` row as publication-ready was rejected because PostgreSQL BUG #19483/#19513 demonstrate a real recovery failure mode. Rejecting damaged state during source capture was rejected because observation must preserve the external database state instead of falsifying it. Publishing raw dangling OIDs in a validation error was rejected because the existing aggregate projection is sufficient for deterministic gating and keeps the narrower privacy surface.

## Acceptance still required

This source repair is not exact-head GREEN. The final PR head still needs repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential. The differential must prove that the public validation verdict agrees with the captured unresolved-reference counts while raw dangling OIDs remain unexposed.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
