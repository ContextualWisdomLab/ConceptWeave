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
- published semantic truth is immutable: a published artifact must never be overwritten in place, including when an audit trail exists; corrections are issued as a new release that explicitly supersedes the prior release while retaining both releases and their provenance.

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
10. governance bypass from Proposed/Validated directly to Published;
11. in-place mutation or overwrite of previously published semantic truth;
12. credential disclosure or endpoint interposition on provider-defined local transports that do not cryptographically authenticate or encrypt the peer channel.

## Zotero Local API write-back boundary

Zotero 10+ write authorization and mutation use the provider-defined loopback HTTP Local API. Loopback pinning, redirect rejection, and `Zotero-Server-ID` continuity checks do not encrypt `Zotero-API-Key` traffic and do not authenticate the local peer before the key is transmitted. `Zotero-Server-ID` is a database continuity/precondition coordinate, not cryptographic server authentication.

A hostile same-host process capable of binding, observing, or interposing on the loopback endpoint therefore remains an unresolved credential-confidentiality threat. The currently documented Zotero Local API does not provide an HTTPS or OS-authenticated IPC write endpoint that ConceptWeave can substitute. Consequently, mock/local orchestration may be tested, but enterprise-secure live write-back remains fail closed. It may become release-eligible only if Zotero provides a protected transport or an explicit product-security/governance decision narrows the supported threat model and accepts the residual same-host risk. The detailed actor, asset, residual-risk, and release decision is maintained in `THREAT_MODEL.md`, and `docs/TRD.md` carries the same technical boundary.

Security findings become tests before the related runtime capability can be marked release-ready.