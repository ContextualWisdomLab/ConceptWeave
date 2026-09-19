# PostgreSQL transform-converter initial-privilege recovery validation

## Decision

`pg_init_privs` remains Source Observation truth. ConceptWeave does not repair a dangling PostgreSQL role reference, discard the affected ACL entry, or reinterpret a raw role OID as a role name. The immutable converter initial-privilege material preserves that source state and exposes only aggregate unresolved grantee/grantor counts for governance.

Recovery validation is derived governance evidence, not a new PostgreSQL source fact. It must nevertheless prove which immutable Source Observation receipt it evaluated. The canonical validator therefore consumes an owner-issued `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt`, not detached ACL material. Every verdict retains the receipt's source registry identity, immutable connection-policy binding, source-content digest, extractor revision, observation timestamp, and collision-safe converter location. Present rows additionally carry the exact initial-privilege material digest; row absence carries no material digest but remains fully receipt-bound.

## Validation-boundary repair

The first missing boundary was deterministic validation. Before that repair, an immutable material could report nonzero unresolved-reference counts but no ConceptWeave domain service turned that condition into a recovery/publication verdict. A caller could therefore move from observation toward publication without a canonical interpretation of the diagnostic.

Finding review `5254918439` at exact head `b88836369e5481f8f91eda86003af979f28bc7c9` records that defect. Structural RED `21817d01f0aaa69902699eb21109c2a057401a8c` referenced the missing public validator. Production commit `97ff3903278bafbd2133a88ded475ca481e8b1b7` added the pure Rust recovery-validation service and composition commit `da84d2afc99d41f7af9273cd192047704d779a47` exposed it through the relation/index-partition public surface.

## Non-forgeable verdict repair

Fresh review at exact head `7769b8b21b2d321f6d1f15390643a1f2c512a7cf` found that the first validator API was not yet trustworthy publication evidence. Its public enum exported constructible `Ready` and `Blocked` variants, so a consumer could mint a successful verdict without invoking the canonical validator. The verdict also carried no reference to the immutable material it had validated.

Finding review `5254963033` pins that defect. Structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38` stopped constructing or pattern-matching public verdict variants and required present rows to expose the exact material digest. Production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc` replaced the public enum with a public struct whose fields are private and whose production construction path is the canonical validator.

That repair made present-material verdicts non-forgeable and payload-bound, but it was not yet observation-bound. `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::digest()` commits `privtype` plus the canonical initial EXECUTE ACL. It deliberately does not commit converter coordinate or receipt provenance, and row absence has no material digest.

## Source-receipt replay repair

Fresh review `5255023556` at exact head `83b23116bad4106c2fe1c71953fb2453175dad0c` found that the validator still accepted only `Option<&IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>`. A legitimate `Ready` result could therefore be detached from the owner-issued observation that produced it and reused as purported validation evidence for another converter direction or source snapshot. The defect was strongest for row absence because `material_digest() == None` supplied no binding at all.

Structural RED `8734883edcb145933030cccd7177cff95abde772` moved the focused contract onto the full Source Observation fixture chain before production changed. It requires byte-identical ACL material at different converter locations to remain distinct validation evidence, the same material in distinct content snapshots to retain distinct source-digest evidence, and row absence to stay recovery-ready while still binding to an owner receipt.

Production repair `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2` changed the validator input to `&IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt` and initially retained source digest plus canonical location. That closed location/content-snapshot replay but a fresh exact-head review found that the implementation still did not retain the *complete* receipt provenance it claimed to bind.

## Complete receipt-provenance repair

Review `5255032673` at exact head `d16254bbf116c955a14ca3d6d92b48b70cfb6fa6` identified the remaining gap. `InitialPrivilegeSourceReceipt` carries `source_id`, `connection_policy_binding`, `source_digest`, `extractor_revision`, `observed_at_utc`, and exact observation location as separate public provenance fields. Copying only `source_digest` and location cannot prove the exact receipt when identical source content is observed under a different source-policy binding, extractor revision, or observation epoch.

