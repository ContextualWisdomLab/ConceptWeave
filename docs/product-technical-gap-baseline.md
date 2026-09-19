# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before converter-function security-label evidence is preserved at `docs/archive/product-technical-gap-baseline-through-106cfb3f.md`; its matching changelog is `docs/archive/CHANGELOG-through-106cfb3f.md`. Earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, and transform-object extension membership.

Fresh PostgreSQL 18 review found a separate security distinction that survives all of those facts. `SECURITY LABEL` permits an arbitrary number of labels on a database object, one per label provider. `pg_seclabel` stores exact provider/label text. PostgreSQL delegates label validity and meaning to the provider, and `sepgsql` demonstrates that function labels can participate in mandatory-access-control decisions and can determine trusted-procedure behavior. Therefore two converter functions can have identical owner, EXECUTE ACL, `SECURITY DEFINER`, leakproofness, extension lifecycle, function definition, and transform binding while differing materially in authorization behavior.

Finding review `5254227769` pinned that lossiness at exact pre-finding head `106cfb3f1278afe12f7f126b4458a21649745e05`. Structural RED `49059a31e9445f364fca190dba5b049a440e78e1` referenced the public security-label API before that API existed. Production `58abc98906919ca803adb4bbe3f197df61ffc37c` added `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel`, observation, immutable receipt, and snapshot. Public composition followed at `0b1a513c6c4ca50353160b986e70a088381360e6`.

Each converter direction now carries the complete same-generation security-label map for the exact converter function. Capture is from `pg_seclabel` rows for `classoid=pg_proc`, exact function `objoid`, and `objsubid=0`. Empty and populated maps remain distinct. Provider ordering is canonicalized because catalog row order is not semantic, but provider text and label text remain exact. Provider identity must be nonblank and unique per function. Label text is deliberately not trimmed, parsed, or normalized because the loaded provider owns validity and semantics.

The successor preserves complete converter coordinates, direction, exact schema/function binding, source generation metadata, and the immutable raw `converter_snapshot_digest`. Missing/extra/duplicate converter coordinates, binding drift, duplicate providers, blank provider identity, zero key position, unknown receipt coordinates, and qualified-type provenance collisions fail closed.

Transform-object extension membership was ordinary-forward restacked at `03d434c88e1251d6d28977be4805738c374d183b` so the security-label snapshot is now its digest predecessor. Its retained contract was restacked at `e764a73751bb88fd362fbc5af4948691d8dd02aa`, and the same-generation direction/function/raw-root hostile lineage contract at `c52fce2d4ea782e83bae4b31636335ad8c417a06`. The transform-object successor still receives the separately supplied raw transform-converter snapshot and first proves equal source generation plus exact immutable raw converter root before validating complete FROM SQL / TO SQL cardinality and exact converter schema/function identity. A change in converter security-label state therefore changes the later transform-object lifecycle digest instead of disappearing from the final chain.

Test correction `f4c0ce73237062b2a28b2f0ee69731d72ab6853b` ensures duplicate-provider rejection is exercised at the observation constructor boundary rather than hidden by a helper unwrap. Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-security-label-integrity.md`.

Converter-function `deptype='e'`, converter-function `deptype='x'`, converter-function security labels, and transform-object `deptype='e'` remain distinct facts. ConceptWeave does not infer one from another. Provider policy, SELinux policy, extension-owned `extversion`, configuration, control/update scripts, package inventory, and application metadata remain outside ConceptWeave.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: SECURITY LABEL*. https://www.postgresql.org/docs/18/sql-security-label.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.46. pg_seclabel*. https://www.postgresql.org/docs/18/catalog-pg-seclabel.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: F.40. sepgsql — SELinux-, label-based mandatory access control (MAC) security module*. https://www.postgresql.org/docs/18/sepgsql.html
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html
- PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html

## Observation versus governance

Source Observation records exact external database state before policy. Validation or publication may later reject a label, require a known provider, or impose organization-specific MAC policy, but it must reason over observed provider/label state rather than normalize or reinterpret it. The same boundary applies to post-creation volatility drift and extension lifecycle facts.

## Acceptance boundary

**Source and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new security-label contract, retained auto-extension and raw-root/direction/function regressions on the new predecessor shape, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture existing function facts, `deptype='e'` membership, the complete `deptype='x'` dependency set, and the complete `pg_seclabel` provider/label map for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership. Reconstruction from ACL, function name, extension state, provider policy, operating-system policy, another source generation, or application metadata is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

This successor is admitted because PostgreSQL primary authority demonstrates an independent label-based security policy surface; it is not catalog-field enumeration. No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists.

The next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction that is not already represented by the current raw-root, exact function binding, function facts, extension lifecycle, security-label, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their exact heads, mergeability, reviews, required checks, workflow inventory, and protected-main relationship must be read fresh before landing. Sibling or predecessor evidence does not transfer into #2040, #35, or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including converter `deptype='e'`, complete converter `deptype='x'` sets, complete converter security-label maps, immutable raw converter-root lineage, and transform-object `deptype='e'` -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.