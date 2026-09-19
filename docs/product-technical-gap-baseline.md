# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before the dangling-role diagnostic-projection repair is preserved at `docs/archive/product-technical-gap-baseline-through-8348be96.md`; its matching changelog is `docs/archive/CHANGELOG-through-8348be96.md`. Earlier pre-dangling-role surfaces remain under the `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The initial-privilege successor separates `pg_init_privs` from current `pg_proc.proacl`, preserves row absence versus presence and exact `privtype`, and retains the complete object-level initial EXECUTE ACL. PostgreSQL BUG #19483 and #19513 demonstrate a recovery defect in which `pg_init_privs.initprivs` still refers to a dropped role OID and a later dump/restore or `pg_upgrade` fails. Source Observation therefore preserves missing role lookups as unresolved nonzero raw OIDs instead of dropping the ACL entry, stringifying the OID as a role name, or rejecting the observation.

Finding review `5254787838` identified that raw dangling-OID identity alone did not expose enough privacy-preserving information for deterministic validation. Structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a` and production repair `fde6d3a8776b75e3dd014713b671734b9226a0fe` added `unresolved_grantee_count()` and `unresolved_grantor_count()` without changing Source Observation digest identity or publishing raw OIDs.

Finding review `5254918439` identified the absence of a canonical owner interpretation for those diagnostics. RED `21817d01f0aaa69902699eb21109c2a057401a8c`, production `97ff3903278bafbd2133a88ded475ca481e8b1b7`, and composition `da84d2afc99d41f7af9273cd192047704d779a47` added the recovery-validation domain service: row absence and fully resolved material are ready; any unresolved grantee or grantor blocks readiness.

Finding review `5254963033` identified that the first verdict API was forgeable and detached from its material. RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38` and production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc` replaced public enum variants with a private-field public struct and bound every present verdict to `material.digest()`.

Review `5255023556` at exact head `83b23116bad4106c2fe1c71953fb2453175dad0c` then found that material identity was not observation provenance. `material.digest()` intentionally commits only `privtype` and canonical initial EXECUTE ACL; it does not commit converter coordinate or receipt provenance, and row absence has no material digest. RED `8734883edcb145933030cccd7177cff95abde772` moved validation onto owner-issued Source Observation receipts. First production repair `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2` changed the validator input to `InitialPrivilegeSourceReceipt` and retained source digest plus canonical location.

Fresh follow-up review `5255032673` at exact head `d16254bbf116c955a14ca3d6d92b48b70cfb6fa6` found that source digest plus location still did not prove the exact receipt. `InitialPrivilegeSourceReceipt` deliberately carries stable source registry identity, immutable connection-policy binding, source-content digest, extractor revision, observation timestamp, and exact location as separate provenance. Equal content may legitimately retain the same content digest while policy, extractor, or observation epoch changes. A governance artifact that drops those fields cannot demonstrate which exact receipt it validated.

Structural RED `c5f9686ec16c27311abeefd858b0d317ac3c4d6d` requires every non-secret receipt provenance field to survive validation and adds `matches_source_receipt()` contract checks. Production causal repair `b466a6fcb207061684e104a51010fed6a0bbca55` extends the private verdict with `source_id`, `connection_policy_binding`, `source_digest`, `extractor_revision`, `observed_at_utc`, canonical converter location, optional present-row material digest, and aggregate unresolved-reference counts. `matches_source_receipt()` compares the complete receipt binding plus row absence/presence material identity against a candidate owner receipt.

This remains a governance/provenance repair, not another PostgreSQL catalog successor. Equal ACL material keeps equal material identity by design; equal source content may retain equal content identity; validation evidence is still distinguishable across source registry, policy, extractor, observation time, and converter location. Row absence still has `material_digest() == None`, but the successful verdict is fully receipt-bound.

Each initial ACL identity continues to distinguish PUBLIC, resolved role names, unresolved nonzero role OIDs, and exact grant option. OID zero remains invalid for unresolved-role constructors. A real role named decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`. Existing resolved-role and dangling-role digest framing remains stable. Neither aggregate counts nor recovery-validation evidence enters Source Observation digest identity.

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

