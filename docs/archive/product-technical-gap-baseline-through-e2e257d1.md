# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. Earlier surfaces remain under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The valid ordinary-forward converter chain now preserves raw nullable `pg_proc.proargmodes` and `pg_proc.proargnames` after the previously established converter definition/owner/ACL/config/security/strictness/volatility/parallel/planner/cost/shape facts. The argument-mode lineage is RED `6ecc6418fce88b3aaf1519e7c422b2a1583b5407` -> production `39602ea78c4bbff856c6274052918c4c0541e9df` -> public `a3c61e33fbd84c86e9167d17377072df3e0e6ec0` -> doctoring `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`. The argument-name lineage is review `5252092881` -> RED `2ccf80dd19f7710d14512f2b184d8551142ad4d5` -> production `653aa476578cb3e75bf19103c37ffec4e17e9a5e` -> public `bbcf388dfd01323da86b25df300ba112be4dc2cb` -> decision-surface head `3dc7c81c8432f370b54aeac8375482d88b99a0cc`.

Fresh PostgreSQL 18 catalog/function review then established a separate runtime-semantic lossiness: each converter function's own nullable `pg_proc.protrftypes` was not represented. PostgreSQL stores this independently from the converter binding and defines `CREATE FUNCTION ... TRANSFORM FOR TYPE ...` as the set of SQL/procedural-language transforms a call to that function applies. If a language does not know a type and no transform is selected, fallback conversion behavior is language-specific. A same-identity converter can therefore retain schema/name, input/return types, implementation language/body and `pg_transform` binding while its function-call transform-selection semantics change under `CREATE OR REPLACE FUNCTION`.

Converter-function transform-selection lineage: finding review `5252233297` on `3dc7c81c8432f370b54aeac8375482d88b99a0cc` -> RED `20be74cacfce08778d00cb767bf05379eeba6626` -> production observation/snapshot/receipt `3e97370dc221738b0d408ed2a19be32f01ebff5c` -> public composition `303677ee2e1807aae883f0b752b897dbf2c23f01` -> focused doctoring `ff4aa7bf52e0479cb9d9f5538273bc25deac919a`.

`IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot` is an ordinary-forward successor of the argument-name snapshot. It preserves NULL separately from a nonempty selected-type set, resolves selected OIDs to qualified types in the same source generation, sorts only for deterministic set identity, rejects duplicate/empty sets, and requires exact predecessor coordinate coverage and converter schema/function binding. The digest is domain-separated and includes every selected qualified type exactly.

This successor deliberately does not recursively inline the selected types' mutable `pg_transform` rows. Those rows remain represented by the existing transform binding surface; the new fact is only the converter function's own `protrftypes` selection.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains: converter definition and exact input/return types, implementation language/body digest, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, raw `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, exact one-`pg_catalog.internal` input boundary, raw nullable `proargmodes`, raw nullable `proargnames`, and now raw nullable converter-function `protrftypes`.

## Observation versus admission

The raw catalog layer does not normalize away post-creation drift. PostgreSQL 18 `CREATE TRANSFORM` has structural and volatility admission constraints, but function properties can later change without replacing the transform binding. Raw `provolatile='v'` therefore remains observable drift evidence even though a volatile converter is not admissible for a fresh transform.

Argument names and function transform-selection follow the same owner boundary. Source Observation records the exact catalog state. It does not infer argument names from signatures, infer `protrftypes` from implementation language/return type, or recursively claim ownership of the selected types' `pg_transform` rows. Validation/publish policy may reason over these facts later without changing what was observed.

Primary authority:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html
- PostgreSQL source `check_transform_function()` remains the structural fresh-transform authority already traced in prior doctoring; it checks volatility/kind/set-return/input count/internal input while direction-specific return type is checked by `CreateTransform`.

## Acceptance boundary

**Source repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter to one exact-generation `pg_proc` row and capture all predecessor facts plus raw nullable `proargmodes`, raw nullable `proargnames`, and raw nullable converter-function `protrftypes` from that same row. Non-NULL `protrftypes` OIDs must be resolved to qualified types inside the same source snapshot. Reconstructing selected transforms from function language/body, target-function transform settings, return type, application metadata, or another generation is a capture failure.

No pull-request workflow run has established native or hosted GREEN on the current post-repair generation. A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

Fresh review did not justify blindly adding `proallargtypes`: under the current converter invariants, its positional shape is constrained by the one exact `internal` input, direction-specific return type and already-preserved `proargmodes`. Likewise defaults are not automatically promoted merely because `pg_proc` contains them; a successor requires evidence of an independent governed distinction in transform execution or governance.

The next material candidates are therefore semantic, not enumerative: verify whether any remaining converter-function property changes actual converter invocation or governed security/operability while escaping the current definition/config/ACL/shape/transform-selection chain. If no such distinction is found, stop extending the converter sub-chain and move to exact-head acceptance/differential evidence instead of mirroring the catalog field-by-field.

## Canonical prerequisite state

Central `.github#2040` remains exact `12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` on the latest sweeps. Its next valid movement remains ordinary/non-force path-wise reconciliation of the shared scheduler while preserving its owned v2 producer, no-restamp behavior, repository-scoped credential proof, stale-run revalidation, rationale/tests, compatible current-main queue/coalescing/capacity behavior, and the stronger repository-identity invariant.

Queue-health #2268's terminal SAST failure was traced to two shared dynamic-urllib sinks; canonical sink repair is #2272, not a duplicate queue-health patch. #2272's own Agent Review Runtime Quality run is separately terminal-failing on a response context-manager contract mismatch; owner issue #2277 records the causal RED->GREEN path. These sibling central results do not transfer into #2040.

Product bootstrap #35 remains a separate prerequisite and its prior successful Security/SAST lanes do not override terminal CodeQL failure evidence.

## Required order

`.github#2040` path-wise reconciliation plus repository-identity repair -> fresh exact-head central GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> exact #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 differential including raw `proargmodes`, `proargnames`, and converter-function `protrftypes` -> fresh semantic-gap review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.