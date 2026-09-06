# Full-text write admission implementation plan

> **Execution:** Apply Superpowers test-driven-development and verification-before-completion in this existing dedicated ConceptWeave checkout. The user has authorized autonomous implementation; routine design confirmation is not a gate.

**Goal:** Make a fully reviewed paper classification eligible for separately authorized, explicit Zotero metadata changes without detaching the captured evidence during execution or recovery.

**Architecture:** Keep the existing intake owner and Local API executor. Separate the current local validation bodies from authority verification privately, compose both validations, check changed labels against the complete golden set, then verify the complete full-text review and the explicit write scope. Never use an allow-all callback to bridge the old boundaries. Typed inputs are deliberately not deserializable executable authority.

**Tech stack:** Existing Rust 1.98.0 workspace, serde and SHA-256 dependency; no new dependency, service, transport, credentials or live-write CLI.

## Baseline and scope

Parent PR #38 is OPEN Draft at `8e057652ee7784b373beeeec865d80dd3db773be`, base `692cb588b26a9cc878fbaa2b47aa30fd83ea47de`, as reread on 2026-09-06. Preserve both and stack the successor normally. Root is the sole writer for this increment; #6 and central workflow repair stay with their existing owners. The organization master-context and product-directive blobs remain `2ce09ee89ed9b8243684958e616d8ca934a3788e` and `c76c4226e4450f9b1714974fee0a62b60ea59bc9`.

Run from `/Users/seonghobae/Documents/ChatGPT/ConceptWeave`:

```sh
/Users/seonghobae/.cargo/bin/cargo +1.98.0 test --workspace --locked
```

Baseline expectation: existing 240 tests pass. New admission capability is absent. Actual paper decisions and independently verified approvals remain each 0/3,715. Test doubles are synthetic unit-only inputs, never research labels or approvals.

Extract the unfiltered result count without accidentally matching nested `90 filtered out` summaries:

```sh
awk '/^test result: ok\./ && /; 0 filtered out;/ {total += $4; suites += 1} END {print "passed=" total, "unfiltered_suites=" suites}' /tmp/conceptweave-fulltext-write-final-tests-20260906.log
```

Final verified result: `passed=252 unfiltered_suites=41`, including six doctests. Focused filtered results are not a substitute for this full count.

## Task 1 — Preserve a failing contract

Add `/Users/seonghobae/Documents/ChatGPT/ConceptWeave/crates/conceptweave-zotero/src/full_text_write_tests.rs`, reusing the adjacent private capture and completed-view fixture functions. Register it in `full_text_capture_tests.rs`.

Commit the experiment before running the focused test, as required by autoresearch. The first RED is the absent full-text write API, not a missing dependency. Preserve it in history; do not reset shared history.

```sh
/Users/seonghobae/.cargo/bin/cargo +1.98.0 test --locked -p conceptweave-zotero --lib full_text_write
```

Acceptance matrix: capture/report/proposal mismatch, incomplete or changed labels, invalid later write, destination/mode substitution, either authority denial, unchanged full envelopes delivered once, dry-run zero reads/writes, stale complete preflight, partial write, conditional rollback, retry and delayed rollback reconciliation. Every successful bound output must carry the same versioned scope digest. No public executable legacy-plan or freely mixed rollback-operation projection is allowed.

## Task 2 — Reuse validation and preserve authority

Modify private preparation in `/Users/seonghobae/Documents/ChatGPT/ConceptWeave/crates/conceptweave-zotero/src/lib.rs` and `src/full_text_review.rs`; public legacy APIs retain their contracts and error precedence. Implement `src/full_text_write.rs` beneath the existing full-text owner. Use a required typed full-text review, reviewed write set and explicit mode. Bind exact serialized inputs with a versioned SHA-256 domain. Full-text and write authority are different verifiers; local validation precedes both, semantic denial prevents write-authority verification.

