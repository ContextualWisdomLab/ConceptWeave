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

Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable. Central workflow settlement remains owned outside ConceptWeave. The current protected-handler bootstrap is `.github#2106@24bb6591ab7df23558cb793b4af60c567ff9da97` on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`; SAST Semgrep, Python Security, and Runtime Quality have current-head success, while Security Scan remains queued and CodeQL PR remains in progress. #2106 must reach exact-head terminal acceptance and land normally before #2040 can reconcile/switch protocol; unchanged #35 then needs fresh Product acceptance. ConceptWeave must not copy, weaken, synthetically satisfy, or manually retrigger that owner gate.

## Source Observation boundary

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, evidence locations, and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority, or foreign product truth. Historical v2 evidence is frozen. PostgreSQL successor facts are additive and domain-separated. Catalog OIDs are capture-local join coordinates, never governed semantic identity.

## PostgreSQL 18 representation-v3 state

The active successor keeps exact relation/type/index/constraint coordinates, true-array identity, direct type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, and explicit temporal constraint evidence. `pg_constraint.conperiod` remains the declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index or operator shape never invents it. Temporal final columns resolve to range/multirange through observed type/domain evidence, including domains over range/multirange. PostgreSQL 18 `MATCH PARTIAL` remains fail-closed while SIMPLE/FULL are preserved. PERIOD FKs retain exact action, reference, and referenced-key timing requirements.

## Current temporal exclusion-operator correction

Concurrent ordinary-forward work added resolved `pg_constraint.conexclop` signatures to `ConstraintPeriodObservation`, including position, operator namespace/name, and qualified operand types. It was adopted, not rewritten. The initial repair then over-constrained those signatures by requiring literal `=` for every prefix key and literal `&&` for the final temporal key.

Corrective review `5186175514` verified against PostgreSQL 18 implementation that this spelling rule is not source-authoritative. `ComputeIndexAttrs()` requests `COMPARE_EQ` for non-final `WITHOUT OVERLAPS` keys and `COMPARE_OVERLAP` for the final key, then asks the resolved GiST operator class for the matching operator through `GetOperatorFromCompareType()`. Extensible operator classes can therefore select valid operators with different catalog names. Literal spelling can reject valid PostgreSQL catalogs and still does not prove operator-family membership.

Corrective lineage:

- source finding `5186175514` on predecessor `55d53adfa13297ff9eaa19555b6c2c2eb5890a3f`;
- behavioral RED `a9065d460af4c00d84c2453b744796effd4d0485`, preserving custom-named opclass-selected temporal operators;
- production repair `e286c3524f036138546d91cb0d53631e8c8e41bf`, removing operator-name inference while retaining non-empty contiguous signatures and aggregate arity enforcement;
- primary-source doctoring `eef825ec486089ce1579305e05d3a309c8cae508`.

Exact operator namespace/name and qualified operand types remain retained/digested provenance. Equality/overlap semantic verification belongs to the PostgreSQL adapter ACL where catalog OIDs, each backing-index key's exact operator class/operator family, and compare-type translation can be resolved inside one bounded snapshot. The adapter must verify that `COMPARE_EQ`/`COMPARE_OVERLAP` translation returns the same operator represented by `conexclop`; neither operator spelling nor catalog OID alone becomes governed identity.

The preceding backing-index invariant remains in force: `conperiod=true` PK/UNIQUE requires the exact same-name index, material catalog flags, `indisexclusion=true`, and exact observed `gist`; ordinary `conperiod=false` keys do not acquire mandatory index-family observation.

This lane is **source-repaired / exact-head native-and-Product-acceptance-pending**. It is not Ready, merged, adopted by #45/#6, transported to PostgreSQL, published, or released.

## Acceptance still required

Before Ready/adoption/merge, one unchanged exact #46 successor must produce repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, workspace/doc tests including temporal operator/backing-index witnesses and retained v2/v3 contracts, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review workflows terminal on the same head. Draft state, bot-only status, mechanical mergeability, predecessor GREEN, manual/no-op reruns, and synthetic statuses are not evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate; resolve least-privilege credentials only through the authorized source/policy binding; use bounded `REPEATABLE READ READ ONLY` catalog capture; and never keep an explicit database transaction/lock open while waiting on LLM or long external computation.

Catalog OIDs may only join the captured snapshot. The ACL must cross with exact qualified names and complete evidence from `pg_type`, `pg_range`, `pg_class`, `pg_index`, `pg_constraint`, `pg_opclass`/operator-family catalogs, and `pg_operator`. A represented temporal key must preserve the complete per-column `conexclop` vector and durable namespace/name/type signatures, verify the appropriate `COMPARE_EQ` or `COMPARE_OVERLAP` mapping through each resolved backing-index operator class, retain same-name GiST/exclusion index evidence, temporal type/domain chain, exact timing/action/match facts, and policy-admitted row/byte/concurrency ceilings. Reconstructed DDL is provenance text, never the sole semantic carrier.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE* — `WITHOUT OVERLAPS` exclusion semantics, supporting index, PERIOD FK behavior.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: GiST indexes* — operator-class extensibility and compare-type translation for temporal constraints.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint / pg_index / pg_type / pg_range / pg_class*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL source: `ComputeIndexAttrs()`* — `COMPARE_EQ`/`COMPARE_OVERLAP` operator lookup for `WITHOUT OVERLAPS`.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes* — domains over range/multirange for `WITHOUT OVERLAPS`.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | TEMPORAL_OPERATOR_SOURCE_REPAIRED | `5186175514 -> a9065d4... -> e286c35... -> eef825e...`; exact-head native/Product acceptance required. |
| Product CI | BLOCKED_OWNER_ACCEPTANCE | #35 unchanged; central #2106 exact `24bb659...` has partial current-head success, Security Scan queued and CodeQL PR in progress. |
| Quality gate | NATIVE_ACCEPTANCE_REQUIRED | No Ready/adoption/merge before one unchanged head passes Rust 1.98 and hosted gates. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption; compare-type/opclass verification is an explicit ACL requirement. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Keep #46 Draft and obtain one unchanged exact-head Rust 1.98/Product/security/dependency/review acceptance.
2. Repair only real failures ordinary-forward and restart exact-head acceptance whenever the head moves.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, central `.github#2106@24bb659...` must reach terminal checks and land normally; #2040 then reconciles/switches protocol and lands before unchanged #35 can obtain Product acceptance.
5. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, including exact operator-class compare-type verification, followed by deterministic validation, independent evaluation, steward review, and immutable publication under canonical owner boundaries.
