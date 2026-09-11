# Procedural self-application and governed revision admission

Status: Proposed / source-shaped Rust repair / native Rust acceptance unverified / not operational. Tracking: ConceptWeave #42, parent #43, ContextualWisdomLab/.github#2067. Observed parent at this revision: `2c6d3037acdeae2b152c335ee2f60a59b4472831`; Foundation observation: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`. Live GitHub state supersedes recorded coordinates. This document extends ADR-PG-20260910, PRD, TRD and the product/technical gap baseline; it is not publication authority.

## Ownership and epistemic boundary

ConceptWeave owns procedural representation engineering inside its semantic-engineering lifecycle: `observe -> discover -> propose -> align -> validate -> review -> publish`. The self-application work has two separate uses:

- object-level semantic authoring: evidence-bound procedures and relations for semantic discovery/alignment/validation and steward review;
- meta-level refinement: retain a baseline, observe training outcomes, propose a revision, preserve rejected alternatives, validate it, pass it to independent evaluation and then to steward decision.

Neither path may edit its own permissions, self-approve, relabel failed evidence, change acceptance criteria, publish itself or activate a runtime graph. Noema owns execution-local projection/lifecycle/runtime authorization integration; contextual-orchestrator owns production model routing; context-graph-contracts owns released interchange; Keyverse owns identity/credential/authentication trust. ConceptWeave retains authorization of its own exact proposal/base resources after authenticated identity evidence is admitted.

Lu et al. (2026) motivate procedural graphs, local guidance, retained rejected proposals and offline refinement. Those method claims do not prescribe CWL repository ownership, authentication or publication design, and this branch does not claim a paper replication or performance gain.

## Current source and contract map

| Artifact / symbol | Current role |
| --- | --- |
| `contracts/procedural-model-draft.schema.json` | Local unreleased Draft 2020-12 candidate shape; `draft/inferred`, bounded evidence/artifact coordinates, eight locale slots and the procedural cross-runtime nonblank text contract |
| `contracts/procedural-revision-proposal.schema.json` | Local offline proposal envelope; `proposed`, `decision_authority:none`, exact base coordinate, training evidence, retained rejected-edit references and rationale |
| `profiles/semantic_engineering/semantic_authoring.draft.json` | First-party inferred Draft authoring profile; runtime activation remains disabled |
| `profiles/semantic_engineering/procedural_refinement.draft.json` | Separate inferred Draft refinement profile with independent evaluation/review and rejection paths |
| `profiles/semantic_engineering/profile_manifest.json` | Exact local-Git source/profile coordinates and digests; provenance metadata, not signature or approval |
| `scripts/parse_strict_json.mjs` | Repository-side duplicate-decoded-member-safe JSON admission used before AJV for owned schema/fixture artifacts |
| `crates/conceptweave-domain/src/procedural_model_transport.rs` | Private std-only raw-byte/UTF-8/strict-JSON recognizer with byte/depth/surrogate/duplicate-member bounds |
| `crates/conceptweave-domain/src/procedural_model_ingress.rs` | Private canonical Draft/revision mapping, cross-runtime text admission, proposal/base/scope expectation binding and typed proposal retention |
| `crates/conceptweave-domain/src/procedural_model_validation.rs` | Borrowed semantic/topology validation: scope equality, evidence closure, semantic/tool coordinates, topology, reachability and schema-significant text semantics |
| `crates/conceptweave-domain/tests/procedural_model_transport.rs` and `procedural_model_ingress.rs` | Raw transport and Draft mapping regression contracts, including canonical blank-union witnesses |
| `crates/conceptweave-domain/tests/procedural_model_semantics.rs` and `procedural_model_validation.rs` | Direct semantic/topology contracts, including alternate-ingress parity witnesses |
| `crates/conceptweave-domain/tests/procedural_revision_ingress.rs` and `procedural_revision_semantic_preservation.rs` | Revision-envelope/context-binding and lossless proposal-semantic contracts |

The procedural Rust modules are intentionally not exported as accepted product API. Integration tests path-import the exact source modules while the branch is Draft. Widening/exporting these seams requires one unchanged exact head with pinned Rust 1.98 fmt, strict all-target Clippy, native tests, rustdoc, release and owned-production coverage plus the required hosted Product/security/review evidence. A source-shaped repair is not a substitute for that evidence.

## Admission sequence and invariants

The current private path is deliberately layered:

