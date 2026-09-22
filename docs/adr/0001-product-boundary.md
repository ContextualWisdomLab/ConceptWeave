# ADR 0001 — Product and bounded-context boundary

**Status:** Accepted

## Context

CWL already has products that reconstruct lineage, operate semantic catalogs, define shared graph contracts, and route LLM calls. Placing automatic semantic-model engineering inside any one of those products would blur system-of-record and reuse boundaries.

## Decision

ConceptWeave owns **Semantic Model Engineering**: observing source evidence, discovering semantic candidates, validating them, governing review, and publishing versioned ontology/semantic-layer releases.

`semantic-data-portal` remains the consumer/catalog/governance plane for published semantic context. `LineageWeave` remains an inference/lineage evidence producer. `context-graph-contracts` remains a contract-only interoperability repository. `contextual-orchestrator` remains the LLM routing boundary.

External code-analysis or graph-generation tools may be optional source adapters, but external forks are not ConceptWeave product authority or required internal dependencies.

## Consequences

- ConceptWeave can be used by GRC, EA, analytics, HR, billing, or other products without copying their authoritative data.
- No direct cross-service application-table SQL.
- Source systems remain authoritative for source facts.
- Published semantic releases are authoritative only within their explicitly reviewed semantic scope.
