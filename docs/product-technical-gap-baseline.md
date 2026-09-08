# Product / Technical Gap Baseline

**Snapshot:** 2026-09-08 (current top checkpoint; older exact coordinates below are retained as dated evidence)

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after a dated checkpoint.

## September 8 research lifecycle, abstention, and replay-evidence checkpoint

PR #40 exact head `0202f6551ad8a36bdda10225149d0225043b7888` now rejects
stored source-resolution JSON whose nested `library_version` differs from the
envelope revision. The regression was reproduced at `b2f05085` before the
minimal deserializer repair. Full workspace tests, focused source-resolution
tests, strict Clippy and diff checks pass locally; the PR remains Draft/Open
with no protected hosted workflow evidence.

The `0d4d677ddad471a0634cdaeed99d8e1afec66043` repair checkpoint additionally binds source-resolution decisions to the Local API `server_id` whenever the provider exposes one; this is dated evidence, not a mutable-head claim.

PR #40 exact-head repair `a87ee2d4d92f97fe61639696ae5e06f8f20c84d6` now rejects missing or blank Local API server identity before evaluating pending resolutions, including the empty-pending case; focused Zotero tests, locked workspace tests, strict Clippy, warnings-denied rustdoc, and formatting pass locally. Hosted protected checks remain unavailable while the PR is Draft.

Pinned coverage rerun at merged exact head `d8560e8870f4419c73dc7530af7fdd1d0f238658` is RED at 181/186 functions, 2,773/2,825 normalized regions, and 160/166 normalized branches; no exclusions or threshold weakening are accepted. The remaining gaps are concentrated in generic/transport and CLI error paths and require targeted owner tests before any GREEN claim.

After removing the now-unreachable identity sub-branch behind the early fail-closed guard, exact head `66501eafc21aed967d20750dd2a5e621bafd760d` measures 181/186 functions, 2,771/2,823 normalized regions, and 159/164 normalized branches. The gate remains RED; the change removes duplicate control flow and does not exclude uncovered behavior.

The HTTP-error regression at exact code head `a9b67f6300fcd2edf21cb447ff0ee54ea1898586` raises normalized coverage to 182/186 functions, 2,774/2,823 regions, and 159/164 branches. One platform-gated CLI branch remains uncovered; transport and generic gaps remain explicit rather than excluded.

Exact code head `7d922896f0afb848a0cb879563b5f3a3db1e5313` adds the required artifact-boundary check that rejects missing or blank `server_id` during `SourceResolutionReview` JSON deserialization. Its full pinned run passes tests but is RED at 184/188 functions, 2,789/2,840 regions, and 160/166 branches because the custom deserializer adds exercised and generic paths; this is a security repair, not a reason to weaken coverage.

Owner coverage checkpoint `f71286e` (before documentation-only `60d01f8`) improved after the source-resolution error-contract test to 180/185 functions, 2,766/2,818 normalized regions, and 159/164 normalized branch outcomes; the gate remains RED and no exclusion or threshold weakening is accepted.

PR #40 executable checkpoint `1308ac6e4403a67c61b84f47c9d0142286da3c64` is recorded as a dated Draft/open/CLEAN evidence point, based on Research Intake head `ff2e78aaa4ce75c7eb5f41bc9612b8dcf3bb7d38`. Its source-resolution aggregate, ontology discovery signal, and visual/API population-boundary lesson are pushed, while independent approval and protected workflow evidence remain absent; the live PR head is authoritative for later documentation-only descendants and no merge or release is claimed.

PR #39 review adds a downstream repair finding: `FullTextWriteScope` still lacks the source-resolution aggregate and exact snapshot binding, so its successor must consume PR #40 without clearing or inferring the four pending sources.

PR #38 review confirms the same boundary one stage earlier: full-text worksheet, finalization, and evaluation accept no `SourceResolutionReview`. After PR #40 adoption, their successor must bind the exact report-bound resolution beside the independent capture digest and reject missing or stale resolutions; pending keys must not be cleared to bypass the gate.