1. bound raw bytes before UTF-8 conversion;
2. parse strict JSON with decoded duplicate-member rejection, bounded nesting and valid surrogate handling;
3. map only the canonical Draft 2020-12 shape, rejecting unknown/missing/wrong-typed/schema-invalid members before domain construction;
4. preserve procedure kind, locale labels, optional `semantic_refs` presence, tool-contract coordinates, relation condition/guidance/pitfalls and evidence references;
5. apply one procedural cross-runtime nonblank policy before inherited Foundation evidence construction, then validate logical procedure/relation identity, evidence closure, topology and explicit reachability policy;
6. for a revision proposal, retain proposal origin, exact base coordinate, complete candidate, training evidence, rejected-edit references and rationale in one typed `ProceduralRevisionAdmission`;
7. compare proposal/base/candidate scope with a separately supplied `ProceduralRevisionExpectation` before deterministic candidate validation.

`ProceduralRevisionExpectation` is not an authentication receipt. Its equality checks only prove that two independently supplied coordinate sets match. Production construction must wait for an immutable released Keyverse trust contract at the application boundary, then apply ConceptWeave-owned authorization for the exact proposal/base resource. Mutable Keyverse source, JWT/provider verification inside `conceptweave-domain`, or a caller-minted `authenticated=true` substitute are invalid integrations.

Artifact references are coordinates only. Semantic references and tool-contract references still require released-owner authenticity, ACL/capability resolution and consumer authorization before use. Training evidence remains proposed evidence; it is not evaluation truth, approval or publication authority.

## Canonical text semantics and alternate-ingress parity

The original procedural Draft contract used ECMAScript regular-expression `\S` as its nonblank test. That was not a stable cross-runtime definition. ECMAScript 2026 treats U+FEFF ZERO WIDTH NO-BREAK SPACE as White Space and explicitly excludes Unicode `White_Space` code points that are not `Space_Separator`, while Rust `str::trim()` follows Unicode `White_Space`. U+0085 NEXT LINE is the material inverse witness: Unicode classifies it as `White_Space`, but ECMAScript does not include it in its regular-expression whitespace set. A contract expressed only as ECMAScript `\S` could therefore accept U+0085-only text that a later Foundation `str::trim()` constructor rejected, even after the earlier U+FEFF repair.

Review `5173430684` first captured the U+FEFF direction. Source-level RED `029502b278552929677e62ee9df23afd48fd77d6` added U+FEFF-only witnesses for locale annotations and evidence locations, and repair `1ace968b309334bdba8475f8357a207d405ca96b` stopped the borrowed validator from relying on Rust `str::trim()` for that boundary.

Review `5173839274` then captured the inverse U+0085 mismatch. Source-level RED `692928c81bbfdec2bc5f3ed652f09d15a7021e77` requires the direct Rust semantic boundary to reject U+0085-only annotation text. Repairs `dfc532689d9b23a999ac6d296eb3d6af0c6c8857` and `bb07a5d6c5d9b5bc4da77051e20ee2b310e04e21` define the same procedural canonical blank union in the borrowed validator and manual Draft mapper: ECMAScript whitespace/line terminators, including U+FEFF, plus U+0085. Contract successor `832f7f847315dde7d1f23df252047ec1d4b6a48c` encodes the same rule in Draft 2020-12 `annotation_text` as `[^\s\u0085]`, while `6ea9aeb9af019955ca808a548e6660aacb7c9c75` adds AJV witnesses and `eee54d250e1318634ea611bb849e113c8c34ac88` pins the transport-to-domain mapping boundary. Visible text containing U+0085 remains admissible; blank-only U+0085, U+FEFF, or their combination does not.

This is deliberately a procedural contract refinement, not an attempt to mutate the historical Foundation source from a child branch. Foundation review `5173461698` still records the generic `EvidenceReference::new` / `SemanticCandidate::new` versus `semantic-candidate.schema.json` U+FEFF discrepancy. That generic owner repair remains sequenced after #35 normally integrates and Foundation is ordinary/non-force restacked. The procedural path now fails closed before it reaches the inherited generic evidence constructor, so its accepted text language no longer depends on which runtime performed the first nonblank check.

Authoritative references for this distinction:

- Ecma International. (2026). *ECMAScript® 2026 language specification*, §§12.2–12.3, White Space and Line Terminators. TC39. https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-white-space
- The Rust Project Developers. (2026). *Primitive type `str`: `trim`*. Rust standard library documentation. https://doc.rust-lang.org/std/primitive.str.html#method.trim
- The Unicode Consortium. (2026). *Unicode Character Database: White_Space property*. Unicode Standard Annex and property data. https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt

The standards define the differing character sets. Review/commit coordinates above are the implementation evidence; no standards citation substitutes for exact-head executable acceptance.

## Provenance and security boundary

