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

## Proposed admission and owner follow-up

Research Intake remains in ConceptWeave. Full text needs a separate immutable capture receipt binding server/API/schema observations, attachment and bibliographic-parent identities, content and index-statistics digests, read interval, returned status and partial/unknown coverage. Reusing an old metadata digest or governance receipt for later text is forbidden. An availability sweep may guide retrieval and steward work but cannot make unsupported content authoritative or renew prior approval.

The provider fix belongs in Zotero: cover upgrade from synced records, local indexing/reindexing, sync downloads/uploads, local API content writes, missing/pending content, and list/header consistency with regression tests. Until a released provider contract proves those semantics, a consumer must re-enumerate from zero and treat version fields as opaque observations. A complete capture can claim exactly the bytes observed, never an atomic cross-endpoint snapshot on the evidence available here. No upstream issue or provider patch was published by this audit.

### Released orchestration evidence

The 2026-09-05 08:58 UTC owner audit verified public MIT source at protected `contextual-orchestrator/main@a080297d2546bb61e89520d637cabc202db331ec`. Paginated GitHub queries returned zero releases and tags; the exact PyPI project and organization container-package name `contextual-orchestrator` returned 404. These observations do not rule out other names, registries or deployments. The [owner changelog](https://github.com/ContextualWisdomLab/contextual-orchestrator/blob/a080297d2546bb61e89520d637cabc202db331ec/CHANGELOG.md#L3-L11) explicitly labels 0.2.0 Unreleased. Documented `/openapi.json` and local `/v1/responses` examples establish source contracts, not a published schema digest or deployed gateway receipt.

GitHub does contain deployment records: 66 observed records had 57 failure, one queued and eight success states. All eight successes resolve to Provider catalog sync. The latest successful deployment `6276208512` binds `2e414d15ba58f28597751b625a8a2f00fc9fadcf` to [run 33934725405](https://github.com/ContextualWisdomLab/contextual-orchestrator/actions/runs/33934725405); its job refreshes a PostgreSQL-backed provider catalog, stops its containers and supplies no environment URL. An environment named production does not establish a running model gateway. No model request or credential inspection was performed in this audit.

The existing owner [PR #1030](https://github.com/ContextualWisdomLab/contextual-orchestrator/pull/1030), Draft at `f753f453ce4fc3dbc612bb9bdbb8db4cbfd93c16`, already owns immutable release work under ADR 0129. Its owner confirmed that release artifacts, schema digest and gateway deployed-version evidence are still unproved; this audit requested those results there rather than duplicating release machinery. At the subsequent check, both branch and Git-ref endpoints still returned `a080297d2546bb61e89520d637cabc202db331ec`, while PR #1030's base object returned `2e414d15ba58f28597751b625a8a2f00fc9fadcf`. The PR base observation is not substituted for the default-branch ref.

Admission requires an immutable owner artifact and schema digest, protected-source provenance, an identified deployed gateway version and exact-consumer contract evidence. Until then, no model-assisted proposal is generated through copied source, a temporary branch or a direct provider. Catalog-sync success, source documentation and a Draft release PR cannot satisfy this gate. Review labels must not be invented to compensate for unavailable model assistance.

## Follow-up: privately retained content, not reclassification

The [capture evidence](zotero_fulltext_capture_evidence.json) records a separate live run of the proposed Rust command at `2c2226f1d583c3091cc126c96d27d55d1084c0d1`. Unlike the earlier availability audit, this run preserves exact source-response JSON privately. It performed 6,950 sequential requests: two library bookends, two complete manifests and metadata/content reads for all 3,473 manifest entries. The source-read interval was 28,898 ms; total command elapsed time was 33.12 seconds. Maximum resident memory was 283,426,816 bytes and measured peak memory footprint was 332,956,272 bytes. These are one observed local run, not a latency SLO or a production load benchmark.

The capture retains 3,432 successful content responses, 41 missing responses and five empty content strings. Nonempty text is now privately retained for 3,203/3,715 bibliographic parents; 2,561 have nonempty text with complete index counters under the earlier predicate. The 512 without demonstrated nonempty text remain in the denominator. Counter partitions remain 2,755 complete, 67 partial and 610 unknown. No source text was summarized, classified or treated as instructions during capture.

Response bodies total 232,366,711 bytes, including attachment metadata absent from the earlier sweep. The encoded file is 235,602,798 bytes, a new single-link `0600` file outside the repository. The production verifier checked report binding, response bounds, structure, parent/item revision and observed content versions before writing. A separate saved-file audit checked permissions, the complete artifact/evidence/report digests, parent mapping, counts and unchanged report hash without printing identities or source text. Synthetic tests independently exercise Rust deserialization/replay; the saved-file audit does not add an authority verifier. The public JSON retains only aggregate counts, times, limits and digests. No private file is published here.

The original metadata report remains byte-identical. Retained-text coverage improves from zero replayable full-text artifacts to 3,203/3,715 parents; new text-bound proposals, authentic decisions and approved labels remain zero. Follow-up work must present the captured evidence under a new proposal/review binding, preserve missing/partial coverage, and prove a released contextual-orchestrator integration before model assistance. The Zotero version-space defect remains upstream; a consumer capture does not repair it.

## Consumer transport and replay root-cause repairs

- Environment proxy inheritance: RED `9aafff5` reproduced forwarding in 18 synthetic subprocess cases covering six environment-variable spellings and metadata/authorization/item paths. GREEN `a2848e5` explicitly disables proxies in both existing agents. No actual key or live write was involved, and same-host HTTP authentication remains unresolved.
- Inclusive body limits: RED `f3a2847` and `2d9ec5c` reproduced rejection at exactly 2 bytes, the 512-byte authorization limit and the 8 MiB metadata limit. Installed `ureq 3.4.0` source `src/body/limit.rs:21–25`, pinned by `Cargo.lock`, errors when its remaining counter reaches zero before checking EOF. GREEN `c959505` uses the standard library's bounded `Read::take(N+1)`, strict UTF-8 and an explicit `length > N` rejection across the common reader and metadata pagination. Above-limit, invalid-encoding, truncated and chunked-response tests still fail closed. This is a caller-side adaptation to the dependency behavior, not an upstream patch.
- Replay resource order: RED `f7d3530` proved an oversized restored response reached digest work first. GREEN `301d9d5` checks record/body limits before parsing or hashing and uses the installed SHA-256 type's standard `Write` implementation with streamed serialization. A regression proves byte-for-byte digest equivalence with the earlier compact JSON representation; no new wrapper or dependency is introduced.
- Time/failure propagation: RED `7854b3a` and GREEN `2c2226f` exercise invalid clock observations, late responses, the completion deadline and every request failure without sleeping or changing the production deadline. Public transport remains fixed loopback; only private seams accept a synthetic endpoint/clock for tests.

These fixes are integrated into the proposed full-text branch. Earlier open owner/transport PRs must inherit the applicable repairs through ordinary history before promotion; success at this tip does not prove their old heads safe or checked. No predecessor delta was discarded.

## References

Zotero. (2026a, July 29). *Zotero local API*. https://www.zotero.org/support/dev/web_api/v3/local_api

Zotero. (2026b, July 29). *Zotero Web API full-text content requests*. https://www.zotero.org/support/dev/web_api/v3/fulltext_content

Zotero. (n.d.). *Zotero* (Version 10.0.1, commit 36749bd0bd4fdac9ee46c16f7aa7bed094a0851f) [Computer software]. GitHub. https://github.com/zotero/zotero/tree/36749bd0bd4fdac9ee46c16f7aa7bed094a0851f