Source Observation records exact external database state before policy. A dangling `pg_init_privs` role reference is an observed recovery/security defect, not a reason to falsify the source by dropping the ACL entry or aborting observation. The immutable material supplies a privacy-preserving diagnostic projection. The recovery validator consumes the owner-issued receipt for the exact observation, gives those diagnostics one deterministic ConceptWeave interpretation, and emits non-forgeable evidence retaining the complete receipt provenance. Present rows also retain material identity. Nonzero unresolved-reference counts block recovery/publication readiness without exposing raw dangling OIDs.

ConceptWeave does not repair PostgreSQL catalogs and does not own role lifecycle. Current ACL, extension lifecycle, security labels, `pg_init_privs`, immutable raw converter-root identity, and transform-object extension membership remain separate facts.

## Acceptance boundary

**Source, validation, and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, dangling-role identity, diagnostic-projection, full receipt-bound recovery-validation contracts, all retained initial-privilege/security-label/auto-extension/raw-root/direction/function regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, complete `deptype='x'` dependencies, complete `pg_seclabel`, and exact `pg_init_privs` row absence/presence with `privtype` plus complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership.

The differential must include at least one nonzero initial-ACL grantee or grantor OID absent from `pg_roles`. Capture must preserve that raw identity without stringification or lookup-gated loss. Public unresolved counts must match the ACL without exposing raw OIDs. Recovery validation must consume the owner-issued receipt, retain and match its full source/policy/content/extractor/time/location provenance, carry the exact material digest when present, bind row absence to the same receipt, block damaged material, and admit row-absent or fully resolved observations. It must exercise equal material at distinct converter locations and retained material across distinct content snapshots. Where the harness can vary source registry, policy, extractor, or observation time while retaining source content, the validation evidence must remain provenance-distinct. A numeric-looking resolved role name and equal-valued unresolved OID must remain distinct. At least one ordinary differential still holds current `proacl` constant while changing the initial privilege baseline.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The current repair is admitted because the previous validation artifact could be replayed without proving which complete owner-issued Source Observation receipt it validated. That is a provenance/governance defect in an admitted recovery boundary, not another catalog enumeration.

No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists. A next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction not represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, complete initial-privilege identity, dangling-reference diagnostics, complete receipt-bound canonical recovery-validation evidence, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

The protected reusable-workflow authority remains `ContextualWisdomLab/.github@main` and must be read fresh before any ConceptWeave landing. Product bootstrap #35 remains a separate downstream prerequisite. The live central owner graph is no longer represented completely by `.github#2040` alone:

- `.github#2279@25f83aaee9eb97e423f6ef2467e722035bc2e362` owns the shared GitHub API initial-URL and redirect-authority boundary. Its exact-head hosted Python Security, CodeQL, SAST, Security Scan, and agent-review runs exist but are queued; queued evidence is not GREEN.
- `.github#2271@2b849c874122961e025c29f7fa0bb697863c3d68` owns CodeQL dispatch repository-identity admission and explicitly waits on #2279 before ordinary-forward reconciliation.
- `.github#2268@142e5b2617778e79f665693be1e6f8c04d7533aa` owns queue-health repository-identity admission.
- `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d` remains the scheduler owner. Its protected-main reconciliation must preserve its own credential/settlement deltas, adopt compatible current-main scheduler behavior, enforce the stronger repository-identity invariant, and must not regress the #2271/#2268 repaired slices if those paths enter the reconciliation.

These are canonical-owner source authorities, not transferable acceptance evidence. Their exact heads, mergeability, review state, workflow inventory, and relationship to protected `.github/main` must be reread before downstream landing. Sibling or predecessor GREEN never transfers into #2040, #35, or #46.

## Required order

Foundation security/identity owners must first reach accepted protected-main authority without weakening gates: #2279 must obtain exact-head terminal hosted evidence and qualifying review before its repair is treated as accepted; #2271/#2268 remain their respective owner lanes and must be ordinary-forward reconciled rather than copied from mutable heads. #2040 then completes ordinary/non-force protected-main path-wise scheduler reconciliation against the accepted foundation, preserving applicable repaired owner slices, followed by fresh exact-head scheduler/required GREEN and qualifying non-self approval. Only then may #35 obtain fresh compatible acceptance/normal landing. #46 still requires final Rust 1.98/coverage GREEN and the bounded PostgreSQL 18 same-generation differential including complete receipt-bound initial-privilege recovery validation, followed only by independently proven semantic-gap work, complete ordinary/non-force #46 adoption into #45, fresh #45 acceptance, #6 propagation, and immutable governed release from an accepted protected head.
