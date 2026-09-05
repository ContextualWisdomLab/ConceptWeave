# ADR 0006: Keep Zotero research intake read-only and proposal-based

- Status: Proposed
- Date: 2026-09-04

## Context

CWL needs a reproducible inventory of ontology research without turning keyword matches into authoritative library organization. The current desktop is Zotero 9.0.6, whose Local API supports reads but not writes. Zotero documents Local API writes only for Zotero 10+, where they require user-granted authorization and `Zotero-Server-ID`; this slice therefore has no mutation capability. The library is mutable while pagination is in progress, duplicate metadata does not prove that two records should be merged, and the local report contains titles and item keys that must not be written into the repository.

Zotero's Local API documentation states that production clients should request `Zotero-API-Version: 3`; the response exposes `Zotero-API-Version` and `Zotero-Schema-Version`. The API version is the compatibility contract. The schema version is therefore recorded and required to remain stable across the snapshot, but it is not hard-coded to the developer workstation's current schema 42 because Zotero can legitimately revise the local data schema while retaining API v3 compatibility.

Primary capability references: Zotero, *Local API* (updated 2026-07-29), https://www.zotero.org/support/dev/web_api/v3/local_api; Zotero, *Basics*, https://www.zotero.org/support/dev/web_api/v3/basics.

## Decision

ConceptWeave owns a small read-only Anti-Corruption Layer from Zotero into research evidence intake. Every request explicitly sends `Zotero-API-Version: 3`; a response reporting another API version fails closed. `Total-Results`, library version, Zotero version, schema version, and optional server identity must remain unchanged across pagination. API and schema versions are retained in the live report provenance.

The adapter links child records, emits exactly one deterministic proposed disposition per top-level bibliographic item, and abstains when evidence is weak or ambiguous. Every abstention preserves a deterministic reason distinguishing missing classification metadata, vocabulary outside the current deterministic rules, present-but-unmatched metadata, and conflicting specific disposition families. Specific rule families are evaluated together rather than by first-match priority. When evidence matches multiple families, the proposal becomes `NeedsStewardReview` and all matching evidence is retained.

Matched metadata values are copied into the local-only evidence receipt for replay. This is necessary for abstract-only matches because a later Zotero revision cannot reconstruct the exact text used for an earlier proposal from item key/version alone. An abstention likewise retains its nonempty abstract so a steward can resolve unsupported or unmatched vocabulary from the same immutable report. If matched evidence already contains the abstract, the review-only field is omitted so sensitive text appears once; decided items also omit that extra copy. On supported Unix platforms, the sensitive local report is created with exact owner-only `0600` permissions after applying the process umask; other platforms fail closed. DOI/title matches remain reversible duplicate candidates, including legacy `dx.doi.org` resolver forms.

Duplicate candidates become canonical references only through externally verified steward decisions bound to the raw digest, complete item-key/item-version snapshot, and exact candidate membership. Overlapping candidates form one connected component and must select one component-level canonical item. Every resulting operation retains all component source revisions and complete before/after/rollback key mappings. It changes downstream identity resolution only; classification does not merge, delete, or mutate Zotero source records.

Classifier quality is measured only against local steward-reviewed labels whose complete reviewed set is verified outside this crate and bound to the exact library version, rule revision, canonical SHA-256 raw-snapshot digest, and every observed parent/child item-key/item-version coordinate. `NeedsStewardReview` is an abstention prediction and cannot be approved truth. Evaluation returns the verified revisions and opaque digest with aggregate integer evidence; Zotero keys, reviewer identity, and bibliographic text are omitted. Missing, stale, content- or label-mismatched, unverified, unknown, duplicate, or invalid review identities fail closed.
Every successful report includes an aggregate audit summary computed from the same captured snapshot. Zotero 9 item version zero is preserved as a valid never-synced source coordinate, not treated as missing provenance. Partial reads never produce a report, so successful output explicitly records zero failures alongside snapshot, proposal, provenance, abstention, duplicate, and per-disposition totals.

The reader fails closed above 50,000 items or 256 MiB of cumulative response bodies, while retaining the 8 MiB per-page bound, finite request timeouts, redirect denial, total-count checks, snapshot-version checks, and duplicate-key detection. Pagination, consistency, resource-budget, and provider-contract behavior are separated from the narrow `ureq` transport so deterministic tests exercise the production reader core rather than excluding the entire reader from coverage.

