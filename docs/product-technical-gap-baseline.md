# Product / Technical Gap Baseline

**Snapshot:** 2026-09-05

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot. Refresh the changed PR's metadata after each documentation commit; evidence from another PR or an earlier head does not transfer.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

The active roots observed immediately before this baseline refresh are:

1. Foundation PR #1 — exact head `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open. The current Foundation makes Product CI draft-aware while keeping Ready/non-Draft quality requirements intact. Product CI still cannot materialize from protected `main` because that branch does not yet contain `.github/workflows/product.yml`.
2. Product-CI bootstrap PR #35 — exact head `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft. It adds the pull-request form of Product CI and removes no-op closed/converted-to-Draft triggers. Scope detection and review admission have executed successfully, while CodeQL, Semgrep, Noema, Strix, Trivy and Scorecard remain queued. The workflow-only diff skips Dependency Review and OSV; these skips do not prove Foundation's dependency-changing checks. No independent approval exists for this head.
3. Client Consumption PR #5 — exact head `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open. The current source retains language-neutral semantic-release admission, integrity, compatibility, diff/resolution and supersession validation. Previously valid review findings are source-repaired, but current protected evidence remains independently required.
4. Source Observation PR #6 — exact head `51a7344c6b159df8daaf2fca6540f7b712f5f8c6`, Draft/open. PostgreSQL targeted `ON DELETE SET NULL (...)` / `SET DEFAULT (...)` column provenance and registry/ACL-resolved source identity are source-repaired. The next P0 slice is the concrete bounded read-only PostgreSQL adapter.
5. Zotero Research Classification root PR #9 — exact head `256076d12dec80997960b1db89bec0809f129c90`, Draft/open. Its dependent stack ends at review-batch PR #34, measured at `062a0d9bca086d5a2aaa5d4122f58364115d4f91` before this documentation repair. The stack remains proposal/review oriented and does not elevate local classifier output to semantic authority.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and the public Draft 2020-12 semantic-candidate schema enforce compatible publication-state/truth-status semantics. Hosted exact-head Product evidence still requires the bootstrap workflow on protected `main`. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, targeted delete-column provenance, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and registry-authorized opaque source identity exist. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Current exact-head protected evidence and prerequisite integration remain outstanding. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | CONSUMER_REVALIDATION_PENDING | The earlier public non-fork exact-range HTTP 403 was traced to an uninitialized repository dependency graph, not to a retryable central workflow defect. `.github#1873` was closed unmerged after enabling Dependabot vulnerability alerts initialized affected graphs and the same exact comparison returned HTTP 200. The hard gate remains fail closed; a current ConceptWeave head must still execute the pinned Dependency Review action successfully before acceptance. |
| Review / runner admission | PENDING_CURRENT_CHECKS | #35's scope/admission jobs have executed successfully, while scanner and model-review jobs remain queued. The active rules require one approving review, resolved review threads and seven central workflows. Queueing is not a reason to stop repository-owned work. |
| Zotero Research Intake | CAMPAIGN_INCOMPLETE | PRD FR-9 and ADRs 0006/0007 have executable local proposal, review, dry-run and recovery contracts. Saved-snapshot proposals cover 3,715/3,715 bibliographic items; unverified steward decisions and externally approved labels both remain 0/3,715. See the campaign evidence below. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring remains bound to authoritative standards/primary research and exact implementation contracts; hosted exact-head evidence remains independently required after head changes. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Dependency Review incident correction

The prior Foundation predecessor exposed a real hosted failure: the authenticated Dependency Review compare preflight returned HTTP 403 for a public, non-fork ConceptWeave exact range. The initially proposed central repair retried the same token-bound request while retaining fail-closed behavior.

Fresh owner RCA invalidated that causal hypothesis. The same authenticated exact-range request returned HTTP 200 for a repository whose dependency graph was initialized and HTTP 403 for affected repositories whose graph was not initialized. Enabling Dependabot vulnerability alerts initialized the dependency graph in ConceptWeave and pingora-gateway, after which the exact compare endpoint returned HTTP 200. Therefore `.github#1873` was correctly closed without merge: retries would extend queue occupancy but would not establish repository capability.

