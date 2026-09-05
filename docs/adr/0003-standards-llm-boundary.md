# ADR 0003 — Standards and LLM engineering boundary

**Status:** Accepted

## Context

ConceptWeave must publish portable semantic artifacts while standards evolve and LLM-based ontology engineering remains an active research area.

## Decision

1. Stable ontology publication targets are RDF 1.1, OWL 2, SKOS, SHACL 1.0, JSON-LD 1.1, and PROV-O as applicable.
2. RDF 1.2 and SHACL 1.2 are tracked as 2026 W3C in-progress work and may be implemented behind explicit experimental/versioned adapters; they are not labeled final standards.
3. Apache Ossie (incubating, formerly Open Semantic Interchange) is tracked as an emerging vendor-neutral semantic-model exchange format for datasets, fields/dimensions, relationships, and metrics. Any adapter is explicitly version-bound until the required specification subset is stable.
4. LLM-backed ontology learning, matching, labeling, and candidate generation must use `contextual-orchestrator` and produce structured proposals with evidence. LLMs do not validate, approve, or publish semantic truth.
5. Evaluation combines deterministic conformance/consistency checks, benchmark fixtures, and human-reviewed cases. Model-as-judge evidence is supplementary only.

## Consequences

ConceptWeave is standards-oriented without falsely promoting drafts to Recommendations, and model assistance can improve recall while governance remains fail-closed.
