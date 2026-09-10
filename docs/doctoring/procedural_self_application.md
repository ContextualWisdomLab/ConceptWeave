# Procedural self-application and topology validation candidate

Status: Proposed / native Rust unverified / not operational. Tracking: ConceptWeave
#42, parent #43, ContextualWisdomLab/.github#2067. Observed parent at authoring:
`2c6d3037acdeae2b152c335ee2f60a59b4472831`; Foundation at observation remains
`60f14a6e85a83d56c2eea43b34d52b3366bb1735`. Live GitHub state supersedes these observations.
This extends, rather than replaces, ADR-PG-20260910 and the existing PRD/TRD.

## Additional ownership gap

The earlier implementation made ConceptWeave a procedural-model authoring producer.
It did not yet represent ConceptWeave's own semantic-engineering work as a first-party
consumer. These are separate uses of the method, not a reason to add another runtime:

- object-level: observe -> discover -> propose -> align -> validate -> steward review
  -> publication, with explicit revision paths;
- meta-level: retain baseline -> observe training outcomes -> contrast success/failure
  -> propose a procedure revision -> validate -> independent evaluation -> steward
  review -> publish successor, or retain the rejected proposal and revisit the hypothesis.

The meta-level process may propose changes to procedural knowledge. It cannot edit
its own permissions, judge/approve itself, change evaluation acceptance criteria,
relabel failed checks, or turn a Proposed ADR into operational authority. ConceptWeave
owns representation engineering; Noema remains the runtime/lifecycle consumer, CO the
model-call owner, CGC the released interchange owner and the evaluator an independent
source of measurement evidence. Product facts and policy authority stay with their owners.

This separation is a CWL architecture decision. Lu et al. describe procedural triplets,
local situational guidance and offline refinement with retained rejected proposals;
their method does not prescribe CWL repository ownership or its authorization design.
No new empirical performance result or paper replication is claimed here.

## Source and effect map

| Artifact / symbol | Role and actual integration state |
| --- | --- |
| `profiles/semantic_engineering/semantic_authoring.draft.json` | First-party authoring profile: 8 procedures, 9 relations, including correction paths |
| `profiles/semantic_engineering/procedural_refinement.draft.json` | Separate meta-level profile: 9 procedures, 11 relations, independent evaluation/review and rejection paths |
| `profiles/semantic_engineering/profile_manifest.json` | Exact source coordinates and profile SHA-256 values; inferred draft and activation false, not a signature or approval |
| `crates/conceptweave-domain/src/procedural_model_validation.rs` | Rust source candidate for topology/evidence membership; no network/provider/database dependency |
| `validate_procedural_model` | Borrowed structural projection + independently supplied scope + explicit reachability rule -> counts or a fixed error |
| `crates/conceptweave-domain/tests/procedural_model_validation.rs` | 20 authored native contract tests; the path import compiles the actual source module during Cargo integration tests |
| `scripts/tests/semantic_engineering_profiles.test.mjs` | Checked-in artifact assertions, not a production graph validator |
| `scripts/check_semantic_engineering_profiles.mjs` | Reuses parent's `createProceduralValidators` and locked AJV; no new package resolver |
| `package.json` | Only extends the existing `check:json-contracts` script; dependencies/version and lock file unchanged |

The Rust candidate is deliberately not exported by `src/lib.rs` yet. Its integration-test
path is an explicit pre-acceptance seam, not a second implementation. After native
verification, export the module through the library and switch the test import to that
public API, then exercise a strict JSON-to-domain adapter. Remove the path-import seam
in the same accepted integration. Do not describe this slice as a shipped library API.

## What the Rust candidate checks

The view has exact model/tenant/task/domain-owner scope; scope equality is not identity
or token verification. Logical procedure IDs must be unique even when record content
differs. Entry and both endpoints must be present. Relation identity is the complete
source/type/target triple. Evidence coordinates include source ID, SHA-256 syntax and
location; node/relation references must be members of the supplied root inventory.
Repeated coordinates and a source snapshot identity with conflicting digests fail.

Bounds: 1–256 procedures, 0–512 relations, 1–64 references per evidence set, local
128-byte ASCII identifier grammar and 2048 Unicode scalar values per evidence location.
A single 1 MiB budget covers the projection's identity/evidence text across the whole
validation, not a per-node reset. It does not cover omitted annotations or raw transport
bytes; those need pre-parse limits. Error messages contain fixed codes, not input text.

Reachability is selected explicitly: partial drafts may be disconnected; a complete
profile may require every node to be reachable from its entry. Cycles are allowed in
both modes and traversal is iterative. `requires` is an advisory label, not an executable
condition or an OWL restriction. The validator never executes or evaluates edge prose.

