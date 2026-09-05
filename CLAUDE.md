# CLAUDE.md — ConceptWeave

Follow `AGENTS.md`, `ARCHITECTURE.md`, accepted ADRs, and the organization master context before making changes.

ConceptWeave's core invariant is: **inference is not authority**. Every generated concept, relation, constraint, dimension, measure, or physical mapping must retain evidence and pass the explicit governance lifecycle before publication.

Keep domain logic in bounded domain modules, LLM/provider logic behind ports/adapters, and source/consumer systems independent. Prefer deterministic validation and explicit abstention over plausible unsupported output.

Zotero source capture must not alter the metadata report or renew its approval. Preserve private-file protections and the full bibliographic denominator, including missing and partial text.

The separate full-text review view does not authorize decisions or writes. Keep its evidence binding intact; existing metadata-only apply/finalization cannot establish full-text-reviewed approval.
