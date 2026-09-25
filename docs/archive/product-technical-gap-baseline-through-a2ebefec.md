# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-function transform-selection surface is preserved at `docs/archive/product-technical-gap-baseline-through-e2e257d1.md`; earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The ordinary-forward converter chain retains all previously established definition/owner/ACL/config/security/strictness/volatility/parallel/planner/cost/shape facts plus raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, and raw nullable converter-function `pg_proc.protrftypes`.

Fresh PostgreSQL 18 dependency and extension review established a separate governed lifecycle distinction after the transform-selection successor: the converter **function** can be associated with or disassociated from an extension while retaining the same function identity, body/language, exact input/return contract, owner, ACL/config/security/planner/cost facts, argument metadata, `protrftypes`, and `pg_transform` binding.

PostgreSQL records extension membership as a `pg_depend` edge with `deptype='e'`. `ALTER EXTENSION ... ADD FUNCTION` attaches an existing function to an extension and `ALTER EXTENSION ... DROP FUNCTION` disassociates it without dropping the function. A member object can only be dropped through its extension and is handled as extension-owned state by `pg_dump`. This changes lifecycle, upgrade and dump/restore semantics even when every already-observed converter fact is identical.

Converter-function extension-membership lineage: finding review `5252657912` on `e2e257d1c498a906dc844060d1e3b8cf2303967a` -> RED `ddc04f03c97ae78cccddefe34bd656aec32bd688` -> production observation/snapshot/receipt `30976b643aa96203a203705a75100e6374ff4977` -> public composition `64080cdc8188ca951f73faa95d6f9f656e6860dc` -> focused doctoring `441eadaa35d2fa8606a9c8a1bd9763cd517d2451`.

`IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot` is layered over the converter-function transform-selection snapshot. It preserves no membership separately from one exact resolved extension name, requires complete predecessor `(constraint, key_position, transform_type, direction)` coverage and exact converter schema/function binding, and domain-separates the successor digest. Blank extension names fail closed rather than becoming absence.

The PostgreSQL extractor contract is stricter than application metadata: it must use the exact converter function OID already selected by the same-generation predecessor, inspect `pg_depend` for an extension-membership edge from `pg_proc` to `pg_extension`, and resolve the referenced extension OID to exact `pg_extension.extname` in the same source snapshot. Zero edges means absence; an unsupported multiple-edge state must fail closed rather than be arbitrarily collapsed.

This successor deliberately does **not** copy extension-owned truth such as `extversion`, extension configuration tables, control-file metadata or update scripts. It records only the membership edge. PostgreSQL 18 also permits `ALTER EXTENSION ... ADD/DROP TRANSFORM FOR type LANGUAGE language`; that is a separate membership edge for the `pg_transform` object itself and must not be inferred from converter-function membership.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: ALTER EXTENSION*. https://www.postgresql.org/docs/18/sql-alterextension.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: 36.17. Packaging related objects into an extension*. https://www.postgresql.org/docs/18/extend-extensions.html

## Observation versus governance

Source Observation records the exact external database state before policy. It does not infer extension membership from a package name, function namespace, extension installation list, application metadata, or the transform object's membership. Validation/publish policy may later reject an unexpected member/standalone state, but it must reason over the observed edge instead of rewriting it.

The same rule continues to apply to mutable function state. Raw post-creation `provolatile='v'` remains observable drift evidence even though a volatile function is not admissible when creating a fresh transform. Observation and admission remain separate boundaries.

## Acceptance boundary

**Source repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter to one exact-generation `pg_proc` row, preserve all predecessor facts, capture raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, and then resolve converter-function extension membership from `pg_depend` -> `pg_extension` in that same generation. Reconstruction from extension naming conventions, language packages, transform-object membership, application metadata, or another source generation is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

Fresh review still does not justify field-by-field mirroring of `pg_proc`. `proallargtypes` remains constrained by the exact one-`internal` input, direction-specific return type and already-preserved argument modes; defaults, `prorows`, and variadic metadata require proof of an independent governed effect before they can become successors.

The newly proven extension edge is semantic rather than enumerative. The next candidate is correspondingly narrow: PostgreSQL exposes the **transform object itself** as an `ALTER EXTENSION ... ADD/DROP TRANSFORM FOR type LANGUAGE language` member object. A fresh review must determine whether transform-object extension membership is already represented elsewhere in the relation/index Source Observation chain and, if not, whether its independent drop/update/dump lifecycle warrants a separate successor. Converter-function membership must not be used as a proxy.

If no additional independent distinction survives that review, stop extending this converter sub-chain and move to exact-head acceptance and same-generation PostgreSQL 18 differential evidence.

## Canonical prerequisite state

Central `.github#2040` remains a separate canonical workflow prerequisite. Its last verified authority before this source movement was `12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`. It requires ordinary/non-force path-wise reconciliation of the shared scheduler while preserving its v2 producer, no-restamp behavior, repository-scoped credential proof, stale-run revalidation, rationale/tests, compatible current-main queue/coalescing/capacity behavior, and stronger repository-identity invariant. The ending fresh sweeps, not this paragraph, determine whether those coordinates remain current.

Queue-health #2268's SAST failure was traced to shared dynamic-urllib sinks; canonical sink repair is #2272, not a duplicate queue-health patch. #2272's separate Agent Review Runtime Quality failure is tracked by owner issue #2277. Sibling central evidence does not transfer into #2040 or #46.

Product bootstrap #35 remains a separate prerequisite; its previously observed successful security lanes do not override terminal CodeQL failure evidence.

## Required order

fresh central prerequisite verification -> `.github#2040` causal reconciliation/repair if still current -> fresh central exact-head GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> exact #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including converter-function extension membership -> fresh transform-object extension-membership review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.