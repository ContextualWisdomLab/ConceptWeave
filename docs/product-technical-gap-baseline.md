# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, reviews, checks, and runs are evidence coordinates, not mutable dependencies. Evidence from an earlier PR head does not transfer after head movement unless the successor reproduces it.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation/validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, `contextual-orchestrator` owns production LLM routing, and Keyverse owns authentication/identity trust evidence. Consumers use only released/versioned `semantic_release`/contract/ACL coordinates; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, active representation/index successor. Production lifecycle repair commit `ff842b358106c0adeda97fd2405952ff7be029b1` is an ordinary-forward child of prior authority `28998888161cf570be79ea17fb0dfd31017f4895`; the latest branch head remains the sole Source Observation write authority.

#46 stays Draft until one unchanged exact head has repository-pinned Rust 1.98 plus applicable Product/security/dependency/review terminal evidence. #45/#6 ordinary/non-force adopt the complete verified child only after #46 exact-head GREEN; partial cherry-picks and duplicate fixes are invalid succession.

Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable. Central workflow settlement remains owned outside ConceptWeave. Current owner bootstrap `.github#2106@24bb6591ab7df23558cb793b4af60c567ff9da97` remains outside this leaf's write boundary on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`. Provider-failure provenance predecessor `.github#2115` is closed only after verified complete successor carryover. Canonical successor `.github#2114@3c43dd165009d503b2ebf56324b975db440e2fdb` is OPEN/non-Draft/mergeable after ordinary-forward malformed-body authority repair. On that exact head Security Scan `34706027952`, Python Security `34706027947`, and SAST Semgrep `34706027904` are GREEN. CodeQL PR `34706027995` is terminal RED because both compatibility shards failed closed while the authenticated verdict was still pending before the later dispatch job succeeded; this is central settlement/handler evidence, not a ConceptWeave source finding. Runtime Quality `34706002008` was still in progress at this snapshot. No central predecessor evidence transfers to ConceptWeave and no leaf workflow copy/bypass is permitted.

## Source Observation boundary

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, evidence locations, and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority, or foreign product truth. Historical v2 evidence is frozen. PostgreSQL successor facts are additive and domain-separated. Catalog OIDs are capture-local join coordinates, never governed semantic identity.

## PostgreSQL 18 representation-v3 state

The active successor keeps exact relation/type/index/constraint coordinates, true-array identity, direct type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, and explicit temporal constraint evidence. `pg_constraint.conperiod` remains the declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index or operator shape never invents it. Temporal final columns resolve to range/multirange through observed type/domain evidence, including domains over range/multirange. PostgreSQL 18 `MATCH PARTIAL` remains fail-closed while SIMPLE/FULL are preserved. PERIOD FKs retain exact action, reference, and referenced-key timing requirements.

## Retained temporal repairs

Review `5186175514`, RED `a9065d460af4c00d84c2453b744796effd4d0485`, production repair `e286c3524f036138546d91cb0d53631e8c8e41bf`, and doctoring `eef825ec486089ce1579305e05d3a309c8cae508` corrected literal exclusion-operator-name inference. Exact operator namespace/name and qualified operand types remain provenance and digest material; equality/overlap authority belongs to the PostgreSQL adapter's operator-class/operator-family compare-type verification.

Review `5186323924`, behavioral RED `d692773a5050b6d5486c40a97d5d8589601fe995`, doctoring `6ae821dfa5925a16eb690933bdf29ea96cb87c4d`, and production repair `1462105f2ad36a736ebb8263c6e97968490414b2` made outbound positive PERIOD references fail closed when the exact referenced relation/temporal key evidence is missing. Ordinary non-PERIOD outbound foreign keys remain unchanged.

The earlier `WITHOUT OVERLAPS` backing-index presence repair also remains active: positive temporal PK/UNIQUE evidence requires a same-name backing index, material `pg_index` flags, `indisexclusion=true`, and observed access method `gist`.

## Repaired P1 — temporal backing-index key-shape coherence

Review `5186585545` found that `canonicalize_constraint_periods()` stopped its positive temporal-key backing check at same-name index + exclusion + GiST. Exact ordered key-column shape and other static PK/UNIQUE backing facts were checked only by the separate optional timing family. Consequently a snapshot observing `conperiod=true` without timings could admit a same-name GiST/exclusion index whose key order did not match the temporal constraint.

Behavioral RED `ffe75eddf530ad3963c087afdfe1109da15bad14` extends `constraint_period_backing_index_presence_contract.rs`: a constraint `(document_id, valid_during WITHOUT OVERLAPS)` paired with same-name GiST/exclusion key `(valid_during, document_id)` must fail `constraint_period_backing_index`.

Production repair `e258b39474454b4c84f51f23dcfb6b00285f4db3` adds one shared `key_constraint_backing_index_static_shape_matches()` predicate and uses it from both timing and period canonicalization. It verifies exact ordered key-column equality, uniqueness, no partial predicate, PK versus UNIQUE catalog role, and observed UNIQUE null treatment. Positive period admission additionally retains `indisexclusion=true` and `gist`; timing admission separately retains `indimmediate` and exclusion-access-method coherence. Optional-family composition is preserved: static temporal backing coherence no longer depends on timing evidence, while timing-specific facts remain timing-owned.

## Repaired P1 — explicit unusable backing-index lifecycle

Review `5186802339` found a cross-family contradiction. `IndexObservation` already preserves exact observed `pg_index.indisready`, `indisvalid`, and `indislive`, while the shared backing-index predicate previously ignored those fields. PostgreSQL 18 states that `indisvalid=false` can leave a unique index without a guaranteed uniqueness property, `indisready=false` makes INSERT/UPDATE ignore the index, and `indislive=false` means the index is being dropped and must be ignored for all purposes. An explicitly unusable index therefore cannot coherently serve as governed PK/UNIQUE backing evidence.