PR #40's publication cleanup repair (`1b9b096` RED → `da867a2` GREEN) now preserves an already-linked final report when temporary-file cleanup fails; rollback no longer deletes the customer-visible artifact. Follow-up RED `1ce4dee` and GREEN `4ee2116` preserve the original I/O kind while identifying the post-publication cleanup state.

The current executable also completed a fresh read-only Local API replay: 8,326 records, 3,715 proposals, 4,611 nonbibliographic records, and four pending sources (Zotero 10.0.1/API 3/schema 44, library version 2). The private 0600 report digest is `sha256:5b2d81bddf09475de76ce18efa9fb5b486c44b794c9db6b697d183da5c268b48`; decisions and independent approvals remain 0/3,715.

The same replay provides a bounded ontology-library signal without claiming semantic authority: 32 proposals matched `ontology`, 24 matched `owl`, 20 matched `rdf`, two matched `knowledge graph`, and one matched `linked data`. These phrase counts overlap by proposal and are discovery evidence only; they do not approve a paper, infer a library owner, or authorize Zotero mutation.

Foundation PR #35 remains blocked by shared workflow evidence rather than a reported ConceptWeave source defect: the CodeQL dispatch submission is accepted, but the authenticated shared scan `34208633449` is still queued; Noema recorded provider HTTP 429 capacity and OpenCode has no current-head authenticated verdict. These are terminal-gate prerequisites for protected integration, not grounds for a manual rerun, synthetic status, or deployment claim.

Research Intake source-resolution P1 is now addressed at its owner seam by RED `520a9bf` and GREEN `416e35a`. `prepare_source_resolution_review` produces a typed aggregate only when decisions exactly cover `pending_source_item_keys` and match each retained source's key, version, type, parent, and report library coordinates. Outcomes are explicit and non-authoritative (`retain_standalone_evidence`, `rebind_requires_separate_authorization`, `exclude_from_research_scope`); missing, unknown, duplicate, stale, and blank-reason inputs fail closed. Successor tests `e636624`, `3d2c2e7`, and `3d17622` cover identity rejection, owned JSON round-trip, and cross-library snapshot binding; `e2a5355` changes the serialized rule revision to owned data after review finding, and `4ce83e8` covers the nonalphabetic abstention branch. This closes the producer contract but not downstream paper-review/full-text/approval integration, protected checks, or Zotero writes.

Research Intake PR #9 remains the canonical Zotero/read-only research-classification owner lane at exact head `375a375de6c36e1e441ab34e2b0d378e9188a0bd`, based on Foundation `b538470c963e6524ddc0c3f652a46a4fc8265150`; the earlier `ea204ae970ef7639927ee0ab4c60e00ad0fd156b` coordinate is retained only as a dated coverage checkpoint. Committed lifecycle RED `0a2d63a32fa782c15eb79970213433efba17f0cd` requires separate serialized `truth_status="proposed"` and `publication_state="proposed"`; minimal source repair `72bf15a4200b6496656ce4faaf86faeff779e2ef` projects those proposal-only fields without adding validation, review or publication transitions, a second lifecycle state machine, or a dependency.

PR #9 owner coverage checkpoint `28fb662` improves the pinned run to 174/179 functions, 1,679/1,706 lines, 2,708/2,763 raw regions, 155/162 raw branches, and 1,090/1,149 normalized regions with 159/162 normalized branches; the gate remains RED and no exclusions or threshold weakening are accepted.

The follow-up punctuation-boundary test at `ea204ae` raises normalized branches to 160/162; PR #9 remains RED only in two platform/CLI branch outcomes (normalized regions remain 1,090/1,149).

The owner serialization helper at `375a375` exercises flush failure and raises the owner normalized branch result to 159/160, with 1,096/1,152 normalized regions; the sole remaining branch is the platform-gated conventional `/tmp` path. No coverage exclusions or threshold weakening are accepted.