Acceptance remains stricter than the RCA. HTTP 200 availability alone is not GREEN. A fresh exact ConceptWeave consumer run must reach and complete the pinned Dependency Review action; 403, transport failure, skipped substitution or a sibling scanner cannot satisfy the hard gate.

## Central control-plane evidence

Protected central source is `.github/main@6d7fbebec8aec31d88a30a36e71ca5b3925d241d` at this snapshot. This is evidence only, not a mutable ConceptWeave dependency.

- The current central source includes queue/admission and changed-scope/review-runtime repairs already integrated through ordinary protected history.
- `.github#1873@41935494aa234eb458f1cc08f006daaa278b9760` is closed/unmerged because repository dependency-graph initialization, not its retry/sleep source delta, was the verified root cause of the observed public-repository 403.
- #35 remains a consumer canary for runner admission and applicable workflow security checks. Its workflow-only change skips Dependency Review, so a dependency-changing Foundation run must separately prove that action's success. Already-created runs remain bound to their own central workflow revisions.

## Zotero research campaign evidence

### Current runtime transition and integrity gates

A fresh read at `22030ae6c8510d9eb8f7b07d98959bb69d2bd286` observed Zotero 10.0.1, API 3/schema 44, library version 2 and a present server identity. It produced a distinct report/worksheet pair without overwriting the historical artifacts below. The full read still counted 8,326 records and 3,715 bibliographic proposals, with 56 adjacent-evidence proposals, one semantic-consumption bridge, 3,658 abstentions and 49 duplicate candidates. The new worksheet's aggregate checkpoint remains 0/3,715, incomplete. Equal totals do not prove unchanged content across the Zotero 9-to-10 version-space transition.

Both new files have mode `0600`. The report is 6,890,050 bytes with file SHA-256 `d56c8ac70da7f094355748f6611ba47f9d2256bb87f0e24ab84683536e56fb9e`; the worksheet is 1,640,941 bytes with file SHA-256 `919ad3b875846c018eb92df2b2caf5d9a8ed491ede0b4718c07e56dc69bca0d9`. Their implementation-reported snapshot digest is `sha256:bcc50fdf4e16789e7d2651b431817dfc178fdcaad7e0c73360fda2d83351d7b5`, which is not yet proof of complete raw-field binding.

PR #10's existing findings remain source-confirmed at this capture head: provider fields omitted by the typed input are lost before raw-snapshot hashing, and mutable report predictions can be evaluated under an unchanged approval receipt. These invalidate stronger integrity claims, not the observed aggregate counts. Repair the canonical owner, propagate through the stack and regenerate before approval/promotion. Zotero 10 availability alone does not resolve the loopback confidentiality finding or grant write authority.

The [CWL ontology capability inventory](doctoring/cwl_ontology_capability_inventory.md) records eight bounded owner candidates with exact default heads. Seven returned no GitHub release; RankWeave v0.18.0 resolves to an exact commit but does not prove Rust runtime availability or ConceptWeave adoption. `context-graph-contracts` and `enterprise-architecture-core` use protected `develop`, not `main`, as their default adoption baseline. This is owner-selection evidence, not an organization-wide census or permission to bypass missing releases.

### Historical Zotero 9 snapshot

The following record preserves earlier measurements and execution guidance. It does not authorize applying the old worksheet or batch to the current Zotero 10 snapshot.

The parent integration ending at `062a0d9bca086d5a2aaa5d4122f58364115d4f91` replaced the baseline with Foundation's document and removed the research section present at `a84e6d49aba2a4fd0b0ef303a342922c4ce909bb`. This section restores FR-9 traceability using the saved private artifacts and the current executable. It preserves Foundation's updated status. These records must survive later parent integrations alongside the Foundation, Client and Source Observation evidence.