A passing result reports counts only. It does not construct a publishable aggregate,
authenticate source evidence, verify a release signature, compare revision-parent
artifacts, validate label meanings/tool contracts, attest evaluation quality or approve
an execution. These remain separate, mandatory owner boundaries.

## Profile evidence and applicability

Profiles are authored in Korean/English and remain `inferred`/`draft`, with no concrete
tool-operation contracts because no released executable mapping was established. They
are derived from exact internal PRD/Proposed ADR source files, not from live customer
trajectories. The source digests and locator text were verified against retrieved Git
blob identities; the manifest is still neither cryptographic source authentication nor
proof that a source statement is normatively correct.

The raw historical ADR source at `300fed966...` is pinned only as method/design evidence.
Its earlier AJV/CI notes are not current operational instructions. The parent #43 uses
locked in-process AJV; this change preserves that repair rather than reviving `npx`.

Noema currently has its own local graph/Start-node representation. These profiles are
not asserted to be compatible with it. A released projection contract must preserve or
explicitly map entry and procedure identities, relation direction, locale/evidence and
abstention states; it must not silently rename, truncate or activate a graph. No mutable
sibling PR head is an operational dependency.

## Measured evidence and non-evidence

The existing exact schema blob `311937137224cdd3036a9a614901c4a99ba62a6b` accepts
three malformed graph shapes: missing entry, dangling endpoint and duplicate logical
node ID. An independent diagnostic expecting semantic rejection failed **3/3**.
This reproduces the existing boundary gap; it is not a Rust execution result and that
diagnostic is intentionally not rewritten to pretend the schema gained semantic checks.

On the final authoring bytes, Node 22.16.0 executed **9/9 profile tests**, with no skips.
Independent Python jsonschema validated **2/2 profiles**, and all declared source
locations resolved in their pinned source files. Node syntax checks passed for the new
CI-only scripts. These results do not prove native Rust or production behavior.

The authoring environment has no rustc/cargo/rustup and cannot resolve compiler/package
hosts. No usable compiler artifact was obtained. The **20 Rust tests are authored but
unexecuted**; compile, fmt, Clippy, rustdoc, coverage, full native AJV and hosted checks
remain unverified. Do not replace the parent's locked validator, fabricate results or
merge on the basis of the narrower Node/Python checks. Python is used only for an
independent authoring check and is not added to the product.

## Todo / continuation contract

| Action | Prerequisite | Completion evidence | State |
| --- | --- | --- | --- |
| Refresh owner/source/PR/review structure | Live repository access | Parent SHA and inherited repair inspection | Done at dated observation |
| Reproduce schema-only gap | Exact schema bytes | 3 failing semantic-expectation diagnostics | Done; gap remains until native integration |
| Author Rust validator and tests | Reproduced boundary cases | Candidate source + 20 tests | Source candidate; native verification blocked |
| Add first-party authoring/refinement profiles | Exact source bindings | 2 profiles, 9 Node tests, source/digest checks | Local artifact checks passed |
| Finish native/public API/strict transport integration | Compiler and parent prerequisite | Exact-head native tests/fmt/Clippy/docs/coverage + input-to-domain tests | Pending; path seam must be removed |
| Released CGC/Noema projection | Semantic admission + publication contract | Cross-language/consumer round trip; explicit unsupported cases | Pending |
| Controlled shadow evaluation | Real isolated integrations and approved task sample | Matched no-graph/fixed/evolved evidence with all failure denominators | Pending; no performance baseline yet |
| Authenticated evolution, publication and product rollout | Independent evaluation/review | Persistent rejection history, CAS, revocation, rollback and product controls | Pending |

Native continuation commands on an exact full checkout, using the repository-pinned
Rust toolchain and lock files:

```sh
cargo test --locked --workspace
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
./scripts/check_coverage.sh
npm ci --ignore-scripts --no-audit --no-fund
npm run check:json-contracts
```

Check/test failure is not a reason to loosen those commands. Read current parent/review
state, repair on the responsible path, and use ordinary non-force integration. The
source module's public export, actual release, deployment and canary each need separate
evidence. There is no background execution or new recurring schedule in this slice.

## References

Lu, Y., Chen, Y., Wu, S., & Arık, S. Ö. (2026). *Procedural graphs: Self-evolving execution
structures for LLM agents* (arXiv:2609.09153v1). arXiv. https://arxiv.org/abs/2609.09153

코난쌤. (2026, September 10). *Procedural Graph: LLM 에이전트를 위한 자가진화 절차 그래프
(arXiv 2609.09153) 논문 정리*.
https://conanssam.com/posts/2026-09-10-procedural-graphs-self-evolving-llm-agents

Method statements were checked against the primary abstract and retrieved section
excerpts; no full appendix or reference implementation audit is claimed in this slice.
