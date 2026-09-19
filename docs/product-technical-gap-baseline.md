# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before the dangling-role diagnostic-projection repair is preserved at `docs/archive/product-technical-gap-baseline-through-8348be96.md`; its matching changelog is `docs/archive/CHANGELOG-through-8348be96.md`. The earlier pre-dangling-role surface remains under the `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The initial-privilege successor separates `pg_init_privs` from current `pg_proc.proacl`, preserves row absence versus presence and exact `privtype`, and retains the complete object-level initial EXECUTE ACL. PostgreSQL BUG #19483 and #19513 demonstrate a concrete recovery defect in which `pg_init_privs.initprivs` still refers to a dropped role OID and a later dump/restore or `pg_upgrade` fails. Source Observation therefore preserves missing role lookups as unresolved nonzero raw OIDs instead of dropping the ACL entry, stringifying the OID as a role name, or rejecting the entire observation.

A follow-up review found that representability alone was not enough. At exact head `8348be96316f625525b3d89abb732e952c0240e5`, raw dangling OIDs were committed into the privacy-preserving digest, but the material publicly exposed only `privilege_type`, total `grant_count`, and `digest`. A later deterministic `validate` or recovery-diagnostic stage could not determine from the immutable observation whether unresolved grantee/grantor references existed. That contradicted the documented boundary that validation/publication may flag the damaged state.

Finding review `5254787838` pinned this projection gap. Structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a` required diagnostic accessors before production exposed them. Production causal repair `fde6d3a8776b75e3dd014713b671734b9226a0fe` added `unresolved_grantee_count()` and `unresolved_grantor_count()` to the existing initial-privilege material. The counts are deterministic projections of the canonical ACL; they do not add a new catalog successor and do not change digest identity. Raw dangling OID values stay inside the privacy-preserving digest namespace.

Fresh review then found a separate governance defect at exact head `b88836369e5481f8f91eda86003af979f28bc7c9`: the immutable observation exposed the aggregate diagnostics but ConceptWeave had no canonical validation service that consumed them. A caller could therefore receive `unresolved_grantee_count() > 0` or `unresolved_grantor_count() > 0` and still advance unless it invented its own policy. That left the owner chain `observe -> ... -> validate -> ... -> publish` permissive by omission even though the source fact itself was lossless.

Finding review `5254918439` records that validation gap. Structural RED `21817d01f0aaa69902699eb21109c2a057401a8c` referenced the missing public recovery validator before production defined it. Production repair `97ff3903278bafbd2133a88ded475ca481e8b1b7` added the canonical recovery validator, and public composition `da84d2afc99d41f7af9273cd192047704d779a47` exposed it through the relation/index-partition crate surface.

Fresh exact-head review at `7769b8b21b2d321f6d1f15390643a1f2c512a7cf` then found that this first verdict API was still not governed evidence: its public enum variants allowed any consumer to construct `Ready` without executing the validator, and the verdict was not bound to the immutable material it supposedly validated. A valid verdict could therefore be replayed against a different `pg_init_privs` baseline.

Finding review `5254963033` pins the evidence-integrity defect. Structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38` required `material_digest()` before production exposed it and stopped constructing or pattern-matching public verdict variants. Production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc` replaces the constructible public enum with a public struct whose fields are private and whose only production constructor is the canonical validator. Row absence carries no material digest. Every present resolved or damaged baseline carries `Some(material.digest())`; readiness is derived from the private unresolved-reference counts. This binds validation evidence to immutable Source Observation identity without adding a new source fact or exposing raw dangling OIDs.

Each initial ACL identity still distinguishes PUBLIC, resolved role names, unresolved nonzero role OIDs, and exact grant option. OID zero remains invalid for unresolved-role constructors. A real role whose name is decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`. Existing resolved-role and dangling-role digest framing remains byte-for-byte stable. Neither the aggregate counts nor the recovery-validation evidence enters Source Observation digest identity.

