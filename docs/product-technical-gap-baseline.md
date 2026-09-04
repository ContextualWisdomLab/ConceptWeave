# Product / Technical Gap Baseline

**Snapshot:** 2026-09-04

This file records code-current product/technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. GitHub live branch/protected-branch state remains authoritative whenever it has advanced after this snapshot.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap README is shipped there and no ConceptWeave release exists yet.

The active dependency stack is:

1. Foundation PR #1 — pre-update exact head `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81`, open/non-Draft/mergeable. Repository Product `33527150325` and SAST `33527150417` succeeded on that exact head. Security `33527150445` failed closed at authoritative Dependency Review availability after real runner assignment; OSV/Scorecard/Trivy do not substitute for that gate. Required OpenCode and substantive Strix remain non-passing central lanes. This baseline repair legitimately creates a newer documentation head, so predecessor terminal evidence does not transfer.
2. Client Consumption PR #5 — `cd99eb4a42011206f8efa376106aa4b121d2010e`, Draft/open/mergeable and stacked on Foundation. Product `33822984126`, job `100869494950`, is queued before runner assignment. Detached-artifact API and public-document drift have real hosted RED→repair lineage. The next intended RED is the missing language-neutral supersession/publication JSON Schema plus valid/invalid fixtures, only after this exact head reaches that boundary.
3. Source Observation PR #6 — `1fdfb3af14c126c270861eb541e9e57d47418bb8`, Draft/open/mergeable and stacked on Foundation. The prior UTC provenance RED executed and was repaired. The current registry-identity test proves that immutable snapshot/source-receipt provenance must not bypass the Source Observation port's opaque ≤128-byte lowercase multiword `snake_case` registry-key boundary. Predecessor `c9af2255...` reached a real hosted runner but failed at formatting before the semantic test. Current `1fdfb3af...` contains only the remaining rustfmt wrapping repair; Product `33834639272`, job `100904527699`, is queued before runner assignment. Production registry-key validation remains intentionally unchanged until the intended semantic RED executes.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases, and the stable Client contract. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | ACTIVE_PR | Rust domain lifecycle enforces explicit governance authorization at steward/publication boundaries, immutable publication and supersession semantics, and evidence-bound authoritative state. All returned inline review threads are resolved. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, canonical snapshot digest, UTC provenance, exact receipts, bounded request budgets/cancellation and opaque source registry keys exist. Registry-key consistency at the immutable snapshot boundary is the current TDD lane. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, explicit compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and supersession validation exist. Language-neutral public supersession/publication schema/fixtures remain intentionally absent pending current-head RED. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | BLOCKED_OWNER | Foundation Security fails closed because GitHub Dependency Review availability is not satisfied. `.github#810` owns the authoritative central repair; scanner substitution and 403-as-success are forbidden. |
| Review / runner admission | BLOCKED_OWNER | Selective/intermittent hosted-runner admission remains observable. Foundation siblings have acquired runners while OpenCode/Strix substantive lanes remain queued; current #5/#6 Product lanes are also pre-runner queued. `.github#712/#1531/#1796` own the central queue/review amplification paths. |
| Standards / research | ACTIVE_PR | Stable recommendations remain distinct from drafts. As of 2026-09-04 authoritative W3C history still lists RDF 1.2 Concepts as Candidate Recommendation Snapshot (2026-04-07) and SHACL 1.2 Core as Working Draft (2026-08-03); SHACL 1.2 Rules has a newer 2026-08-19 Working Draft. Apache Ossie remains incubating and its current v0.1 specification is pre-first-Apache-release, so it is tracked as emerging interop rather than a final standard. Detailed APA/capability mapping stays in `docs/doctoring/REFERENCES.md` and `RESEARCH_CAPABILITY_TRACEABILITY.md`. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Central control-plane evidence

Protected central source is `.github/main@07d9ec23fb265c76539d23249e1dfa124ea7b23b` at this snapshot. This is evidence only, not a ConceptWeave dependency.

- `.github#1796` now has test-first Draft PR #1821 `test/1796-org-sweep-queue-owner@9c79cf775ad6a125a94dedcae9683c20a65a0339`. The test contract removes repository-wide queued/in-progress Actions inventory and `ORG_SWEEP_STALE_QUEUE_HOURS` ownership from the organization sweep while retaining exact live-head coalescing in the target repository. Production central source is unchanged until the RED executes.
- `.github#1822@7a5cc1b1c43946d210405cd051ae629ff2c44966` is a separate Draft lane that makes top-level `CoalescingRefused` honor the coalescer's documented safe-no-op behavior while preserving fail-closed handling for other failures. Do not conflate it with #1821's duplicate-owner removal.
- Fresh `.github` queued inventory observed `1,906` runs on 2026-09-04, above the earlier ~1,604 trough. Aggregate queue movement is diagnostic only; consumer GREEN requires actual runner assignment, exact checkout and terminal evidence on unchanged current heads.

## P0 product gaps after the current TDD lanes

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local credential resolution; explicit read-only mode; statement timeout, cancellation, row/byte/concurrency budgets; complete immutable snapshot or fail closed; deterministic replay against a frozen anonymized GRC-shaped fixture.
2. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
3. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; do not infer business authority from relational structure alone.
4. **LLM Proposal** — every production model call through released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
5. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
6. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning, explicit unsupported-feature failure.
7. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
8. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
9. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
10. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
11. **CWL integration** — only released/versioned semantic_release/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
12. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
13. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