The self-application manifest is verified against bounded local-Git provenance: exact `commit:path`, object identity/type, regular-file mode, authored-history membership, bounded source bytes and SHA-256. Ambient `GIT_*` overrides are scrubbed and attacker-controlled duplicate keys are not echoed into diagnostics. This establishes local repository provenance only. It does not authenticate remote source ownership, make the source statement normatively correct, establish steward identity or authorize publication.

The profiles are authored in Korean/English and remain `inferred`/`draft`. No released executable tool-operation mapping is asserted. A future CGC/Noema projection must preserve or explicitly map entry/procedure identity, relation direction, locale/evidence and abstention/rejection semantics; it may not silently truncate or activate a graph. Mutable sibling PR heads are not operational dependencies.

## Evidence status

Earlier Node/AJV/Python authoring observations remain historical predecessor evidence and do not transfer to the current Rust source head. The current branch contains source-level regression contracts for transport, schema mapping, semantics/topology, revision context and lossless proposal retention, including the U+FEFF/U+0085 cross-runtime blank-union witnesses above.

Native acceptance is still absent. This execution environment does not provide the repository-pinned Rust 1.98 toolchain, and the current #44 lane has not yet produced hosted Product evidence proving the newest Rust changes. CodeRabbit status alone does not substitute for fmt, strict Clippy, native tests, rustdoc, release, owned coverage, security gates or qualifying independent review. The branch must remain Draft/non-public until those conditions are met.

Native continuation on an exact full checkout uses the repository-pinned toolchain and lock files:

```sh
cargo test --locked --workspace
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
./scripts/check_coverage.sh
npm ci --ignore-scripts --no-audit --no-fund
npm run check:json-contracts
```

A failure is a repair signal, not permission to weaken the gate or reduce the denominator.

## Continuation contract

| Next boundary | Prerequisite | Required evidence |
| --- | --- | --- |
| Native acceptance of current private Rust seams | Product-CI/central prerequisite repaired and branch ordinary-restacked | One unchanged exact head: Rust 1.98 fmt/Clippy/tests/rustdoc/release/owned coverage + hosted required workflows + independent review |
| Authenticated application request adapter | Immutable released Keyverse trust contract | Versioned application port/ACL; subject/tenant trust admitted without copying payload authority; stale/replay/mismatch fail closed |
| ConceptWeave proposal/base authorization | Authenticated request context | Exact proposal/base resource authorization receipt owned by ConceptWeave |
| Released semantic/tool artifact admission | Versioned canonical owner releases | Digest/authenticity + ACL/capability resolution; no mutable head/source copy |
| Independent evaluation | Admitted revision + retained proposal evidence | Evaluation evidence with explicit denominators and no self-issued approval |
| Steward decision and immutable publication | Independent evaluation + authorized steward | CAS/immutable release, provenance/SBOM/signing, supersession/revocation/rollback evidence |
| CGC/Noema consumption | Published semantic release + released projection contract | Consumer round trip, unsupported-case behavior, runtime activation separately authorized |

Current prerequisite order remains central backward-compatible CodeQL handler bootstrap -> ordinary/non-force #2051/#2056 current-main reconciliation and exact terminal GREEN -> unchanged-head #35 acceptance/normal merge -> Foundation restack and Product/CI-contract repair (including its generic text-parity finding) -> #43/#44 ordinary restack -> native #44 acceptance -> released Keyverse trust consumption -> ConceptWeave resource authorization -> released artifact admission -> evaluation -> steward decision -> immutable publication.

## References

Lu, Y., Chen, Y., Wu, S., & Arık, S. Ö. (2026). *Procedural graphs: Self-evolving execution structures for LLM agents* (arXiv:2609.09153v1). arXiv. https://arxiv.org/abs/2609.09153

Ecma International. (2026). *ECMAScript® 2026 language specification*. TC39. https://tc39.es/ecma262/

The Rust Project Developers. (2026). *Primitive type `str`*. Rust standard library documentation. https://doc.rust-lang.org/std/primitive.str.html

The Unicode Consortium. (2026). *Unicode Character Database*. https://www.unicode.org/ucd/

코난쌤. (2026, September 10). *Procedural Graph: LLM 에이전트를 위한 자가진화 절차 그래프 (arXiv 2609.09153) 논문 정리*. https://conanssam.com/posts/2026-09-10-procedural-graphs-self-evolving-llm-agents

The Lu et al. method statements are treated as research context; no unverified benchmark or replication result is promoted to product evidence. Standards are used for contract semantics only. Repository commits, PR reviews and exact-head checks remain the implementation authority.