The diagnostic contract requires resolved-only `0/0`, dangling grantee `1/0`, dangling grantor `0/1`, both dangling `1/1`, and PUBLIC with dangling grantor `0/1`. The validation contract requires row absence to be ready with `material_digest() == None`; every present material must expose its exact immutable digest; any unresolved dimension must block readiness; and public validation diagnostics must omit raw OID values. The successor still preserves complete converter coordinates, direction, exact schema/function binding, source-generation metadata, and immutable raw `converter_snapshot_digest`. Missing/extra/duplicate converter coordinates, binding drift, zero key positions, unknown receipt coordinates, duplicate exact ACL entries, blank resolved role names, and unresolved OID zero fail closed.

The PostgreSQL 18 `pg_init_privs` catalog declaration confirms that `(objoid, classoid, objsubid)` is the unique key and that `initprivs` is forced non-NULL. `privtype` is not part of the unique key. ConceptWeave therefore retains row absence versus exactly one material instead of inventing simultaneous per-`privtype` rows for one object coordinate.

Focused doctoring remains `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md`; the validation and evidence-binding decision is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privilege-recovery-validation.md`. The earlier decision surface remains immutable under the existing archives and Git history.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026c). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026d). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
- Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com
- Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com

## Observation versus governance

Source Observation records exact external database state before policy. A dangling `pg_init_privs` role reference is therefore an observed recovery/security defect, not a reason to falsify the source by dropping the ACL entry or to abort all observation. The immutable material supplies a minimal privacy-preserving diagnostic projection. The ConceptWeave recovery validator gives that projection one deterministic owner interpretation and emits non-forgeable evidence bound to the exact material digest when a row is present. Nonzero unresolved-reference counts block recovery/publication readiness without exposing raw dangling OIDs.

ConceptWeave does not repair PostgreSQL catalogs and does not own role lifecycle. It preserves the source fact and emits the validation evidence needed by later governance. Current ACL, extension lifecycle, security labels, `pg_init_privs`, immutable raw converter-root identity, and transform-object extension membership remain separate facts.

## Acceptance boundary

**Source, validation, and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dangling-role identity, diagnostic-projection, recovery-validation, and evidence-binding contracts, all retained initial-privilege/security-label/auto-extension/raw-root/direction/function regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, the complete `deptype='x'` dependency set, the complete `pg_seclabel` provider/label map, and exact `pg_init_privs` row absence/presence with `privtype` plus the complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership.

The differential must include at least one nonzero `pg_init_privs` grantee or grantor OID with no row in `pg_roles`. Capture must preserve that raw OID in identity, not drop it, not fail solely because role lookup misses, and not reinterpret its decimal text as a role name. The material must report unresolved grantee/grantor counts consistent with the ACL while not exposing raw OID values through the diagnostic API. Recovery validation must bind every present verdict to the exact material digest, leave row absence unbound, block damaged material, and admit row-absent or fully resolved baselines. A numeric-looking resolved role name and equal-valued unresolved OID must remain distinct. At least one ordinary differential still holds current `proacl` constant while changing the initial privilege baseline.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

This repair is admitted because primary PostgreSQL evidence demonstrates a concrete dump/restore and `pg_upgrade` failure mode and the prior owner contract lacked trustworthy validation evidence needed to prevent damaged recovery state from being treated as publication-ready. It is not another catalog successor.

No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists. The next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction that is not already represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, complete initial-privilege identity, privacy-preserving dangling-reference diagnostics, digest-bound canonical recovery-validation evidence, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their exact heads, mergeability, reviews, required checks, workflow inventory, and protected-main relationship must be read fresh before landing. Sibling or predecessor evidence does not transfer into #2040, #35, or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including current ACL, converter `deptype='e'`, complete converter `deptype='x'` sets, complete converter security-label maps, exact converter initial privilege baselines including dangling raw role-OID identity, unresolved-reference diagnostics and digest-bound canonical recovery-validation evidence, immutable raw converter-root lineage, and transform-object `deptype='e'` -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
