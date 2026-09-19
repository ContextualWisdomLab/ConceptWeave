# PostgreSQL transform-converter initial-privilege recovery validation

## Decision

`pg_init_privs` remains Source Observation truth. ConceptWeave does not repair a dangling PostgreSQL role reference, discard the affected ACL entry, or reinterpret a raw role OID as a role name. The immutable converter initial-privilege material preserves that source state and exposes only aggregate unresolved grantee/grantor counts for governance.

The first missing boundary was deterministic validation. Before that repair, an immutable material could report nonzero unresolved-reference counts but no ConceptWeave domain service turned that condition into a recovery/publication verdict. A caller could therefore move from observation toward publication without a canonical interpretation of the diagnostic. That contradicted the product boundary `observe -> discover -> propose -> align -> validate -> review -> publish` even though the source digest itself was lossless.

Finding review `5254918439` at exact head `b88836369e5481f8f91eda86003af979f28bc7c9` records that defect. Structural RED `21817d01f0aaa69902699eb21109c2a057401a8c` referenced the missing public validator and required row absence plus fully resolved material to validate ready, while any unresolved grantee or grantor count validates blocked. It also requires public diagnostics to expose only aggregate counts and never the raw OID values carried by source identity.

Production commit `97ff3903278bafbd2133a88ded475ca481e8b1b7` added the pure Rust recovery-validation service. Composition commit `da84d2afc99d41f7af9273cd192047704d779a47` exposed it through the existing relation/index-partition public surface. The validator consumes only `unresolved_grantee_count()` and `unresolved_grantor_count()` from the already-admitted material. It does not change the source digest, receipt, converter coordinate, PostgreSQL capture contract, or raw-OID privacy boundary.

## Evidence-binding repair

Fresh review at exact head `7769b8b21b2d321f6d1f15390643a1f2c512a7cf` found that the first validator API was not yet a trustworthy publication artifact. Its public enum exported constructible `Ready` and `Blocked` variants, so a consumer could mint a successful verdict without invoking the canonical validator. The verdict also carried no reference to the immutable material it had validated. A legitimate `Ready` value could therefore be detached from one observation and replayed as evidence for another.

Finding review `5254963033` pins that governance defect. Structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38` changed the public contract first: row absence must yield no material digest, while every present resolved or damaged baseline must return the exact `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::digest()` that was validated. The contract no longer constructs or pattern-matches public verdict variants.

Production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc` replaces the public enum with a public struct whose fields are private. There is no public constructor. The canonical validator is the only production constructor, stores `Option<material.digest()>`, and carries the two aggregate unresolved-reference counts. `is_ready()` derives readiness from those private counts. This prevents a caller from manufacturing a successful verdict and binds every present-material verdict to the immutable initial-privilege evidence it evaluated.

The material digest is evidence binding, not a new Source Observation fact. It does not enter the source digest recursively and does not widen the raw-OID diagnostic surface. A byte-identical material with the same immutable digest may legitimately share validation evidence; a different digest cannot.

## Invariants

- `pg_init_privs` row absence is a legitimate source state and validates ready with `material_digest() == None`.
- A present material with zero unresolved grantee and grantor references validates ready and carries `Some(material.digest())`.
- Any positive unresolved grantee or grantor count validates blocked and still carries `Some(material.digest())`.
- Recovery-validation fields are private and consumers have no public constructor for the verdict.
- Public validation diagnostics expose only the immutable material digest and aggregate counts; raw dangling OIDs stay inside immutable source identity.
- Validation is derived governance evidence and does not participate in Source Observation digest identity.
- ConceptWeave observes and validates; PostgreSQL role/catalog remediation remains outside this bounded context.

## Alternatives rejected

Treating every `pg_init_privs` row as publication-ready was rejected because PostgreSQL BUG #19483/#19513 demonstrate a real recovery failure mode. Rejecting damaged state during source capture was rejected because observation must preserve the external database state instead of falsifying it. Publishing raw dangling OIDs in a validation error was rejected because the existing aggregate projection is sufficient for deterministic gating and keeps the narrower privacy surface. Keeping a public enum was rejected because Rust callers could construct its variants directly. Returning only counts without the source digest was rejected because it provides no immutable link between the verdict and the observation being reviewed or published.

## Acceptance still required

This source/validation repair is not exact-head GREEN. The final PR head still needs repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential. The differential must prove that the public validation verdict agrees with the captured unresolved-reference counts, carries the exact present-material digest, leaves row absence unbound, and never exposes raw dangling OIDs.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
