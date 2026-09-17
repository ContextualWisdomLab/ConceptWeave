# Product / Technical Gap Baseline

**Snapshot:** 2026-09-18

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding semantic surface through exact `4256f8dc361ae8ed00980799d524a261b161b1a8` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-4256f8dc.md`; its matching CHANGELOG is preserved at `docs/archive/CHANGELOG-through-4256f8dc.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution/review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs remain authoritative; no issued digest domain is rewritten. Ordinary `pg_constraint.contype='x'` evidence retains independent constraint/backing-index identity, ordered `conkey`/`conexclop`, operator kind/commutator/result/procedure binding, target-function scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, nonzero converter definition identity, converter owner identity, converter object-level `EXECUTE` ACL identity, converter nullable `proconfig` identity, converter raw `prosecdef` identity, converter raw `proleakproof` identity, operator-family/strategy, namespace/name/lifecycle/access-method controls, and exact v3 source-content-generation binding. Temporal `WITHOUT OVERLAPS`/`PERIOD` families remain separate.

## EXCLUDE transform-converter leakproof integrity

Review `5235830453` on exact predecessor `1153c0b6f121b5b99c533d73e3dc9e7b4305c276` found that every nonzero converter was definition-, owner-, ACL-, configuration-, and security-context-bound while raw same-row `pg_proc.proleakproof` remained independently mutable and ungoverned. PostgreSQL 18 permits `ALTER FUNCTION ... [NOT] LEAKPROOF` without changing those predecessor facts, and leakproof classification affects security-barrier/RLS evaluation ordering and planner-statistics access.

The ordinary-forward successor adds `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation`, provenance receipt, and `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot`. Both Boolean states are representable; the snapshot is observational rather than a policy rule. It takes `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot` as its immutable direct predecessor and requires exactly one raw Boolean observation for every exact `(constraint, key_position, transform_type, direction)` converter coordinate with the same converter schema/function binding. Missing/extra directions, duplicate coordinates, zero positions, binding drift, and unknown receipt coordinates fail closed.

### Provenance-location P1 repair

Review `5236499737` on exact `4256f8dc361ae8ed00980799d524a261b161b1a8` found that the leakproof digest was collision-safe but its claimed collision-safe `canonical_location()` was not. `QualifiedTypeName` preserves exact quoted PostgreSQL identifiers, while raw `schema.type` serialization collapsed valid pairs such as `(payload.domain, json)` and `(payload, domain.json)` into the same path text. PostgreSQL 18 permits quoted identifiers to contain arbitrary characters except zero, so dots, slashes, percent signs, whitespace, and Unicode cannot be assumed absent.

Realistic RED `008c99a2e82105046df0b9ed7061bc6cb931ebb9` adds a focused collision regression. Minimum causal fix `3ce7e0bcf6d6a3a0935465147e2eaf5fd3a63e81` percent-encodes the transform schema and type components independently before the dot separator; ordinary ASCII identifier characters stay readable while every separator-capable/special UTF-8 byte is `%HH` encoded. This changes only provenance/error string identity, not the leakproof digest domain or typed lookup key. The pre-repair decision surface is preserved at `docs/archive/*through-4256f8dc.md`.

## PostgreSQL 18 and governance authority

Primary authority is PostgreSQL 18 `CREATE FUNCTION`, `ALTER FUNCTION`, lexical structure, row-security, rules/privileges, and planner-statistics security documentation. PostgreSQL explicitly allows changing leakproof classification and permits quoted identifiers containing arbitrary characters except code zero. NIST control-assessment guidance supports preserving independently testable security-relevant evidence, but PostgreSQL remains the technical authority for catalog and identifier semantics.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- leakproof finding review: `5235830453`
- location finding review: `5236499737`
- location RED: `008c99a2e82105046df0b9ed7061bc6cb931ebb9`
- location fix: `3ce7e0bcf6d6a3a0935465147e2eaf5fd3a63e81`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_leakproof.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_leakproof_contract.rs`
- direct predecessor: `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot`
- exact catalog fact: each nonzero selected converter function's raw same-row `pg_proc.proleakproof`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 is not available in the current execution host, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned statement/branch/edge coverage remain unexecuted. The location RED is logically executable against the predecessor because both valid quoted type coordinates serialized to the same string, but no native test execution is claimed. One unchanged exact head must obtain native and hosted terminal acceptance before any differential or publication claim.

The bounded PostgreSQL 18 live differential must retain every predecessor ordinary-EXCLUDE fact, resolve each selected `(trftype, target prolang)` `pg_transform` row, resolve every nonzero converter OID to the exact same-generation converter `pg_proc`, and independently capture converter definition, raw `proowner` plus role resolution, raw `proacl` NULL-state plus canonical object-level `EXECUTE` grants, raw nullable `proconfig`, raw `prosecdef`, and raw `proleakproof`. Missing converter/leakproof/security-context/configuration/role/ACL resolution, mixed-generation joins, or inferred Boolean state is capture failure rather than an unknown placeholder.

## Residual material gap

Converter definition, owner, ACL, local configuration, execution security context, leakproof classification, and collision-safe leakproof provenance location are source-bound, but complete converter runtime/planner identity is not claimed. Converter strictness, volatility, parallel safety, planner support/cost and other independently mutable `pg_proc` facts remain later reviewed successors. No next auxiliary field is promoted into source until this successor obtains exact-head native/hosted acceptance and a bounded PostgreSQL 18 differential, avoiding unchecked accumulation of validation debt.

## Canonical prerequisite state

Canonical `.github#2106` has merged normally and is no longer the open workflow-owner prerequisite. The active central owner path is `.github#2040`.

