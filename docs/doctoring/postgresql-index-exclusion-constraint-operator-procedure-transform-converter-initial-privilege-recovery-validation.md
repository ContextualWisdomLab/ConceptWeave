# PostgreSQL transform-converter initial-privilege recovery validation

## Decision

`pg_init_privs` remains Source Observation truth. ConceptWeave does not repair a dangling PostgreSQL role reference, discard the affected ACL entry, or reinterpret a raw role OID as a role name. The immutable converter initial-privilege material preserves that source state and exposes only aggregate unresolved grantee/grantor counts for governance.

Recovery validation is derived governance evidence, not a new PostgreSQL source fact. It must nevertheless prove which immutable Source Observation it evaluated. The canonical validator therefore consumes an owner-issued `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt`, not detached ACL material. Every verdict is bound to the receipt's source snapshot digest and collision-safe canonical converter location. Present rows additionally carry the exact initial-privilege material digest; row absence carries no material digest but remains source- and location-bound.

## Validation-boundary repair

The first missing boundary was deterministic validation. Before that repair, an immutable material could report nonzero unresolved-reference counts but no ConceptWeave domain service turned that condition into a recovery/publication verdict. A caller could therefore move from observation toward publication without a canonical interpretation of the diagnostic.

Finding review `5254918439` at exact head `b88836369e5481f8f91eda86003af979f28bc7c9` records that defect. Structural RED `21817d01f0aaa69902699eb21109c2a057401a8c` referenced the missing public validator. Production commit `97ff3903278bafbd2133a88ded475ca481e8b1b7` added the pure Rust recovery-validation service and composition commit `da84d2afc99d41f7af9273cd192047704d779a47` exposed it through the relation/index-partition public surface.

## Non-forgeable verdict repair

Fresh review at exact head `7769b8b21b2d321f6d1f15390643a1f2c512a7cf` found that the first validator API was not yet trustworthy publication evidence. Its public enum exported constructible `Ready` and `Blocked` variants, so a consumer could mint a successful verdict without invoking the canonical validator. The verdict also carried no reference to the immutable material it had validated.

Finding review `5254963033` pins that defect. Structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38` stopped constructing or pattern-matching public verdict variants and required present rows to expose the exact material digest. Production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc` replaced the public enum with a public struct whose fields are private and whose production construction path is the canonical validator.

That repair made present-material verdicts non-forgeable and payload-bound, but it was not yet observation-bound. `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::digest()` commits `privtype` plus the canonical initial EXECUTE ACL. It deliberately does not commit converter coordinate, source snapshot generation, extractor revision, or observation time. Two converter directions or two source generations may therefore have byte-identical material and the same material digest. Row absence has no material digest at all.

## Source-receipt replay repair

Fresh review `5255023556` at exact head `83b23116bad4106c2fe1c71953fb2453175dad0c` found that the validator still accepted only `Option<&IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>`. A legitimate `Ready` result could therefore be detached from the owner-issued observation that produced it and reused as purported validation evidence for another converter direction or source generation. The defect was strongest for row absence because `material_digest() == None` supplied no binding at all. This contradicted the earlier rationale that recovery evidence must have an immutable link to the observation being reviewed or published.

Structural RED `8734883edcb145933030cccd7177cff95abde772` moved the focused contract onto the full Source Observation fixture chain before production changed. It requires three distinctions that detached material cannot express:

- byte-identical initial ACL material at FROM SQL and TO SQL converter locations must share the material digest while retaining distinct validation locations;
- the same converter material observed in two snapshots whose overall Source Observation digest differs must retain distinct validation source digests;
- row absence remains recovery-ready, but its successful verdict must still bind to the exact owner-issued source digest and converter location.

Production causal repair `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2` changes the validator input to `&IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt`. The private verdict now carries the receipt's `source_digest`, the observation's collision-safe `canonical_location`, optional present-row `material_digest`, and the aggregate unresolved-reference counts. Readiness is still derived solely from unresolved grantee/grantor counts. Raw dangling OIDs remain committed only inside Source Observation identity and are never copied into public validation diagnostics.

The source digest and canonical location are evidence binding, not new Source Observation facts. They do not enter the source digest recursively and they do not change the material digest. This repair narrows replay: equal ACL material may still have equal material identity, but validation evidence for one observation is no longer interchangeable with evidence for another location or source generation.

## Invariants

- Validation consumes an owner-issued initial-privilege source receipt, not caller-detached ACL material.
- Every verdict carries the exact source snapshot digest and canonical converter location from that receipt.
- `pg_init_privs` row absence is a legitimate source state and validates ready with `material_digest() == None`, but the verdict remains source- and location-bound.
- A present material with zero unresolved grantee and grantor references validates ready and carries `Some(material.digest())`.
- Any positive unresolved grantee or grantor count validates blocked and still carries `Some(material.digest())`.
- Byte-identical material at different converter locations has equal material identity but distinct validation location evidence.
- Byte-identical material across distinct snapshot generations has equal material identity but distinct validation source-digest evidence.
- Recovery-validation fields are private and consumers have no public constructor for the verdict.
- Public validation diagnostics expose source digest, canonical location, optional immutable material digest, and aggregate counts; raw dangling OIDs stay inside immutable Source Observation identity.
- Validation is derived governance evidence and does not participate in Source Observation digest identity.
- ConceptWeave observes and validates; PostgreSQL role/catalog remediation remains outside this bounded context.

## Alternatives rejected

Treating every `pg_init_privs` row as publication-ready was rejected because PostgreSQL BUG #19483/#19513 demonstrate a real recovery failure mode. Rejecting damaged state during source capture was rejected because observation must preserve the external database state instead of falsifying it. Publishing raw dangling OIDs in a validation error was rejected because aggregate projections are sufficient for deterministic gating and keep the narrower privacy surface. Keeping a public enum was rejected because Rust callers could construct its variants directly.

Binding only to `material.digest()` was also rejected. The material digest intentionally represents ACL material rather than provenance, so equal ACLs at different locations or generations collide by design and absence has no material digest. Accepting a caller-supplied source digest or location separately was rejected because it would recreate a forgeable provenance seam. The existing owner-issued source receipt already binds source digest and exact observation location without duplicating foreign truth.

## Acceptance still required

This source/validation repair is not exact-head GREEN. The final PR head still needs repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential.

The differential must prove that the public validation verdict agrees with the captured unresolved-reference counts, carries the exact source snapshot digest and canonical converter location from the owner-issued receipt, carries the exact material digest when a row is present, binds row absence to the exact receipt, and never exposes raw dangling OIDs. It must include equal material at distinct converter locations and a retained material across distinct snapshot generations so material identity and provenance identity are demonstrably not conflated.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
