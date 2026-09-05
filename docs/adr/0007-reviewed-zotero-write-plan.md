# ADR 0007: Separate reviewed Zotero write planning from execution

- Status: Proposed
- Date: 2026-09-04
- Supersedes: the future-write deferral in ADR 0006; its read-only Zotero 9 decision remains valid

## Context

Issue #8 requires classification changes to default to dry-run, preserve complete collection and tag state, reject stale review input, and make rollback reconstructable. The installed Zotero 9.0.6 Local API cannot write. Zotero 10+ writes additionally require a runtime-granted key, the same server identity, and fresh library/item versions. A planner can establish the review and recovery contract now without inventing authority or adding an unsafe Zotero 9 mutation path.

## Decision

ConceptWeave builds a local-only `ClassificationWritePlan` from an externally verified complete review set. Dry-run is the default. The review must match the exact Zotero version, server identity, library version, classifier revision, raw-snapshot digest, complete item-key/item-version coordinates, and observed collection/tag state. The plan retains the reviewed Zotero version used for execute eligibility, while private fields and read-only accessors prevent external callers from mutating validated execution state. Every receipt copies the plan's review and snapshot coordinates so an outcome cannot be detached from its authority or evidence. It rejects unknown or duplicate items, detached item revisions, blank or duplicate metadata, unsupported tag types, no-op changes, and `NeedsStewardReview` as a write decision. Operations are deterministic and retain complete before, after, and rollback states. Manual tag markers `None` and `0` are canonicalized to `None`; automatic tag type `1` is preserved.

Execute planning fails closed for Zotero versions below 10. The plan contains no API key and performs no network call. Dry-run enumerates every operation as not attempted. The execution core accepts caller-owned preflight and write functions, preflights the complete plan before the first mutation, and verifies server, library, item revision, collection, and typed-tag responses. After a failed or invalid write response, it reuses the same read boundary to distinguish unchanged, applied, and indeterminate state. A reconciled applied item receives a rollback operation. An unexpected mutation remains indeterminate but retains an inverse operation only when the same server/item identity and newer library/item revisions establish a safe conditional rollback target. An unprovable state is named explicitly and requires operator reconciliation.

The authenticated Zotero 10+ adapter is a narrow loopback transport for those injected functions. A caller may supply credentials directly or perform one official `/api/local/authorize` request with a bounded nonblank application name and expected server identity. Every authorization, read, and write response must repeat that identity before status classification. Success returns an exact 32-character header-safe key plus the remembered decision; denial requires same-server bounded JSON with `denied: true`. The private authorization wrapper can only disclose the remembered decision or be consumed into the existing adapter; neither value is debuggable or serializable. Denial and rate limiting never trigger an automatic retry or repeated prompt, and only a bounded integer retry delay is retained. Writes distinguish an expired authorization from a matching-server stale precondition, while a different-server `412` invalidates the read/write partition as a database switch. Static error categories cannot echo a credential, response body, or URL. Cross-item transactionality is not claimed, and source records and attachments are never deleted.

## Consequences

- Review and rollback semantics can be tested on Zotero 9 without changing the library.
- Exact before-state checks prevent silent loss of unrelated collections or automatic-tag metadata.
- AC5 is implemented. AC6 now has deterministic preflight, partial-failure, rollback-receipt, and synthetic authenticated transport evidence. AC6 remains incomplete until approved live Zotero 10 write, partial-failure, and rollback behavior is verified.

## Alternatives considered

- Writing through Zotero 9 was rejected because the provider does not support it.
- Storing only collection/tag deltas was rejected because Zotero array updates are complete replacements and cannot prove lossless rollback.
- Treating mock transport coverage as live proof was rejected because no approved Zotero 10 authorization, runtime write, and rollback exercise has been performed. No live prompt ran and no key is committed.
