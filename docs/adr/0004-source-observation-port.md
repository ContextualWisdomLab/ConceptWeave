# ADR 0004 — Bounded Source Observation port

- **Status:** Proposed
- **Date:** 2026-09-02
- **Owners:** Source Observation bounded context
- **Related:** Issue #2, PR #6, ADR 0001, `docs/product-technical-gap-baseline.md`

## Problem

ConceptWeave needs to observe PostgreSQL metadata without turning source connectivity into hidden coupling or allowing an adapter to run indefinitely, inspect unauthorized schemas, invent a partial snapshot after source disappearance, or leak credentials into domain contracts. The existing `conceptweave-observation` crate already owns immutable observed facts and provenance receipts, but it intentionally does not own source execution policy.

## Constraints

- Source systems are read-only inputs; ConceptWeave does not own their business truth.
- Only an opaque source registry key may cross the port: at most 128 bytes, lowercase multiword `snake_case`. An authorized registry lookup must issue the capability accepted by immutable snapshots; syntax alone is not provenance authority. Passwords, tokens, DSNs, URLs, shell-style connection parameters, and provider-specific connection objects may not cross this boundary.
- Every request needs an explicit non-empty exact-schema allowlist and positive statement-timeout, row, byte, and concurrency bounds.
- Caller cancellation and source disappearance must fail closed rather than return a fabricated or partial success.
- Exact source identifiers keep original case/text; canonicalization may order an allowlist but must not normalize identifier meaning.
- The port must remain provider-independent and free of PostgreSQL driver, credential, semantic-inference, publication, or LLM responsibilities.
- The concrete PostgreSQL adapter must remain outside `conceptweave-domain`, `conceptweave-observation`, and the port contract.

## Options considered

### Put limits and source execution into `conceptweave-observation`

Rejected. That crate owns immutable observation facts. Mixing driver execution policy into the fact model would collapse the Source Observation aggregate boundary and make deterministic replay depend on live-source concerns.

### Let each PostgreSQL adapter define its own timeout/allowlist/error vocabulary

Rejected. This would make resource safety and cancellation non-portable, weaken conformance tests, and allow downstream adapters to silently diverge on what counts as bounded observation.

### Pass a raw connection string plus arbitrary SQL callback through a generic utility layer

Rejected. Raw credentials would cross the boundary, arbitrary SQL would make read-only enforcement unauditable, and a generic utility bucket would erase the Source Observation ubiquitous language.

### Define a small provider-independent Source Observation port

Selected. `conceptweave-source-port` owns request budgets, exact schema authorization, bounded opaque source registry keys, caller cancellation, and typed fail-closed outcomes. Concrete adapters resolve each registry key behind their credential ACL and produce a complete immutable snapshot only after all bounds are satisfied.

## Decision

Introduce the Rust workspace crate `conceptweave-source-port` as a Supporting-domain port contract. `ObservationLimits` requires positive operation/statement-timeout, row, byte, and concurrency limits. `ObservationRequest` requires an opaque source registry key of at most 128 bytes using lowercase multiword `snake_case`, plus a non-empty exact schema allowlist. It rejects raw DSNs/URLs/key-value connection material, one-word/generic keys, malformed registry identifiers, and blank or duplicate schema identifiers, and sorts the allowlist only for deterministic request identity. `SourceConnectionRegistry` resolves the exact key and issues `ResolvedSourceConnection`; `PostgresSchemaSnapshot` accepts only that opaque capability. `ObservationCancellation` carries caller cancellation. `SourceObservationPort` defines the adapter seam. `SourceObservationFailure` distinguishes cancellation, source disappearance, timeout, invalid captured metadata, and row/byte/concurrency-limit exhaustion.

This decision does **not** claim that a production PostgreSQL adapter exists. The next owner-side implementation must select a maintained Rust PostgreSQL driver, resolve the registry key to credentials inside the adapter ACL, establish read-only transaction/session behavior, enforce every port limit in execution rather than configuration only, populate the immutable `conceptweave-observation` contracts, and prove cancellation/source-disappearance behavior against a frozen anonymized reference fixture before live-source readiness is claimed.

## Evidence

