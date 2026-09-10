# CLAUDE.md — ConceptWeave

Follow `AGENTS.md`, `ARCHITECTURE.md`, accepted ADRs, and the organization master context before making changes.

ConceptWeave's core invariant is: **inference is not authority**. Every generated concept, relation, constraint, dimension, measure, or physical mapping must retain evidence and pass the explicit governance lifecycle before publication.

Keep domain logic in bounded domain modules, LLM/provider logic behind ports/adapters, and source/consumer systems independent. Prefer deterministic validation and explicit abstention over plausible unsupported output.

For procedural knowledge, apply the same inference/authority separation through
[ADR-PG-20260910](docs/adr/pg_20260910_procedural_model_engineering.md) and issue #42.
Do not turn a shape-valid authoring draft, training-partition label, or digest string
into authenticated evidence. Noema's execution-local projection is not a competing
ConceptWeave publisher, and this schema slice is not a deployed self-evolving agent.