The platform-explicit fixture repair at owner head `b98ba39f9efa4c18c1bcdc3bcae4a8ef4f0abd41` removes that runtime-only branch: normalized branches are now 158/158 (100%), with normalized regions 1,096/1,152 and raw functions 178/183. Remaining RED is limited to uncovered regions/generic instantiations; no exclusion or threshold weakening is accepted.

A second current semantic-integrity sequence addresses P2 review `PRRT_kwDOUKg5E86fUr-x`. RED `cd791f9c97dfd6048fff44e67bab0414af2aa819` proves that ordinary accented text (`naïve`) and an otherwise ASCII abstract containing scientific symbol `α` must not be labeled unsupported vocabulary merely because they contain a non-ASCII alphabetic character; a wholly non-ASCII alphabetic control remains explicit unsupported vocabulary under the current English deterministic rule set. Minimal repair `67d0d5e55ed77bfcc65ae83a986a0d7dc690dc61` confines `UnsupportedRuleVocabulary` to alphabetic metadata with no ASCII alphabetic signal, leaving rule phrases, disposition families, lifecycle projection, source denominator and governance authority unchanged.

A third evidence-integrity sequence addresses P1 review `PRRT_kwDOUKg5E86fT2sS`. Committed RED `1c8368707cf8c195eb65e6af4801abb3959a07d3` requires a nonempty abstract that leads to `NeedsStewardReview` to remain verbatim as local-only `review_abstract_note`, while deterministic matches and empty abstracts omit that field. Minimal source repair `2400afe150d7e0c19f1251c41a84c9753b1ab33d` adds only the optional, `skip_serializing_if = "Option::is_none"` projection and populates it when the final proposed disposition is `NeedsStewardReview` and the original abstract is nonblank. It does not reinterpret the abstract as matched evidence, change classification rules, or grant validation/review/publication authority.

A fourth evidence-integrity sequence addresses P2 review `PRRT_kwDOUKg5E86fT2sX`. Committed RED `d0e87d7236bd6f43cb2c2fd0ace6d85a1e604b62` requires each reversible DOI duplicate candidate to retain the exact DOI string observed for each source item in addition to the normalized duplicate identity. Minimal source repair `bfbe3596dc97bcf277b70da0dfbd9f8474000917` adds deterministic `source_identity_values` keyed by Zotero item key and carries the original DOI/title identity value through duplicate grouping. DOI/title normalization, duplicate admission, item retention and semantic/governance authority are unchanged.

A fifth test-reliability sequence addresses P2 review `PRRT_kwDOUKg5E86fT2tV`. Exact pre-fix head `0a7d0218e8c21c4207e54220b0f9d127274eb1b3` performed one `TcpStream::read` into a 4096-byte buffer and immediately asserted that the returned prefix contained `zotero-api-version: 3`; that is a reality RED because TCP does not promise the complete request headers in one read. Minimal test-harness repair `fd4884214ffb01d874a6c8820287f2da2999cefb` deliberately reads one byte at a time until `\r\n\r\n`, rejects peer closure before complete headers, and caps accumulated request headers at 16 KiB before asserting the API-version header. Production Zotero transport and classification behavior are unchanged.

A sixth, narrower privacy/minimization sequence begins with committed RED `43a18114657f783f52aadbc9dd4a50632c38d3f8` on the same abstention-review thread. A conflict proposal can match one specific family in the title and another in the abstract, making the final disposition `NeedsStewardReview` while the exact abstract is already retained in `evidence.field_values["abstract_note"]`. The RED requires one-copy replay evidence: ordinary unmatched abstentions keep the review-only copy, deterministic decisions and empty abstracts omit it, and conflict abstentions reuse already matched abstract evidence. Minimal source repair `b56e8f2cb6892023e0d453f38fca701c00853db3` adds the matched-evidence exclusion to `review_abstract_note`: steward review and nonblank text remain required, and the review-only projection is now suppressed when `field_values` already contains `abstract_note`. Classification rules, conflict semantics, lifecycle state, source denominator and governance authority are unchanged.

