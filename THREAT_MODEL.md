# ConceptWeave Threat Model

## Scope and authority

This document records product security boundaries that affect ConceptWeave semantic engineering and release eligibility. `SECURITY.md` defines baseline controls; this file names concrete actors, assets, trust assumptions, residual risks, and fail-closed decisions for implemented capabilities. Product-domain truth remains with its canonical owner. External providers and CWL sibling products are treated through explicit ports/contracts rather than copied authority.

## Protected assets

- immutable source snapshots, source coordinates, digests, and observation receipts;
- semantic candidates, validation evidence, steward review receipts, and immutable semantic releases;
- Zotero bibliographic records and reviewed write plans;
- credentials, authorization tokens, API keys, and source-registry capability material;
- tenant/workspace authorization context when introduced;
- release provenance, SBOM/provenance evidence, and rollback coordinates.

Credentials and authorization material must never become semantic evidence, serialized domain artifacts, logs, model prompts, test fixtures, or immutable release payloads.

## Trust boundaries

Source artifacts, imported ontologies, provider responses, model outputs, web-retrieved content, and external system metadata are untrusted until validated at their owning boundary. LLM proposals are non-authoritative until deterministic validation and steward publication. Source Observation accepts only bounded adapter evidence and must not turn a syntactically plausible identifier into authorization provenance. Client Consumption admits only released immutable contracts.

## Primary threats

1. malicious or malformed source content causing semantic poisoning, parser/resource exhaustion, or provenance confusion;
2. model output being promoted to authority without deterministic validation and steward review;
3. credential or source-authorization leakage across domain, evidence, log, or model boundaries;
4. source-system mutation from discovery/validation code or hidden cross-service SQL coupling;
5. stale, ambiguous, or mismatched source coordinates being recorded as immutable evidence;
6. in-place mutation of published semantic truth instead of explicit supersession;
7. tenant/workspace evidence disclosure;
8. SSRF, DNS rebinding, unsafe redirects, or unbounded external retrieval;
9. dependency/provider compromise or unexpected retention;
10. write-back without reviewed before/after/rollback evidence and exact preconditions;
11. same-host filesystem races replacing a checked owner-only review artifact path with a symlink before the file descriptor is opened.

## Zotero 10+ Local API transport boundary

Zotero's documented Local API endpoint is `http://localhost:23119/api/`. Read requests are unauthenticated. Write requests require a user-granted local API key and, in Zotero 10+, the expected `Zotero-Server-ID` continuity coordinate.

`Zotero-Server-ID is not cryptographic server authentication`. It identifies the Zotero database instance and supports stale/database-switch detection, but it does not authenticate the loopback peer before a request transmits `Zotero-API-Key`. Loopback pinning and redirect rejection reduce network exposure but do not encrypt HTTP traffic or provide OS-authenticated IPC.

### Threat actor

A hostile same-host process that can bind, observe, or interpose on the loopback endpoint is inside the unresolved threat boundary for Zotero write credentials. ConceptWeave currently has no provider-documented HTTPS or equivalent OS-authenticated IPC endpoint that can replace the Local API write path.

### Current decision

The Zotero adapter may be used for read-only intake and for mock/local verification of authorization and write orchestration. It must not be represented or released as enterprise-secure live write-back while confidentiality against a hostile same-host process is unproven. Live enterprise write-back therefore remains fail closed.

A future release may cross this boundary only when one of the following is true:

- Zotero exposes an authenticated encrypted or OS-authenticated IPC transport and ConceptWeave verifies it before transmitting a key; or
- a product-security decision explicitly narrows the supported threat model to exclude hostile same-host observation/interposition, records the residual credential risk, and receives the required governance approval.

Neither path may reinterpret `Zotero-Server-ID` as cryptographic peer authentication.

## Zotero write invariants retained regardless of transport decision

- local API keys stay private and non-serializable;
- authorization is user initiated and denial/rate-limit outcomes remain fail closed;
- server/library/item preconditions are verified before mutation;
- database switches are surfaced as a distinct failure;
- dry-run performs no mutation;
- approved writes preserve exact before/after evidence, partial-failure reconciliation, and rollback coordinates;
- attachments and bibliographic source records are not deleted by classification write-back;
- descendant integration evidence never back-proves an unresolved predecessor contract.

## Owner-only review artifact filesystem boundary

Saved report, worksheet, approval, progress, and golden-set artifacts are sensitive local review material. Path policy alone is not the security boundary. A direct-temp-child path may be checked as a regular non-symlink and still be replaced by a symlink before a later symlink-following open.

The opened file descriptor must therefore be obtained with a Unix final-component no-follow primitive such as `O_NOFOLLOW` or an equivalent safe abstraction. After open, ConceptWeave still verifies that the opened device/inode matches the checked regular file, link count is one, mode is exactly `0600`, and the bounded-read contract is satisfied. A second pathname check is not an equivalent repair because it leaves another check/open race window.

PR #30 commit `9733d28` reproduced the inode-preserving final-component symlink swap and left the no-follow contract RED. Commit `7ccbbbe` repairs the shared input-open helper with Unix `O_NOFOLLOW`; focused security and artifact-identity tests then pass. Offline review/finalization remains acceptance-gated until this repair has terminal protected checks and independent approval on one unchanged exact head.

## Release gate

The optional full-text review view must not be confused with the metadata-only decision or approval artifacts. A required versioned outer envelope and strict legacy parsers prevent silent direct application; an intentional caller can still strip fields, so no full-text-reviewed approval is claimed. The complete capture is reverified before selecting text, unrelated parent text is excluded, and output serialization itself is bounded. The larger serialized-capture input ceiling is separate from the unchanged report/worksheet limit. Source text is untrusted data, never instructions, and source content must not be included in input-error diagnostics.

Capture-bound review uses separate required versioned payloads, with no import of previous metadata decisions. Atomic application permits only completed decision slots in the exact current view; recursive duplicate-key rejection precedes projection so a changed first field cannot hide behind a canonical later field. Finalization and evaluation revalidate the capture against the complete current report, including proposal records, before authority is contacted. Governance must authenticate the entire outer reviewed set and every label. Owner-only storage, private fields and hashes do not defeat a malicious local replacement or prove human review. Preserve old artifacts; stale views fail rather than overwrite concurrent work. These APIs neither implement authenticated governance nor authorize the independent Zotero write contract. Restoring report/capture/worksheet/approval JSON still requires caller-owned bounded private-file admission.

A capability is not release-ready while a valid security finding lacks a deterministic test or equivalent machine-verifiable contract, while required exact-head checks are non-terminal, or while the implemented transport cannot satisfy the advertised security claim. Documentation must describe residual risk without upgrading provider guarantees by inference.
