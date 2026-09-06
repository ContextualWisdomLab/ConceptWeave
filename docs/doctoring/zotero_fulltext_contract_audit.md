# Zotero full-text availability and version-contract audit

Status: observed provider-contract gap; proposed consumer requirements, not implemented full-text classification. Observation: 2026-09-05 07:36:42 UTC. Related work: [PR #34](https://github.com/ContextualWisdomLab/ConceptWeave/pull/34), [PRD FR-9](../PRD.md), [TRD Research Intake](../TRD.md), [Proposed ADR 0006](../adr/0006-zotero-research-intake.md).

## Motivation and scope

The repaired metadata campaign covers 3,715 bibliographic items, but 3,658 proposals abstain and 1,000 reports retain no abstract. Missing metadata is not evidence of irrelevance. Before requesting another acquisition service or model integration, this audit tests the existing Local API's full-text surface. It neither reads Zotero's database directly nor modifies source records, review decisions or earlier reports.

The [aggregate audit record](zotero_fulltext_read_audit.json) binds this observation to ConceptWeave `2a75051f0082103511222e278de24b2690fe6bfe`, the repaired report's file/content digests, and Zotero 10.0.1/API 3/schema 44. It contains no item keys, bibliography, full text, reviewer identity, server identity or credentials. This was a bounded read-only diagnostic, not a new released adapter or a classifier experiment.

## Observed full-denominator results

The manifest returned 3,473 entries, all linked to the prior metadata snapshot. They identify 3,241 candidate bibliographic parents. An entry is not proof that its content exists: the subsequent sweep attempted every entry, yielding 3,432 HTTP 200 responses and 41 HTTP 404 responses, with no other status. Five returned content strings were empty after trimming. Nonempty content linked to 3,203 distinct bibliographic parents.

| Retained metadata | Bibliographic denominator | Nonempty full text returned | Nonempty full text and complete index counters |
| --- | ---: | ---: | ---: |
| Abstract absent from report | 1,000 | 800 | 598 |
| Abstract present in report | 2,715 | 2,403 | 1,963 |
| Total | 3,715 | 3,203 | 2,561 |

At attachment level, 2,755 responses had complete index counters, 67 partial counters, and 610 did not prove completeness under the audit predicate. That predicate requires at least one pages/characters pair; every present pair must consist of nonnegative integers, a positive total and indexed count no greater than total. Completeness additionally requires equality for every present pair. The 610 are not declared corrupt: absent, zero or otherwise unusable counters cannot establish completeness. Counter equality does not verify OCR quality, language coverage, scientific relevance or extraction accuracy.

The remaining 512 bibliographic parents have no demonstrated nonempty full text in this sweep; 200 also lack a retained abstract. They remain in the full campaign denominator. These are follow-up research needs, not exclusion decisions. Full-text-bound proposals and independently approved labels both remain zero.

## Reproduction and privacy boundary

The diagnostic read the existing owner-only repaired report in memory. It requested `items?limit=1` before the sweep, `fulltext?since=0`, every listed attachment's `items/{itemKey}/fulltext`, then the manifest and one-item endpoint again. Item keys were interpolated only in process memory; no URL, content or key was printed. All requests used fixed loopback port 23119, API v3, the report's expected server identity, redirect rejection and API/schema/server continuity checks. Reads required no authorization prompt or key.

The execution was sequential with a 20-second request timeout, an 8 MiB response bound, a 512 MiB cumulative bound and a five-minute before-request admission budget. Actual elapsed time was 10,737 ms and transferred response bodies totaled 224,842,838 bytes. These diagnostic limits are not the production metadata reader's limits or model timeouts. A deadline/budget/identity/JSON failure would make the sweep incomplete; omitted requests must remain visible in its denominator.

Raw text was held for one response at a time and discarded. The ordered-response SHA-256 streams compact JSON arrays of attachment key, HTTP status, observed content-version header and body SHA-256, in lexicographic key order. The public record retains only the final aggregate digest, not those arrays. Body hashes are over received bytes, not canonical JSON. They identify this sweep's responses but cannot replay discarded source content or serve as an approval receipt.

These standalone probes disable curl's configuration-file loading, bypass proxies, restrict the protocol to HTTP and allow no redirects. They expose only aggregate counts and public protocol values, not response bodies or identities:

```sh
curl --disable --noproxy '*' --proto '=http' --max-redirs 0 \
  --silent --show-error --fail --max-time 20 --max-filesize 8388608 \
  -H 'Zotero-API-Version: 3' \
  'http://127.0.0.1:23119/api/users/0/fulltext?since=0' \
  | jq '{manifest_entries: length, version_types: ([.[] | type] | unique), minimum_version: ([.[]] | min), maximum_version: ([.[]] | max)}'

curl --disable --noproxy '*' --proto '=http' --max-redirs 0 \
  --silent --show-error --fail --max-time 20 --max-filesize 8388608 --output /dev/null \
  --write-out 'HTTP %{http_code}; version=%header{last-modified-version}; API=%header{zotero-api-version}; schema=%header{zotero-schema-version}\n' \
  -H 'Zotero-API-Version: 3' \
  'http://127.0.0.1:23119/api/users/0/fulltext?since=0'
```

These probes do not reproduce the parent/content sweep or establish a coherent snapshot. The complete audit additionally requires the original private report and the guarded enumeration described above. No raw content was committed or used for generated classifications.

## Root cause: a mixed-origin cursor and a missing header

The official documentation says Zotero 10 full-text versions are local and describes a library `Last-Modified-Version` on the list response (Zotero, 2026a, 2026b). The observed list omitted that header. The one-item endpoint reported local library version 2, while the manifest contained 41 zero versions and 3,432 versions greater than 2, with a maximum of 12,403. Each successful content response's version matched its manifest entry.

The official `10.0.1` tag resolves to `36749bd0bd4fdac9ee46c16f7aa7bed094a0851f`. Its source explains the discrepancy:

| Producer / endpoint | Exact behavior and evidence |
| --- | --- |
| List and individual content reads | The [endpoint source](https://github.com/zotero/zotero/blob/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f/chrome/content/zotero/xpcom/server/server_localAPI.js#L1431-L1487) reads the stored full-text version directly. The list returns a raw response tuple without the library-version header. |
| Sync download and upload | The [sync engine](https://github.com/zotero/zotero/blob/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f/chrome/content/zotero/xpcom/sync/syncFullTextEngine.js#L99-L168) passes remote content/server library versions into the same full-text version storage. |
| Ordinary indexing | The [index writer](https://github.com/zotero/zotero/blob/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f/chrome/content/zotero/xpcom/fulltext.js#L510-L527) defaults a missing version to zero. |
| Local API writes | The [write endpoint](https://github.com/zotero/zotero/blob/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f/chrome/content/zotero/xpcom/server/server_localAPI.js#L2594-L2622) supplies the incremented local library version to that same storage. No such write was performed in this audit. |
| Upgrade migration | [Migration 129](https://github.com/zotero/zotero/blob/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f/chrome/content/zotero/xpcom/schema.js#L3726-L3730) adds local versions to metadata objects/libraries, not full-text records. |
| Missing-content entries | The [missing-content path](https://github.com/zotero/zotero/blob/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f/chrome/content/zotero/xpcom/fulltext.js#L1450-L1459) can insert an empty zero-version row. Manifest presence is therefore weaker than retrievability. |

This is a provider contract mismatch, not evidence of a corrupt user library. The manifest bytes and metadata library version were unchanged at the bookends, but those observations do not establish atomicity or rule out a same-version full-text edit. The mixed-origin field must not become a reliable incremental cursor, an item revision or a write precondition. Adding the missing header alone would not fix the version semantics.

### Current upstream recheck

At 2026-09-05 09:16 UTC, official maintenance `10.0@a5b4b4d20d12cf07af43d928bd66090faed1a655` and development `main@fc17dcd24ad34686cb24e6b3ffb06a6a7a5e0e5d` retained the same Local API, full-text storage and sync-engine blobs as 10.0.1. Their respective blob identities are `6cfbaf8247a5e914c92e8711be46d4431e79923e`, `ca981b95fdbd41fa00927dd606ac5a8fde0e1cb6` and `225b0449b362201ad5d8717510af3478ef15d2b9`. The [current list endpoint](https://github.com/zotero/zotero/blob/fc17dcd24ad34686cb24e6b3ffb06a6a7a5e0e5d/chrome/content/zotero/xpcom/server/server_localAPI.js#L1459-L1487) still returns the stored version without the promised library-version header. No provider repair was verified.

Bounded official issue/PR searches returned 34 fulltext matches but no dedicated matching defect report. Historical [issue #5002](https://github.com/zotero/zotero/issues/5002) and merged [PR #5004](https://github.com/zotero/zotero/pull/5004) introduce the endpoint; [Full-text v2 draft #5673](https://github.com/zotero/zotero/issues/5673) concerns a richer format, and [semantic-search Draft PR #6012](https://github.com/zotero/zotero/pull/6012) retains the same endpoint blob at its inspected head `19e79625b1c6fbbdd75367aa85b62d5a7080d7f6`. None establishes this cursor repair. Search results are not global absence proof. No upstream issue, patch, installation change or private-library read was made during this recheck.

## Proposed admission and owner follow-up

Research Intake remains in ConceptWeave. Full text needs a separate immutable capture receipt binding server/API/schema observations, attachment and bibliographic-parent identities, content and index-statistics digests, read interval, returned status and partial/unknown coverage. Reusing an old metadata digest or governance receipt for later text is forbidden. An availability sweep may guide retrieval and steward work but cannot make unsupported content authoritative or renew prior approval.

The provider fix belongs in Zotero: cover upgrade from synced records, local indexing/reindexing, sync downloads/uploads, local API content writes, missing/pending content, and list/header consistency with regression tests. Until a released provider contract proves those semantics, a consumer must re-enumerate from zero and treat version fields as opaque observations. A complete capture can claim exactly the bytes observed, never an atomic cross-endpoint snapshot on the evidence available here. No upstream issue or provider patch was published by this audit.

### Released orchestration evidence

The 2026-09-05 08:58 UTC owner audit verified public MIT source at protected `contextual-orchestrator/main@a080297d2546bb61e89520d637cabc202db331ec`. Paginated GitHub queries returned zero releases and tags; the exact PyPI project and organization container-package name `contextual-orchestrator` returned 404. These observations do not rule out other names, registries or deployments. The [owner changelog](https://github.com/ContextualWisdomLab/contextual-orchestrator/blob/a080297d2546bb61e89520d637cabc202db331ec/CHANGELOG.md#L3-L11) explicitly labels 0.2.0 Unreleased. Documented `/openapi.json` and local `/v1/responses` examples establish source contracts, not a published schema digest or deployed gateway receipt.

GitHub does contain deployment records: 66 observed records had 57 failure, one queued and eight success states. All eight successes resolve to Provider catalog sync. The latest successful deployment `6276208512` binds `2e414d15ba58f28597751b625a8a2f00fc9fadcf` to [run 33934725405](https://github.com/ContextualWisdomLab/contextual-orchestrator/actions/runs/33934725405); its job refreshes a PostgreSQL-backed provider catalog, stops its containers and supplies no environment URL. An environment named production does not establish a running model gateway. No model request or credential inspection was performed in this audit.

The existing owner [PR #1030](https://github.com/ContextualWisdomLab/contextual-orchestrator/pull/1030), Draft at `f753f453ce4fc3dbc612bb9bdbb8db4cbfd93c16`, already owns immutable release work under ADR 0129. The contacted CO integration task reported release artifacts, schema digest and gateway deployed-version evidence as unproved. It later clarified that its single-writer scope is #1067/#1074, not #1030, and will identify the existing release owner's exact evidence. Confirmation from that actual release owner remains pending; no duplicate release machinery was requested. At the subsequent branch check, both branch and Git-ref endpoints still returned `a080297d2546bb61e89520d637cabc202db331ec`, while PR #1030's base object returned `2e414d15ba58f28597751b625a8a2f00fc9fadcf`. The PR base observation is not substituted for the default-branch ref.

The 12:05:30–12:07:20 UTC refresh confirmed the same protected default SHA, zero releases/tags and unchanged Draft #1030. In the new-artifact window since 08:58, the newest-100 listing added three SBOMs and one Strix report, not a released client/schema bundle; this was not an audit of all 7,026 historical artifacts. Deployment `6277953573` became successful at 11:27:06 through [Provider catalog sync run 33948079376](https://github.com/ContextualWisdomLab/contextual-orchestrator/actions/runs/33948079376). New queued deployment `6280414175` likewise points to a [provider-catalog job](https://github.com/ContextualWisdomLab/contextual-orchestrator/actions/runs/33961185349/job/101299036078), with no gateway environment URL. The earlier 66-record state partition is historical, not the new total. The existing integration task was idle with no new release evidence and was not restarted or reassigned release ownership.

Admission requires an immutable owner artifact and schema digest, protected-source provenance, an identified deployed gateway version and exact-consumer contract evidence. Until then, no model-assisted proposal is generated through copied source, a temporary branch or a direct provider. Catalog-sync success, source documentation and a Draft release PR cannot satisfy this gate. Review labels must not be invented to compensate for unavailable model assistance.

## Follow-up: privately retained content, not reclassification

The [capture evidence](zotero_fulltext_capture_evidence.json) records a separate live run of the proposed Rust command at `2c2226f1d583c3091cc126c96d27d55d1084c0d1`. Unlike the earlier availability audit, this run preserves exact source-response JSON privately. It performed 6,950 sequential requests: two library bookends, two complete manifests and metadata/content reads for all 3,473 manifest entries. The source-read interval was 28,898 ms; total command elapsed time was 33.12 seconds. Maximum resident memory was 283,426,816 bytes and measured peak memory footprint was 332,956,272 bytes. These are one observed local run, not a latency SLO or a production load benchmark.

The capture retains 3,432 successful content responses, 41 missing responses and five empty content strings. Nonempty text is now privately retained for 3,203/3,715 bibliographic parents; 2,561 have nonempty text with complete index counters under the earlier predicate. The 512 without demonstrated nonempty text remain in the denominator. Counter partitions remain 2,755 complete, 67 partial and 610 unknown. No source text was summarized, classified or treated as instructions during capture.

Response bodies total 232,366,711 bytes, including attachment metadata absent from the earlier sweep. The encoded file is 235,602,798 bytes, a new single-link `0600` file outside the repository. The production verifier checked report binding, response bounds, structure, parent/item revision and observed content versions before writing. A separate saved-file audit checked permissions, the complete artifact/evidence/report digests, parent mapping, counts and unchanged report hash without printing identities or source text. Synthetic tests independently exercise Rust deserialization/replay; the saved-file audit does not add an authority verifier. The public JSON retains only aggregate counts, times, limits and digests. No private file is published here.

The original metadata report remains byte-identical. Retained-text coverage improves from zero replayable full-text artifacts to 3,203/3,715 parents; new text-bound proposals, authentic decisions and approved labels remain zero. The next section records the separate read-only review view. Follow-up decision and approval work must retain its exact context, preserve missing/partial coverage, and prove a released contextual-orchestrator integration before model assistance. The Zotero version-space defect remains upstream; a consumer capture does not repair it.

### Follow-up: offline inspection of the first 25 pending papers

The [review-view evidence](zotero_fulltext_review_evidence.json) is bound to `54383eae2e83863c4cb72ee00f16cd504ff66151`. The release-profile Rust executable restored the existing private report, worksheet and 235,602,798-byte capture without another provider request. It validated the complete capture before selecting the canonical 25 pending rows. Those rows match the earlier metadata-only batch exactly; 21 have nonempty retained text and four have none. All 21 copied content responses are HTTP 200, with 16 complete and five unknown index-counter results under the predicate above. These are this batch's counts, not a new full-library or abstract-missing-subgroup measurement.

The create-new single-link `0600` view is 1,590,742 bytes. A separate local audit recomputed capture, complete-report and proposal digests, compared every selected parent/revision/content response and confirmed the original report, worksheet, capture and batch hashes remained unchanged. The command took 1.60 seconds with maximum resident memory 288,014,336 bytes and peak memory footprint 286,966,336 bytes. The saved-file audit is outside that command measurement; neither constitutes a load benchmark. The public record includes aggregates only, never bibliography, item identities or raw text.

The view retains the original metadata proposal digest and a separate capture binding. Unchanged predictions do not require a fresh proposal merely to display additional evidence. Its outer versioned envelope is intentionally not accepted by either existing decision-application command. Full-text evidence binding through decision application, worksheet history, finalization and external approval remains unimplemented; stripping the envelope does not preserve that provenance. Pending rows with an evidence view improve from zero to 25, including 21 with nonempty captured text and four without it, while remaining decisions and externally approved labels stay 3,715 and zero respectively.

The implementation reuses the canonical pending selector and capture verifier, borrows retained responses during projection, and writes into the standard library's fixed-size slice-backed cursor. It adds no service, dependency, mutable report fields or duplicate security helper. The separately bounded buffered capture reader uses pinned `serde_json` 1.0.151; installed source confirms whole-stream completion checks and the caller separately checks the actual byte count. Metadata and output remain bounded at 16 MiB, capture files at 512 MiB, and raw response bodies at the existing 256 MiB cumulative bound. High JSON escaping can exceed a file/output ceiling without exceeding the raw-body ceiling; such input fails without truncation. The Proposed [ADR 0006 amendment](../adr/0006-zotero-research-intake.md) records alternatives, consequences and the outstanding approval contract.

## Consumer transport and replay root-cause repairs

- Environment proxy inheritance: RED `9aafff5` reproduced forwarding in 18 synthetic subprocess cases covering six environment-variable spellings and metadata/authorization/item paths. GREEN `a2848e5` explicitly disables proxies in both existing agents. No actual key or live write was involved, and same-host HTTP authentication remains unresolved.
- Inclusive body limits: RED `f3a2847` and `2d9ec5c` reproduced rejection at exactly 2 bytes, the 512-byte authorization limit and the 8 MiB metadata limit. Installed `ureq 3.4.0` source `src/body/limit.rs:21–25`, pinned by `Cargo.lock`, errors when its remaining counter reaches zero before checking EOF. GREEN `c959505` uses the standard library's bounded `Read::take(N+1)`, strict UTF-8 and an explicit `length > N` rejection across the common reader and metadata pagination. Above-limit, invalid-encoding, truncated and chunked-response tests still fail closed. This is a caller-side adaptation to the dependency behavior, not an upstream patch.
- Replay resource order: RED `f7d3530` proved an oversized restored response reached digest work first. GREEN `301d9d5` checks record/body limits before parsing or hashing and uses the installed SHA-256 type's standard `Write` implementation with streamed serialization. A regression proves byte-for-byte digest equivalence with the earlier compact JSON representation; no new wrapper or dependency is introduced.
- Time/failure propagation: RED `7854b3a` and GREEN `2c2226f` exercise invalid clock observations, late responses, the completion deadline and every request failure without sleeping or changing the production deadline. Public transport remains fixed loopback; only private seams accept a synthetic endpoint/clock for tests.

These fixes are integrated into the proposed full-text branch. The earlier owner backports now preserve their own committed regressions: metadata PR #9 RED `31b507a` → GREEN `a2a8488`; authenticated transport PR #17 RED `f83a63d` → production repair `53bd1fe`; extracted authorization PR #18 RED `178e03b` → GREEN `ba1bbd2`. The shared synthetic server also needed request-framing RED `b6b618b` → test-helper repair `7bcb791`: reading only one TCP chunk could miss the POST body and close while the large response was still arriving. This was test infrastructure, not a new production failure or a reason to reduce the response-size case.

At #17 `7bcb791853ffa794529418ee9de1337fea4e1b15`, 91 workspace tests and 20 repetitions of five focused tests passed. At #18 `ba1bbd203c2f90afbf97ac3d7eab989982e8bc09`, 99 workspace tests and ten repetitions of eight focused tests passed, including exactly 512-byte success/denial and one-byte-over rejection. Strict Clippy and formatting passed at both heads. All requests were synthetic; no actual key, authorization or Zotero write was used.

The later #19 coverage failure, 333/334 normalized branch outcomes, exposed nondeterministic TCP fragmentation in the shared test server. Test-only canonical #17 `b388810be8bceb3a4f81c336708cf1c56a20d057` adds an 8 KiB header and 8 KiB body through the unchanged 4 KiB read buffer. #17 then passes 92 workspace tests and 318/318 normalized branches; ordinary restacks give #18 `21a7ee8f8b4b0988c13bb45aecbb016242c21308` 100 tests and #19 `aa74f8642e9e8c3804996ce443650df29f08bf5f` 101 tests with 334/334 branches. No exclusion or production behavior changed for this regression.

The final non-force cascade reaches #34 `b0119a57047e7b1fe5ddfbbf4b973de0f15de172`, with 156 workspace tests and its existing coverage gate passing. Root full-text integration `75da75cf01704d9aae47f1e5573e3bbe3fb42bb0` passes 186 workspace tests across 37 unfiltered suites, strict Clippy, formatting, rustdoc, CI contract and the existing coverage gate. It preserves one unchanged shared reader, inherited regression modules and all earlier full-text safeguards. Source-normalized coverage is 3,710/3,710 regions and 674/674 branches; functions are 347/347. Raw LLVM totals remain 4,159/4,255 lines, 6,129/6,274 regions and 603/674 branches. Independent source review found no actionable merge finding, not approval. No predecessor delta was discarded; every head still requires its own protected acceptance evidence.

### Review binding follow-up, 2026-09-05

The [blank initialization evidence](zotero_fulltext_review_binding_evidence.json) and [baseline](../product-technical-gap-baseline.md#capture-bound-review-application-and-blank-initialization) record the initial separate review contract at `da406cf6110888808fa530592bbce2b774b73f33`. Its CLI then only initialized work. The original source files remain unchanged, 3,715 new review slots are blank, and no approval, model call or Zotero write occurred.

The subsequent [private command evidence](zotero_bound_review_commands_evidence.json), at `2fadbdaba3cd546f6c345ed4a950baef79325982`, verifies the new capture-bound pending-view command against those real saved files. It created a separate 1,590,742-byte `0600` single-link output in 2.22 seconds, with maximum resident memory 288,505,856 bytes. A separate read-only audit at 12:21:47.264 UTC confirmed all input hashes, the exact capture marker, 25 blank pending rows, 21 parents with nonempty text and four without, and byte equality with the earlier read-only view. Equality proves reuse, not more reviewed papers or additional text coverage. The same CLI now exposes atomic application and complete-review finalization, exercised only with test inputs; authenticated governance verification remains outside it. Authentic decisions and independently approved labels remain 0/3,715. Neither this work nor source-free error diagnostics resolve the provider version-space defect or supply released orchestration.

For restored owned review artifacts, unknown fields must fail instead of disappearing during deserialization. Serde documents that distinction and disallows combining its strict container attribute with flattening (Serde contributors, n.d.). JSON object-name duplication has inconsistent receiver behavior under RFC 8259; ConceptWeave rejects duplicate decoded keys recursively before comparing completed evidence views, rather than accepting last-key-wins projection (Bray, 2017, Section 4). These primary references support the input-contract choice, not semantic-label correctness. Context7's monthly quota was exhausted and DeepWiki did not index this repository during this follow-up; direct official documentation and current source/tests supplied the evidence.

## References

Bray, T. (Ed.). (2017). *The JavaScript Object Notation (JSON) data interchange format* (RFC 8259, Section 4). Internet Engineering Task Force. https://www.rfc-editor.org/rfc/rfc8259#section-4

Serde contributors. (n.d.). *Container attributes*. Serde. Retrieved September 5, 2026, from https://serde.rs/container-attrs.html

Serde contributors. (n.d.). *Field attributes*. Serde. Retrieved September 6, 2026, from https://serde.rs/field-attrs.html

Serde contributors. (n.d.). *Implementing Serialize*. Serde. Retrieved September 6, 2026, from https://serde.rs/impl-serialize.html

Zotero. (2026a, July 29). *Zotero local API*. https://www.zotero.org/support/dev/web_api/v3/local_api

Zotero. (2026b, July 29). *Zotero Web API full-text content requests*. https://www.zotero.org/support/dev/web_api/v3/fulltext_content

Zotero. (n.d.). *Zotero* (Version 10.0.1, commit 36749bd0bd4fdac9ee46c16f7aa7bed094a0851f) [Computer software]. GitHub. https://github.com/zotero/zotero/tree/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f