Fresh owner authority on 2026-09-18 is:

- protected `ContextualWisdomLab/.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`;
- `.github#2040@609be40b7be3a53ac5a8baf2b48b3af5ad7da237`, OPEN / Draft / non-mergeable;
- compare status `diverged`, 163 ahead / 244 behind, merge base `fb17ef556f94f673234aa557254ae52779e9a7b0`;
- no qualifying independent current-head approval and no exact-head hosted terminal GREEN.

The earlier producer protocol RED `cab6cdad2683ce7887b62ad089f93e0317f70602` remains valid. Focused source repair `5edd9cc8033bd9c4327a67ffa6769e397f875f84` changes the central producer to `codeql-scan-v2` and adds the schema-1 nested `pr_head` envelope while retaining flat head identity, producer-source SHA, base-bound receipt/SARIF authority, required run/job identity, rerun semantics and the target-App credential boundary.

Owner review `5238816675` identifies another valid delta that must survive ordinary/non-force current-base reconciliation: #2040 commit `a9b18b4b24980c7ceb8b8cc0d143a24db20c90bf` removes source-neutral same-tree scheduler restamps for last-push approval and zero-job Actions startup failure. `tests/test_pr_review_merge_scheduler_source_neutral_commit_contract.py` pins both cases. Adopting protected changes must not reintroduce wake/no-op commits or purpose-complete self-modifying workflow behavior.

Fresh current-base review `5239422933` found a separate inherited private-consumer permissions gap on #2040 predecessor `b1542f0e21d6709d273af04b80f595a7d0fcdaf7`. `analyze-head` and `dispatch-current-head` read live target PR metadata and target commit statuses, but the predecessor job tokens omitted `pull-requests: read` and `statuses: read`; protected main already carried both grants after a live private-consumer HTTP 403. Structural RED `2a7e1252a57fd2aee934bb9c531b4cbecfc49f66` pins those job-scoped grants and forbids `actions: write`. Minimal production repair `9f435af94de5e02897b8d132105dea5f03b33547` adds only the four missing read-grant lines, preserving all v2 producer, base-bound receipt/direct-evidence, rerun/job-set, OIDC/app-token and no-source-neutral-restamp behavior.

The current remaining repository-identity P1 is deterministic at `609be40b...`. Protected main carries #2217's stricter `GITHUB_REPOSITORY_RE`, which rejects repository owner/name components containing `..` or ending in `.` before interpolation into GitHub API paths. This branch still has the older permissive production regex, while `tests/test_pr_review_merge_scheduler_repository_identity_contract.py` on the same exact head rejects `ContextualWisdomLab/..`, `ContextualWisdomLab/repository.`, `ContextualWisdomLab./repository`, and `../repository`.

This run attempted that one-line repair in ordinary child `d278f972a018f813d11a5684d4ba16c32bb6f45c`. Exact commit review found that the available complete-file write also removed non-obvious scheduler rationale comments concerning coverage retry, stale auto-merge authorization, in-flight-check restart avoidance, per-invocation cache reset, stacked-PR ordering and shared rate-limit propagation. Because that was not a minimum causal change, ordinary-forward repair `609be40b7be3a53ac5a8baf2b48b3af5ad7da237` restores the predecessor scheduler-core blob byte-for-byte without ref rewind or force update. Current-head owner review `5241404620` records the failed-minimality check and persistent RED. A subsequent repair must be hunk-safe or part of a reviewed path-wise protected-main reconciliation; no source GREEN transfers from `d278f972...`.

Exact `.github#2040@609be40b...` currently has no PR-triggered workflow run generation and no qualifying independent current-head `APPROVED` review. No owner GREEN is claimed.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN / Ready / mergeable. SAST Semgrep `34434790777` and Security Scan `34434790791` are terminal success, but CodeQL PR `34434790860` is terminal failure. Its language-detection job succeeded; the actions compatibility consumer later enforced `DISPATCH_OUTCOME=success` with `VERDICT_STATE=pending` and failed before the separate dispatch job completed successfully. This remains central owner-settlement evidence, not a new product-source defect, and does not authorize a blind/manual rerun or predecessor-success transfer.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_LOCATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_LEAKPROOF_DIFFERENTIAL_OPEN / CANONICAL_WORKFLOW_OWNER_CURRENT_BASE_RECONCILIATION_OPEN / CANONICAL_WORKFLOW_OWNER_REPOSITORY_IDENTITY_RED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / PRODUCT_BOOTSTRAP_CODEQL_ACCEPTANCE_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted. Version/tag/package/immutable semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Minimal ordinary-forward repository-identity repair on `.github#2040@609be40b7be3a53ac5a8baf2b48b3af5ad7da237` without losing the v2 producer, no-source-neutral-restamp, private-consumer grants or repository-scoped credential deltas -> remaining path-wise ordinary/non-force reconciliation with protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` -> fresh exact-head terminal applicable checks plus qualifying independent approval -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including converter `proleakproof` and quoted-identifier location regression -> bounded PostgreSQL 18 live differential including raw converter `proleakproof` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> review remaining converter auxiliary `pg_proc` surfaces -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, source-neutral wake/no-op commits, blind/manual reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
