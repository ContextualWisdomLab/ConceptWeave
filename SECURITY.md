# Security Baseline

## Trust boundaries

All source artifacts, generated candidate payloads, external ontology files, model outputs, and future web-retrieved content are untrusted input.

## Required controls

- source size, type, nesting, archive/decompression, and parser-time bounds;
- immutable source digests and parser/extractor provenance;
- no credentials, secrets, tokens, DSNs, or raw authorization material in semantic evidence;
- prompt-injection text is source data, never tool or policy instruction;
- LLM calls only through `contextual-orchestrator` with minimum necessary context;
- outbound retrieval, when introduced, uses a reviewed SSRF/DNS-rebinding-safe CWL egress boundary;
- no source-system writes from discovery or validation;
- reviewed authorization required before publication;
- future tenant isolation applies to source snapshots, candidates, review receipts, releases, exports, and object storage;
- published artifacts retain origin/provenance and cannot silently overwrite prior releases.

## Threats tracked from foundation

1. semantic poisoning by malicious source text;
2. hallucinated concepts/relations treated as facts;
3. ontology import cycles or reasoning/resource exhaustion;
4. unsafe generated query/expression execution;
5. cross-tenant evidence exposure;
6. provenance stripping during export;
7. malicious or oversized schema/API artifacts;
8. external-source SSRF or credential leakage;
9. model/provider compromise or unexpected retention;
10. governance bypass from Proposed/Validated directly to Published.

Security findings become tests before the related runtime capability can be marked release-ready.
