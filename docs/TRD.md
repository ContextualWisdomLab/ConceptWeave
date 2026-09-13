# ConceptWeave Technical Requirements Document

## 1. Architectural style

ConceptWeave starts as a Rust-first modular monolith with explicit bounded contexts and ports. Network-service extraction is deferred until independent scaling, trust, or deployment boundaries are demonstrated.

## 2. Bounded contexts

1. **Source Observation** — immutable source snapshots and parser receipts.
2. **Semantic Discovery** — evidence-bound candidate generation.
3. **Model Validation** — deterministic structural, ontology, constraint, and semantic-model validation.
4. **Governance & Publication** — review decisions, immutable releases, supersession.
5. **Interoperability** — import/export adapters and CWL anti-corruption layers.

The Core Domain is **Semantic Model Engineering**, represented by the discovery-to-publication lifecycle. Identity, LLM routing, outbound web access, observability, and catalog consumption are external/generic responsibilities.

## 3. Dependency direction

`domain <- application <- ports/contracts <- adapters <- delivery`

Domain code must not import web frameworks, databases, provider SDKs, LLM SDKs, or another CWL product's internals.

## 4. Source observation contract

Every observed source will eventually carry at least a source snapshot identifier, source kind, immutable content digest, source authority, observed/recorded time, parser/extractor version, tenant/workspace scope when tenancy exists, and bounded source locations for extracted evidence. A digest proves content identity only; source authority requires evidence from the canonical adapter that actually observed the source.

## 5. Candidate contract

The initial Rust and JSON contracts cover candidate kind, truth status, publication state, and source evidence. Later revisions add ontology IRIs, language-tagged labels, relation endpoints, cardinality, units, measure expressions, physical mappings, confidence/evaluation receipts, and temporal validity without breaking v0.1 consumers.

## 6. LLM boundary

LLM calls go through `contextual-orchestrator`. The application sends bounded evidence/context and receives structured proposals. LLM output is never a database command, publication decision, validation result, or source-system mutation. Deterministic checks must be able to reject the output without another model call.

## 7. Standards strategy

Stable publication targets use stable recommendations first: RDF 1.1, OWL 2, SKOS, SHACL 1.0, JSON-LD 1.1, and PROV-O as applicable. RDF 1.2 and SHACL 1.2 are tracked as 2026 drafts/candidate work and are not silently treated as final standards. Apache Ossie (incubating; formerly OSI) is tracked as an emerging semantic-model exchange format for metrics, dimensions, relationships, and datasets.

## 8. Persistence

No durable product database is claimed by the foundation slice. When persistence is introduced it must be PostgreSQL, 3NF by default, use descriptive two-or-more-word `snake_case` objects, preserve business/effective time separately from system-recorded time when facts vary over time, enforce tenant-scoped references, and use explicit migration ownership rather than runtime DDL races.

## 9. Security

Source artifacts are untrusted input. Adapters must enforce source size/type bounds, parser timeouts, archive/decompression limits, SSRF-safe outbound access where external retrieval exists, and prompt-injection isolation for LLM-assisted extraction. Credentials and raw secrets never become semantic evidence.

## 10. Evaluation

Evaluation must separate extraction recall, semantic correctness, structural correctness, ontology consistency, mapping accuracy, measure correctness, and governance outcomes. Model-judge scores may supplement but never replace deterministic golden fixtures and human-reviewed expert cases.

## 11. Zotero research intake and golden-set evaluation

`ClassificationReport` remains the Research Intake trusted aggregate. Its provenance, inventory and proposal state are private and constructor-bound; consumers use read-only accessors. Golden-set evaluation must not reopen those fields or deserialize caller-authored JSON into a trusted report merely to create corruption tests.

The `conceptweave-zotero` package keeps `src/lib.rs` as its canonical crate root. Golden-set evaluation is attached there as a sibling module and re-exported from that root; a second wrapper crate root is not an acceptable reconciliation seam because crate-level safety, lint and conditional nightly-coverage attributes belong to the Research Intake root that owns them.

