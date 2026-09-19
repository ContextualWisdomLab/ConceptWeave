# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The immediately preceding Source Observation/recovery-validation decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-c1599559.md`; its matching changelog is `docs/archive/CHANGELOG-through-c1599559.md`. Earlier recovery surfaces remain under the `through-440d5e3f`, `through-8348be96`, and `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The initial-privilege successor separates `pg_init_privs` from current `pg_proc.proacl`, preserves row absence versus presence and exact `privtype`, and retains the complete object-level initial EXECUTE ACL. PostgreSQL BUG #19483 and #19513 demonstrate a recovery defect in which `pg_init_privs.initprivs` still refers to a dropped role OID and a later dump/restore or `pg_upgrade` fails. Source Observation therefore preserves missing role lookups as unresolved nonzero raw OIDs instead of dropping the ACL entry, stringifying the OID as a role name, or rejecting the observation.

Earlier repairs established representability, aggregate unresolved diagnostics, a canonical recovery validator, non-forgeable material-bound verdicts, complete owner-issued receipt provenance, and purpose-bound exact dangling-role sets. Review `5255638361` at exact pre-finding head `c159955939ddcfa7b69419816414a67d7424e69e` found a remaining log-safety breach in that admitted boundary: the public `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` and its private unresolved-role enum variants still derived `Debug`, so formatting a grant could emit the raw numeric OID before the value reached the already-redacted material or validation types.

Structural RED `3626a658d45b59802f632845708bf8ec378296cb` adds direct grant-level hostile formatting coverage. It requires unresolved grantee/grantor OIDs to remain absent from the diagnostic string while retaining unresolved-role kind and `grant_option` diagnostics. Production causal repair `cfe4eba0e68eca6d4f4cf6b50f18478e61d14c23` replaces the unresolved-role derived `Debug` path with redacted formatters. Equality, ordering, constructors, exact raw OID identity, typed recovery accessors, canonical OID sets, material/source digests, and validation receipt binding are unchanged.

The immediately preceding exact dangling-role actionability repair remains authoritative: finding review `5255447654` at `440d5e3f1cd12011047025c499aec76792c95c83`; structural RED `f23aefdcb8416c9828d2a46c12fcf3b6190fcd47`; material repair `b7b9e3d1b17a840436fbde7420f88a9c9539335f`; recovery-validation repair `57b5b209e9ade1f84c1c036a2bca81e7dacbac73`; log/rustdoc follow-up `a49243baddbc3142286de4c38c29bd5dd06cbdcb`; canonical-set contract `dd8d5adf65638ae5c5e2f61686b37d14df17062c`.

Grant, material, and validation `Debug` implementations now intentionally omit raw role OIDs. Grant diagnostics retain PUBLIC/resolved/unresolved shape, resolved role names, and grant-option state; material/validation diagnostics retain counts and existing digest/provenance. PostgreSQL role OIDs are catalog identifiers, not credentials, but ordinary logs do not need them. Exact identifiers remain available only through explicit typed recovery accessors on non-forgeable validation evidence. ConceptWeave still does not repair PostgreSQL catalogs or own role lifecycle.

