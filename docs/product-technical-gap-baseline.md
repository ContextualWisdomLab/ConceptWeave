# Product / Technical Gap Baseline

**Snapshot:** 2026-09-20

This file records stable ConceptWeave product/technical authority. Mutable sibling PR heads, workflow run IDs, mergeability, and check states are live coordination evidence and are deliberately not versioned here. Historical decision surfaces remain under `docs/archive/`; focused PostgreSQL rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interoperability contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The `pg_init_privs` successor is recovery evidence distinct from current `pg_proc.proacl`. It preserves row absence versus presence, exact `privtype`, complete object-level initial EXECUTE ACL, source-array order, and multiplicity. Existing catalog damage can leave ACL role OIDs dangling; Source Observation preserves those nonzero OIDs rather than dropping entries or converting OIDs to role-name strings.

Resolved role identity remains `(role_oid, role_name)` from one source generation. Review `5256559272` on pre-finding exact head `a82dced2a154e1ff105864309a74559255898bbc` found that successful role lookup had collapsed identity to name alone even though PostgreSQL `AclItem` stores grantee/grantor as OIDs and `pg_authid` stores OID and `rolname` separately. Structural RED `b1c8fef6a526b22566b65d8f7962c3af0ebd2031` and production repair `52e1c12d5e592e624d8d55badbdc24c52b428a00` retain exact resolved OIDs privately, move material framing to `.material.v2`, and commit both OID and name without exposing numeric resolved OIDs through routine API or `Debug`.

A separate source-fidelity defect was found at review `5256811598` on exact pre-finding head `69315444d91690886d165a2a9dbde2b193e786ca`. This module used `str::trim().is_empty()` for resolved ACL role names and converter schema/function names. PostgreSQL 18 defines role names by SQL identifier rules; delimited identifiers may contain arbitrary characters except NUL and can contain spaces. The trim-based predicate therefore rejected legal catalog identifiers such as a quoted whitespace-only role, schema, or function name before immutable observation.

Structural RED `650a77aed375798ac2b241bc93ab3dc7cd5e328e` changes the retained contract to accept whitespace-only quoted identifier content while preserving it byte-for-byte, and still requires rejection of zero-length identifiers and NUL content. Production repair `74aa0c47f8ec30ccc824831383e7c6dcf0b09e99` replaces trim-based validation in that bounded module with PostgreSQL-identifier validation. ACL OID identity, source order, multiplicity, recovery evidence, and digest semantics are otherwise unchanged.

Routine grant/material diagnostics remain privacy-conscious. Resolved role names, PUBLIC/unresolved shape, grant option, counts, and immutable digest remain available; numeric resolved OIDs are private identity material and dangling OIDs remain exposed only by receipt-bound recovery-validation accessors. Exact OID identity participates in equality/order and digest identity without becoming routine product API. Source names are not trimmed or normalized: quoted PostgreSQL identifier content remains exact evidence.