A seventh exact-evidence sequence addresses tag review `PRRT_kwDOUKg5E86fT2sL`. Committed RED `2ba8c1f5b1b2ce712a298d493233f5ba93dfb242` proves that although the earlier per-tag repair prevents cross-tag phrase synthesis, `ClassificationEvidence.field_values` retains only the first source tag under the shared `tags` key when separate tags support different rule families. Minimal source repair `bdad99c39a4b87e17a8debe7502250948a01b07b` adds parallel `matched_tag_values`, collects only tags that actually matched rule phrases, then replays them in source order while deduplicating equal values; the compatibility `field_values` shape, phrase matching, disposition/conflict semantics, lifecycle state, denominator and governance authority remain unchanged. Test successor `1532e80303153dbeb522a7170f118f1e0935613c` adds reversed-source-order plus duplicate-tag coverage so family iteration order cannot masquerade as source order. The unrelated deadline-test rationale accidentally omitted during whole-file replacement was restored at `afd2f0b4812f4d29a1850339b6e5431c7a9b028a`; no runtime semantics changed in that restoration.

All seven sequences are now source/test repaired pending current-head execution. Exact pre-baseline head `afd2f0b4812f4d29a1850339b6e5431c7a9b028a` has no executable Rust evidence in this runtime, which lacks a Rust toolchain. Rust 1.98 workspace tests, fmt, all-target Clippy, warnings-denied rustdoc/release, owned-production 100% function/normalized-region/branch coverage and applicable hosted checks remain mandatory on one unchanged successor head before review resolution or Ready. Earlier per-tag boundary, repeated DOI-wrapper, CLI and security/durability repairs remain ancestors but do not inherit executable GREEN after later head changes.

Independent evidence-integrity gaps remain separate: matched-tag replay, duplicate-source provenance, fragmented-header transport, production CLI/source coverage and single-copy abstention replay all require current-head executable evidence; the three standalone PDFs plus one standalone note remain governed pending sources until exact-snapshot restoration/identity/digest reconciliation. Zero pending keys, deterministic classification, duplicate grouping or local report serialization never grants semantic or write authority.

## September 6 source inventory checkpoint

The existing #9 owner now retains every nonbibliographic metadata record and derives unresolved ancestry rather than silently discarding standalone sources. [Source-scope doctoring](doctoring/zotero_source_scope.md) binds committed REDs, final source `1e95d6eb979e66ecb7dae4f81f18a6b0a91b7624`, **47 tests / 10 unfiltered suites**, strict checks and the unchanged coverage gate. The earlier inventory executable at `48c3525` genuinely reads 8,326 records into 3,715 unchanged bibliographic proposals plus 4,611 other records, with exactly the four previously audited standalone identities pending. A later shared-reader guard also rejects blank identities; no actual final-guard executable replay is implied.

The earlier source findings are repaired locally, not yet propagated into root #39. Required downstream restoration/identity accounting, pending-source reconciliation, approval binding and full-library completion gates remain open; neither zero pending keys nor successful classification grants semantic or write authority. Historical source scope, authentic worksheet decisions/independent approvals 0/3,715, plus four unresolved sources remain distinct. This checkpoint does not imply protected merge or release.

## Protected truth and active stack

PR #9 exact owner verification checkpoint: `e8f7f83ee0d7f7ca3d2bb0b655040974786c1e6c` (locked Rust 1.98 tests, strict Clippy, warnings-denied rustdoc, fmt and diff checks passed locally).

The same exact owner run also executed `./scripts/check_coverage.sh` with `nightly-2026-08-20`: all tests passed, but the owned gate is RED at 168/179 functions, 1,007/1,149 normalized regions, and 143/162 normalized branch outcomes. Uncovered paths include source-resolution admission and CLI/error branches; no coverage exclusion or threshold change is accepted.

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

