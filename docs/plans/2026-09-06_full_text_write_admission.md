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