- Test-first commit `7cafba262aca070fa6bdccc95284641436a81224` specifies positive resource budgets, exact allowlist behavior, cancellation, and bounded failure outcomes.
- Production commit `016b0aff5a6866d6071e02dd1afa6e116a8ce92b` implements the provider-independent contract.
- Test-first security commit `2f6cd4e6f80b60a0d8118de2162d974bbabde4cc` demonstrates that DSNs, shell-style connection parameters, one-word identifiers, mixed-case identifiers, hyphenated identifiers, and malformed underscore forms must fail before adapter access.
- Production commit `339222cba31f126a5f5f36fe00f890fc82c4aa79` turns `source_connection_key` into the bounded opaque registry-key contract instead of attempting heuristic secret scanning.
- Edge-coverage commit `729820490f7d072d28444432a082d9fae263f194` covers the 128-byte registry-key bound.
- Test-first commits `2194a4ed1b8262d76dca0e7708cfd30114372a2b`, `d073aed`, `a39fa08`, and `38ecdf0` pin targeted foreign-key delete columns and registry-resolved snapshot identity; production commits `eb96251`, `cbfa38a`, and `17c5067` implement those boundaries.
- `docs/product-technical-gap-baseline.md` records the port as implemented-pending-checks and keeps the concrete PostgreSQL adapter OPEN.
- Exact-head hosted Product evidence remains required; predecessor or queued runs are not completion evidence.

## Risks and mitigations

- **Configuration without enforcement:** a concrete adapter could accept limits but ignore them. Mitigation: adapter conformance tests must force timeout, row, byte, concurrency, cancellation, and disappearance failures and verify no snapshot is returned.
- **Credential-shaped caller input:** a caller could otherwise place a DSN or connection parameter string in `source_connection_key` even though the field was documented as non-credential. Mitigation: the port accepts only bounded multiword `snake_case` registry keys; credential lookup remains exclusively inside the adapter ACL.
- **Blocking execution:** a blocking driver could stall an asynchronous product executor. Mitigation: adapter design must isolate blocking work or use an async Rust driver; no blocking database call may run on an async web executor thread.
- **Authorization drift:** a broad or normalized schema selector could observe unintended metadata. Mitigation: exact non-empty allowlists are part of the port and must be applied before catalog results become observations.
- **Partial evidence:** a source can disappear mid-capture. Mitigation: incomplete captures fail with `SourceUnavailable`; immutable snapshot identity is issued only after a complete bounded capture.

## Effects

The Source Observation Context Map now has three explicit layers: caller/application -> `conceptweave-source-port` -> concrete source adapter -> `conceptweave-observation` immutable facts. Semantic Discovery consumes completed observation facts and receipts only; it never receives a live connection handle. Governance & Publication remains downstream and does not gain source execution authority. The caller can reference an approved source connection only through a registry key; adapter-local credential resolution remains an Anti-Corruption Layer concern.

## Concrete scenes

- **Data architect:** selects an approved source registry key and exact schemas. A raw PostgreSQL URL, generic one-word key, blank schema name, or duplicate schema name is rejected before source access.
- **Operator:** sets a finite statement timeout plus row/byte/concurrency budgets. A source that exceeds any budget fails explicitly instead of producing a misleading partial model.
- **User cancellation:** cancellation is propagated across the port; the adapter must stop/abort as supported and return `Cancelled`, not a success receipt.
- **Source restart/disappearance:** a connection loss during metadata capture returns `SourceUnavailable`; no immutable snapshot is published from the incomplete capture.
- **Security review:** credentials remain adapter-owned and absent from request/domain objects; the port admits only a bounded opaque registry key while schema authorization and resource limits remain visible, typed, and testable.

## Follow-up

1. Implement the concrete read-only PostgreSQL adapter behind this port with Rust and an explicit dependency/release decision.
2. Add conformance tests for registry-key credential resolution, timeout, cancellation, row/byte/concurrency exhaustion, source disappearance, quoted identifiers, cross-schema collisions, composite keys, nullable FKs, CHECK/FK validation-enforcement state, domains, enums, indexes, and comments.
3. Bind successful adapter output to immutable extractor receipts and deterministic snapshot identity.
4. Freeze an anonymized GRC-shaped reference fixture without copying foreign product source/DB internals.
5. Revisit this ADR for Accepted status only after the adapter and exact-head conformance evidence are integrated; until then it remains Proposed.