Earlier repairs remain in force: complete material readback, source-order preservation, repeated ACLITEM multiplicity, dangling-role representability, canonical sorted/deduplicated remediation OID sets, non-forgeable receipt-bound recovery validation, exact material/provenance binding, and raw-OID Debug redaction. Derived remediation sets may sort/deduplicate dangling OIDs, but they never reorder or deduplicate the source ACL.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-integrity.md`; damaged-role and recovery-validation details remain in the neighboring dangling-role and recovery-validation doctoring documents.

### Source-repaired systemic PostgreSQL identifier boundary

Review `5257069180` on predecessor exact `427587e8be39d38240e57a2ebcc9cdaf291f6ebe` established that the converter-specific repair did not cover the shared Source Observation representation. `model::validate_nonblank()` uses `value.trim().is_empty()` and was consumed by identifier-bearing legacy/v3 constructors. That policy rejects legal quoted PostgreSQL identifiers whose exact catalog content consists only of whitespace.

Systemic structural RED `a1638573b9d99f78c7fcd211fc93559d86d4f93a` requires `QualifiedTypeName`, `QualifiedCollationName`, `QualifiedOperatorClassName`, and `ColumnObservationV3` identifier coordinates to preserve whitespace-only quoted content byte-for-byte and reject zero-length/NUL identifiers. Bounded RED `9395d141c70f074a1b60ff1c5d26097973db768a` plus production repair `c2b247b24c45f9e45c90aad7b4fb68e296d0225e` applies the exact identifier rule to `ColumnIdentityObservation` schema/relation/column coordinates.

Review `5257371727` found a second-order contract gap: the first systemic RED could be made green by repairing only the named core types while separately exported prefixed families continued to call the trim-based shared validator. Structural RED `3291d594da042d8497b1cc17493fc180f213f397` therefore adds `postgresql_identifier_fidelity_prefixed_contract.rs`. It requires `ColumnCollationObservation` and `ColumnExpressionObservation` schema/relation/column coordinates to preserve quoted whitespace byte-for-byte, reject empty/NUL identifiers, and simultaneously retain rejection of whitespace-only rendered expression text. The last assertion is a negative control against globally weakening generic nonblank-text validation.

Fresh review `5257607599` on exact predecessor `fe70b0c5077fb436c69d75e72502f12e4e209f80` reconfirmed the causal split between identifier coordinates and rendered expression text. Reusable identifier admission was exposed internally at `7de4d9b9072869b562ad34719241a774a170e903`. `ColumnCollationObservation` was repaired at `7961c0229938048442c7b1839230f136bb59fd04`; `ColumnExpressionObservation` was repaired at `a8ad7cf1430a13aaae33bdeff31b29885eaed06f` while preserving generic nonblank validation for the rendered expression payload.

The same source invariant was carried across generated-column coordinates (`d295661d0360b88a0da24ec1129b6a9aaf8b72e9`), constraint timing (`943518644b28d6255453ce574c90ddcef22a21c6`), and temporal-constraint plus exclusion-operator identifiers (`4bbe2ab1438a0949a7e994e3150c0b413737c74d`). Dedicated NOT NULL structural RED `7efd69164df643b2bf325207df70c93ffb6318dc` precedes production repair `6b032de91b034f9ab76541eeb1a02150dad1d1d1`, covering local constraints, resolved parent constraints, and partition-parent relation coordinates. Retained cross-family regression contract `263091310809b223881c2a54e1ee4f5fb761dbe5` covers generated-column, timing, and temporal-constraint coordinates after their repair.

The legacy shared model is repaired rather than merely listed as residual work. Structural RED `3136e841ed925deffa904f578acc4e46c80bb763` covers legacy column/table/constraint/foreign-key/location identifiers and includes negative controls for rendered data-type and CHECK-definition text. Production repair `b4068f5e78de4f5e76ca1eef70f17cce68b83c1c` routes only those identifier-bearing fields through `validate_postgresql_identifier`; the generic `validate_nonblank()` function remains unchanged. Review `5257904564` records the inspected predecessor finding after the commits were created and is explicitly not treated as evidence that review preceded the RED/fix sequence.

V3 catalog structural RED `5ed597aab1e0770e9702e766beeb78c3aa03c84d` extends exact identifier handling to qualified type/collation/operator-class names, v3 column/relation/domain/enum/index/location coordinates, domain constraint names, and simple index-column attributes. `IndexTablespace` contract movement `fe2d3f01efae5fdad03b39a785ffd096f5d91fdd` removes the false assumption that whitespace-only tablespace names are presentation-invalid and instead requires exact whitespace preservation plus empty/NUL rejection. V3 production repair `1f2c4470849dacbc06632c6fe87e59a9f6d9df87` moves those identifier-bearing fields to the PostgreSQL identifier boundary while rendered type text, reconstructed CHECK definitions, rendered index expressions/predicates, extractor revision, operator/storage option text, and other non-identifier metadata retain their field-specific policy. Ordinary-forward recovery `9aa5034c21328fd1dbe0b6d4d9e614ef6079d098` preserves the focused tree after a prior broad file write without rewriting history.

Residual structural RED `31639335a1c65583997917133bfe1f9cf6e6d4aa` then isolated index access-method admission: `RelationObservation::with_indexes()` still used trim-based blank detection even though access-method names are PostgreSQL identifiers. Production repair `ae86c5a885d5f5a02a6f19957485d397f44f1e3c` now requires access-method evidence and applies `validate_postgresql_identifier(access_method, "access_method")`; exact quoted-whitespace content is preserved while empty/NUL identifiers fail closed. Review `5259712533` verifies that the change is confined to the owning admission block and rustdoc and does not weaken generic nonblank validation.

The known systemic identifier defect is therefore **source-repaired but not exact-head GREEN**. The remaining work is acceptance, not another speculative identifier successor: one unchanged final head must execute the retained identifier contracts, workspace/doc tests, strict Rust gates, owned coverage, and PostgreSQL 18 same-generation differential. Any further semantic successor requires a newly demonstrated uncovered identifier-bearing field or an independent buyer/semantic/security/lifecycle/provenance distinction.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: ACL data structures, src/include/utils/acl.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/utils/acl.h
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 source: role catalog, src/include/catalog/pg_authid.h* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_authid.h
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 source: ACL validation/update/owner-change behavior, src/backend/utils/adt/acl.c* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/adt/acl.c
- PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 source: pg_dump initial-privilege capture, src/bin/pg_dump/pg_dump.c* [REL_18_STABLE]. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/bin/pg_dump/pg_dump.c
- PostgreSQL Global Development Group. (2026f). *PostgreSQL 18 documentation: 21.1. Database Roles*. https://www.postgresql.org/docs/18/database-roles.html
- PostgreSQL Global Development Group. (2026g). *PostgreSQL 18 documentation: 4.1. Lexical Structure*. https://www.postgresql.org/docs/18/sql-syntax-lexical.html
- PostgreSQL Global Development Group. (2026h). *PostgreSQL 18 documentation: CREATE TABLESPACE*. https://www.postgresql.org/docs/18/sql-createtablespace.html

## Observation versus governance

Source Observation records exact external database state before policy. A resolved role lookup enriches an ACLITEM with a readable role name; it does not replace the ACLITEM's OID identity. The immutable material therefore retains `(OID, name)` for resolved roles, raw OID for dangling roles, PUBLIC as the PostgreSQL grantee sentinel, source order, multiplicity, and grant option. Catalog identifier strings are preserved exactly rather than trimmed, case-folded, or otherwise normalized inside repaired boundaries. The known shared legacy/v3 identifier admission defect is now repaired in source; acceptance evidence remains outstanding.

Recovery validation consumes an owner-issued source receipt, retains complete non-secret provenance and exact purpose-bound dangling-role sets, and blocks readiness whenever either dangling-role set is non-empty. ConceptWeave validates and governs publication; PostgreSQL role/catalog remediation remains outside this bounded context.

## Acceptance boundary

**The current Source Observation head remains RED / not GREEN as an acceptance state, despite the source repair.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 same-generation differential. No predecessor execution result transfers after source or documentation movement.

The differential must resolve every selected converter and transform from one source generation and verify current function facts/ACL, extension lifecycle, complete security-label/dependency evidence, exact `pg_init_privs`, immutable raw converter-root lineage, and independent transform-object extension membership. For initial ACL and identifier evidence it must prove:

- row absence versus present material and exact `privtype`;
- exact source-array order and repeated-entry multiplicity;
- PUBLIC, resolved grantee/grantor names and grant-option readback;
- equal role names with different same-generation OIDs remain different material/source identity for both grantee and grantor;
- legal quoted identifier content, including whitespace-only schema/relation/table/column/constraint/reference/type/collation/operator-class/tablespace/access-method and converter role/schema/function names, survives observation byte-for-byte rather than being rejected or trimmed;
- separately exported identifier-bearing families do not escape that rule merely because their field labels or modules differ;
- zero-length and NUL identifiers remain rejected as impossible PostgreSQL identifier states;
- non-identifier text validation, including rendered expression, data-type and CHECK-definition text, remains unchanged by the identifier repair;
- resolved numeric OIDs do not appear in routine grant/material `Debug`;
- dangling grantee/grantor OIDs remain actionable only through recovery validation;
- repeated dangling identities canonicalize only in derived remediation sets;
- a constant-current-`proacl` / different-initial-baseline case changes initial-privilege identity.

Synthetic Rust fixtures are unit-contract evidence only and do not substitute for the PostgreSQL 18 differential. A source-neutral wake commit or unchanged-head manual rerun is not acceptable causal execution evidence.

## Residual material gap

Within the converter/transform `pg_init_privs` slice, complete ACL readability, exact OID/name identity, source order/multiplicity, exact quoted-identifier content, actionable dangling-role recovery identity, receipt binding, and routine-log safety are represented in source and focused contracts. Across wider Source Observation, the shared legacy model, separately exported catalog families, and `representation_v3.rs` identifier-bearing constructors including tablespace and access-method identity are now source-repaired. The residual gap is exact-head execution/coverage/PostgreSQL differential acceptance rather than a known unpatched identifier call site.

A next semantic successor requires an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction not represented by the current chain. Otherwise, after exact-head acceptance, this sub-chain moves to complete adoption and release preparation.

## Canonical prerequisite state

The protected reusable-workflow authority remains `ContextualWisdomLab/.github@main` and must be read fresh before any ConceptWeave landing. ConceptWeave versions only the stable owner boundaries:

- `.github#2279` owns the shared GitHub API initial-URL and redirect-authority boundary and is already part of protected-main foundation.
- `.github#2271` owns CodeQL dispatch repository-identity admission.
- `.github#2268` owns queue-health repository-identity admission.
- `.github#2040` owns the protected-main scheduler/credential/settlement path and must preserve accepted foundation behavior during ordinary/non-force reconciliation.

Immediately before downstream acceptance, reread each owner PR's exact head, review/thread state, workflow inventory, mergeability, and relationship to protected `.github/main`. Volatile observations belong in live PR/review coordination evidence, not this versioned baseline. Sibling or predecessor GREEN never transfers into #2040, #35, or #46.

## Required order

#2271/#2268 must reach exact-head hosted acceptance and qualifying review on the accepted protected foundation. #2040 then requires its own exact-head hosted GREEN and qualifying independent approval. Only then may #35 obtain fresh compatible acceptance/normal landing. In parallel, #46's known PostgreSQL identifier source repair is complete through `ae86c5a...`; its final exact head must now obtain Rust/coverage GREEN plus the PostgreSQL 18 same-generation differential including resolved `(OID, name)` identity, source-order/multiplicity semantics, exact identifier fidelity including access methods, dangling-role recovery evidence, `Debug` redaction, and receipt-bound validation. After prerequisite and exact-head acceptance converge, only independently proven semantic-gap work may extend the chain; otherwise #46 is completely ordinary/non-force adopted into #45, #45 obtains fresh acceptance, #6 propagates the accepted authority, and immutable governed release proceeds from an accepted protected head.
