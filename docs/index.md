# ConceptWeave

ConceptWeave turns enterprise evidence into governed, reviewable semantic models while keeping source systems authoritative.

## Product goal

The first release target converts an immutable relational-schema snapshot into evidence-bound semantic candidates for concepts, relationships, dimensions, measures, constraints, and physical mappings. Candidates remain proposed until deterministic validation and authorized review permit publication.

## Current status

ConceptWeave is in foundation development. The active foundation work establishes its Rust domain model, lifecycle, public schema, architecture, security, test strategy, and operability baseline. Source adapters, persistence, model-assisted induction, steward interfaces, and publication adapters remain explicit gaps until implemented and verified.

## Start here

- [Repository overview](../README.md)
- [Product requirements](PRD.md)
- [Technical requirements](TRD.md)
- [Architecture](../ARCHITECTURE.md)
- [DeepWiki](https://deepwiki.com/ContextualWisdomLab/ConceptWeave)

## Governance boundary

Generated or LLM-assisted meaning is never authoritative by default. Every published semantic release must preserve source evidence and pass the product's deterministic validation and governance lifecycle.