Each initial ACL identity continues to distinguish PUBLIC, resolved role names, unresolved nonzero role OIDs, and exact grant option. OID zero remains invalid for unresolved-role constructors. A real role named decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`. Existing resolved-role and dangling-role digest framing remains stable. Diagnostic redaction, canonical dangling-role sets, and recovery-validation evidence do not change Source Observation digest identity.

The PostgreSQL 18 `pg_init_privs` catalog declaration confirms that `(objoid, classoid, objsubid)` is the unique key and `initprivs` is forced non-NULL; `privtype` is not part of the unique key. ConceptWeave therefore retains row absence versus exactly one material rather than inventing simultaneous per-`privtype` rows for one object coordinate.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md` for damaged-role capture, exact identity retention, and the complete log-safety boundary, and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privilege-recovery-validation.md` for receipt-bound validation/provenance/actionability.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026c). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026d). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
- Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com
- Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com

## Observation versus governance

Source Observation records exact external database state before policy. A dangling `pg_init_privs` role reference is an observed recovery/security defect, not a reason to falsify the source by dropping the ACL entry or aborting observation. The immutable material preserves the exact raw identity in the ACL digest and retains canonical missing-role sets for recovery validation. The recovery validator consumes the owner-issued receipt, emits non-forgeable evidence retaining complete receipt provenance and exact purpose-bound missing-role identities, and blocks readiness whenever either set is non-empty.

This boundary deliberately separates identity/actionability from routine diagnostics. Typed recovery accessors expose exact role OIDs only on the recovery-validation surface; routine grant/material/validation `Debug` formatting redacts unresolved raw OIDs. ConceptWeave validates and governs publication. PostgreSQL catalog/role remediation remains outside this bounded context.

## Acceptance boundary

**Source, validation, and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, dangling-role identity, grant-level Debug redaction, canonical-set/actionability contracts, full receipt-bound recovery-validation contracts, all retained initial-privilege/security-label/auto-extension/raw-root/direction/function regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, complete `deptype='x'` dependencies, complete `pg_seclabel`, and exact `pg_init_privs` row absence/presence with `privtype` plus complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership.

The differential must include nonzero initial-ACL grantee/grantor OIDs absent from `pg_roles` and preserve those identities without stringification or lookup-gated loss. Recovery evidence must expose exact canonical missing grantee/grantor OID sets through typed accessors, derive counts from the canonical distinct sets, and remain receipt-bound. Routine formatting of unresolved public-grantor, grantee-only, grantor-only, both-unresolved grant values, material, and validation evidence must not emit the numeric raw OID. It must also include equal unresolved counts with different OID identities, repeated OIDs that sort/deduplicate deterministically, a numeric-looking resolved role name versus equal-valued raw OID, row absence, fully resolved material, equal material at distinct converter locations, and retained material across distinct source snapshots. At least one ordinary differential must hold current `proacl` constant while changing the initial privilege baseline.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The actionable recovery-identity and routine-log-safety gaps are now represented in source, validation evidence, and focused contracts. No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists. A next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction not represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, complete initial-privilege identity, exact dangling-role recovery evidence, receipt-bound canonical recovery-validation evidence, complete routine Debug redaction, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

The protected reusable-workflow authority remains `ContextualWisdomLab/.github@main` and must be read fresh before any ConceptWeave landing. Product bootstrap #35 remains a separate downstream prerequisite. ConceptWeave records only stable owner lanes and their contract boundaries here; it does not version mutable foreign PR head SHAs, run IDs, mergeability, or check status as local product truth.

- `.github#2279` owns the shared GitHub API initial-URL and redirect-authority boundary.
- `.github#2271` owns CodeQL dispatch repository-identity admission and remains ordered after the accepted #2279 foundation.
- `.github#2268` owns queue-health repository-identity admission.
- `.github#2040` remains the scheduler owner. Its protected-main reconciliation must preserve its credential/settlement deltas, adopt compatible current-main scheduler behavior, enforce the stronger repository-identity invariant, and preserve applicable accepted #2271/#2268 owner slices if those paths enter reconciliation.

These are canonical-owner responsibilities, not transferable acceptance evidence or mutable source dependencies. Immediately before downstream acceptance, reread each owner PR's exact head, review/thread state, workflow inventory, mergeability, and relationship to protected `.github/main`. Record that volatile observation in live PR/review coordination evidence rather than copying it into this versioned baseline. Sibling or predecessor GREEN never transfers into #2040, #35, or #46.

## Required order

Foundation security/identity owners must first reach accepted protected-main authority without weakening gates: #2279 must obtain exact-head terminal hosted evidence and qualifying review before its repair is treated as accepted; #2271/#2268 remain their respective owner lanes and must be ordinary-forward reconciled rather than copied from mutable heads. #2040 then completes ordinary/non-force protected-main path-wise scheduler reconciliation against the accepted foundation, preserving applicable repaired owner slices, followed by fresh exact-head scheduler/required GREEN and qualifying non-self approval. Only then may #35 obtain fresh compatible acceptance/normal landing. #46 still requires final Rust 1.98/coverage GREEN and the bounded PostgreSQL 18 same-generation differential including exact dangling-role recovery evidence, complete routine Debug redaction, and complete receipt-bound initial-privilege validation, followed only by independently proven semantic-gap work, complete ordinary/non-force #46 adoption into #45, fresh #45 acceptance, #6 propagation, and immutable governed release from an accepted protected head.
