# Security Baseline

## Trust boundaries

All source artifacts, generated candidate payloads, external ontology files, model outputs, future web-retrieved content, and semantic-release payloads received by a client are untrusted input. Source Observation request metadata is also untrusted until it passes ConceptWeave's provider-independent structural caps and trusted local policy binds source identity and explicitly admits exact schema scope plus the complete provider-independent resource envelope.

## Required controls

- source size, type, nesting, archive/decompression, and parser-time bounds;
- immutable source digests and parser/extractor provenance;
- no credentials, secrets, tokens, DSNs, or raw authorization material in semantic evidence;
- Source Observation keys and connection-policy bindings are bounded opaque identifiers, never connection material;
- authorization-metadata budgets are capped before trusted source policy: no request may retain more than 4,096 exact schema identifiers or 1,048,576 UTF-8 schema-name bytes, and over-cap budget construction fails with typed errors before registry/database access;
- the structural caps are product-level denial-of-service guardrails, not PostgreSQL identifier semantics or source authority; trusted source policy may only admit an equal-or-narrower effective envelope;
- source-key recognition, exact-schema authorization, and complete resource-envelope admission are distinct controls; schema/resource policy defaults to deny;
- positive caller-selected metadata/runtime limits are structurally bounded requests, not effective policy; wider-than-policy schema-count/schema-byte/operation/statement/row/byte/concurrency ceilings fail before adapter/source/snapshot side effects;
- schema and resource policy are evaluated against the same immutable `ResolvedSourceConnection`; stale key-to-binding mappings must fail before credential/source access;
- one monotonic operation budget begins before local registry source/binding/schema/resource policy and continues through adapter connection/transaction/statements/cancellation; adapters receive only the remaining duration and may not restart the original timeout;
- `AuthorizedObservationRequest` is a single-use operation capability: it is not cloneable, `SourceObservationPort::observe` consumes it, and cancellation/failure/completion requires fresh authorization before any retry so one grant cannot amplify row/byte/concurrency/source-access budgets through replay;
- the synchronous source registry is bounded local policy only; remote credential or network resolution belongs after authorization in the adapter ACL;
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
9. caller-selected authorization metadata attempting pre-policy memory/resource exhaustion or caller-self-authorized schema/resource ceilings reaching a broadly privileged source credential;
10. mutable source-key retargeting that reuses an old authorization for a different physical/policy source;
11. replay of one authorized Source Observation capability to multiply policy-admitted source access or resource consumption;
12. model/provider compromise or unexpected retention;
13. governance bypass from Proposed/Validated directly to Published;
14. in-place mutation or overwrite of previously published semantic truth;
15. consumer use of an incompatible, unpublished, non-authoritative, stale, or superseded release;
16. false integrity claims caused by checking digest syntax without hashing the exact detached artifact bytes;
17. manifest/artifact scope confusion that validates bytes other than the semantic artifact named by the release digest.

Security findings become tests before the related runtime capability can be marked release-ready.