Structural RED `c5f9686ec16c27311abeefd858b0d317ac3c4d6d` requires recovery validation to preserve every non-secret receipt provenance field and adds `matches_source_receipt` contract checks. It also proves a verdict for one converter location does not match the other location and that a verdict from one distinct content snapshot does not match another.

Production causal repair `b466a6fcb207061684e104a51010fed6a0bbca55` extends the private verdict with the complete owner-issued receipt provenance: stable source registry identity, immutable connection-policy binding, source-content digest, extractor revision, canonical UTC observation timestamp, and canonical converter location. `matches_source_receipt()` compares all of those fields plus row absence/presence material identity against a candidate owner receipt. Present rows retain `material_digest`; absence remains `None` at the material layer while still being fully receipt-bound. Raw dangling role OIDs remain committed only inside Source Observation identity and are never copied into public validation diagnostics.

The receipt provenance fields are evidence binding, not new Source Observation facts. They do not enter the source digest recursively and do not change the material digest. Equal ACL material may retain equal material identity, and identical source content may retain equal content identity, while validation evidence remains distinguishable across source registry, policy, extractor, time, and converter-location provenance.

## Invariants

- Validation consumes an owner-issued initial-privilege source receipt, not caller-detached ACL material.
- Every verdict preserves the exact receipt `source_id`, `connection_policy_binding`, `source_digest`, `extractor_revision`, `observed_at_utc`, and canonical converter location.
- `matches_source_receipt()` compares that full provenance plus row absence/presence material identity before a verdict can be treated as evidence for a candidate receipt.
- `pg_init_privs` row absence is a legitimate source state and validates ready with `material_digest() == None`, but the verdict remains fully receipt-bound.
- A present material with zero unresolved grantee and grantor references validates ready and carries `Some(material.digest())`.
- Any positive unresolved grantee or grantor count validates blocked and still carries `Some(material.digest())`.
- Byte-identical material at different converter locations has equal material identity but distinct validation-location evidence.
- Equal source content observed under a different policy/extractor/time epoch cannot be proven equivalent merely by comparing `source_digest`; the complete receipt binding remains authoritative.
- Recovery-validation fields are private and consumers have no public constructor for the verdict.
- Public validation diagnostics expose only receipt provenance, optional immutable material digest, and aggregate counts; raw dangling OIDs stay inside immutable Source Observation identity.
- Validation is derived governance evidence and does not participate in Source Observation digest identity.
- ConceptWeave observes and validates; PostgreSQL role/catalog remediation remains outside this bounded context.

## Alternatives rejected

Treating every `pg_init_privs` row as publication-ready was rejected because PostgreSQL BUG #19483/#19513 demonstrate a real recovery failure mode. Rejecting damaged state during source capture was rejected because observation must preserve the external database state instead of falsifying it. Publishing raw dangling OIDs in a validation error was rejected because aggregate projections are sufficient for deterministic gating and keep the narrower privacy surface. Keeping a public enum was rejected because Rust callers could construct its variants directly.

Binding only to `material.digest()` was rejected because equal ACL material at different locations or generations collides by design and absence has no material digest. Binding only to `source_digest` plus location was rejected because the owner receipt deliberately keeps source identity, policy binding, extractor revision, and observation time outside that content digest. Accepting those values as free caller arguments was rejected because it would recreate a forgeable provenance seam. The owner-issued receipt is the canonical source of all binding fields.

## Acceptance still required

This source/validation repair is not exact-head GREEN. The final PR head still needs repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential.

The differential must prove that the public validation verdict agrees with captured unresolved-reference counts, matches the complete owner-issued receipt provenance, carries the exact material digest when a row is present, binds row absence to the same receipt, and never exposes raw dangling OIDs. It must include equal material at distinct converter locations and retained material across distinct source snapshots. Where the harness can vary source registry/policy/extractor/time while retaining content, it must prove those provenance fields remain distinct rather than being inferred from content identity.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table*. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