Behavioral RED commits `681e280fa608b90495a035c04272bc89c546eaca` and `ed4882e7138f8b055b2913a7b25b8089ddc1b95c` cover both positive `conperiod=true` backing evidence and explicitly observed PK/UNIQUE timing. They require each explicitly false lifecycle flag to fail closed while preserving an explicitly ready + valid + live control. Primary-source doctoring is `docs/doctoring/source-observation-key-backing-index-lifecycle.md`.

Production repair `ff842b358106c0adeda97fd2405952ff7be029b1` applies the causal fix once in `key_constraint_backing_index_static_shape_matches()`: `ready`, `valid`, and `live` must each be either unobserved or true. Only explicit `Some(false)` is rejected; `None` stays unobserved, no lifecycle check is duplicated in timing/period callers, and lifecycle state does not invent constraint or temporal truth. The preflight candidate was built from the exact predecessor tree and compared before branch movement; the resulting commit is exactly one file with three added predicate clauses and no deletions.

Current state is **source repaired / exact-head acceptance pending**. Do not classify #46 as native/Product GREEN, Ready, merge-authorized, or released until the unchanged exact successor passes the required execution and review gates.

## Acceptance still required

One unchanged exact #46 successor must produce repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, workspace/doc tests including both backing-index lifecycle RED contracts and retained temporal/type/index contracts, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review workflows terminal on the same head. Draft state, bot-only status, mechanical mergeability, predecessor GREEN, manual/no-op reruns, and synthetic statuses are not evidence.

No pull-request workflow run materialized immediately on production repair commit `ff842b358106c0adeda97fd2405952ff7be029b1`; hosted acceptance therefore remains absent unless a later unchanged-head owner workflow provides it. The available execution host has no Rust toolchain and cannot reach GitHub for an independent exact-tree clone, so local execution cannot substitute for repository-pinned hosted evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate; resolve least-privilege credentials only through the authorized source/policy binding; use bounded `REPEATABLE READ READ ONLY` catalog capture; and never keep an explicit database transaction/lock open while waiting on LLM or long external computation.

Catalog OIDs may only join the captured snapshot. The ACL must cross with exact qualified names and complete evidence from `pg_type`, `pg_range`, `pg_class`, `pg_index`, `pg_constraint`, `pg_opclass`/operator-family catalogs, and `pg_operator`. `pg_constraint.conindid` may bind a constraint to its backing index during capture but must not become durable semantic identity. A represented temporal key must preserve the complete per-column `conexclop` vector and durable namespace/name/type signatures, verify the appropriate `COMPARE_EQ` or `COMPARE_OVERLAP` mapping through each resolved backing-index operator class, retain exact backing-index key layout and static flags, temporal type/domain chain, exact timing/action/match facts, and policy-admitted row/byte/concurrency ceilings. Explicitly observed backing-index `indisready`, `indisvalid`, and `indislive` must not contradict supporting-index admission. Referenced temporal keys outside the initially bounded relation set must trigger an explicitly authorized evidence-expansion flow or remain fail-closed; the adapter must never silently widen schema authorization. Reconstructed DDL is provenance text, never the sole semantic carrier.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE* — `WITHOUT OVERLAPS` and PERIOD FK requirements, referenced-key eligibility, generated backing-index name/key semantics.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint* — `conperiod`, `conindid`, `conkey`, `confkey`, and `conexclop` catalog facts.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index* — `indisunique`, `indisprimary`, `indisexclusion`, `indisready`, `indisvalid`, `indislive`, key positions, predicate, and related material index facts.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: GiST indexes* — operator-class extensibility and compare-type translation for temporal constraints.
- PostgreSQL Global Development Group. (2026). *PostgreSQL source: `ComputeIndexAttrs()`* — `COMPARE_EQ`/`COMPARE_OVERLAP` operator lookup for `WITHOUT OVERLAPS`.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18.4 release notes* — domains over range/multirange for `WITHOUT OVERLAPS`.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | SOURCE_REPAIRED_ACCEPTANCE_PENDING | `5186802339 -> 681e280f... -> ed4882e... -> ff842b35...`; explicit unusable supporting-index lifecycle now fails closed in the shared predicate; execution evidence is still required. |
| Product CI | CENTRAL_SETTLEMENT_PENDING | #35 unchanged; central `.github#2114@3c43dd16...` has Security/Python/Semgrep GREEN and CodeQL settlement failure, with Runtime Quality still settling at this snapshot. Central owner repair/settlement and independent review remain outside this leaf. |
| Quality gate | EXACT_HEAD_ACCEPTANCE_PENDING | Reacquire unchanged-head Rust 1.98 and hosted Product/security/dependency/review gates after the source repair. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Keep #46 Draft and obtain one unchanged exact-head Rust 1.98/Product/security/dependency/review acceptance for the lifecycle repair; repair only real failures ordinary-forward and restart exact-head acceptance whenever the head moves.
2. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
3. Independently, central `.github#2114@3c43dd16...` must resolve its run-wide CodeQL settlement boundary and finish fresh exact-head Runtime Quality/independent review before normal landing and canonical successor reconciliation; only then replay `.github#2106@24bb659...`, whose Noema/Strix owner-path convergence and qualifying independent approval still remain. #35 then reacquires fresh unchanged-head acceptance without copied workflows or leaf-side provider workarounds.
4. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by deterministic validation, independent evaluation, steward review, and immutable publication under canonical owner boundaries.
