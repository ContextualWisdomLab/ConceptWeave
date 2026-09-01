# ConceptWeave

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/ConceptWeave)

**Automatic ontology and semantic-layer engineering for governed enterprise meaning.**

ConceptWeave turns heterogeneous enterprise evidence—schemas, APIs, event contracts, documents, code structure, existing vocabularies, and lineage—into **reviewable semantic-model candidates**. It does not make model-generated meaning authoritative by itself. Candidates must retain source evidence, pass deterministic validation, and move through an explicit governance lifecycle before publication.

## Product boundary

ConceptWeave owns **semantic model engineering**:

`observe -> discover -> propose -> validate -> review -> publish`

It does **not** own:

- enterprise catalog/search/runtime consumption (`semantic-data-portal`),
- lineage reconstruction (`LineageWeave`),
- cross-product graph/event contracts (`context-graph-contracts`),
- LLM provider routing (`contextual-orchestrator`), or
- the authoritative business data of source systems.

External source-analysis tools may be integrated behind adapters, but no external fork is treated as ConceptWeave product authority.

## First release target

The first vertical is a **relational-schema-to-governed-semantic-model proposal**:

1. ingest an immutable schema snapshot;
2. derive observed physical entities and relationships;
3. propose concepts, taxonomy/semantic relations, dimensions, measures, constraints, and physical mappings;
4. bind every proposal to exact source evidence;
5. validate structure and consistency;
6. require steward review before publication; and
7. export a versioned semantic package suitable for ontology and analytics consumers.

Planned publication targets include OWL/RDFS/SKOS, SHACL, JSON-LD, and an Apache Ossie-compatible semantic-model projection when the emerging specification is sufficiently stable for the required subset.

## Current state

This foundation PR establishes the Rust domain contract, candidate truth/publication lifecycle, JSON Schema, DDD architecture, standards/research baseline, security/test/operability baselines, and CI. Source adapters, LLM-assisted induction, persistence, reasoning, review UI, and publication adapters remain explicit product gaps.

## Rust

The repository is pinned to Rust 1.98.0. The current core has no third-party runtime dependencies.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

See [`docs/PRD.md`](docs/PRD.md), [`docs/TRD.md`](docs/TRD.md), [`ARCHITECTURE.md`](ARCHITECTURE.md), and the [documentation home](docs/index.md).