Keep plans and recovery evidence opaque and serialize-only. Receipts retain the scope commitment, not source text or adapter errors. A serialized document is an audit artifact, not restoration or authority issuance. Independently unknown write state must not be reported as successfully restored by an empty rollback. Persisted admission, external issuer integration and live write evidence remain separate requirements.

## Task 3 — Verify and hand off

Run the focused test, full workspace, strict all-target Clippy, formatting, warnings-denied rustdoc, existing CI contract check and existing coverage gate. Do not change thresholds, coverage exclusions, locked versions or fixtures to obtain GREEN. Record exact commands, counts and raw versus normalized coverage separately in the Gap baseline and ignored `results.tsv`.

Update PRD/TRD/Proposed ADR 0007/architecture guidance/UML/CHANGELOG to state implemented behavior and remaining gates. Create a Draft successor against #38, reread its exact head/base/body, inspect current reviews/checks, and retain all protected gates. Update the existing hourly task with exact evidence and next work; do not treat local GREEN as approval or protected publication.

## Tooling and experiment evidence

CodeGraph was healthy and synchronized before exploration and after edits; its large-file queries trimmed the desired planner bodies, so their specific missing ranges were read directly. No code-review-graph MCP tool or executable is available in this environment; do not claim its indexing. DeepWiki returned repository-not-found for structure/content/question requests. Context7 returned its monthly quota limit; no alternate credentials or paid route were used. Existing serde APIs were checked against the official field-attribute and Serialize documentation, with APA entries in the full-text contract audit. The ADR skill's referenced identity instructions are absent, so the existing Proposed ADR was extended with a Y-Statement and no allocator, Accepted transition or new ADR number.

Admission RED `47d4e89` and recovery RED `79e1c22` preserve absent-API failures. Initial admission GREEN `d36dad8` passed 3 focused tests; recovery GREEN `425fb8c` passed 7. The first full coverage run found two missing branch outcomes (dry-run recovery refusal and indeterminate reconciliation retry refusal), despite passing workspace/static checks. Added cases in `bdf55bf` also cover already restored delayed observations, failed reads, known partial writes and compile-time opaque/persistence boundaries. Nine focused tests pass at this candidate; final unfiltered measurements are recorded in the Gap baseline after fresh full verification. No coverage threshold, exclusion or dependency changed.

## Task 4 — Observe an indeterminate original request without clearing it

Baseline `789637f6a54373c3176e42dda78b1075937fd273` passes 252 tests in 41 unfiltered suites on Rust 1.98.0. The existing receipt loses the actual failed request's library precondition after earlier successful operations. Its plan-level library version is not a substitute. Preserve that exact request inside the opaque receipt using the existing executor callback, without another executor or mutation path.

Add a read-only `observe_full_text_write` function in `crates/conceptweave-zotero/src/full_text_write.rs` and export it with its serialize-only observation type from `src/lib.rs`. The output borrows the complete original receipt and contains the later, explicitly unverified response or a missing observation. Read once only for an indeterminate attempt; refuse dry-run, failed preflight, successful and known-failure receipts before I/O. Preserve earlier inverses, untouched items, original failure and scope commitment. Never infer causal success, quiescence, peer authentication, retry permission or restoration authority from matching metadata. Keep original rollback refusal unchanged. No separate state classifier is needed for this audit-only API.

In `src/full_text_write_tests.rs`, commit RED tests before implementation. Test unknown first/second writes, actual advanced request preconditions, delayed before/after/foreign/malformed responses, failed reads, preserved receipt bytes and authority redaction, zero-I/O refusal of non-unknown receipts and continued rollback refusal. Synthetic unit fixtures are not campaign decisions. Run the focused command from Task 1, then all Task 3 gates unchanged. Keep failed experiments in history and append their evidence to ignored `results.tsv`.

Zotero's official write documentation says write tokens are redundant for versioned requests; local tokens are memory-only and disappear on restart. Do not add token replay or assume a delayed read proves the outcome of an earlier request. Durable restoration and governed resolution remain separate gaps. Record these primary references and this ceiling in ADR 0007, the audit and Gap baseline; update the existing PR and hourly continuation, not another writer or task.