Report output is restricted to a new direct child of the operating system temporary directory. Relative paths, nested paths, existing files, and symlinks are rejected before write; the file is opened with create-new semantics so a path swap cannot cause repository or arbitrary-file overwrite. The buffered writer is explicitly flushed and a final filesystem error fails the command. Reports stay local and are never committed.

No dedicated utility repository or Zotero mutation path is created. A future Zotero 10+ write adapter is a separate decision and must use authenticated loopback access, server identity, optimistic version preconditions, reviewed item-level changes, before/after receipts, and rollback evidence.

## Consequences

### 2026-09-05 integrity amendment (Proposed)

In the context of replaying a Zotero research classification against a steward's approved labels, facing source fields lost during projection and predictions mutable after review, we decided for separate source-and-input and proposal-content digests verified with the complete reviewed set, and against typed-only source hashing or a report's self-declared cached proposal identity, to preserve the exact evidence used for evaluation, accepting a receipt-format break, report regeneration and fresh governance approval.

The source identity uses `conceptweave-zotero-snapshot-v2` over item-key-ordered pairs of complete captured provider JSON and actual typed classifier input. Capturing only unknown flattened fields was rejected after the omitted-title versus explicit-empty-title regression showed another collision. Hashing only captured JSON was also rejected: mutating a decoded public title would otherwise change classification without changing source identity. The pair binds both representations without another cloned source snapshot; JSON object order is canonicalized while field presence, nested metadata and array order remain meaningful.

The approval additionally requires `proposal_digest`, computed from all current proposal records under `conceptweave-classification-proposals-v1`, sorted by item key and revision. Every proposal field is bound, including evidence and records outside a reviewed sample. Evaluation recomputes this value before invoking the external verifier. Changing both the prediction and the submitted digest cannot renew an independently issued approval; the governance verifier must authenticate the complete reviewed set, not merely accept a receipt identifier. Local checks remain before this authority boundary.

Positive consequence: same-version metadata edits, changed classifier inputs and altered evaluated predictions invalidate the relevant binding. Negative consequence: prior source digests and approval formats are incompatible; no automatic backfill or transfer of approval is allowed. Captured source and typed input also consume memory until classification finishes. Private Rust fields alone were not selected as the approval solution because they cannot authenticate a deserialized report; storing another full source snapshot was unnecessary for proposal binding. No new authority service, external dependency, or repository is introduced.

The regressions in `raw_provider_snapshot_binding.rs`, `golden_set_integrity_contract.rs` and `snapshot_content_binding.rs` exercise the two existing PR #10 findings, including failed intermediate designs. Source repair is distinct from hosted exact-head checks, independent review, protected integration and externally approved live classification. The dependent review/finalization stack must adopt the required digest and reject old approval JSON before promotion. ADR status remains Proposed.

- A complete snapshot can be audited and replayed without changing the research library.
- Rule evidence and explicit abstention reasons are visible; automated classification is not governance approval.
- Cross-cutting papers cannot be silently forced into whichever rule family happens to be evaluated first.
- Whole-snapshot resource use is bounded independently from per-page limits.
- Local API compatibility is explicit through API v3 while schema revision remains traceable and snapshot-stable.
- Sensitive local reports cannot be directed into the repository by the CLI and final write failures are observable.
- Human review remains necessary for ambiguous records and every duplicate merge.
- Zotero 9 cannot apply approved collection/tag changes automatically.

## Alternatives considered

- First-match classification was rejected because FR-9 requires ambiguous evidence to abstain rather than acquire an arbitrary priority-based disposition.
- Hard-coding Zotero schema 42 was rejected because the documented compatibility contract is API v3; schema revision is instead recorded and checked for within-read drift.
- Direct Zotero 9 writes were rejected because the supported Local API write capability is Zotero 10+ only.
- Cloud Web API mutation was rejected because it expands credential and network scope without being needed for classification.
- Arbitrary report output paths were rejected because local bibliographic titles and item keys are intentionally not repository artifacts.
- A new repository was rejected because one bounded adapter does not yet justify another lifecycle and release surface.
