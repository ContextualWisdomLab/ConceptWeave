# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The immediately preceding Source Observation/recovery-validation decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-ae860199.md`; its matching changelog is `docs/archive/CHANGELOG-through-ae860199.md`. Earlier recovery surfaces remain under the `through-c1599559`, `through-440d5e3f`, `through-8348be96`, and `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The initial-privilege successor separates `pg_init_privs` from current `pg_proc.proacl`, preserves row absence versus presence and exact `privtype`, and retains the complete object-level initial EXECUTE ACL. PostgreSQL BUG #19483 and #19513 demonstrate a recovery defect in which `pg_init_privs.initprivs` still refers to a dropped role OID and a later dump/restore or `pg_upgrade` fails. Source Observation therefore preserves missing role lookups as unresolved nonzero raw OIDs instead of dropping the ACL entry, stringifying the OID as a role name, or rejecting the observation.

Review `5255771158` at exact pre-finding head `ae86019978a9d1fbce6b905d65df92835e851b39` found that this “complete ACL” contract was not actually represented after material construction. `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::new` sorted and hashed every ACL grant but then discarded the grant vector, retaining only `privilege_type`, `grant_count`, dangling-role sets and an opaque digest. A digest is a stable integrity/identity commitment, but it cannot let downstream validate/review/recovery inspect PUBLIC versus resolved grantee, resolved grantor, grant option, unresolved-role shape, or damaged entry pairing.

Structural RED `3d03f9b9d3ce27ae2d96732ec03780a345493afc` requires the immutable material to expose its canonical ACL with log-safe grant semantics. Production causal repair `d406375f8989dae5dc06c50c1f4a776408a00bd8` retains the already-sorted grant vector inside the material and adds `grants()` plus PUBLIC/resolved-role/unresolved-kind/grant-option accessors. The existing material/source digest framing is byte-for-byte unchanged. Raw dangling OIDs remain private on the ordinary grant/material surface and available only through the receipt-bound recovery-validation evidence.

Earlier repairs established dangling-role representability, deterministic diagnostics, non-forgeable receipt-bound validation, exact canonical missing-role sets, and complete routine Debug redaction. The immediately preceding grant-level log-safety repair remains authoritative: finding review `5255638361` at `c159955939ddcfa7b69419816414a67d7424e69e`; structural RED `3626a658d45b59802f632845708bf8ec378296cb`; production repair `cfe4eba0e68eca6d4f4cf6b50f18478e61d14c23`.

Grant, material, and validation `Debug` implementations intentionally omit raw role OIDs. Grant diagnostics retain PUBLIC/resolved/unresolved shape, resolved role names, and grant-option state; material/validation diagnostics retain counts and existing digest/provenance. The canonical material ACL is now inspectable without exposing numeric dangling OIDs through its ordinary semantic accessors. Exact missing identifiers remain purpose-bound to explicit recovery-validation accessors. ConceptWeave still does not repair PostgreSQL catalogs or own role lifecycle.

