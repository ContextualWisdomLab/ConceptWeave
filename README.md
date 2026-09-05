# ConceptWeave

## Local Zotero research proposal

With Zotero running locally:

```sh
cargo +1.98.0 run --bin conceptweave-zotero -- /tmp/conceptweave-zotero-classification.json
```

The command reads one stable library-version snapshot and creates a local, reviewable JSON report. On Unix, output is restricted to a new owner-only (`0600`) direct child of canonical `/tmp` or the system temporary directory; the CLI fails closed on other platforms. The command never changes Zotero records.

Apply a small steward-reviewed decision patch to a new worksheet without overwriting the current one:

```sh
cargo +1.98.0 run --bin conceptweave-zotero -- --apply-decision-patch /tmp/report.json /tmp/current-worksheet.json /tmp/patch.json /tmp/updated-worksheet.json
```

All three inputs must be separate owner-only files from the same immutable snapshot. The output is create-new and owner-only; this offline step neither changes Zotero nor grants governance approval.

Create a small deterministic view of the next pending records for human review:

```sh
cargo +1.98.0 run --bin conceptweave-zotero -- --review-batch /tmp/report.json /tmp/current-worksheet.json 25 /tmp/review-batch.json
```

The batch repeats on unchanged input and is not a reservation or assignment. It contains sensitive bibliographic context and must remain owner-only. After a steward fills every `reviewed_disposition`, validate the complete displayed context and create a new worksheet:

```sh
cargo +1.98.0 run --bin conceptweave-zotero -- --apply-review-batch /tmp/report.json /tmp/current-worksheet.json /tmp/review-batch.json /tmp/updated-worksheet.json
```

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/ConceptWeave)

**Automatic, evidence-bound ontology and semantic-layer engineering for governed enterprise meaning.**

ConceptWeave turns heterogeneous enterprise evidence—schemas, APIs, event contracts, documents, code structure, vocabularies, and lineage—into **reviewable semantic-model candidates**. Generated meaning never becomes authoritative merely because a model proposed it: candidates retain source evidence, pass deterministic validation, and move through an explicit review/publication lifecycle.

## Why it exists

Enterprise semantic models are valuable only when teams can explain where meaning came from, what was inferred, who reviewed it, and what is actually published. ConceptWeave makes that lifecycle explicit instead of collapsing discovery, generation, governance, and publication into one opaque step.

| Need | What ConceptWeave provides |
| --- | --- |
| Semantic discovery | Evidence-bound candidate concepts, relations, dimensions, measures, constraints, and mappings |
| Governance | Separate truth status from publication state with explicit review before publication |
| Traceability | Exact source-evidence bindings carried with semantic candidates |
| Deterministic validation | Machine-checkable structural and lifecycle invariants before authority changes |
| Interoperability | Versioned semantic packages and explicit integration boundaries |
| Safe LLM assistance | Proposal assistance only; model output is never publication authority |

## Product boundary

ConceptWeave owns the semantic-model engineering lifecycle:

```text
observe → discover → propose → validate → review → publish
```

Adjacent responsibilities remain separate:

- [`semantic-data-portal`](https://github.com/ContextualWisdomLab/semantic-data-portal) owns published semantic catalog, governance, and consumption surfaces.
- [`LineageWeave`](https://github.com/ContextualWisdomLab/LineageWeave) provides inferred/proposed lineage evidence.
- `context-graph-contracts` owns cross-product provider-neutral graph/event interoperability contracts.
- [`contextual-orchestrator`](https://github.com/ContextualWisdomLab/contextual-orchestrator) owns LLM/provider discovery and routing.
- Source systems remain authoritative for their own business data.

External source-analysis tools can sit behind adapters, but no external fork or model output becomes ConceptWeave product authority.

## First vertical

The first product vertical is **relational schema → governed semantic-model proposal**:

1. ingest an immutable schema snapshot;
2. derive observed physical entities and relationships;
3. propose concepts, semantic/taxonomy relations, dimensions, measures, constraints, and physical mappings;
4. bind every proposal to exact evidence;
5. validate structure and consistency;
6. require authorized review; and
7. publish a versioned semantic package only after the lifecycle permits it.

Future publication adapters may target standards such as OWL/RDFS/SKOS, SHACL, and JSON-LD. Emerging formats such as Apache Ossie are tracked as evolving interoperability targets rather than represented as finalized standards.

## Current implementation

The current foundation establishes the reusable domain and governance core rather than claiming the entire product is complete.

Implemented in this branch:

- Rust workspace and `conceptweave-domain` core;
- evidence-bound `SemanticCandidate` contract;
- independent truth-status and publication-state semantics;
- fail-closed candidate lifecycle with rejection and supersession paths;
- Draft 2020-12 JSON Schema for the public candidate contract;
- DDD Context Map and Ubiquitous Language;
- architecture, PRD/TRD, ADR, security, test, operability, and research baselines;
- pinned product CI for formatting, Clippy, tests, rustdoc, coverage, schema validation, lock freshness, and clean-tree checks.

Source adapters, LLM-assisted induction, persistence, reasoning, review UI, and publication adapters remain explicit product gaps until they land with evidence.

## Quick start

The repository is pinned to Rust 1.98.0. The current domain core has no third-party runtime dependencies.

```bash
cargo test --workspace
```

Run the full local quality set used by the foundation contract:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

The repository CI also validates the JSON Schema, lock/toolchain freshness, documentation contracts, and coverage expectations defined by the current source.

For the proposed local research intake, preserve available paper text separately from an existing private report:

```bash
cargo +1.98.0 run --locked -p conceptweave-zotero -- --capture-full-text /tmp/REPORT.json /tmp/CAPTURE.json
```

Zotero 10+ must be running. The report must be an unchanged owner-only file from that library; the capture path must be a new file directly in the system temp directory. Missing or partial text stays visible, and the command does not classify papers, approve decisions or modify Zotero. Keep both files private; see [operation and retry limits](OPERABILITY.md).

## Core contract

A semantic candidate is not the same thing as published semantic truth.

```text
Observed evidence
      │
      ▼
Semantic candidate
      │
      ├─ Draft
      ├─ Proposed
      ├─ Validated
      ├─ Reviewed
      └─ Published
```

Publication is an authority boundary. A candidate must preserve the evidence and lifecycle invariants required by the current domain contract; callers must not bypass those invariants by mutating public state or treating a validated proposal as published truth.

The machine-readable public shape is in [`contracts/semantic-candidate.schema.json`](contracts/semantic-candidate.schema.json).

## Architecture at a glance

```text
Enterprise evidence
 schemas · APIs · events · docs · code · vocabularies · lineage
                         │
                         ▼
┌──────────────────────────────────┐
│          ConceptWeave            │
│ semantic-model engineering       │
├──────────────────────────────────┤
│ observe / evidence normalization │
│ candidate discovery & proposal   │
│ deterministic validation         │
│ review / publication lifecycle   │
└───────────────┬──────────────────┘
                │ versioned published semantics
                ▼
     catalog / analytics / ontology consumers
```

ConceptWeave is the owner of semantic candidate engineering and publication lifecycle rules—not the catalog UI, source-system truth, LLM provider layer, lineage inference engine, or enterprise-wide application data.

## Standards and research posture

Stable standards and recommendations are distinguished from drafts and emerging specifications. LLM-assisted ontology engineering is treated as proposal assistance and must pass deterministic validation plus authorized review before publication.

The standards/research register and design implications live in [`docs/doctoring/`](docs/doctoring/) and are linked through the repository traceability documents.

## Documentation map

| Goal | Start here |
| --- | --- |
| Product requirements | [`docs/PRD.md`](docs/PRD.md) |
| Technical requirements | [`docs/TRD.md`](docs/TRD.md) |
| Architecture | [`ARCHITECTURE.md`](ARCHITECTURE.md) |
| Bounded contexts | [`docs/CONTEXT_MAP.md`](docs/CONTEXT_MAP.md) |
| Domain language | [`docs/UBIQUITOUS_LANGUAGE.md`](docs/UBIQUITOUS_LANGUAGE.md) |
| Lifecycle / sequence views | [`docs/UML.md`](docs/UML.md) |
| Architecture decisions | [`docs/adr/README.md`](docs/adr/README.md) |
| Security | [`SECURITY.md`](SECURITY.md) |
| Test strategy | [`TEST_STRATEGY.md`](TEST_STRATEGY.md) |
| Operations | [`OPERABILITY.md`](OPERABILITY.md) |
| Current product/technical gaps | [`docs/product-technical-gap-baseline.md`](docs/product-technical-gap-baseline.md) |
| Documentation home | [`docs/index.md`](docs/index.md) |

## Product principles

1. **Evidence before authority.** Semantic meaning remains traceable to source evidence.
2. **Proposal is not publication.** Discovery and LLM assistance cannot self-authorize semantic truth.
3. **Deterministic gates matter.** Lifecycle and structural invariants are executable contracts.
4. **Product boundaries stay explicit.** Integrations use contracts rather than copying adjacent product responsibilities.
5. **Standards claims stay precise.** Drafts and emerging specifications are never presented as stable standards.
6. **Current source is the truth boundary.** Planned adapters and open-PR behavior are not described as already shipped.

## Contributing

Before changing the domain contract or lifecycle, read [`AGENTS.md`](AGENTS.md), the PRD/TRD, architecture, applicable ADRs, and the current product-gap baseline. Behavioral changes should preserve the repository's test-first and evidence-bound publication discipline and update the matching public contracts/documentation in the same change.

## License

ConceptWeave is licensed under the [Apache License 2.0](LICENSE). Third-party tools and future adapters retain their own licenses and must satisfy the repository's commercial-use and attribution policy before incorporation.
