# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before the dangling-role diagnostic-projection repair is preserved at `docs/archive/product-technical-gap-baseline-through-8348be96.md`; its matching changelog is `docs/archive/CHANGELOG-through-8348be96.md`. Earlier pre-dangling-role surfaces remain under the `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The initial-privilege successor separates `pg_init_privs` from current `pg_proc.proacl`, preserves row absence versus presence and exact `privtype`, and retains the complete object-level initial EXECUTE ACL. PostgreSQL BUG #19483 and #19513 demonstrate a concrete recovery defect in which `pg_init_privs.initprivs` still refers to a dropped role OID and a later dump/restore or `pg_upgrade` fails. Source Observation therefore preserves missing role lookups as unresolved nonzero raw OIDs instead of dropping the ACL entry, stringifying the OID as a role name, or rejecting the entire observation.

Finding review `5254787838` identified that raw dangling-OID identity alone did not expose enough privacy-preserving information for deterministic validation. Structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a` and production repair `fde6d3a8776b75e3dd014713b671734b9226a0fe` added `unresolved_grantee_count()` and `unresolved_grantor_count()` without changing Source Observation digest identity or publishing raw OIDs.

Finding review `5254918439` then identified the absence of a canonical owner interpretation for those diagnostics. RED `21817d01f0aaa69902699eb21109c2a057401a8c`, production `97ff3903278bafbd2133a88ded475ca481e8b1b7`, and composition `da84d2afc99d41f7af9273cd192047704d779a47` added the recovery-validation domain service: row absence and fully resolved material are ready; any unresolved grantee or grantor blocks readiness.

Finding review `5254963033` identified that the first verdict API was forgeable and detached from its material. RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38` and production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc` replaced public enum variants with a private-field public struct and bound every present verdict to `material.digest()`.

Fresh review `5255023556` at exact head `83b23116bad4106c2fe1c71953fb2453175dad0c` found a remaining provenance defect. `material.digest()` intentionally commits only `privtype` and canonical initial EXECUTE ACL. It does not commit converter coordinate or source snapshot generation, and row absence has no material digest. A legitimate `Ready` value could therefore be replayed as purported validation evidence for another converter direction with byte-identical ACL material, another source generation retaining that material, or any other absent row. That contradicted the stated invariant that validation evidence must link immutably to the observation being reviewed or published.

Structural RED `8734883edcb145933030cccd7177cff95abde772` moved the validation contract onto the full owner-issued Source Observation fixture chain. It requires byte-identical material at FROM SQL and TO SQL locations to retain distinct validation locations, the same material across distinct snapshot generations to retain distinct validation source digests, and row absence to remain ready while still binding to its exact receipt.

Production causal repair `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2` changes the validator input from detached `Option<&...InitialPrivilegeMaterial>` to `&IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt`. The private verdict carries the receipt `source_digest`, the observation `canonical_location`, optional present-row `material_digest`, and aggregate unresolved-reference counts. Readiness remains a pure function of unresolved counts. Raw dangling OIDs remain inside immutable Source Observation identity and never become public validation diagnostics.

This is a governance/provenance repair, not another PostgreSQL catalog successor. Equal ACL material still has equal material identity by design, but validation evidence is not interchangeable across source generations or converter locations. Row absence still has `material_digest() == None`; it is no longer an unbound successful verdict because source digest and canonical location identify the exact absent observation validated.

Each initial ACL identity continues to distinguish PUBLIC, resolved role names, unresolved nonzero role OIDs, and exact grant option. OID zero remains invalid for unresolved-role constructors. A real role whose name is decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`. Existing resolved-role and dangling-role digest framing remains stable. Neither aggregate counts nor recovery-validation evidence enters Source Observation digest identity.

The PostgreSQL 18 `pg_init_privs` catalog declaration confirms that `(objoid, classoid, objsubid)` is the unique key and `initprivs` is forced non-NULL; `privtype` is not part of the unique key. ConceptWeave therefore retains row absence versus exactly one material rather than inventing simultaneous per-`privtype` rows for one object coordinate.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md` for damaged-role capture and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privilege-recovery-validation.md` for validation/provenance.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026c). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026d). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
- Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com
- Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com

## Observation versus governance

Source Observation records exact external database state before policy. A dangling `pg_init_privs` role reference is an observed recovery/security defect, not a reason to falsify the source by dropping the ACL entry or aborting observation. The immutable material supplies a privacy-preserving diagnostic projection. The recovery validator consumes the owner-issued receipt for the exact observation, gives those diagnostics one deterministic ConceptWeave interpretation, and emits non-forgeable evidence bound to source generation and converter location. Present rows also retain the material digest. Nonzero unresolved-reference counts block recovery/publication readiness without exposing raw dangling OIDs.

ConceptWeave does not repair PostgreSQL catalogs and does not own role lifecycle. Current ACL, extension lifecycle, security labels, `pg_init_privs`, immutable raw converter-root identity, and transform-object extension membership remain separate facts.

## Acceptance boundary

**Source, validation, and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, dangling-role identity, diagnostic-projection, receipt-bound recovery-validation contracts, all retained initial-privilege/security-label/auto-extension/raw-root/direction/function regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, complete `deptype='x'` dependencies, complete `pg_seclabel`, and exact `pg_init_privs` row absence/presence with `privtype` plus complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership.

The differential must include at least one nonzero initial-ACL grantee or grantor OID absent from `pg_roles`. Capture must preserve that raw identity without stringification or lookup-gated loss. Public unresolved counts must match the ACL without exposing raw OIDs. Recovery validation must consume the owner-issued receipt, match its source snapshot digest and canonical converter location, carry the exact material digest when present, bind row absence to the same receipt, block damaged material, and admit row-absent or fully resolved observations. It must exercise equal material at distinct converter locations and retained material across distinct source snapshot digests so material identity and validation provenance are not conflated. A numeric-looking resolved role name and equal-valued unresolved OID must remain distinct. At least one ordinary differential still holds current `proacl` constant while changing the initial privilege baseline.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The current repair is admitted because the previous validation artifact could be replayed without proving which owner-issued Source Observation it validated. That is a provenance/governance defect in the existing admitted recovery boundary, not another catalog enumeration.

No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists. A next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction not represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, complete initial-privilege identity, dangling-reference diagnostics, receipt-bound canonical recovery-validation evidence, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their exact heads, mergeability, reviews, required checks, workflow inventory, and protected-main relationship must be read fresh before landing. Sibling or predecessor evidence does not transfer into #2040, #35, or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including receipt-bound initial-privilege recovery validation -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
