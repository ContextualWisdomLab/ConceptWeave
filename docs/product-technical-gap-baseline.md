# Product / Technical Gap Baseline

**Snapshot:** 2026-09-12

This is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, reviews, checks, and runs are evidence coordinates, not mutable dependencies. Evidence from an earlier PR head does not transfer after head movement unless the successor reproduces it.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation/validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, `contextual-orchestrator` owns production LLM routing, and Keyverse owns authentication/identity trust evidence. Consumers use only released/versioned `semantic_release`/contract/ACL coordinates; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, active representation/index successor.

#46 stays Draft until one unchanged exact head has repository-pinned Rust 1.98 plus applicable Product/security/dependency/review terminal evidence. #45/#6 ordinary/non-force adopt the complete verified child only after #46 exact-head GREEN; partial cherry-picks and duplicate fixes are invalid succession.

Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable. Central workflow settlement remains owned outside ConceptWeave. The current protected-handler bootstrap is `.github#2106@24bb6591ab7df23558cb793b4af60c567ff9da97` on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`. #2106 is OPEN/non-Draft/mergeable. Historical exact-head runs have Runtime Quality, SAST, Python Security, and Security GREEN while CodeQL PR `34688687870` failed at compatibility enforcement after successful dispatch with `VERDICT_STATE=pending`; canonical handler run `34689581833` reproduced the owner-side matrix-wake HTTP 403 race. The Ready transition kept the same source head and has now queued fresh CodeQL PR `34692079677`, SAST `34692079701`, Python Security `34692079678`, and Security Scan `34692079700`; those queued runs are not terminal evidence. Independent qualifying approval is still absent. ConceptWeave must not copy, weaken, synthetically satisfy, or manually retrigger that owner gate.

## Source Observation boundary

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, evidence locations, and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority, or foreign product truth. Historical v2 evidence is frozen. PostgreSQL successor facts are additive and domain-separated. Catalog OIDs are capture-local join coordinates, never governed semantic identity.

## PostgreSQL 18 representation-v3 state

The active successor keeps exact relation/type/index/constraint coordinates, true-array identity, direct type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, and explicit temporal constraint evidence. `pg_constraint.conperiod` remains the declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index or operator shape never invents it. Temporal final columns resolve to range/multirange through observed type/domain evidence, including domains over range/multirange. PostgreSQL 18 `MATCH PARTIAL` remains fail-closed while SIMPLE/FULL are preserved. PERIOD FKs retain exact action, reference, and referenced-key timing requirements.

## Retained temporal exclusion-operator correction

Review `5186175514`, RED `a9065d460af4c00d84c2453b744796effd4d0485`, production repair `e286c3524f036138546d91cb0d53631e8c8e41bf`, and doctoring `eef825ec486089ce1579305e05d3a309c8cae508` corrected the earlier literal-operator-name inference. Exact operator namespace/name and qualified operand types remain provenance and digest material; equality/overlap authority must later be verified by the PostgreSQL adapter through the exact backing-index operator class/operator family and PostgreSQL `COMPARE_EQ`/`COMPARE_OVERLAP` translation. The same-name GiST/exclusion backing-index, temporal type, action, match, and timing invariants remain in force.

## Active P1 — outbound PERIOD reference evidence

Fresh review found a fail-open branch in `canonicalize_constraint_periods()`. For `conperiod=true` foreign keys, the representation verifies that the referenced columns resolve to a positive `WITHOUT OVERLAPS` PK/UNIQUE and that the referenced key is `NOT DEFERRABLE` only when the referenced relation is present in the bounded `relations` inventory. If the referenced relation is absent, the predecessor silently skips the referenced-key authority check and admits the local PERIOD FK.

PostgreSQL 18 requires a PERIOD foreign key to reference a PRIMARY KEY or UNIQUE constraint declared `WITHOUT OVERLAPS`; the local FK's `conperiod=true` value cannot independently prove the immutable referenced-key identity or timing. A governed positive temporal relationship therefore must not use absence of captured reference evidence as success.

Current lineage:

- P1 finding review `5186323924` on exact predecessor `cd758faa7a9798fe0cf04ead6f0c1fb50f926106`;
- behavioral RED `d692773a5050b6d5486c40a97d5d8589601fe995`, `constraint_period_external_reference_contract.rs`, requiring missing referenced temporal-key evidence to fail with `constraint_period_reference`;
- primary-source doctoring `6ae821dfa5925a16eb690933bdf29ea96cb87c4d`, `source-observation-period-external-reference-evidence.md`.

This P1 is **behavioral RED active**. The minimal causal production repair is to fail closed when a positive PERIOD FK's exact referenced relation is absent from the current bounded representation, while preserving the already-supported fully observed referenced `WITHOUT OVERLAPS` + `NOT DEFERRABLE` path and leaving ordinary non-PERIOD outbound foreign keys unchanged. A future external referenced-key evidence family may relax this only if it is explicit, immutable, domain-separated, digest-material, provenance-bearing, and authorization-bounded.

## Acceptance still required

After the active RED is repaired, one unchanged exact #46 successor must produce repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, workspace/doc tests including the outbound PERIOD reference witness and retained temporal/type/index contracts, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review workflows terminal on the same head. Draft state, bot-only status, mechanical mergeability, predecessor GREEN, manual/no-op reruns, and synthetic statuses are not evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate; resolve least-privilege credentials only through the authorized source/policy binding; use bounded `REPEATABLE READ READ ONLY` catalog capture; and never keep an explicit database transaction/lock open while waiting on LLM or long external computation.

Catalog OIDs may only join the captured snapshot. The ACL must cross with exact qualified names and complete evidence from `pg_type`, `pg_range`, `pg_class`, `pg_index`, `pg_constraint`, `pg_opclass`/operator-family catalogs, and `pg_operator`. A represented temporal key must preserve the complete per-column `conexclop` vector and durable namespace/name/type signatures, verify the appropriate `COMPARE_EQ` or `COMPARE_OVERLAP` mapping through each resolved backing-index operator class, retain same-name GiST/exclusion index evidence, temporal type/domain chain, exact timing/action/match facts, and policy-admitted row/byte/concurrency ceilings. Referenced temporal keys outside the initially bounded relation set must trigger an explicitly authorized evidence-expansion flow or remain fail-closed; the adapter must never silently widen schema authorization. Reconstructed DDL is provenance text, never the sole semantic carrier.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE* — `WITHOUT OVERLAPS` and PERIOD FK requirements, referenced-key eligibility, supporting index semantics.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint* — `conperiod`, `conkey`, and `confkey` catalog facts.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: GiST indexes* — operator-class extensibility and compare-type translation for temporal constraints.
- PostgreSQL Global Development Group. (2026). *PostgreSQL source: `ComputeIndexAttrs()`* — `COMPARE_EQ`/`COMPARE_OVERLAP` operator lookup for `WITHOUT OVERLAPS`.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes* — domains over range/multirange for `WITHOUT OVERLAPS`.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | PERIOD_EXTERNAL_REFERENCE_RED | `5186323924 -> d692773... -> 6ae821d...`; production fail-closed repair required. |
| Product CI | BLOCKED_OWNER_ACCEPTANCE | #35 unchanged; central #2106 exact `24bb659...` is non-Draft with fresh CodeQL/SAST/Python Security/Security runs queued after Ready, so no new terminal acceptance transfers yet; independent approval is absent. |
| Quality gate | RED_REPAIR_THEN_NATIVE_ACCEPTANCE | No Ready/adoption/merge before causal repair and one unchanged head passes Rust 1.98 plus hosted gates. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption; explicit referenced-key completeness is now an ACL requirement. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Repair the active outbound PERIOD reference RED in #46 without widening ordinary foreign-key scope or source authorization.
2. Keep #46 Draft and obtain one unchanged exact-head Rust 1.98/Product/security/dependency/review acceptance; repair only real failures ordinary-forward and restart exact-head acceptance whenever the head moves.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, central `.github#2106@24bb659...` must let the fresh Ready-triggered checks settle, resolve any owner-side failure, obtain independent acceptance, and land normally before the documented #2040/#35 sequence can proceed.
5. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by deterministic validation, independent evaluation, steward review, and immutable publication under canonical owner boundaries.
