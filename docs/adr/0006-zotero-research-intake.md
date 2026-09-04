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

Duplicate candidates become canonical references only through externally verified steward decisions bound to the raw digest, complete item key/version/parent snapshot, and exact candidate membership. Duplicate sources must be observed top-level records, and local parent, component, and retained-key validation finishes before approval verification. Overlapping candidates form one connected component and must select one component-level canonical item. Every resulting operation preserves all component source coordinates and complete before/after/rollback key mappings. It changes downstream identity resolution only; classification does not merge, delete, or mutate Zotero source records.

Classifier quality is measured only against local steward-reviewed labels whose reviewed set is verified outside this crate and bound to the exact library version, rule revision, canonical SHA-256 raw-snapshot digest, and every observed parent/child item-key/item-version coordinate. `NeedsStewardReview` is an abstention prediction and cannot be approved truth. Sampled labels may measure classifier quality, but a full-reclassification completion result requires exactly one approved label for every classified bibliographic item. Cardinality, snapshot, key, disposition, and duplicate checks run before the external approval verifier so invalid local input cannot consume approval authority. Evaluation returns the verified revisions and opaque digest with aggregate integer evidence; Zotero keys, reviewer identity, and bibliographic text are omitted. Missing, incomplete, stale, content- or label-mismatched, unverified, unknown, duplicate, or invalid review identities fail closed at the applicable completion boundary.
Every successful report includes an aggregate audit summary computed from the same captured snapshot. Zotero 9 item version zero is preserved as a valid never-synced source coordinate, not treated as missing provenance. Partial reads never produce a report, so successful output explicitly records zero failures alongside snapshot, proposal, provenance, abstention, duplicate, and per-disposition totals.

To make full review executable without copying sensitive text again, ConceptWeave derives a local worksheet from the validated report. The worksheet carries the exact snapshot binding, complete item key/version/parent coordinates, deterministic proposal and abstention reason, plus one empty steward-decision slot per bibliographic item in item-key order. Restored artifacts must preserve each observed child-to-parent relation; a classified item cannot masquerade as a child, and an in-snapshot child cannot be reassigned to another parent. Titles, abstracts, tags, collections, and matched evidence remain only in the owner-only report. Invalid report identity or coverage fails worksheet construction.

The report is a losslessly deserializable owner-only artifact. Evidence field names, matched phrases, and the rule revision use owned values so an offline process can reconstruct the exact canonical worksheet from the saved report; it must not reread a mutable Zotero library to finalize an earlier review.

The CLI finalizes an original report, completed worksheet, and approval receipt into the reviewed golden set without another Zotero read. Each path argument and each opened input device/inode identity must be distinct. Inputs remain direct temporary-directory children, regular single-link files, exact owner-only `0600`, and bounded to 16 MiB; output retains create-new `0600` semantics. This keeps sensitive review material local and makes snapshot drift or aliased artifacts a validation failure instead of silently substituting current library state.

The same offline boundary may emit an aggregate progress checkpoint for a partial worksheet. The checkpoint revalidates every immutable coordinate and proposal field, counts only human-supplied non-abstention decisions, contains no item or reviewer identity, and treats zero required decisions as incomplete. It is operational coverage evidence, not an approval receipt or semantic-quality result.

Incremental steward work is integrated by a snapshot-bound decision patch rather than editing or merging the complete worksheet structure. The patch carries only library/rule/digest coordinates and unique item key/version/disposition updates. The canonical report and current worksheet are revalidated first; applying to a clone makes invalid or conflicting batches atomic failures, while an identical replay is idempotent. We reject direct in-place mutation and last-writer-wins merging because either can silently discard concurrent review work. The patch remains local review input and does not mint governance authority.

The offline CLI consumes the saved report, current worksheet, and patch as distinct owner-only file identities and emits only a separate create-new worksheet. Reusing the existing private artifact boundary keeps the command local, bounded, and fail-closed without introducing a second review service or repository. The CLI never rereads Zotero, overwrites the source worksheet, or verifies approval.

Human review uses deterministic batches of at most 100 pending records. A batch projects only the matching report context and blank decision slots and remains owner-only. We choose a repeatable view instead of reservation state because the current campaign has no independent assignment service or cross-product consumer; concurrency ownership must be added only when such a contract exists. Creating a batch does not advance review or approval KPIs.
Completed batches cross a dedicated validation boundary before becoming decision patches. The owner rebuilds the pending view from the immutable report and current worksheet, requires every displayed context field to match, and rejects direct batch deserialization as a context-free patch. This keeps the steward's decision bound to what was shown without adding another aggregate or service.

The reader fails closed above 50,000 items or 256 MiB of cumulative response bodies, while retaining the 8 MiB per-page bound, finite request timeouts, redirect denial, total-count checks, snapshot-version checks, and duplicate-key detection. Pagination, consistency, resource-budget, and provider-contract behavior are separated from the narrow `ureq` transport so deterministic tests exercise the production reader core rather than excluding the entire reader from coverage.

Report output is restricted to a new direct child of the operating system temporary directory. Relative paths, nested paths, existing files, and symlinks are rejected before write; the file is opened with create-new semantics so a path swap cannot cause repository or arbitrary-file overwrite. The buffered writer is explicitly flushed and a final filesystem error fails the command. Reports stay local and are never committed.

No dedicated utility repository or Zotero mutation path is created. A future Zotero 10+ write adapter is a separate decision and must use authenticated loopback access, server identity, optimistic version preconditions, reviewed item-level changes, before/after receipts, and rollback evidence.

## Consequences

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
