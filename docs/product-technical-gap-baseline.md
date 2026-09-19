# Product / Technical Gap Baseline

**Snapshot:** 2026-09-20

This file records stable ConceptWeave product/technical authority. Mutable sibling PR heads, workflow run IDs, mergeability, and check states are live coordination evidence and are deliberately not versioned here. Historical decision surfaces remain under `docs/archive/`; focused PostgreSQL rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interoperability contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The `pg_init_privs` successor is recovery evidence distinct from current `pg_proc.proacl`. It preserves row absence versus presence, exact `privtype`, complete object-level initial EXECUTE ACL, source-array order, and multiplicity. Existing catalog damage can leave ACL role OIDs dangling; Source Observation preserves those nonzero OIDs rather than dropping entries or converting OIDs to role-name strings.

A further lossless-identity defect was found at review `5256559272` on pre-finding exact head `a82dced2a154e1ff105864309a74559255898bbc`. PostgreSQL 18 `AclItem` stores `ai_grantee` and `ai_grantor` as OIDs, while `pg_authid` stores role OID and `rolname` separately. The previous domain model retained raw OIDs only when lookup failed; successful lookup collapsed identity to the role name. A role dropped and recreated under the same name could therefore alias the earlier ACLITEM in the material digest.

Structural RED `b1c8fef6a526b22566b65d8f7962c3af0ebd2031` requires equal-name/different-OID grantee and grantor states to produce different material digests while routine diagnostics remain OID-redacted. It is source-level structural RED evidence, not an executed failing CI result.

Production repair `52e1c12d5e592e624d8d55badbdc24c52b428a00` retains exact resolved OIDs privately alongside readable role names, validates that resolved role OIDs are nonzero, and moves the material digest framing to `.material.v2`. A resolved grantee/grantor commits both OID and name; PUBLIC and unresolved variants remain separately tagged. Test adoption `2780f5edb6c086a3d13b0dc47d6ca4b22963eec9`, dangling-role adaptation `a7274b7de87f904b8aae7fbdf0208ed8b3066d36`, and recovery-fixture adaptation `8ba4e304fa4b24cb4e49ae31474985b400aeddc7` move retained contracts to the exact-OID constructors.

Routine grant/material diagnostics remain privacy-conscious. Resolved role names, PUBLIC/unresolved shape, grant option, counts, and immutable digest remain available; numeric resolved OIDs are private identity material and dangling OIDs remain exposed only by receipt-bound recovery-validation accessors. Exact OID identity participates in equality/order and digest identity without becoming routine product API.

Earlier repairs remain in force: complete material readback, source-order preservation, repeated ACLITEM multiplicity, dangling-role representability, canonical sorted/deduplicated remediation OID sets, non-forgeable receipt-bound recovery validation, exact material/provenance binding, and raw-OID Debug redaction. Derived remediation sets may sort/deduplicate dangling OIDs, but they never reorder or deduplicate the source ACL.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-integrity.md`; damaged-role and recovery-validation details remain in the neighboring dangling-role and recovery-validation doctoring documents.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: ACL data structures, src/include/utils/acl.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/utils/acl.h
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 source: role catalog, src/include/catalog/pg_authid.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_authid.h
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 source: ACL validation/update/owner-change behavior, src/backend/utils/adt/acl.c* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/adt/acl.c
- PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 source: pg_dump initial-privilege capture, src/bin/pg_dump/pg_dump.c* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/bin/pg_dump/pg_dump.c

## Observation versus governance

Source Observation records exact external database state before policy. A resolved role lookup enriches an ACLITEM with a readable role name; it does not replace the ACLITEM's OID identity. The immutable material therefore retains `(OID, name)` for resolved roles, raw OID for dangling roles, PUBLIC as the PostgreSQL grantee sentinel, source order, multiplicity, and grant option.

Recovery validation consumes an owner-issued source receipt, retains complete non-secret provenance and exact purpose-bound dangling-role sets, and blocks readiness whenever either dangling-role set is non-empty. ConceptWeave validates and governs publication; PostgreSQL role/catalog remediation remains outside this bounded context.

## Acceptance boundary

**Source and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential. No predecessor execution result transfers after source or documentation movement.

The differential must resolve every selected converter and transform from one source generation and verify current function facts/ACL, extension lifecycle, complete security-label/dependency evidence, exact `pg_init_privs`, immutable raw converter-root lineage, and independent transform-object extension membership. For initial ACL evidence it must prove:

- row absence versus present material and exact `privtype`;
- exact source-array order and repeated-entry multiplicity;
- PUBLIC, resolved grantee/grantor names and grant-option readback;
- equal role names with different same-generation OIDs remain different material/source identity for both grantee and grantor;
- resolved numeric OIDs do not appear in routine grant/material Debug;
- dangling grantee/grantor OIDs remain actionable only through recovery validation;
- repeated dangling identities canonicalize only in derived remediation sets;
- a constant-current-`proacl` / different-initial-baseline case changes initial-privilege identity.

Synthetic Rust fixtures are unit-contract evidence only and do not substitute for the PostgreSQL 18 differential. A source-neutral wake commit or unchanged-head manual rerun is not acceptable causal execution evidence.

## Residual material gap

Complete ACL readability, exact OID/name identity, source order/multiplicity, actionable dangling-role recovery identity, receipt binding, and routine-log safety are now represented in source and focused contracts. No further converter/transform successor is authorized merely because another catalog field or metadata row exists. A next semantic successor requires an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction not represented by the current chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 differential.

## Canonical prerequisite state

The protected reusable-workflow authority remains `ContextualWisdomLab/.github@main` and must be read fresh before any ConceptWeave landing. ConceptWeave versions only the stable owner boundaries:

- `.github#2279` owns the shared GitHub API initial-URL and redirect-authority boundary and is already part of protected-main foundation.
- `.github#2271` owns CodeQL dispatch repository-identity admission.
- `.github#2268` owns queue-health repository-identity admission.
- `.github#2040` owns the protected-main scheduler/credential/settlement path and must preserve accepted foundation behavior during ordinary/non-force reconciliation.

Immediately before downstream acceptance, reread each owner PR's exact head, review/thread state, workflow inventory, mergeability, and relationship to protected `.github/main`. Volatile observations belong in live PR/review coordination evidence, not this versioned baseline. Sibling or predecessor GREEN never transfers into #2040, #35, or #46.

## Required order

#2271/#2268 must reach exact-head hosted acceptance and qualifying review on the accepted protected foundation. #2040 then requires its own exact-head hosted GREEN and qualifying independent approval. Only then may #35 obtain fresh compatible acceptance/normal landing. #46 requires final Rust/coverage GREEN plus the PostgreSQL 18 same-generation differential including resolved `(OID, name)` identity, source-order/multiplicity semantics, dangling-role recovery evidence, Debug redaction, and receipt-bound validation. After that, only independently proven semantic-gap work may extend the chain; otherwise #46 is completely ordinary/non-force adopted into #45, #45 obtains fresh acceptance, #6 propagates the accepted authority, and immutable governed release proceeds from an accepted protected head.
