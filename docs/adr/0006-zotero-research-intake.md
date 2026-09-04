# ADR 0006: Keep Zotero research intake read-only and proposal-based

- Status: Proposed
- Date: 2026-09-04

## Context

CWL needs a reproducible inventory of ontology research without turning keyword matches into authoritative library organization. The current desktop is Zotero 9.0.6, whose Local API supports reads but not writes. Zotero documents Local API writes only for Zotero 10+, where they require user-granted authorization and `Zotero-Server-ID`; this slice therefore has no mutation capability. The library is mutable while pagination is in progress, duplicate metadata does not prove that two records should be merged, and the local report contains titles and item keys that must not be written into the repository.

Primary capability reference: Zotero, *Local API* (updated 2026-07-29), https://www.zotero.org/support/dev/web_api/v3/local_api.

## Decision

ConceptWeave owns a small read-only adapter that fetches every item under one unchanged Local API library version, links child records, emits exactly one deterministic proposed disposition per top-level bibliographic item, and abstains when evidence is weak. Every abstention preserves a deterministic reason distinguishing missing classification metadata, vocabulary outside the current deterministic rules, and metadata that is present but unmatched. DOI/title matches remain reversible duplicate candidates, including legacy `dx.doi.org` resolver forms.

The reader fails closed above 50,000 items or 256 MiB of cumulative response bodies, while retaining the 8 MiB per-page bound, finite request timeouts, redirect denial, total-count checks, snapshot-version checks, and duplicate-key detection. Report output is restricted to a new direct child of the operating system temporary directory. Relative paths, nested paths, existing files, and symlinks are rejected before write; the file is opened with create-new semantics so a path swap cannot cause repository or arbitrary-file overwrite. Reports stay local and are never committed.

No dedicated utility repository or Zotero mutation path is created. A future Zotero 10+ write adapter is a separate decision and must use authenticated loopback access, server identity, optimistic version preconditions, reviewed item-level changes, before/after receipts, and rollback evidence.

## Consequences

- A complete snapshot can be audited and replayed without changing the research library.
- Rule evidence and explicit abstention reasons are visible; automated classification is not governance approval.
- Whole-snapshot resource use is bounded independently from per-page limits.
- Sensitive local reports cannot be directed into the repository by the CLI.
- Human review remains necessary for ambiguous records and every duplicate merge.
- Zotero 9 cannot apply approved collection/tag changes automatically.

## Alternatives considered

- Direct Zotero 9 writes were rejected because the supported Local API write capability is Zotero 10+ only.
- Cloud Web API mutation was rejected because it expands credential and network scope without being needed for classification.
- Arbitrary report output paths were rejected because local bibliographic titles and item keys are intentionally not repository artifacts.
- A new repository was rejected because one bounded adapter does not yet justify another lifecycle and release surface.
