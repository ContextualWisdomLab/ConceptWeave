# Security Baseline

## Trust boundaries

All source artifacts, generated candidate payloads, external ontology files, model outputs, future web-retrieved content, and semantic-release payloads received by a client are untrusted input.

## Required controls

- source size, type, nesting, archive/decompression, and parser-time bounds;
- immutable source digests and parser/extractor provenance;
- no credentials, secrets, tokens, DSNs, or raw authorization material in semantic evidence;
- prompt-injection text is source data, never tool or policy instruction;
- LLM calls only through `contextual-orchestrator` with minimum necessary context;
- outbound retrieval, when introduced, uses a reviewed SSRF/DNS-rebinding-safe CWL egress boundary;
- no source-system writes from discovery or validation;
- reviewed authorization required before publication;
- client authoritative-use admission must fail closed on unsupported contract versions, non-Published state, or non-Authoritative truth status;
- client admission does not substitute for consuming-product tenant/purpose authorization;
- declared release digest syntax is not an integrity claim: `SemanticReleaseClient::verify_detached_artifact` must hash the exact detached immutable semantic-artifact bytes and compare them with the declared digest before those supplied bytes are accepted as the referenced artifact;
- the release manifest's digest names detached artifact bytes rather than claiming a self-referential digest of the manifest bytes that contain that field;
- signature authenticity and provenance-chain verification remain separate controls until Governance & Publication defines a stable signing contract;
- future tenant isolation applies to source snapshots, candidates, review receipts, releases, exports, and object storage;
- published semantic truth is immutable: a published artifact must never be overwritten in place, including when an audit trail exists; corrections are issued as a new release that explicitly supersedes the prior release while retaining both releases and their provenance.

## Threats tracked from foundation

1. semantic poisoning by malicious source text;
2. hallucinated concepts/relations treated as facts;
3. ontology import cycles or reasoning/resource exhaustion;
4. unsafe generated query/expression execution;
5. cross-tenant evidence exposure;
6. provenance stripping during export or consumption;
7. malicious or oversized schema/API/release artifacts;
8. external-source SSRF or credential leakage;
9. model/provider compromise or unexpected retention;
10. governance bypass from Proposed/Validated directly to Published;
11. in-place mutation or overwrite of previously published semantic truth;
12. consumer use of an incompatible, unpublished, non-authoritative, stale, or superseded release;
13. false integrity claims caused by checking digest syntax without hashing the exact detached artifact bytes;
14. manifest/artifact scope confusion that validates bytes other than the semantic artifact named by the release digest.

Security findings become tests before the related runtime capability can be marked release-ready.