Fresh September 8 active roots:

1. Foundation PR #1 — exact head `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open/mergeable. Product CI still cannot materialize from protected `main` because that branch does not yet contain `.github/workflows/product.yml`; #35 remains its direct bootstrap prerequisite.
2. Product-CI bootstrap PR #35 — exact head `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft/mergeable. Security Scan and SAST have retained terminal success while historical CodeQL/OpenCode/Strix failures and Noema `CHANGES_REQUESTED` are not current acceptance. Router schedule materialization recovered; `.github#814` owns the exact review-request traversal/exclusion/claim/receipt evidence gap. Keep the leaf head stable unless a real ConceptWeave defect appears.
3. Client Consumption PR #5 — exact head `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open/mergeable on Foundation. It retains provider-independent semantic-release admission, integrity, compatibility, diff/resolution and supersession validation; current protected exact-head evidence remains independently required.
4. Source Observation PR #6 — exact head `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`, Draft/open/mergeable on Client #5. Single-use authorization and PostgreSQL UNIQUE null-comparison semantics are source-repaired with local exact-head Rust evidence; hosted evidence, protected prerequisites and the concrete bounded read-only PostgreSQL adapter remain outstanding.
5. Zotero Research Classification PR #9 — live exact head `e8f7f83` (full head from the live PR) follows matched-tag source repair `bdad99c39a4b87e17a8debe7502250948a01b07b`, order/dedup regression `1532e80303153dbeb522a7170f118f1e0935613c`, matched-tag RED `2ba8c1f5b1b2ce712a298d493233f5ba93dfb242`, single-copy abstention source repair `b56e8f2cb6892023e0d453f38fca701c00853db3`, and earlier fragmented-header, duplicate-source provenance, lifecycle, non-ASCII abstention, ordinary/conflict abstention replay, per-tag phrase boundary, nested DOI, source-inventory, blank-key, item-version, deadline, private/atomic report and CLI repairs. Exact-head Rust 1.98 tests, strict Clippy, warnings-denied rustdoc, fmt and diff checks pass locally; hosted checks, independent approval and owned coverage evidence remain pending. No predecessor executable evidence transfers beyond the exact-head run.
6. Full-text/write/recovery terminal PR #39 — fresh live head `45636773b9ba6fececa2e31cbe873f0e6491289f`, Draft/open/mergeable on exact #38 base `7678236ed3ec467e93b97bb2ad7ad26b3dc0e5b9`. Its PR body still labels older `ef05b8183c28bc445f1ea6f82c277b52bb00c0d2` as current and explicitly withdraws previously reported unique-repository coverage counts after duplicate audits. Treat its inventory/count checkpoints as dated or reconciliation-pending evidence rather than current KPIs; production/runtime GREEN remains bound to the explicitly verified ancestor rather than later docs-only heads.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_CI | The canonical domain and semantic-candidate contract separate epistemic truth from governance publication state. Research Intake now projects proposal-only `proposed` values separately, but exact-head Rust/hosted evidence remains required before the P1 can resolve. |
| Research abstention semantics | SOURCE_REPAIRED_PENDING_CI | Ordinary accented/scientific characters no longer alone imply unsupported vocabulary; wholly non-ASCII alphabetic metadata remains a bounded current-rule-set control. Exact-head Rust/coverage evidence remains required before resolving the review. |
| Research abstention replay evidence | SOURCE_REPAIRED_PENDING_CI | The original no-match replay RED `1c836870...` is source-repaired by `2400afe...`; narrower conflict-path RED `43a181...` is source-repaired by `b56e8f2...`, which suppresses the redundant review-only abstract when exact matched evidence already retains it. Exact-head Rust/coverage/hosted evidence remains required before resolving the review. |
| Research tag replay evidence | SOURCE_REPAIRED_PENDING_CI | RED `2ba8c1f...` is source-repaired by `bdad99c...`: `matched_tag_values` now carries all and only matching source tags in source order with value deduplication while retaining the existing compatibility field. `1532e803...` covers reversed source order and duplicate values. Exact-head Rust/coverage/hosted evidence remains required before resolving the review. |
| Research duplicate provenance | SOURCE_REPAIRED_PENDING_CI | RED `d0e87d...` is source-repaired by `bfbe3596...`: reversible DOI/title candidates retain exact per-item source identity values alongside normalized identity and keys. Exact-head Rust/coverage/hosted evidence remains required before resolving the review. |
| Research transport test harness | TEST_REPAIRED_PENDING_CI | Pre-fix one-shot `TcpStream::read` could observe only a request prefix. `fd488421...` reads deliberately fragmented bytes until the HTTP header terminator with closure and 16 KiB bounds; exact-head Rust/hosted evidence remains required before resolving P2. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, targeted delete-column provenance, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and registry-authorized opaque source identity exist. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Current exact-head protected evidence and prerequisite integration remain outstanding. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | CONSUMER_REVALIDATION_PENDING | The earlier public non-fork exact-range HTTP 403 was traced to an uninitialized repository dependency graph, not to a retryable central workflow defect. `.github#1873` was closed unmerged after enabling Dependabot vulnerability alerts initialized affected graphs and the same exact comparison returned HTTP 200. The hard gate remains fail closed; a current ConceptWeave head must still execute the pinned Dependency Review action successfully before acceptance. |
| Review / runner admission | BLOCKED_OWNER | #35's Router schedule now materializes and successful organization sweeps exist, but the exact review request still lacks durable request-level visit/exclusion/claim/receipt evidence. `.github#814` owns that central gap; it blocks that validation lane only and is not a reason to stop repository-owned semantic work. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring remains bound to authoritative standards/primary research and exact implementation contracts; hosted exact-head evidence remains independently required after head changes. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Dependency Review incident correction