Each initial ACL identity continues to distinguish PUBLIC, resolved role names, unresolved nonzero role OIDs, and exact grant option. OID zero remains invalid for unresolved-role constructors. A real role named decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`. Existing resolved-role and dangling-role digest framing remains stable. Retaining the canonical ACL vector changes evidence availability, not digest identity.

The PostgreSQL 18 `pg_init_privs` catalog declaration confirms that `(objoid, classoid, objsubid)` is the unique key and `initprivs` is forced non-NULL; `privtype` is not part of the unique key. ConceptWeave therefore retains row absence versus exactly one material rather than inventing simultaneous per-`privtype` rows for one object coordinate.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md` for complete ACL capture, damaged-role integrity and log-safety, and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privilege-recovery-validation.md` for receipt-bound validation/provenance/actionability.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: src/include/catalog/pg_init_privs.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_init_privs.h
- PostgreSQL Global Development Group. (2026c). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- PostgreSQL Global Development Group. (2026d). *BUG #19513: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/19513-ad75b550762d3d09%40postgresql.org
- Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com
- Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com

## Observation versus governance

Source Observation records exact external database state before policy. An initial ACL must remain inspectable evidence, not merely a digest commitment. The immutable material therefore retains canonical grants while the digest continues to provide stable identity. A dangling `pg_init_privs` role reference is an observed recovery/security defect, not a reason to falsify the source by dropping the ACL entry or aborting observation. Canonical missing-role sets feed recovery validation; ordinary ACL semantics remain available without raw-OID disclosure.

The recovery validator consumes the owner-issued receipt, emits non-forgeable evidence retaining complete receipt provenance and exact purpose-bound missing-role identities, and blocks readiness whenever either set is non-empty. Typed recovery accessors expose exact role OIDs only on the recovery-validation surface; routine grant/material/validation `Debug` formatting redacts unresolved raw OIDs. ConceptWeave validates and governs publication. PostgreSQL catalog/role remediation remains outside this bounded context.

## Acceptance boundary

**Source, validation, and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, canonical ACL readability, dangling-role identity, grant-level Debug redaction, canonical-set/actionability contracts, full receipt-bound recovery-validation contracts, all retained initial-privilege/security-label/auto-extension/raw-root/direction/function regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, complete `deptype='x'` dependencies, complete `pg_seclabel`, and exact `pg_init_privs` row absence/presence with `privtype` plus complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership.

The differential must prove the captured initial ACL remains readable after Source Observation construction: PUBLIC/resolved grantee, resolved grantor, grant-option and unresolved-kind semantics must round-trip from canonical material, not only change an opaque digest. It must include nonzero initial-ACL grantee/grantor OIDs absent from `pg_roles`; recovery evidence must expose exact canonical missing grantee/grantor OID sets through typed accessors while ordinary grant/material accessors and Debug output must not emit numeric raw OIDs. Equal unresolved counts with different OID identities, repeated-OID canonicalization, numeric-looking resolved names, row absence, fully resolved material, equal material at distinct converter locations, retained material across distinct source snapshots, and a constant-current-`proacl`/different-initial-baseline case remain mandatory.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The complete-ACL readability, actionable recovery identity, and routine-log-safety gaps are now represented in source and focused contracts. No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists. A next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction not represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, inspectable complete initial-privilege ACL, exact dangling-role recovery evidence, receipt-bound recovery validation, routine Debug redaction, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

The protected reusable-workflow authority remains `ContextualWisdomLab/.github@main` and must be read fresh before any ConceptWeave landing. Product bootstrap #35 remains a separate downstream prerequisite. ConceptWeave records only stable owner lanes and their contract boundaries here; it does not version mutable foreign PR head SHAs, run IDs, mergeability, or check status as local product truth.

- `.github#2279` owns the shared GitHub API initial-URL and redirect-authority boundary.
- `.github#2271` owns CodeQL dispatch repository-identity admission and remains ordered after the accepted #2279 foundation.
- `.github#2268` owns queue-health repository-identity admission.
- `.github#2040` remains the scheduler owner. Its protected-main reconciliation must preserve its credential/settlement deltas, adopt compatible current-main scheduler behavior, enforce the stronger repository-identity invariant, and preserve applicable accepted #2271/#2268 owner slices if those paths enter reconciliation.

These are canonical-owner responsibilities, not transferable acceptance evidence or mutable source dependencies. Immediately before downstream acceptance, reread each owner PR's exact head, review/thread state, workflow inventory, mergeability, and relationship to protected `.github/main`. Record that volatile observation in live PR/review coordination evidence rather than copying it into this versioned baseline. Sibling or predecessor GREEN never transfers into #2040, #35, or #46.

## Required order

Foundation security/identity owners must first reach accepted protected-main authority without weakening gates: #2279 must obtain exact-head terminal hosted evidence and qualifying review before its repair is treated as accepted; #2271/#2268 remain their respective owner lanes and must be ordinary-forward reconciled rather than copied from mutable heads. #2040 then completes ordinary/non-force protected-main path-wise scheduler reconciliation against the accepted foundation, preserving applicable repaired owner slices, followed by fresh exact-head scheduler/required GREEN and qualifying non-self approval. Only then may #35 obtain fresh compatible acceptance/normal landing. #46 still requires final Rust 1.98/coverage GREEN and the bounded PostgreSQL 18 same-generation differential including readable complete ACL semantics, exact dangling-role recovery evidence, complete routine Debug redaction, and receipt-bound initial-privilege validation, followed only by independently proven semantic-gap work, complete ordinary/non-force #46 adoption into #45, fresh #45 acceptance, #6 propagation, and immutable governed release from an accepted protected head.
