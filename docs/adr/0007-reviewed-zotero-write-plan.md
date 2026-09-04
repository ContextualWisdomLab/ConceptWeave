# ADR 0007: Separate reviewed Zotero write planning from execution

- Status: Proposed
- Date: 2026-09-04
- Supersedes: the future-write deferral in ADR 0006; its read-only Zotero 9 decision remains valid

## Context

Issue #8 requires classification changes to default to dry-run, preserve complete collection and tag state, reject stale review input, and make rollback reconstructable. The installed Zotero 9.0.6 Local API cannot write. Zotero 10+ writes additionally require a runtime-granted key, the same server identity, and fresh library/item versions. A planner can establish the review and recovery contract now without inventing authority or adding an unsafe Zotero 9 mutation path.

## Decision

ConceptWeave builds a local-only `ClassificationWritePlan` from an externally verified complete review set. Dry-run is the default. The plan must match the exact server identity, library version, classifier revision, raw-snapshot digest, item version, and complete observed collection/tag state. It rejects unknown or duplicate items, blank or duplicate metadata, no-op changes, and `NeedsStewardReview` as a write decision. Operations are deterministic and retain complete before, after, and rollback states. Zotero tag `type` is preserved.

Execute planning fails closed for Zotero versions below 10. The plan contains no API key and performs no network call. A later adapter must preflight all operations and return item-level partial-success and rollback receipts; it must not claim cross-item transactionality or delete source records and attachments.

## Consequences

- Review and rollback semantics can be tested on Zotero 9 without changing the library.
- Exact before-state checks prevent silent loss of unrelated collections or automatic-tag metadata.
- AC5 advances, while AC6 remains incomplete until an authenticated Zotero 10+ adapter and approved live write/rollback are verified.

## Alternatives considered

- Writing through Zotero 9 was rejected because the provider does not support it.
- Storing only collection/tag deltas was rejected because Zotero array updates are complete replacements and cannot prove lossless rollback.
- Adding the HTTP writer now was rejected because no Zotero 10+ runtime or approved local key is available for end-to-end verification.