The prior Foundation predecessor exposed a real hosted failure: the authenticated Dependency Review compare preflight returned HTTP 403 for a public, non-fork ConceptWeave exact range. The initially proposed central repair retried the same token-bound request while retaining fail-closed behavior.

Fresh owner RCA invalidated that causal hypothesis. The same authenticated exact-range request returned HTTP 200 for a repository whose dependency graph was initialized and HTTP 403 for affected repositories whose graph was not initialized. Enabling Dependabot vulnerability alerts initialized the dependency graph in ConceptWeave and pingora-gateway, after which the exact compare endpoint returned HTTP 200. Therefore `.github#1873` was correctly closed without merge: retries would extend queue occupancy but would not establish repository capability.

Acceptance remains stricter than the RCA. HTTP 200 availability alone is not GREEN. A fresh exact ConceptWeave consumer run must reach and complete the pinned Dependency Review action; 403, transport failure, skipped substitution or a sibling scanner cannot satisfy the hard gate.

## Central control-plane evidence

Protected central source is `.github/main@78a4937c684a54ca8e415822c913742f41c6efc4` at this snapshot. This is evidence only, not a mutable ConceptWeave dependency.

- The current central source contains subsequent queue/admission, review-router and CodeQL compatibility repairs. Historical leaf failures are not current ConceptWeave acceptance.
- `.github#1873` is closed/unmerged because repository dependency-graph initialization, not its retry/sleep source delta, was the verified root cause of the observed public-repository 403.
- #35 remains the exact consumer canary. Review Agent Mention Router schedule materialization has recovered, while `.github#814` owns durable exact-request traversal/exclusion/claim/receipt evidence and the current review publication gap.

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local registry/credential resolution; explicit read-only session/transaction; exact schema allowlist; total operation and statement deadlines; cancellation plus row/byte/concurrency budgets; complete immutable snapshot or fail closed; source-disappearance handling; deterministic replay against a frozen anonymized GRC-shaped fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; do not infer business authority from relational structure alone.
5. **LLM Proposal** — every production model call through a released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
12. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
