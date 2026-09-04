# ADR 0006: Keep Zotero research intake read-only and proposal-based

- Status: Proposed
- Date: 2026-09-04

## Context

CWL needs a reproducible inventory of ontology research without turning keyword matches into authoritative library organization. The current desktop is Zotero 9.0.6, whose Local API supports reads but not writes. The library is mutable while pagination is in progress, and duplicate metadata does not prove that two records should be merged.

## Decision

ConceptWeave owns a small read-only adapter that fetches every item under one unchanged Local API library version, links child records, emits exactly one deterministic proposed disposition per top-level bibliographic item, and abstains when evidence is weak. DOI/title matches remain reversible duplicate candidates. Reports stay local and are never committed.

No dedicated utility repository or Zotero mutation path is created. A future Zotero 10+ write adapter is a separate decision and must use authenticated loopback access, server identity, optimistic version preconditions, reviewed item-level changes, before/after receipts, and rollback evidence.

## Consequences

- A complete snapshot can be audited and replayed without changing the research library.
- Rule evidence and abstentions are visible; automated classification is not governance approval.
- Human review remains necessary for ambiguous records and every duplicate merge.
- Zotero 9 cannot apply approved collection/tag changes automatically.

## Alternatives considered

- Direct Zotero 9 writes were rejected because the supported Local API is read-only.
- Cloud Web API mutation was rejected because it expands credential and network scope without being needed for classification.
- A new repository was rejected because one bounded adapter does not yet justify another lifecycle and release surface.
