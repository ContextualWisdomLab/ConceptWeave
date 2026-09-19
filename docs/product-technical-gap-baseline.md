# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before dangling-role repair in converter-function initial privileges is preserved at `docs/archive/product-technical-gap-baseline-through-5daf2a57.md`; its matching changelog is `docs/archive/CHANGELOG-through-5daf2a57.md`. Earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

The previous initial-privilege successor correctly separated `pg_init_privs` from current `pg_proc.proacl`, preserved row absence versus presence and `privtype`, and retained a complete initial EXECUTE ACL. Fresh PostgreSQL 18 recovery review found that one of its admission assumptions was still lossy: every non-PUBLIC grantee and grantor OID had to resolve to a role name before the observation could be represented.

PostgreSQL BUG #19483, reported against 18.3 on 2026-05-18, demonstrates that `pg_init_privs.initprivs` can retain ACL entries that reference a role OID after that role has been dropped. `pg_upgrade --check` may still report compatibility and the later dump/restore can fail because the dangling numeric OID is emitted as an authorization/ACL identity. The subsequent pgsql-hackers repair discussion explicitly checks whether grantor and grantee OIDs still exist through `pg_roles`. This is a material recovery distinction inside an already-admitted Source Observation fact, not a request to enumerate another catalog field.

Finding review `5254640920` pinned the lossiness at exact pre-finding head `5daf2a5725ed52cd1b8d884200fc7786eaba54de`. Structural RED `94e98da8ebb31f1429b44b20cce7b96ea16e1332` referenced dangling-role constructors before production exposed them. Production causal repair `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f` changed the initial EXECUTE-grant identity so source capture can preserve dangling grantee, dangling grantor, or both while retaining existing resolved-role constructors.

Each initial ACL identity now distinguishes:

- PUBLIC grantee;
- resolved grantee role name;
- unresolved nonzero raw grantee OID;
- resolved grantor role name;
- unresolved nonzero raw grantor OID;
- exact grant option.

OID zero is invalid for unresolved-role constructors. A real role whose name is decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`; Source Observation must never stringify the OID and treat that text as a resolved role. Existing resolved-role digest framing remains stable, while unresolved identities use separate framing so old valid observations do not acquire new digests merely because the model can now represent catalog damage.

The successor still preserves complete converter coordinates, direction, exact schema/function binding, source generation metadata, and immutable raw `converter_snapshot_digest`. Missing/extra/duplicate converter coordinates, binding drift, zero key positions, unknown receipt coordinates, duplicate exact ACL entries, blank resolved role names, and unresolved OID zero fail closed. Missing role lookup alone no longer destroys the observation.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md`. The preceding decision surface remains immutable under the `through-5daf2a57` archives.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, May 18, 2026]. https://www.postgresql.org/message-id/19483-80de42dc4e62cfd6%40postgresql.org
- Demir, H. (2026a). *[PATCH] Add pg_upgrade check for invalid role references in pg_init_privs* [pgsql-hackers mailing-list post, June 7, 2026]. https://www.postgresql.org/message-id/CAB5wL7aig%2B%2BXphVjyBjvXG-%3DUE%2B%3Dmk3xfZZxkxV5XS4Hb58aHA%40mail.gmail.com
- Demir, H. (2026b). *Re: BUG #19483: pg_upgrade fails with orphan records in pg_init_priv catalog table* [Mailing-list post, June 26, 2026]. https://www.postgresql.org/message-id/CAB5wL7YQs7CDduxx0cv3iKPnQ7fxT-mj0AqPWv2WPO3aaF_VeQ%40mail.gmail.com

## Observation versus governance

Source Observation records exact external database state before policy. A dangling `pg_init_privs` role reference is therefore an observed recovery/security defect, not a reason to falsify the source by dropping the ACL entry or to abort all observation. Validation/publication may reject the state, require remediation, or block release, but must reason over the preserved raw identity.

ConceptWeave does not repair PostgreSQL catalogs and does not own role lifecycle. It only preserves the source fact needed by later governance. Current ACL, extension lifecycle, security labels, `pg_init_privs`, immutable raw converter-root identity, and transform-object extension membership remain separate facts.

## Acceptance boundary

**Source and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dangling-role initial-privilege contract, all retained initial-privilege/security-label/auto-extension/raw-root/direction/function regressions, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, the complete `deptype='x'` dependency set, the complete `pg_seclabel` provider/label map, and exact `pg_init_privs` row absence/presence with `privtype` plus the complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership.

The differential now also requires a catalog state with at least one nonzero `pg_init_privs` grantee or grantor OID that has no row in `pg_roles`. Capture must preserve that raw OID, not drop it, not fail solely because role lookup misses, and not reinterpret its decimal text as a role name. A numeric-looking resolved role name and an equal-valued unresolved OID must remain distinct. At least one ordinary differential still holds current `proacl` constant while changing the initial privilege baseline.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

This repair is admitted because current PostgreSQL primary evidence demonstrates a concrete dump/restore and pg_upgrade failure mode in the existing initial-privilege fact. No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists.

The next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction that is not already represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, complete initial-privilege identity including dangling ACL OIDs, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their exact heads, mergeability, reviews, required checks, workflow inventory, and protected-main relationship must be read fresh before landing. Sibling or predecessor evidence does not transfer into #2040, #35, or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including current ACL, converter `deptype='e'`, complete converter `deptype='x'` sets, complete converter security-label maps, exact converter initial privilege baselines including dangling raw role OIDs, immutable raw converter-root lineage, and transform-object `deptype='e'` -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