On 2026-09-05, the existing `--review-progress` command at `062a0d9...` revalidated the original report and worksheet offline. The report still binds Zotero 9.0.6, API v3/schema 42, library version 12341, rule `ontology-research-v2`, and snapshot `sha256:c49b08066c4526e520a5f85416543ea20a620a06170e1e15f563088f6bc9e162`. This replay does not claim the mutable Zotero library is still at that version. All three original artifacts retained their hashes and remained outside the repository.

| Measure | Saved-snapshot result | Meaning / remaining work |
| --- | --- | --- |
| Observed records | 8,326 | The original read completed without reported failures; four top-level non-bibliographic items are excluded from the classification denominator. |
| Proposal and provenance coverage | 3,715/3,715 each | Every bibliographic item has a proposal and source coordinates. These counts do not establish classification correctness. |
| Proposed dispositions | 56 adjacent evidence; 1 semantic-consumption bridge; 3,658 abstentions | Deterministic evidence leaves unsupported meaning for review. |
| Duplicate candidates | 49 | Reversible candidate groups, with no record merge or deletion. |
| Unverified worksheet coverage | 0/3,715 | The replay returned `remaining_count=3715` and `complete=false`. |
| First pending batch | 0/25 decisions filled | The existing 32,940-byte batch is a repeatable review view, not an assignment or a completed review. |
| Externally approved full-review coverage | 0/3,715 | No completed full review or externally verified approval receipt is available in this campaign. A sample cannot satisfy this measure. |
| Live write / rollback evidence | Not performed | The saved report originated from Zotero 9.0.6; Zotero 10 adapter tests do not prove approved live execution. |

Artifact SHA-256 values for reproducibility (no bibliography or item identities):

- report: `ff13383b88f89fcef94d2f2d7284838b268fb871bed78c75ce5b53bfab2138a8`;
- worksheet: `ad32c8352cb7d84ac3bdcd3a60c975f61e2e19adc3a8294d4c680360071e752b`;
- pending batch: `7d1a77bd6913bd8c0c826ab60c1a4fa31afede7f7e7e694aad351c31c710b921`;
- replayed aggregate progress: `ac79c719037aca2d67bb4b0ea7e84babd8a701506a7d2ec274139d260328f524` (262 bytes, mode `0600`).

The next campaign step is authentic steward input. After all 25 decisions in the existing batch are filled, `--apply-review-batch` must reconstruct the pending view from the original report/current worksheet, validate every displayed context field, reject unknown fields and blank/abstention decisions, and produce a separate owner-only worksheet. Directly reading that rich batch through `--apply-decision-patch` is rejected because it would discard the displayed context. A successful first application should produce unverified progress of 25/3,715, with 3,690 remaining; no such result is claimed here. The same process must cover all bibliographic items before external full-review approval. PRD FR-9, TRD's Research Intake contract and [ADR 0006](adr/0006-zotero-research-intake.md) define these boundaries.

[ADR 0007](adr/0007-reviewed-zotero-write-plan.md), [the threat model](../THREAT_MODEL.md) and [PR #18's unresolved transport finding](https://github.com/ContextualWisdomLab/ConceptWeave/pull/18#discussion_r3935881086) retain the remaining write boundary: `Zotero-Server-ID` checks database continuity but does not authenticate the loopback peer or protect a key from a hostile process occupying its port. Enterprise-secure live write-back cannot be claimed without protected provider transport or explicit governance acceptance of that remaining risk, followed by approved write, partial-failure and rollback evidence.

ConceptWeave remains the owner of research intake and semantic-model generation. `semantic-data-portal` owns catalog/consumption, `context-graph-contracts` owns versioned interop contracts, and `contextual-orchestrator` owns model calls. These are the existing Context Map boundaries, not claims of released integration. ConceptWeave has no immutable release or verified released consumer adoption yet. A separate Utility Repository has no evidenced independent consumer or deployment contract at this snapshot.

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
