# ADR 0002 — Evidence, truth, and publication lifecycle

**Status:** Accepted

## Context

Automatic ontology learning and LLM-assisted semantic modeling can produce plausible but incorrect concepts and relations. A single `confidence` number cannot establish authority.

## Decision

Separate epistemic truth status from governance publication state. Every semantic candidate requires exact source evidence. New candidates are inferred drafts. Publication requires the ordered lifecycle:

`Draft -> Proposed -> Validated -> Reviewed -> Published`

At Draft/Proposed/Validated/Reviewed stages the artifact is not authoritative. `Published` changes semantic truth to authoritative within the release scope. Rejection is explicit. Published facts are never overwritten; a replacement creates a new release and marks the old release/candidate superseded.

No LLM, embedding similarity, graph centrality, or automated extractor may directly create authoritative semantic truth.

## Consequences

- Review and validation receipts become first-class future persistence objects.
- Replay can reconstruct why a semantic release exists.
- Consumers can filter `authoritative`, `observed`, `inferred`, and `proposed` data without conflating them.
