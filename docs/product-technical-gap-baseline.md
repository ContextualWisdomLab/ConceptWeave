# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, reviews, checks, and runs are evidence coordinates, not mutable dependencies. Evidence from an earlier PR head does not transfer after head movement unless the successor reproduces it.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation/validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, `contextual-orchestrator` owns production LLM routing, and Keyverse owns authentication/identity trust evidence. Consumers use only released/versioned `semantic_release`/contract/ACL coordinates; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, active representation/index successor; current ordinary-forward head is the latest branch head and must remain the sole Source Observation write authority.

#46 stays Draft until one unchanged exact head has repository-pinned Rust 1.98 plus applicable Product/security/dependency/review terminal evidence. #45/#6 ordinary/non-force adopt the complete verified child only after #46 exact-head GREEN; partial cherry-picks and duplicate fixes are invalid succession.

Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable. Central workflow settlement remains owned outside ConceptWeave. Current owner bootstrap `.github#2106@24bb6591ab7df23558cb793b4af60c567ff9da97` remains OPEN/non-Draft/mergeable on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`; its repository-owned CodeQL/SAST/Python Security/Security/Runtime Quality gates are GREEN, while Required Noema Review and Strix remain failed and qualifying independent approval is absent. Provider-failure provenance predecessor `.github#2115` is closed only after verified complete successor carryover. Canonical successor `.github#2114@87510bbb623edf08dcf4acd555cd2ac9321ac6c6` is OPEN/Draft/mergeable after an ordinary-forward successor-boundary repair. Fresh hosted settlement on that exact head is now terminal and not GREEN: Python Security `34703581697` and Semgrep `34703581695` succeeded; Security Scan `34703581703` failed specifically at the `gitleaks (secret scan)` job's `Enforce secret-scan gate` after scan, redacted summary, test-classification filtering and SARIF upload succeeded; Runtime Quality `34703581696` and CodeQL `34703581675` were cancelled. No current-head independent approval exists. This is a central owner-path repair requirement, not evidence transferable to or repairable from ConceptWeave.

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

## Active P1 — explicit unusable backing-index lifecycle

Review `5186802339` found a remaining cross-family contradiction. `IndexObservation` already preserves exact observed `pg_index.indisready`, `indisvalid`, and `indislive`, but `key_constraint_backing_index_static_shape_matches()` ignores those fields. PostgreSQL 18 states that `indisvalid=false` can leave a unique index without a guaranteed uniqueness property, `indisready=false` makes INSERT/UPDATE ignore the index, and `indislive=false` means the index is being dropped and must be ignored for all purposes. An explicitly unusable index therefore cannot coherently serve as governed PK/UNIQUE backing evidence.

Behavioral RED commits `681e280fa608b90495a035c04272bc89c546eaca` and `ed4882e7138f8b055b2913a7b25b8089ddc1b95c` cover both positive `conperiod=true` backing evidence and explicitly observed PK/UNIQUE timing. They require each explicitly false lifecycle flag to fail closed while preserving an explicitly ready + valid + live control. Primary-source doctoring is `docs/doctoring/source-observation-key-backing-index-lifecycle.md`.

Fresh exact-head source verification on 2026-09-13 reconfirmed that the shared predicate still omits lifecycle state and that `IndexObservation::{ready, valid, live}` each return `Option<bool>`. Coordination comment `5646966886` fixes the minimum causal repair boundary: reject only `Some(false)` in the shared predicate, preserve `None` as unobserved, and let both timing and positive `conperiod` admission inherit the same repair rather than duplicating checks in their callers.

Current state is **behavioral RED active**. Do not classify #46 as source-repaired, native/Product GREEN, Ready, merge-authorized, or released until this contradiction is repaired and exact-head acceptance is regenerated.

## Acceptance still required

After the active P1 is repaired, one unchanged exact #46 successor must produce repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, workspace/doc tests including both backing-index lifecycle RED contracts and retained temporal/type/index contracts, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review workflows terminal on the same head. Draft state, bot-only status, mechanical mergeability, predecessor GREEN, manual/no-op reruns, and synthetic statuses are not evidence.

The current successor has no pull-request workflow runs. The available execution host has no Rust toolchain and cannot reach GitHub for an independent exact-tree clone, so local execution cannot substitute for repository-pinned hosted evidence.

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
| Source Observation | BEHAVIORAL_RED_ACTIVE | `5186802339 -> 681e280f... -> ed4882e... -> 5646966886`; explicit unusable supporting-index lifecycle must fail closed; production repair pending. |
| Product CI | CENTRAL_REVIEW_REPAIR_PENDING | #35 unchanged; #2106 core hosted gates are GREEN but Noema/Strix failures and independent approval remain. #2115 is retired by verified successor. #2114 exact `87510bbb...` is Draft and currently has Security failure at the Gitleaks enforce gate plus cancelled Runtime Quality/CodeQL; central owner repair and fresh exact-head acceptance are required. |
| Quality gate | EXACT_HEAD_ACCEPTANCE_BLOCKED_ON_P1 | Repair the active lifecycle contradiction first; then reacquire unchanged-head Rust 1.98 and hosted gates. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before representation GREEN and parent adoption. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Repair the shared backing-index coherence seam so explicitly observed `indisready=false`, `indisvalid=false`, or `indislive=false` cannot authorize PK/UNIQUE timing or positive `conperiod` evidence; preserve unobserved lifecycle state as unobserved for this slice.
2. Keep #46 Draft and obtain one unchanged exact-head Rust 1.98/Product/security/dependency/review acceptance; repair only real failures ordinary-forward and restart exact-head acceptance whenever the head moves.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, central `.github#2114@87510bbb...` must root-cause/fix the exact-head Gitleaks enforce failure and reacquire fresh Security/Runtime/CodeQL plus independent review before normal landing and canonical successor reconciliation; only then replay `.github#2106@24bb659...`, whose Noema/Strix owner-path convergence and qualifying independent approval still remain. #35 then reacquires fresh unchanged-head acceptance without copied workflows or leaf-side provider workarounds.
5. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by deterministic validation, independent evaluation, steward review, and immutable publication under canonical owner boundaries.