`ClassificationReport.unclassified_items()` retains every input record excluded from bibliographic classification. Bibliographic proposals and this inventory are disjoint and together account for the observed record count. Unresolved ancestry remains visible through `pending_source_item_keys()`, including standalone sources, orphan trees and cycles. Empty pending ancestry is accounting evidence only, never semantic approval.

The Local API reader remains bounded to loopback, API v3, 100 records per page, 8 MiB per page, 50,000 items, 256 MiB cumulative response bodies, finite request timeouts, redirect denial, and a monotonic five-minute admission/completion budget. Total count, library version, Zotero version, schema revision and server identity must remain stable across pages. Record revision must not exceed the page library revision. Inconsistent, oversized, malformed, duplicate-key or late observations fail closed; no smaller successful denominator is returned.

Golden evaluation is layered beside, not inside, the trusted report aggregate. `GoldenSnapshot` pairs one constructor-produced `ClassificationReport` with an immutable item key/revision inventory and a versioned snapshot digest. It exposes the report and evidence through read-only accessors only.

Two caller-visible content-binding modes are deliberately distinct:

- `CapturedZoteroItem::try_from(Value)` decodes complete caller-supplied raw JSON into the stable public `ZoteroItem { key, version, data }` projection while retaining the raw JSON privately. `classify_captured_golden_snapshot` hashes canonicalized complete raw JSON together with the exact immutable typed classifier input under `conceptweave-zotero-captured-json-snapshot-v3`. Unknown nested fields, field presence and array order remain bound; JSON object key order does not change identity. Because the constructor is public and caller-supplied, this receipt does **not** authenticate that the bytes came from Zotero.
- `classify_typed_golden_snapshot` is for deterministic offline fixtures or already-controlled typed evidence. It binds the public typed projection under the separate `conceptweave-zotero-typed-snapshot-v3` domain. It likewise makes no provider-origin claim.

The raw-capture path preserves the existing `ZoteroItem` construction contract and prevents unrelated callers from fabricating `source_record=None` as an authenticity signal. Captured raw JSON has no mutable public accessor, so the raw representation and typed classifier input cannot drift independently after capture. That integrity property is not source authentication.

If authenticated Zotero-origin evidence becomes required, a canonical transport adapter that actually observes the Local API response must mint a separate attestation bound to the relevant provider/library/version/server/transport evidence. It may reuse canonical raw-content binding, but a public `Value` constructor or caller-selected digest domain cannot grant source authority. The adapter must reuse Research Intake pagination/budget logic rather than duplicating it.

`classification_proposal_digest` uses `conceptweave-classification-proposals-v3` and binds the current Research Intake report through read-only accessors: Zotero/API/schema/server/library/rule metadata, observed count, all proposals including truth/publication state and supporting evidence, all retained nonbibliographic records, pending-source coordinates and duplicate-candidate provenance. Canonical ordering makes record-order permutations stable. v1/v2 proposal receipts, the earlier v2 snapshot representation, and the superseded Draft `conceptweave-zotero-provider-snapshot-v3` label are historical evidence only and require independent reapproval; they are not migrated by local digest recomputation.

Before governance verification, `validate_classification_report` checks complete unique key/revision inventory, library-version bounds, bibliographic/nonbibliographic partitioning, direct-child coordinates, proposed lifecycle state and recomputed pending ancestry. `evaluate_reviewed_golden_set` then verifies library/rule/snapshot/proposal bindings and rejects blank, duplicate, unknown, stale, abstention-as-truth or otherwise invalid labels. Only after those checks may the caller-owned governance verifier authenticate the complete `ReviewedGoldenSet`.

Evaluation output is aggregate-only: verified revision/digests, reviewed/correct/abstention counts and per-disposition integer denominators. It contains no Zotero item keys, reviewer identity or bibliographic text. Rewriting a digest cannot renew an independently issued approval. Production authorization remains Keyverse/governance-owned and semantic publication remains outside this research-evaluation slice.

The report CLI remains proposal-only and restricted to a new direct child of the canonical operating-system temporary directory. Zotero 9 writes remain unsupported. A future Zotero 10+ writer requires a separate reviewed decision with authenticated loopback access, stable server identity, fresh version preconditions, item-level before/after receipts and rollback evidence.