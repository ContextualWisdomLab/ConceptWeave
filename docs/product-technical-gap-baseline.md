# Product / Technical Gap Baseline

**Snapshot:** 2026-09-04

This branch is the Client Consumption child of Foundation PR #1. Exact PR/run coordinates are evidence snapshots, never mutable production dependencies. Protected/live GitHub state is authoritative when it advances after this snapshot.

## Stack authority

- Protected/default `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; no ConceptWeave release exists.
- Foundation PR #1 advanced by documentation-only live-state repair from `bba351b77bf5f1ab5cfd55979fbb2bd158f78b81` to `447aa0723abd7b582b9acc478ed90238d0d59214`. The delta between those heads is only this gap-baseline file. Current Foundation Product `33835025486`, job `100905656541`, is queued before runner assignment; predecessor Product/SAST success and Security failure remain historical evidence only.
- Client PR #5 pre-restack head `cd99eb4a42011206f8efa376106aa4b121d2010e` is Draft. Product `33822984126`, job `100869494950`, remains queued before runner assignment and is superseded for acceptance by this Foundation-adoption restack.
- Source Observation PR #6 pre-restack head is `1fdfb3af14c126c270861eb541e9e57d47418bb8`; Product `33834639272`, job `100904527699`, remains queued before runner assignment. Its production registry-key validation is intentionally unchanged pending the real semantic RED.

## Client Consumption capability status

| Gap | Status | Evidence / invariant | Next verification |
| --- | --- | --- | --- |
| Offline release admission | IMPLEMENTED_PENDING_CURRENT_HEAD | `SemanticReleaseClient` requires explicit compatibility plus Published + Authoritative and remains provider/network independent. Downstream tenant/purpose authorization and physical execution remain downstream. | Exact-head Rust tests/Clippy/rustdoc/coverage after restack. |
| Versioned semantic release | IMPLEMENTED_PENDING_CURRENT_HEAD | `contracts/semantic-release.schema.json` plus fixtures and Rust `SemanticRelease` carry release/contract/ontology identity, truth/publication state, digest identity, provenance and unique concept IDs. | Draft-2020-12 schema and Rust parity. |
| Detached artifact integrity | REPAIRED_PENDING_CURRENT_HEAD | Predecessor `0c32a7b55d3c687ab76cee789962866573496ba1` produced hosted `E0599` because tests required `verify_detached_artifact` while production exposed the retired API. `9c278598001c502a733100d11e901538c3dc2677` made the minimal API/rustdoc repair. | Exact-head Clippy/tests/rustdoc/coverage. |
| Public documentation parity | REPAIRED_PENDING_CURRENT_HEAD | Product `33741224641`, job `100603361888`, executed exact `1e543bb...`, passed CI/fmt/Clippy and failed the documentation contract because the gap baseline itself reproduced the retired identifier. `fa0e31272097d154427ac53bfb7cc60dc96e72c8` removed that self-reference; `cd99eb4...` then repaired stale sibling Source Observation state. | Current restacked Product gate. |
| Explicit compatibility | IMPLEMENTED_PENDING_CURRENT_HEAD | Current / SupportedLegacy / Unsupported are explicit; unknown versions fail closed and no ordering inference is used. | Unknown/legacy/current edge cases. |
| Deterministic diff / resolution | IMPLEMENTED_PENDING_CURRENT_HEAD | Release diff reports deterministic sorted concept changes; exact concept resolution has no fuzzy/model behavior. | Golden diff and unknown/blank resolution cases. |
| Immutable supersession validation | IMPLEMENTED_PENDING_CURRENT_HEAD | `SemanticReleaseReference`, `ReleaseSupersession` and `validate_supersession` bind exact predecessor/successor IDs and digests, nonblank rationale, no self-supersession and authoritative-use admission. Authority to issue a governed supersession receipt remains Governance & Publication. | Exact-head tests plus public cross-language contract. |
| Language-neutral supersession/publication contract | INTENTIONAL_RED_PENDING | Test-first lineage beginning `143506eb66b6904b770a628ac793af2253559df2` requires `contracts/semantic-release-supersession.schema.json` and valid/invalid fixtures. Production artifacts remain intentionally absent. | Observe the missing-schema/fixture RED only after the restacked head reaches that public-contract boundary, then make the smallest generic repair. |
| Match / align / explain | GAP | OLaLA/LLMs4OM/Complex Matching/MILA/KROMA/LLM4VKG research is mapped to retrieve/filter/match and evaluation. LLM output is proposed only and any production call must use released `contextual-orchestrator`. | OAEI-style P/R/F1, retrieval recall, abstention, reproducibility and LLM-call reduction. |
| Query-plan contract | GAP | ConceptWeave may define semantic plans but cannot own downstream physical authorization/execution. | Versioned DTO + GRC round-trip without cross-service SQL. |

## Sibling Source Observation state

The Source Observation bounded context already contains immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifier preservation, canonical lowercase SHA-256 snapshot identity, UTC provenance, typed exact receipts, explicit resource budgets/cancellation/failures and an opaque bounded source registry key at the port boundary.

The prior UTC RED executed on `2817df62...` and was repaired by `e27ffaf4a40d746781b8012e9fe71467e7e6511f`. A later cross-boundary test on `c9af2255...` requires `PostgresSchemaSnapshot::new` to enforce the same ≤128-byte lowercase multiword `snake_case` registry identity as `ObservationRequest`; its hosted run first failed at formatting before reaching the semantic test. Test formatting was repaired, an accidental whole-file write was neutralized by a non-force forward commit whose tree compared exactly equal to the intended predecessor, and `1fdfb3af...` now contains only the remaining rustfmt wrapping hunk. The semantic production validator still must not land before the real RED.

After registry RED → minimal fix → exact-head GREEN, the next Source Observation buyer slice is a maintained Rust read-only PostgreSQL adapter behind `conceptweave-source-port`, with adapter-local credential resolution, explicit schema allowlist, statement timeout, cancellation, row/byte/concurrency budgets, complete-or-fail snapshot construction and a frozen anonymized GRC-shaped replay fixture.

## Central control-plane evidence

Protected central source is `.github/main@07d9ec23fb265c76539d23249e1dfa124ea7b23b` at this snapshot; this is evidence, not a ConceptWeave dependency.

- `.github#810` owns authoritative Dependency Review availability. OSV/Trivy/Scorecard/SAST are not substitutes and 403 cannot be treated as success.
- `.github#712/#1531` own selective/intermittent runner admission and review/queue amplification.
- `.github#1796` has test-first Draft #1821 `test/1796-org-sweep-queue-owner@9c79cf775ad6a125a94dedcae9683c20a65a0339`, which separates organization-sweep queue inventory from target-repository exact-head coalescing. Production central source remains unchanged until its RED executes.
- `.github#1822@7a5cc1b1c43946d210405cd051ae629ff2c44966` is a separate Draft fix for documented `CoalescingRefused` safe-no-op behavior; other exceptions remain fail closed.
- Fresh central queued inventory reached 1,906 runs. Aggregate backlog movement is diagnostic only; acceptance still requires actual runner assignment, exact checkout and terminal evidence on unchanged current heads.

## Remaining P0 gaps

1. Observe and repair the language-neutral supersession/publication contract RED, then prove exact-head GREEN.
2. Complete signature/provenance verification after Governance & Publication defines a stable signing contract.
3. Add relation, physical-mapping, dimension and measure resolution plus a versioned semantic query-plan contract.
4. Complete deterministic/research-backed match, alignment and explanation with optional bounded contextual-orchestrator assistance only.
5. Add GRC reference fixtures that exercise only released/versioned `semantic_release` contracts while GRC retains business truth, tenant/purpose authorization and physical execution.
6. Complete ontology/semantic-layer discovery, validation, governance persistence, steward review, publication adapters, multilingual evaluation, observability/recovery and immutable release evidence in their owning bounded contexts.

## DDD and release invariants

- No generic `utils/helpers/services/common` domain buckets.
- Client Consumption depends only on governed public release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- No source copying, cross-service SQL or mutable foreign-head dependency.
- Published semantic truth is immutable; correction creates a distinct successor plus supersession evidence.
- Release requires exact protected head, version/CHANGELOG/tag/package/immutable `semantic_release`, SBOM, provenance, reproducibility and rollback evidence.
