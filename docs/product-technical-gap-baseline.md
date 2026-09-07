# Product / Technical Gap Baseline

**Snapshot:** 2026-09-05

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot. Refresh the changed PR's metadata after each documentation commit; evidence from another PR or an earlier head does not transfer.

## September 6 source inventory checkpoint

### September 7 PR37 source continuity verification

Follow-up normal merge `624730d2315b99519dfbe979a9ecb568e7f99045` retains PR37 `9d72bbf` and verified PR36 `c062845`. Only adjacent Gap entries conflicted; both were retained. The public review path still delegates to the fixed-limit capture verifier, confirmed by independent read-only review. Stable tests pass 270/39 unfiltered suites; strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks pass. Unchanged nightly coverage passes 267 tests/37 suites, 393/393 functions, 4,060/4,060 normalized regions and 676/676 normalized branches. Raw LLVM remains 4,909/4,993 lines, 7,349/7,511 regions and 619/676 branches. Logs `/tmp/conceptweave-pr37-verifier-{tests,clippy,rustdoc,coverage}.log` ended successfully in session `14953`. The real 16 MiB JSON expansion test completed without reducing its fixture or limit. A diagnostic sample raced its normal process exit and produced no profile; no runtime performance cause is claimed. PR38 and PR39 still require forward integration and exact-tree validation. Local success is not hosted approval, a protected merge, a release or actual paper classification.

Normal merge `c4d40f7` retains PR37's pending full-text view and PR36's source inventory and private-file protections. Tests `cd830ff` add nested proposal/pending-source identity and stale capture rejection against a fresh worksheet. Independent review found a test-only `String`/`Option` mismatch; the failed compilation is retained in `/tmp/conceptweave-pr37-verified.log`, and `21fd648` corrects it without claiming a production RED. Proposed ADR0006 `d53475f` records the reuse decision and downstream envelope requirement.

The prerequisite subsequently advanced to `3ab00417c229aeae59709f8980c79d5339687893` through manifest-version experiments and reversion. Its tree is exactly the same as `aca72e7` (`01067eb790e9e9f55395a0ddd6fb930dae685bcb`); normal merge `7569a1a` preserves that history. Baseline passed 216 tests/39 unfiltered suites; integration and corrected source `21fd648` each passed 268/39 including three doctests. Filtered subprocess summaries are not counted twice. Strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks pass. Unchanged pinned coverage passes 386/386 functions, 3,960/3,960 normalized regions and 674/674 normalized branches. Raw LLVM remains 4,794/4,878 lines, 7,249/7,411 regions and 617/674 branches, not 100%. Logs `/tmp/conceptweave-pr37-{baseline,integrated,final,clippy,rustdoc,coverage}.log` are terminal success; `verified` is the earlier failed test compilation. Subsequent commits change documentation only. No hosted verification is claimed.

Actual decisions and independent approvals remain 0/3,715 with four unresolved sources. Native Visual Inspection was initially blocked by the Mac lock; the September 7 continuation successfully retrieved both the Zotero accessibility state and an actual window screenshot. The library view displays 3,719 items, collection/tag panes, attachment indicators and an unselected detail pane. Long collection names, titles and creators are visibly ellipsized in the current window width; this screenshot alone does not establish full-title readability, paper count or full-text review. The screenshot remains in the private task, not a public repository artifact. No click, edit, classification, approval or Zotero write was performed during inspection. Root and later consumers still require adoption. Keep PR37 OPEN Draft; normal push and exact-head readback remain separate from independent hosted checks, protected merge and release.

### September 7 persisted-capture verifier follow-up

Supplier `e517fc0a2ef268a5f80ff8e8f90ac24c36a170f3` already enforces a fixed 512 MiB compact-JSON ceiling; its focused serialization test passes. Downstream PR38 integration `21c3b88` exposed one uncovered verifier error-propagation region, not a demonstrated acceptance defect: 4,463/4,464 normalized regions and 710/710 normalized branches passed. The repair stays in PR36, the earliest owner. Test commit `9fe952e` introduced a private test seam and failed compilation with E0425 before implementation; this is not claimed as a new production behavioral RED. Production `308dc31c9804894c06f5e88bf40ab00a4b14929b` extracts the unchanged verifier body behind a private limit parameter. The public ceiling and report-before-size validation order remain unchanged. Its focused test passes at the exact serialized byte count and rejects a one-byte-smaller allowance. Independent read-only review found no semantic or validation-order regression. Full workspace, strict checks and unchanged coverage are pending in `/tmp/conceptweave-pr36-verifier-{tests,clippy,rustdoc,coverage}.log`; no push or hosted result is claimed for this follow-up.

Earlier lock-screen statements below describe historical attempts, not a current blocker. The later September 7 PR37 inspection obtained both a native Zotero screenshot and accessibility state: 3,719 displayed items and visibly ellipsized long names, titles and creators. That display count is not the bibliographic denominator. The image remains private, and no click, edit, paper decision, approval or Zotero write accompanied inspection. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources.

The `308dc31` follow-up subsequently passed 255 tests in 38 unfiltered suites, including three doctests; a nested filtered subprocess result is excluded rather than counted twice. Strict all-target Clippy, warnings-denied rustdoc, formatting, the CI contract and diff checks also pass. Session `48628` is still executing unchanged pinned coverage; the complete verification chain and push remain pending. Commits through `c2c1558` after production repair only update documentation.

The follow-up verification chain `48628` subsequently completed successfully. Unchanged `nightly-2026-08-20` coverage executed 252 tests/36 unfiltered suites and reports 381/381 functions, 3,788/3,788 normalized regions and 658/658 normalized branches. Raw LLVM reports 4,746/4,818 lines, 7,164/7,285 regions and 610/658 branches, not 100%. Fresh fetch still places the supplier at `e517fc0a2ef268a5f80ff8e8f90ac24c36a170f3`, already an ancestor of this repair; no concurrent delta is discarded. Normal push updates the existing PR36, not protected main. PR37, PR38 and PR39 must inherit the repair in order and verify their combined trees before adoption claims.

### September 7 PR36 full-text source continuity verification

Original PR36 `d3f991dcc1b746afed7c36f315e8937c39390c5e` passed 202 tests/39 suites. Normal merge `707f2f2` preserves its private full-text capture/proxy-isolation delta and PR34 `9eb89b8d4e751c34e640261f4883381571c83f25`. Existing capture admission delegates to worksheet construction, so shared inventory and parent consistency checks are inherited before any request. Existing whole-report hashing includes retained metadata and pending keys. No duplicate validator or hash was added. Initial integration failed compilation because two parent test callbacks required mutable borrows under PR36's writer signature; `c7052d9` repairs those calls and adds source-scope regressions. This is not a new production RED claim.

Tests reject omitted retained records and inconsistent parent metadata before fetching; a mixed library capture retains a standalone attachment and unchanged report bytes/pending keys; changed retained metadata invalidates an old restored capture despite unchanged raw snapshot identity. Independent bounded source review found no additional defect and documented the existing at-least-one-bibliographic-item restriction. Final source passes 254 tests/39 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. Unchanged coverage passes 374/374 reported functions, 3,688/3,688 normalized regions and 656/656 normalized branches. Raw 4,631/4,703 lines, 7,064/7,185 regions and 608/656 branches are not 100%. Logs `/tmp/conceptweave-pr36-{baseline,integrated,verified,clippy,rustdoc,coverage}.log`; `integrated` is the failed compile run. Proposed ADR0006 `f26ce5b` records alternatives and downstream obligations; TRD/UML merge preserves both source traversal and separate capture. Failed streamed output now inherits no pathname cleanup or implicit buffer retry.

Root and later consumers still require cascade adoption. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. Capture tests are synthetic preparation evidence, not reviewed meaning, authority, peer authentication or an atomic snapshot. Native Visual Inspection was retried but the Mac remains locked; manual unlock is required and no fresh screenshot exists. Keep OPEN Draft; no real Zotero write, hosted GREEN, protected merge or release is claimed.

### September 7 PR34 completed-view continuity verification

Original PR34 `b9060df2cb1ea02314be429932031fc07de1de30` passed 171 tests/37 suites. Normal merge `bd8f995` preserves that delta and PR33 `93faf6ab750a99469196cf71567498be83c22a6b`: whole-view comparison, strict JSON boundaries, research-source audit, pending-source scope and inherited private-file protections all remain. The required patch identity is projected only after whole-view equality. Fixture correction `d1e8bb7` yields 221 integrated tests/37 suites. Tests `e889ac8`, `5a23660` and `cb8f78c` add tamper/missing-field/stale-view coverage; intermediate compilation and fixture-assumption failures are recorded, not claimed as production RED. The final stale-view case holds raw snapshot identity unchanged while changing retained context and regenerating the worksheet. Independent bounded review found no additional defect; it is not GitHub approval.

Final source `cb8f78c` passes 221 tests/37 suites including three doctests, strict all-target Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. The unchanged coverage script passes 342/342 reported functions, 3,192/3,192 normalized regions and 580/580 normalized branches. Raw 4,308/4,370 lines, 6,555/6,662 regions and 535/580 branches are not 100%. Logs: `/tmp/conceptweave-pr34-{baseline,integrated,integration-fixed,verified,final,final-verified,clippy-final,rustdoc-final,coverage}.log`. `integrated` and `verified` are failed compilation runs; `final` is the failed fixture-assumption run; `final-verified` is terminal GREEN. Documentation `28a2e28` corrects the output-failure claim: admission rejection creates no output, but write failure may retain a private partial file.

PRD/TRD/Proposed ADR0006 preserve complete-view validation without a second hash or approval mechanism. Root and later consumers have not adopted this cascade. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources; synthetic fixture decisions are not paper review. Native Visual Inspection was retried, but the Mac is locked and requires manual unlock; no fresh screen evidence exists. Keep OPEN Draft. No real Zotero write, hosted GREEN, independent protected approval, protected merge or release is claimed.

### September 7 PR33 batch scope repair

Original PR33 `d82b2f2896bc4cfef5d34ac6f6f83f7cee1072f6` passed 166 tests/36 suites. Normal merge `2f3962e` retains it and PR32 `ea25437c54b4e452f008f6b1ccad92d5516dd7b9`, preserving original abstract minimization guards with the shared validator. The existing batch-to-patch test failed for missing proposal identity. Explicit RED `7373579` compiled with one passing and two failing tests for patch compatibility and pending-source projection. `ce15389` passes existing validated progress identity and pending count into the batch, without new hashing or authority. No blank slots means exhausted paper decisions, not resolved source scope. Independent bounded review found no added defect.

Full verification then exposed an inherited stale-review fixture assigning review-only context to a non-abstained proposal; the retained privacy gate correctly returned InvalidReview earlier. Test-only `acbaf1d` mutates the actual abstained item instead, retaining structurally valid changed-content rejection as SnapshotMismatch. Final source passes 216 tests/36 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. Unchanged coverage passes 340/340 reported functions, 3,120/3,120 normalized regions and 568/568 normalized branches. Raw 4,204/4,266 lines, 6,425/6,532 regions and 523/568 branches are not 100%. Logs: `/tmp/conceptweave-pr33-{baseline,integrated,binding-red,verified,final,clippy-final,rustdoc-final,coverage-final}.log`; `verified` is the failed intermediate run, not final GREEN.

PRD/TRD/Proposed ADR0006 distinguish sensitive batch views, exact metadata identity, pending counts and no reservation/approval. PR34 owns completed-view context validation; plain patch compatibility does not prove displayed content integrity or full-text review. Root/later consumers remain unadopted. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. Native Visual Inspection was retried but the Mac is locked; no new screenshot, Zotero write, hosted GREEN, protected merge or release is claimed. Keep Draft.

### September 7 PR32 decision CLI continuity verification

Original PR32 `1e711728d83efc1e60fc3d43ba0c67c467dd6a43` passed 161 tests/34 suites. Normal merge `b7cae29` retains it and PR31 `eaf248afd33dcac477daf1e3a31d79d47c5cbb69`; two synthetic patch constructors adopt required proposal identity, and integrated tests pass 210/34. The thin apply arm remains unchanged: private distinct inputs go to the canonical validator without binding backfill; output is serialized before create-new writing. No additional production defect or new RED is claimed.

Test `d3a2c7a` extends actual binary execution: valid-patch nonoverwrite, missing/blank binding no-output rejection, stale patch against regenerated current worksheet rejection, and unchanged patch/worksheet bytes. Existing valid `0600` output and equal replay remain. Independent bounded review found no additional defect. Final source passes 210 tests/34 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. Unchanged coverage passes 332/332 reported functions, 2,997/2,997 normalized regions and 548/548 normalized branches. Raw 4,060/4,122 lines, 6,228/6,335 regions and 503/548 branches are not 100%. Logs: `/tmp/conceptweave-pr32-{baseline,integrated,verified,clippy,rustdoc,coverage}.log`.

README/TRD/Proposed ADR0006 distinguish current-content binding, no legacy backfill, pre-write rejection and possible retained partial files after write failure. Root/later consumers still require cascade adoption. Authentic decisions and independent approvals remain 0/3,715 plus four unresolved sources; synthetic unit data is not real research. Native Visual Inspection was retried, but the Mac is locked and no fresh screenshot exists. Keep Draft; no Zotero mutation, hosted GREEN, protected merge or release occurred.

### September 7 PR31 decision-patch content repair

Original PR31 `61072a70f7ec5a1fbd0b477430aacb8e770aa109` passed 158 tests/33 suites. Normal merge `ef0ce43` retains it and PR30 `ee1ac9925c5287e9f10c2e9581b7cf513b170bd7`; integrated tests passed 204/33. RED `7208d80` compiled with three passing and two failing tests: an old patch applied after changed report context and fresh worksheet generation, and serialized patches without content binding loaded. The valid-first/unknown-later batch already preserved the original worksheet. `89bb941` adds required patch proposal identity and compares it before updates, reusing the existing digest. Missing fields fail loading, stale/blank identities cannot equal the recomputed expected identity, and no automatic backfill is permitted. Independent bounded review found no additional defect; atomic failure, same-label idempotency and conflicting-decision rejection remain.

Final source `89bb941` passes 207 tests/33 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. Unchanged coverage passes 331/331 reported functions, 2,965/2,965 normalized regions and 544/544 normalized branches. Raw 3,971/4,033 lines, 6,125/6,232 regions and 499/544 branches are not 100%. Logs: `/tmp/conceptweave-pr31-{baseline,integrated,binding-red,verified,clippy,rustdoc,coverage}.log`. PRD/TRD/Proposed ADR0006 trace the separate report/worksheet/patch identities and legacy regeneration requirement.

Root and later patch consumers must inherit this required binding alongside the prior source, progress and FIFO repairs. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. Synthetic fixture labels are not real review evidence. Native Visual Inspection was retried, but the Mac is locked; no fresh screenshot is claimed. Keep Draft. No real Zotero write, hosted GREEN, protected merge or release occurred.

### September 7 PR30 content-bound progress repair

Original PR30 `a11e889d1680ab4d91f3565e3debf7ed0f10ba23` passed 156 tests/32 suites. Normal merge `7251238` retains it and PR29 `e21f14fbb4954762ffc97521af9a6cdd9982c630`; integrated tests passed 200/32. Child private-helper coverage improvements remain, combined with parent FIFO/source/approval/output safety. RED `4099434` compiled with one passing and two failing progress tests: blank proposal binding was accepted and pending/content identity fields were absent. `bee32f4` compares the existing digest in the shared worksheet validator and adds opaque proposal identity plus pending count to aggregate progress. `b2b0ef4` proves filled bibliographic slots do not hide an unresolved standalone source. Independent bounded review found no collateral finalization error-precedence or privacy regression.

Final source `b2b0ef4` passes 202 tests/32 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. The unchanged coverage gate passes 328/328 reported functions, 2,905/2,905 normalized regions and 518/518 normalized branches. Raw 3,923/3,985 lines, 6,065/6,172 regions and 473/518 branches are not 100%. Logs: `/tmp/conceptweave-pr30-{baseline,integrated,progress-red,verified,clippy,rustdoc,coverage}.log`. PRD/TRD/Proposed ADR0006 explicitly distinguish bibliographic slot counts, pending source scope, local preparation, independent approval and applied reclassification. No new digest, dependency or authority issuer was added.

Root and later consumers must inherit content binding, pending completion semantics and prior FIFO protection. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. Synthetic unit fixtures do not count as research review. Native Visual Inspection was retried, but the Mac is locked and no fresh screenshot exists. Keep Draft; no real Zotero write, hosted GREEN, protected merge or release is claimed.

### September 7 PR29 offline finalization continuity repair

Original PR29 `f73705e15f1236fa8bd34fec032bc78d9b57760c` passed 149 tests/28 suites. Normal merge `4845200` retains it and repaired PR28 `63eb0f116408372675f132b9836fe7be4bdd7134`; integrated tests passed 190/28. Offline finalization remains local unverified metadata output. The request enum retains parent canonical-pair admission before capture, both serializations before writes and no pathname cleanup on failure. Complete source/worksheet/approval binding comes from the inherited shared validation, not a new authority issuer.

RED `7c2c0d1` compiled with two failures: direct symlink opening succeeded; nameless-input validation took the old error path. The second is not an unauthorized-read claim. Canonical fixes `b06f54a` and `7ccbbbe` were reused with attribution as `f97ef46` and `630d258`, pinning the admitted parent and refusing symlink opens. A separate review found no existing FIFO repair. RED `bce7efa` replaced a checked unit-test regular file with an actual FIFO and timed out after 2.01 seconds. `ddf3f62` adds `O_NONBLOCK` at the existing opening boundary; existing device/inode validation then rejects the replacement without waiting for a writer. Independent bounded read-only review found no additional defect. No new abstraction or unpinned dependency was introduced.

Final source `ddf3f62` passes 193 tests/28 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. Unchanged coverage passes 318/318 reported functions, 2,778/2,778 normalized regions and 494/494 normalized branches. Raw 3,798/3,860 lines, 5,873/5,980 regions and 449/494 branches are not 100%. Logs: `/tmp/conceptweave-pr29-{baseline,integrated,opening-red,fifo-red,verified,clippy-verified,rustdoc-verified,coverage-verified}.log`. TRD and Proposed ADR0006 trace alternatives, retained failures and consumer obligations.

Root/later private readers must inherit FIFO protection and the source-scope cascade. Authentic decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. Unit fixtures do not count as real research or approval. Native Visual Inspection was retried and the Mac remains locked, so no fresh screen evidence exists. Keep Draft; no hosted GREEN, protected merge, release or Zotero mutation is claimed.

### September 7 PR28 metadata roundtrip continuity repair

Normal merge `13b5529` preserves original PR28 `ba6b3dfc71cf89ed4c57b85da0dd9ca5f983efee` and PR27 `fd5ef23c1c23fa36ef106400e15c9438eaa5cd41`. RED `97fc490` compiled with four passing and two failing roundtrip tests: inconsistent retained parent coordinates passed shared admission, while valid pending orphan metadata failed worksheet restoration. `1157b1d` centralizes parent validation and removes divergent worksheet checks; `e102894` removes an implied evaluator guard and preserves coherent-mutation stale-approval rejection. Independent read-only review found no further production defect in the bounded repair.

Source `e102894` passes 179 tests/26 suites including three doctests, strict all-target Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. The unchanged coverage gate passes 299/299 functions, 2,642/2,642 normalized regions and 470/470 normalized branches. Raw coverage remains 3,449/3,511 lines, 5,245/5,352 regions and 425/470 branches, not 100%. Logs: `/tmp/conceptweave-pr28-{parent-red,final-tests,clippy,rustdoc,coverage}.log`. Documentation amendment `29cc97a` distinguishes the restored metadata projection from raw source/full-text capture. Unknown provider fields stay excluded; no new capture mechanism or dependency was added.

Root and later consumers have not yet inherited these repairs. Authentic decisions and independent approvals remain 0/3,715, with four unresolved standalone sources. Synthetic fixtures are not paper review evidence. Native Zotero Visual Inspection was retried but the Mac is locked; no new screen evidence exists. No real Zotero write, hosted GREEN, protected approval/merge or release is claimed. Preserve Draft and continue normal successor integration with fresh exact-head evidence.

### September 7 PR27 finalization continuity repair

Final source `da2556b` passes 172 tests/25 suites including three doctests, strict all-target Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. The unchanged pinned coverage gate passes 295/295 functions, 2,611/2,611 normalized regions and 468/468 normalized branches. Raw coverage remains 3,436/3,498 lines, 5,214/5,321 regions and 423/468 branches, not 100%. Logs: `/tmp/conceptweave-pr27-verified.log` and `/tmp/conceptweave-pr27-{clippy,rustdoc,coverage}-verified.log`. Both baseline and integration finished before their successors were edited; the final documentation commit receives its own full verification.

Original PR27 `3df0c124f390797bacaba8ffdf229f502b0e9bf3` passed 133 tests/25 suites. Ordinary merge `5fa34b4` retains it and PR26 `7ad6386de13e19bb57fc4519141ac67c7b8bf92b`; integrated tests passed 168/25. RED `f02631e` compiled with 5 passing and 2 failing tests: blank/replaced worksheet bindings and stale worksheet decisions with a current report/receipt were admitted. `d44b9fe` adds two existing-boundary checks: blank worksheet binding is `InvalidReview`; mismatch with the recomputed expected worksheet is `SnapshotMismatch`. Prior cardinality and approval error precedence is preserved.

`e90a02b` verifies the distinction between local conversion and independent authority: locally rewriting both digests can produce self-consistent input, but the original independently verified whole-set receipt still rejects it. Valid pending-source conversion is retained for preparation, while complete evaluation refuses it before governance. Tests cover changed title, evidence and review context. Independent read-only review found no additional production finding; `da2556b` documents this boundary in the public API, PRD, TRD and Proposed ADR0006. Final exact-source verification is recorded below.

This does not complete the library campaign: actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. No real metadata, labels or approval was generated by these synthetic tests. Root and later extracted worksheet validators still need the repair; next PR28 report roundtrip must retain exact binding and source scope. Visual Inspection was attempted but the Mac remains locked, so no new screen evidence is claimed. Local verification is not hosted GREEN, independent protected approval, protected merge, release, full-text authority or Zotero mutation.

### September 7 PR26 private export repair

Final source `58ff5985890d5a0b4aaadaa1f8d604e1bc96a1e2` passes 163 tests/24 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. The unchanged pinned coverage gate passes 294/294 functions, 2,536/2,536 normalized regions and 422/422 normalized branches. Raw coverage remains 3,382/3,444 lines, 5,139/5,246 regions and 377/422 branches, not 100%. Logs: `/tmp/conceptweave-pr26-verified.log` and `/tmp/conceptweave-pr26-{clippy,rustdoc,coverage}-verified.log`. The earlier coverage failure is retained in `/tmp/conceptweave-pr26-coverage.log`; it is not a successful checkpoint.

Original PR26 `e2dc6006ed3e56d8388e82912826cf37efed0541` passed 128 tests/24 suites. Ordinary merge `227b3e9` retains it and PR25 `51631fbf711b403a40f3b9fafa2ec3958d54ceaf`; integrated tests passed 160/24. RED `68575a6` compiled and failed both regression tests: a replacement file was deleted after write failure, and buffered bytes were flushed while dropping a failed writer. `ab391b2` preserves the explicit write result, disassembles the buffer without retry, and removes pathname cleanup from both the shared writer and second-artifact failure. Existing private creation, successful write and overwrite-refusal checks remain.

RED `fb5b5e5` compiled and failed canonical-alias admission. `e928858` rejects equal canonical destinations before the Local API read; both artifacts are still serialized from one captured report before either is written. Independent read-only review found no further production defect. The first coverage run exposed two untested error propagation points in the extracted admission function; `58ff598` adds both invalid-path cases without changing runtime or coverage exclusions. Final verification is recorded separately below.

TRD and Proposed ADR 0006 describe retained partial files and sequential, nontransactional output. A report surviving a failed worksheet write is not a completed pair, approval or durable publication. Root and later CLI owners still require this inherited fix; PR27 finalization must also compare the newly required worksheet proposal binding before governance. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. No real source metadata or authority was created for these synthetic tests. Native Visual Inspection was attempted but the Mac is locked, so no new screen evidence exists. No hosted GREEN, protected approval/merge or release is claimed.

### PR25 worksheet admission and identity repair

Original PR25 `c6b4c17e931951a2e1d4ea79ac79363f6306a5bf` passed 126 tests/24 suites. Normal merge `4a1a3bb` retains it and PR24 `35c57ca4510a65cf48069285d78b95cf47db65ba`; integrated tests passed 153/24. RED `900038e` compiled with 4 passing and 2 failing tests: omitted retained inventory and hidden pending keys still produced worksheets. `5b54d06` reuses shared report validation and removes 54 lines of divergent audit/coordinate checks. Valid standalone, orphan, cyclic and attached source metadata remains reviewable with blank decisions. Completion admission remains distinct.

RED `30aa091` compiled with 6 passing and 2 failing tests: changed retained metadata produced an equal worksheet and missing proposal binding deserialized successfully. Final source `97046c7` adds a required worksheet `proposal_digest` from the existing v2 hash; source/context changes produce different identities and missing binding fails loading. No new hash, library or authority issuer was introduced. Independent read-only review found no additional production regression. Required downstream repair: compare worksheet/report binding before progress, patch and finalization, including blank or locally rewritten digest rejection; never backfill older worksheets or infer full-text authority.

Final local tests are 158/24 suites including three doctests, strict Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks pass. Unchanged pinned coverage passes 282/282 functions, 2,468/2,468 normalized regions and 414/414 normalized branches. Raw coverage remains 3,232/3,293 lines, 4,858/4,960 regions and 369/414 branches, not 100%. Logs: `/tmp/conceptweave-pr25-{baseline,integration,source-red,binding-red,final,clippy-final,rustdoc-final,coverage-final}.log`.

The root runtime and later consumers have not adopted this repair. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources. No real Zotero record, classification label or approval was created. Native Visual Inspection was attempted again but the Mac is locked, so there is no new screenshot evidence. PRD/TRD and Proposed ADR 0006 record scope, compatibility and remaining adoption. Local verification is not hosted current-head GREEN, protected approval/merge or release.

### PR24 pending-source completion repair

Final source `5b2282a` passes strict all-target Clippy, warnings-denied rustdoc, formatting, CI contract and diff checks. The unchanged pinned coverage gate passes 279/279 functions, 2,420/2,420 normalized regions and 408/408 normalized branches. Raw coverage remains 3,198/3,259 lines, 4,810/4,912 regions and 363/408 branches, not 100%. Logs: `/tmp/conceptweave-pr24-{clippy,rustdoc,coverage}-verified.log`. No predecessor or later-head coverage is attributed to this source.

PR24 originally `1e73e1545de32ae9a349c469a7794c5c3fc2ae9b` passed 123 tests/23 suites. Normal merge `b4c16a4` retains that head and PR23 `2d32f96740c708c3f7c13b392386f8bc7a878746`; integrated tests passed 149/23. The existing complete-review evaluator accepted one reviewed bibliographic item even when a standalone attachment remained unresolved. This contradicted the source-scope requirement: a complete bibliography is insufficient when source records remain unaccounted for.

Behavioral RED `8c0ef44` compiled and failed with an unexpected successful evaluation (`/tmp/conceptweave-pr24-pending-red.log`). Repair `ac59477` adds the pending-source condition to the existing completion boundary, without changing sampled evaluation or creating another authority mechanism. `b868f39` adds self-cycle and forged-empty-pending coverage; its initial test failed to compile because the report is not Clone. `5b2282a` moves the test-owned report instead of expanding the production API. The final workspace result is 150 tests/23 suites including three doctests (`/tmp/conceptweave-pr24-verified.log`). Standalone, orphan and cyclic sources reject completion before governance; attached sources remain eligible. Clearing pending keys and rewriting the digest still fails shared inventory validation. Independent read-only review found no additional production regression; its wording finding is reflected in PRD, TRD and the error message.

This is complete metadata-review coverage, not full-text approval, a successful Zotero write, hosted GREEN, independent protected approval, merge or release. The root runtime has not yet adopted the repair. Actual decisions and independent approvals remain 0/3,715 plus four unresolved standalone sources; no real data, labels or authority were created for these synthetic unit tests. Visual Inspection was retried, but the Mac is locked; no new screen evidence is claimed. Preserve Proposed ADR 0006 and the existing owner stack. Next propagate into PR25 `c6b4c17e931951a2e1d4ea79ac79363f6306a5bf` and the later full-text/runtime consumers, then validate protected and live evidence separately.

The existing #9 owner now retains every nonbibliographic metadata record and derives unresolved ancestry rather than silently discarding standalone sources. [Source-scope doctoring](doctoring/zotero_source_scope.md) binds committed REDs, final source `1e95d6eb979e66ecb7dae4f81f18a6b0a91b7624`, **47 tests / 10 unfiltered suites**, strict checks and the unchanged coverage gate. The earlier inventory executable at `48c3525` genuinely reads 8,326 records into 3,715 unchanged bibliographic proposals plus 4,611 other records, with exactly the four previously audited standalone identities pending. A later shared-reader guard also rejects blank identities; no actual final-guard executable replay is implied.

The earlier source findings are repaired locally, not yet propagated into root #39. Required downstream restoration/identity accounting, pending-source reconciliation, approval binding and full-library completion gates remain open; neither zero pending keys nor successful classification grants semantic or write authority. Current native Visual Inspection was attempted but the Mac is locked, so no new screenshot was verified. Historical source scope, authentic worksheet decisions/independent approvals 0/3,715, plus four unresolved sources remain distinct. This checkpoint does not refresh every historical PR coordinate below or imply protected merge/release.

## Historical protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

The active roots observed immediately before this baseline refresh are:

1. Foundation PR #1 — exact head `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open. The current Foundation makes Product CI draft-aware while keeping Ready/non-Draft quality requirements intact. Product CI still cannot materialize from protected `main` because that branch does not yet contain `.github/workflows/product.yml`.
2. Product-CI bootstrap PR #35 — exact head `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft. It adds the pull-request form of Product CI and removes no-op closed/converted-to-Draft triggers. At 10:46–10:48 UTC, Semgrep, Noema execution, admission, Trivy and Scorecard had completed successfully, coverage was queued and Strix running. CodeQL's new red dispatch job handed off to a queued successor; this is not a completed scan verdict. Noema submitted CHANGES_REQUESTED for an unsupported-flag claim contradicted by pinned Cargo help and official documentation, as detailed below. The workflow-only diff skips Dependency Review and OSV; these skips do not prove Foundation's dependency-changing checks. No independent approval exists for this head.
3. Client Consumption PR #5 — exact head `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open. The current source retains language-neutral semantic-release admission, integrity, compatibility, diff/resolution and supersession validation. Previously valid review findings are source-repaired, but current protected evidence remains independently required.
4. Source Observation PR #6 — exact head `c362a73403b6bda2cc0e94de913e39f3139d6205`, Draft/open. Its independent owner preserves registry denial before an authorized-request-only adapter boundary. The counted regression now verifies zero adapter/source/snapshot executions on denial and one of each for an authorized control, retaining the existing denial result. This audit checked source and formatting, not that branch's runtime or coverage. The new submitted repair report is COMMENTED, not approval; current-head Actions/check-runs were absent. The concrete bounded read-only PostgreSQL adapter remains absent.
5. Zotero Research Classification root PR #9 — exact head `a2a84884f67dcac6f6892c958d55450aea6d6c88`, Draft/open. A minimal owner backport reproduces and repairs proxy inheritance and exact-byte-limit rejection without bringing later full-text features backward. Integrity root #10 was `e7d4e59f1b55b5954c5f8436527bc96e7ef2fb13`; all 23 descendants inherited that earlier repair through ordinary merges. The transport cascade now reaches review-batch PR #34 at `b0119a57047e7b1fe5ddfbbf4b973de0f15de172`, preserving its original `2e6448e896e65562ebeee2fd339dec64d9fdf6e5` and every intermediate delta. [Full-text capture PR #36](https://github.com/ContextualWisdomLab/ConceptWeave/pull/36) integrates that parent at locally verified merge `75da75cf01704d9aae47f1e5573e3bbe3fb42bb0`; the 10:46–10:48 UTC audit confirmed its remote documentation head `1e7d23c91116d84a455b6e6e5a6fb00a5e004c04` and actual named base `b0119a5`. It remains Draft, with an explicitly skipped CodeRabbit review and no hosted Product verification or approval. The new private-review-view work below is a child delta, not a replacement or closure of this prerequisite. The stack remains proposal/review oriented and does not elevate local classifier output to semantic authority.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and the public Draft 2020-12 semantic-candidate schema enforce compatible publication-state/truth-status semantics. Hosted exact-head Product evidence still requires the bootstrap workflow on protected `main`. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, targeted delete-column provenance, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and registry-authorized opaque source identity exist. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Current exact-head protected evidence and prerequisite integration remain outstanding. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | CONSUMER_REVALIDATION_PENDING | The earlier public non-fork exact-range HTTP 403 was traced to an uninitialized repository dependency graph, not to a retryable central workflow defect. `.github#1873` was closed unmerged after enabling Dependabot vulnerability alerts initialized affected graphs and the same exact comparison returned HTTP 200. The hard gate remains fail closed; a current ConceptWeave head must still execute the pinned Dependency Review action successfully before acceptance. |
| Review / runner admission | PENDING_CURRENT_CHECKS | #35 has several completed execution checks but an unresolved CHANGES_REQUESTED review, a queued CodeQL successor/coverage and running Strix. The active rules require one approving review, resolved review threads and seven central workflows. Execution success does not imply review approval; queueing is not a reason to stop repository-owned work. |
| Zotero Research Intake | CAMPAIGN_INCOMPLETE | PRD FR-9 and ADRs 0006/0007 have executable local proposal, review, dry-run and recovery contracts. Saved-snapshot proposals cover 3,715/3,715 bibliographic items; unverified steward decisions and externally approved labels both remain 0/3,715. See the campaign evidence below. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring remains bound to authoritative standards/primary research and exact implementation contracts; hosted exact-head evidence remains independently required after head changes. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Dependency Review incident correction

The prior Foundation predecessor exposed a real hosted failure: the authenticated Dependency Review compare preflight returned HTTP 403 for a public, non-fork ConceptWeave exact range. The initially proposed central repair retried the same token-bound request while retaining fail-closed behavior.

Fresh owner RCA invalidated that causal hypothesis. The same authenticated exact-range request returned HTTP 200 for a repository whose dependency graph was initialized and HTTP 403 for affected repositories whose graph was not initialized. Enabling Dependabot vulnerability alerts initialized the dependency graph in ConceptWeave and pingora-gateway, after which the exact compare endpoint returned HTTP 200. Therefore `.github#1873` was correctly closed without merge: retries would extend queue occupancy but would not establish repository capability.

Acceptance remains stricter than the RCA. HTTP 200 availability alone is not GREEN. A fresh exact ConceptWeave consumer run must reach and complete the pinned Dependency Review action; 403, transport failure, skipped substitution or a sibling scanner cannot satisfy the hard gate.

## Central control-plane evidence

The current central branch ref is `.github/main@7fcada597d5b79bdb14445f24322b2c9f6ed4b19`, refreshed at 10:46–10:48 UTC. This is evidence only, not a mutable ConceptWeave dependency or a new audit of central protection. The earlier `8aea81323d93e90c79b71d7718de2798919fa1df` checkpoint followed admission-coverage and echo-only-review-job repairs; its nine-commit range to the current ref changes governance documents while preserving the previously read master-context and product-directive blobs. Already-created consumer runs retain their own exact workflow revisions.

- The current central source includes queue/admission and changed-scope/review-runtime repairs already integrated through ordinary protected history.
- `.github#1873@41935494aa234eb458f1cc08f006daaa278b9760` is closed/unmerged because repository dependency-graph initialization, not its retry/sleep source delta, was the verified root cause of the observed public-repository 403.
- #35 remains a consumer canary for runner admission and applicable workflow security checks. Its workflow-only change skips Dependency Review, so a dependency-changing Foundation run must separately prove that action's success. Already-created runs remain bound to their own central workflow revisions.

At 2026-09-05 09:47 UTC, Foundation #1's two CodeQL failures in [run 33937211620](https://github.com/ContextualWisdomLab/ConceptWeave/actions/runs/33937211620) were verified runner-release handoffs, not observed scan findings. Both dispatches succeeded but their terminal verdicts remained pending. Exact-head successor runs [33958339895](https://github.com/ContextualWisdomLab/.github/actions/runs/33958339895) and [33958340068](https://github.com/ContextualWisdomLab/.github/actions/runs/33958340068) were queued, bound to ConceptWeave `b538470c963e6524ddc0c3f652a46a4fc8265150` and central run source `7fcada597d5b79bdb14445f24322b2c9f6ed4b19`. This audit did not independently reverify that source's branch protection. The originating job promises an exact-job rerun after the terminal verdict; no manual retry, scanner substitution or new repair was justified by the red status alone.

The 10:46–10:48 UTC paginated audit still found 30 open PRs, 29 Draft, unchanged heads and 36 existing unresolved threads. Since 10:14 UTC it found one new submitted review, no new/updated thread comments and one new terminal failure. Foundation's same two successors remain queued at attempt one. #35's 10:35:40 CodeQL failure likewise records successful dispatch with a pending verdict; its exact-head successor [33961083940](https://github.com/ContextualWisdomLab/.github/actions/runs/33961083940) is queued. No scan finding, manual retry or weakening is inferred from these handoffs. ConceptWeave's active organization [ruleset 18156473](https://github.com/ContextualWisdomLab/ConceptWeave/rules/18156473) requires one approval, stale-review dismissal, thread resolution and seven central workflows, and protects against deletion/non-fast-forward updates. A classic-protection endpoint 404 does not negate these effective rules.

At 10:42:56 UTC, [Noema review 5120903874](https://github.com/ContextualWisdomLab/ConceptWeave/pull/35#pullrequestreview-5120903874) requested changes on #35's exact head, claiming `cargo generate-lockfile --locked` is unsupported and reporting adversarial confirmation. Fresh `cargo +1.98.0 generate-lockfile --help` exits zero and lists the flag; [Cargo's official command documentation](https://doc.rust-lang.org/cargo/commands/cargo-generate-lockfile.html#manifest-options) also documents its lock-preserving failure behavior. The consumer guard and subsequent tracked/unchanged-lockfile checks are preserved. An [evidence-based reassessment request](https://github.com/ContextualWisdomLab/ConceptWeave/pull/35#issuecomment-5551270771) was posted without dismissing the review or supplying approval. The central owner received this specimen to investigate whether model assertions were represented as executed verification; no owner diagnosis or repair is claimed yet. Flag support alone does not prove a hosted Product run or all dependency-freshness behavior.

## Zotero research campaign evidence

### Repaired current snapshot and source verification

PR #10 `e7d4e59f1b55b5954c5f8436527bc96e7ef2fb13` binds the complete captured provider JSON value plus the typed inputs actually consumed by the classifier, including omitted/default distinctions and post-decode changes. A separately recomputed complete proposal digest binds the actual records evaluated under a governance receipt. Root tests reproduce lost provider fields and prediction replacement under an unchanged receipt before fixing them. PR #27 records finalization RED `4718073` → GREEN `61fee4c`; PR #28 `25a787a` proves that a valid saved report preserves the proposal digest while changed title/evidence is rejected before external verification. Old approval JSON without that binding fails closed; never synthesize or backfill genuine approval receipts.

All 23 descendants (#11–#34, excluding absent #14) received the root changes by non-force merge/push, with their original deltas and Draft states preserved. Independent local testing at #34 `a359c5b9d1013e84f5832506f5a57aec364e6493` passed 143 tests across 36 suites, strict workspace Clippy, rustdoc with warnings denied, formatting and the CI contract. The first final-tip coverage run correctly failed: five owned regions in the missing-output-filename rejection were not exercised. Test-only `6f27da9` adds that boundary case without weakening file protection or excluding code; the focused test and full coverage run passed. Existing source-normalized coverage reports 3,197/3,197 regions and 596/596 branch outcomes; functions are 314/314. LLVM's raw instantiated totals remain 3,822/3,908 lines, 5,603/5,734 regions and 528/596 branch outcomes. Do not describe those raw totals as 100% or transfer local evidence to hosted checks.

The repaired executable at `a359c5b9d1013e84f5832506f5a57aec364e6493` completed a new read-only Zotero 10.0.1/API 3/schema-44/library-2 capture. It observed 8,326 records, 3,715 bibliographic items, complete proposal/provenance counts of 3,715 each, 56 adjacent-evidence proposals, one semantic-consumption bridge, 3,658 abstentions, 49 duplicate candidates and zero reported read failures. The versioned source digest is `sha256:0666dbebfb0c5aa99deb5a6dda1fc02d84bc46d08aaaddf25f5526a18eceef6d`. A distinct first pending batch was generated at test-only follow-up `6f27da9`; generation does not supply decisions.

All four new artifacts remain private mode `0600`, outside the repository:

| Artifact | Bytes | File SHA-256 |
| --- | ---: | --- |
| Repaired report | 6,890,050 | `bf45248413f433a537fe8fc62c02b93eef3c7e47ff6245f31610e9ba72031d8d` |
| Repaired worksheet | 1,640,941 | `2093aeffd3907e71d310715889b87e3fbc189cfba620338bf9a81b53ced26f87` |
| Aggregate progress | 258 | `1b0a01798e03d8dff0677ef3b13605979b667cb9a74e690327d87ae7c2d0bd25` |
| First pending review batch | 32,848 | `c8c3143bb23e4ebcb15d4ca789727c96a495d5ca50739e69e9eae2d22426286b` |

The repaired worksheet has 0/3,715 decisions and the first batch has 0/25; externally approved full-review coverage remains 0/3,715. No authorization prompt, approval, Zotero write, record merge/deletion or rollback was performed. The three historical Zotero 9 artifact hashes below were rechecked unchanged. The pre-repair schema-44 artifacts are also preserved. Stronger source binding is not classification correctness, business approval or loopback peer authentication.

The [CWL ontology capability inventory](doctoring/cwl_ontology_capability_inventory.md) now separates a 76-repository metadata census from a 15-candidate exact-default-head capability audit, up from 13. Four selected candidates have GitHub releases with resolved source commits: RankWeave, mhtml-etl-gateway, fast-mlsirm and naruon. naruon owns product-specific sender relationships and is explicitly proprietary, so its release does not satisfy permissive-library adoption. pg-erd-cloud implements adjacent relational-schema evidence and heuristic relation proposals under Apache-2.0, with no returned GitHub release or tag. DiskSage owns its implemented Rust filesystem ontology subset, not general semantic publication. Veilpick's protected tree contains only a license; graphify is an upstream fork without a returned CWL release. These are bounded maturity observations, not adoption receipts. Source-level discovery remains incomplete for 61 repositories, and actual ConceptWeave adoption remains unproved. No additional utility owner is justified yet.

Next: apply only authentic snapshot-bound steward decisions to the repaired batch, cover all 3,715 items and independently verify full-review approval; continue owner contract discovery and protected Foundation work while external checks/reviews are pending. The current body notes for PRs #11–#33 distinguish inherited source repair from historical head/check claims. No PR was closed, approved, merged or retargeted by this repair.

### Full-text availability and provider-contract finding

At `2a75051f0082103511222e278de24b2690fe6bfe`, the [read-only full-text sweep](doctoring/zotero_fulltext_contract_audit.md) attempted all 3,473 manifest entries in 10,737 ms, with 224,842,838 response bytes inside explicit diagnostic budgets. HTTP results were 3,432 successful and 41 missing, with five empty content strings. Nonempty text linked to 3,203/3,715 bibliographic parents, including 800/1,000 without retained abstracts; 2,561 parents had nonempty text with complete index counters. The [aggregate record](doctoring/zotero_fulltext_read_audit.json) preserves the full denominator, response partitions, metadata-report binding, read limits and observation digests without exposing bibliography or item identity.

Official Zotero 10.0.1 source `36749bd0bd4fdac9ee46c16f7aa7bed094a0851f` confirms that the full-text version storage receives remote sync versions, zero-valued local indexing and local API client versions, while the list omits the documented library-version header. Unchanged bookend manifest bytes and metadata library version 2 therefore do not prove an atomic full-text snapshot or a safe incremental cursor. No source file/database repair, new text-bound proposal, steward decision, approval or Zotero write is claimed. PRD FR-9, TRD and Proposed ADR 0006 now require separately captured content evidence; lifecycle capability metric remains 25 and approved full review remains 0/3,715.

### Privately retained full-text capture

The new branch implements and verifies the separate content-capture boundary. Its [live evidence](doctoring/zotero_fulltext_capture_evidence.json) is bound to `2c2226f1d583c3091cc126c96d27d55d1084c0d1`: all 3,473 manifest entries retained in a new private file, 3,432 successful and 41 missing content responses, nonempty text for 3,203/3,715 bibliographic parents and complete-counter nonempty text for 2,561. The remaining 512 parents are not excluded. Source-read interval is 28,898 ms, total command time 33.12 seconds, maximum resident memory 283,426,816 bytes and total response bodies 232,366,711 bytes. The single-link `0600` artifact is 235,602,798 bytes; its file digest is `56d385398c8da559aa597a4e3783d946638855bba19ac808ce81d917bf06f94d`. A separate saved-file audit verified content/report digests, parent links and counts. The original metadata report hash remains unchanged.

Replayable retained-text coverage has progressed from 0 to 3,203/3,715 parents. Lifecycle capability metric remains 25; new text-bound proposals, authentic steward decisions and externally approved labels remain 0. Neither stable provider bookends nor a recomputed local digest authenticates source authority. No Zotero write, model-provider bypass, release or protected merge is claimed.

Committed regressions repaired inherited environment proxies, exact-byte-limit rejection and replay checks occurring after digest allocation; clock fault injection verifies late/invalid-clock failures without changing the deadline. Final source verification at `733425df01511d894277fb8682e070f3dde03689` passed 173 tests across 37 suites including documentation tests, strict Clippy, formatting, rustdoc with warnings denied, the CI contract and the existing coverage gate. Coverage is 347/347 functions, 3,710/3,710 source-normalized regions and 674/674 source-normalized branch outcomes. Raw LLVM totals remain 4,159/4,255 lines, 6,129/6,274 regions and 603/674 branch outcomes; those are not 100%. The only source delta after the live run is a writer type alias resolving strict Clippy's complexity finding without changing runtime behavior. Hosted checks and independent protected approval remain separate gates.

Next: revalidate current-head protected gates after the completed transport cascade below; carry the new read-only view's exact context into a separately verified decision and approval contract without stripping its capture binding; continue the 61 remaining repository capability audits and the upstream version-contract repair. Unchanged predictions do not require a new proposal run merely to display retained evidence. A released contextual-orchestrator integration artifact remains unverified at the audited protected owner head, so source documentation alone does not authorize model-provider bypass. No new utility repository is justified by this one intake seam.

### Canonical transport repair and released-owner audit

PR #9's committed RED `31b507ae9feaf58688cf62ddcb597a88d2223366` reproduces six environment-proxy routes and rejection of valid JSON exactly at the 8 MiB response bound. GREEN `a2a84884f67dcac6f6892c958d55450aea6d6c88` disables inherited proxies and introduces the same strict UTF-8 inclusive reader at the original metadata owner. Oversized, invalid-UTF-8 and truncated responses remain rejected. Its 38 workspace tests, strict Clippy, formatting and existing coverage gate pass; source-normalized regions are 686/686 and branches 90/90. Raw LLVM functions are 97/97, lines 981/982 and branches 89/90, not raw 100% coverage. Root review independently reran the four transport tests before the non-force push. Subsequent authenticated adapters must reuse this reader at their own introduction points; full-text feature commits are not reverse-merged into the earlier owner.

Authenticated PR #17's production repair `53bd1fe16c43adc5cb0e7a052183e80b8c6c2e25` and authorization PR #18's `ba1bbd203c2f90afbf97ac3d7eab989982e8bc09` preserve committed RED cases for actual synthetic proxy forwarding and inclusive 1 MiB/512-byte boundaries. The existing synthetic server also needed POST-framing repair `7bcb791853ffa794529418ee9de1337fea4e1b15`; 91/99 workspace tests respectively, focused repetition and strict Clippy/formatting passed at those historical heads. A subsequent #19 coverage run correctly failed at 333/334 normalized branch outcomes because TCP fragmentation did not reliably exercise the helper's full-read loop. Canonical test-only #17 `b388810be8bceb3a4f81c336708cf1c56a20d057` sends an 8 KiB header and 8 KiB body through the unchanged 4 KiB buffer. It closes that gap without a new helper or coverage exclusion: #17 passes 92 workspace tests and 318/318 normalized branch outcomes; restacked #18 `21a7ee8f8b4b0988c13bb45aecbb016242c21308` passes 100 tests; #19 `aa74f8642e9e8c3804996ce443650df29f08bf5f` passes 101 tests and 334/334 branch outcomes.

The ordered non-force cascade completed through #34 `b0119a57047e7b1fe5ddfbbf4b973de0f15de172`: 156 workspace tests, strict Clippy/formatting and the existing coverage gate passed, with 315/315 functions, 3,214/3,214 normalized regions and 598/598 normalized branch outcomes. A fresh REST/named-ref audit found all 24 descendants #10–#13 and #15–#34 open/Draft at their expected heads; original heads, repaired parents and initially prepared local commits remain ancestors. Observed GitHub PR base objects differed from actual branch refs, so each merge resolved the named base through fresh fetch and `ls-remote`, not that field alone; the API discrepancy's cause was not diagnosed.

Root integration `75da75cf01704d9aae47f1e5573e3bbe3fb42bb0` retains one identical shared reader at its original metadata-owner location, all three transport regression modules and the richer request parser with the EOF guard. Full-text capture, replay, CLI and proxy-isolation source remain unchanged from its first parent. Independent source review found no actionable merge finding; this is not approval. Root verification passed 186 workspace tests across 37 unfiltered suites, including three doctests; the isolated subprocess invocation is not counted twice. Strict Clippy, formatting, rustdoc with warnings denied, CI contract and existing coverage gate passed. Functions are 347/347, source-normalized regions 3,710/3,710 and branch outcomes 674/674. Raw LLVM totals remain 4,159/4,255 lines, 6,129/6,274 regions and 603/674 branches, not 100%. No coverage exclusion, dependency, real Zotero request, credential use, classification decision or approval was added by this integration.

The [released-orchestration audit](doctoring/zotero_fulltext_contract_audit.md#released-orchestration-evidence) verified no qualifying artifact or deployed gateway in the inspected channels. Its 66 deployment records include eight successes, all Provider catalog sync, not proof of a serving gateway. Existing CO [PR #1030](https://github.com/ContextualWisdomLab/contextual-orchestrator/pull/1030) owns release work. The contacted CO integration task reported artifact/schema/deployed-version evidence as pending; it is not the #1030 writer, and confirmation from that actual release owner remains outstanding. The handoff requests that evidence without duplicate release machinery or provider bypass. New text-bound proposals and approved labels remain zero.

A 10:48 UTC CO task snapshot exposed a terminal selected-model capacity error without new release evidence. One continuation prompt kept the same task/model settings and prior scope; the subsequent compact snapshot returned active/in-progress. The CO task then clarified that its single-writer scope is integration PRs #1067/#1074, not release PR #1030, and will locate the existing release owner's exact evidence. Its reported `47ae9d65` integration change and ongoing tests are not an immutable artifact, schema digest or deployed-gateway receipt. This is a task handoff report, not independently reverified PR state. No new task, paid-model substitution or duplicate release implementation was created.

### Capture-bound private review view

[Private review view PR #37](https://github.com/ContextualWisdomLab/ConceptWeave/pull/37) is an open Draft child of #36. Its integrated source `54383eae2e83863c4cb72ee00f16cd504ff66151` adds an offline, read-only view of the next pending papers and their retained text. Committed core RED `0027d7d` fails one selected test against the intentional stub; GREEN `e426c2b` reuses the existing canonical batch and complete capture verifier. A prior incorrectly filtered command selected zero tests and is not RED evidence. Boundary tests `3fb930b` and `fcba236` cover changed bindings, direct-parent selection, missing/empty/partial/unknown responses, separate metadata/content versions, standalone exclusion and escaping beyond the fixed 16 MiB output. No text is silently truncated or unselected parent copied.

CLI RED `25a1005` precedes implementation `045031c`, warning correction `ed308bd` and test-only `26a3e6a`. Ordinary merges `b86fe86` and `54383ea` preserve both development histories. The CLI shares the existing canonical-parent, single-open `O_NOFOLLOW`, identity, single-link, exact-`0600` and create-new output boundary. Metadata inputs retain their 16 MiB limit; the capture uses buffered deserialization under a separate 512 MiB file bound, including exact-boundary, size-drift, trailing-data and static-error tests. That file ceiling is not the capture's 256 MiB response-body budget and can reject an otherwise valid high-escaping capture. Both legacy decision-application modes reject the new outer view; copying out its nested metadata batch would discard full-text provenance and is not an approved conversion.

The [saved-file evidence](doctoring/zotero_fulltext_review_evidence.json) records one real offline invocation using the original private repaired report, worksheet and full-text capture, with no Zotero or model requests. It produced 25 pending rows, 21 with nonempty text and four without it; the latter remain visible. The 21 attached responses were HTTP 200, with 16 complete and five unknown index counters under the documented predicate. The new single-link `0600` file is 1,590,742 bytes with SHA-256 `e99a1963f3b5d7adfb62070785f2b69f1d9efd1d4882d365a5ca6f6b8d70f34a`. Command elapsed time was 1.60 seconds, maximum resident memory 288,014,336 bytes and peak memory footprint 286,966,336 bytes. This is one observed local run, not a latency SLO or a load benchmark.

An independent aggregate-only audit recomputed the saved capture, complete report and proposal digests; compared every selected parent, attachment revision, content version and raw response body with the original capture; and proved the nested batch equals the earlier metadata batch. Original report, worksheet, capture and batch file hashes remain unchanged. Pending rows with an evidence view progressed from zero to 25, including 21 with nonempty captured text and four without it; this is an evidence-access measure, not classification progress. The full denominator and remaining decisions stay 3,715, lifecycle capability metric stays 25, and new text-bound proposals, authentic steward decisions and independently approved labels stay zero.

At `54383ea`, 201 workspace tests across 38 unfiltered suites, including three doctests, pass; the nested subprocess result is not counted twice. Strict Clippy, formatting, rustdoc with warnings denied, CI contract and the existing coverage gate pass. Functions are 359/359, source-normalized regions 3,982/3,982 and branch outcomes 692/692. Raw LLVM lines remain 4,322/4,430, regions 6,314/6,500 and branches 612/692, not 100%. An earlier CLI-only coverage run exposed an unreachable test panic arm, repaired without exclusions, and core cases already covered by the integrated newer tests. Independent documentation and integrated source reviews found no actionable regression; neither is independent protected approval.

PRD FR-9, TRD, the Proposed ADR 0006 amendment, Context Map, Ubiquitous Language, sequence diagram, architecture, threat model and contributor rules preserve the same boundary. The next product gap is exact full-text review-context identity through decision application, worksheet history, finalization and fresh external approval. The existing approval contract has no capture digest; a readable file does not close that gap. Hosted current-head checks, prerequisite protected merges and released orchestration evidence remain independent requirements.

### Historical pre-repair Zotero 10 transition

The following schema-44 observations were captured before integrity repair and are retained for provenance, not reused as current approval inputs.

A fresh read at `22030ae6c8510d9eb8f7b07d98959bb69d2bd286` observed Zotero 10.0.1, API 3/schema 44, library version 2 and a present server identity. It produced a distinct report/worksheet pair without overwriting the historical artifacts below. The full read still counted 8,326 records and 3,715 bibliographic proposals, with 56 adjacent-evidence proposals, one semantic-consumption bridge, 3,658 abstentions and 49 duplicate candidates. The new worksheet's aggregate checkpoint remains 0/3,715, incomplete. Equal totals do not prove unchanged content across the Zotero 9-to-10 version-space transition.

Both new files have mode `0600`. The report is 6,890,050 bytes with file SHA-256 `d56c8ac70da7f094355748f6611ba47f9d2256bb87f0e24ab84683536e56fb9e`; the worksheet is 1,640,941 bytes with file SHA-256 `919ad3b875846c018eb92df2b2caf5d9a8ed491ede0b4718c07e56dc69bca0d9`. Their implementation-reported snapshot digest is `sha256:bcc50fdf4e16789e7d2651b431817dfc178fdcaad7e0c73360fda2d83351d7b5`, which is not yet proof of complete raw-field binding.

PR #10's existing findings remain source-confirmed at this capture head: provider fields omitted by the typed input are lost before raw-snapshot hashing, and mutable report predictions can be evaluated under an unchanged approval receipt. These invalidate stronger integrity claims, not the observed aggregate counts. Repair the canonical owner, propagate through the stack and regenerate before approval/promotion. Zotero 10 availability alone does not resolve the loopback confidentiality finding or grant write authority.

At that earlier checkpoint, the inventory covered eight bounded owner candidates: seven returned no GitHub release, while RankWeave v0.18.0 resolved to an exact commit. The expanded inventory above supersedes that audit denominator without rewriting its historical observation. `context-graph-contracts` and `enterprise-architecture-core` use protected `develop`, not `main`, as their default adoption baseline. Neither checkpoint permits bypassing missing owner releases.

### Historical Zotero 9 snapshot

The following record preserves earlier measurements and execution guidance. It does not authorize applying the old worksheet or batch to the current Zotero 10 snapshot.

The parent integration ending at `062a0d9bca086d5a2aaa5d4122f58364115d4f91` replaced the baseline with Foundation's document and removed the research section present at `a84e6d49aba2a4fd0b0ef303a342922c4ce909bb`. This section restores FR-9 traceability using the saved private artifacts and the current executable. It preserves Foundation's updated status. These records must survive later parent integrations alongside the Foundation, Client and Source Observation evidence.

On 2026-09-05, the existing `--review-progress` command at `062a0d9...` revalidated the original report and worksheet offline. The report still binds Zotero 9.0.6, API v3/schema 42, library version 12341, rule `ontology-research-v2`, and snapshot `sha256:c49b08066c4526e520a5f85416543ea20a620a06170e1e15f563088f6bc9e162`. This replay does not claim the mutable Zotero library is still at that version. All three original artifacts retained their hashes and remained outside the repository.

| Measure | Saved-snapshot result | Meaning / remaining work |
| --- | --- | --- |
| Observed records | 8,326 | The original read completed without reported failures; four top-level non-bibliographic items are excluded from the classification denominator. |
| Proposal and provenance coverage | 3,715/3,715 each | Every bibliographic item has a proposal and source coordinates. These counts do not establish classification correctness. |
| Proposed dispositions | 56 adjacent evidence; 1 semantic-consumption bridge; 3,658 abstentions | Deterministic evidence leaves unsupported meaning for review. |
| Duplicate candidates | 49 | Reversible candidate groups, with no record merge or deletion. |
| Unverified worksheet coverage | 0/3,715 | The replay returned `remaining_count=3715` and `complete=false`. |
| First pending batch | 0/25 decisions filled | The existing 32,940-byte batch is a repeatable review view, not an assignment or a completed review. |
| Externally approved full-review coverage | 0/3,715 | No completed full review or externally verified approval receipt is available in this campaign. A sample cannot satisfy this measure. |
| Live write / rollback evidence | Not performed | The saved report originated from Zotero 9.0.6; Zotero 10 adapter tests do not prove approved live execution. |

Artifact SHA-256 values for reproducibility (no bibliography or item identities):

- report: `ff13383b88f89fcef94d2f2d7284838b268fb871bed78c75ce5b53bfab2138a8`;
- worksheet: `ad32c8352cb7d84ac3bdcd3a60c975f61e2e19adc3a8294d4c680360071e752b`;
- pending batch: `7d1a77bd6913bd8c0c826ab60c1a4fa31afede7f7e7e694aad351c31c710b921`;
- replayed aggregate progress: `ac79c719037aca2d67bb4b0ea7e84babd8a701506a7d2ec274139d260328f524` (262 bytes, mode `0600`).

The next campaign step is authentic steward input. After all 25 decisions in the existing batch are filled, `--apply-review-batch` must reconstruct the pending view from the original report/current worksheet, validate every displayed context field, reject unknown fields and blank/abstention decisions, and produce a separate owner-only worksheet. Directly reading that rich batch through `--apply-decision-patch` is rejected because it would discard the displayed context. A successful first application should produce unverified progress of 25/3,715, with 3,690 remaining; no such result is claimed here. The same process must cover all bibliographic items before external full-review approval. PRD FR-9, TRD's Research Intake contract and [ADR 0006](adr/0006-zotero-research-intake.md) define these boundaries.

[ADR 0007](adr/0007-reviewed-zotero-write-plan.md), [the threat model](../THREAT_MODEL.md) and [PR #18's unresolved transport finding](https://github.com/ContextualWisdomLab/ConceptWeave/pull/18#discussion_r3935881086) retain the remaining write boundary: `Zotero-Server-ID` checks database continuity but does not authenticate the loopback peer or protect a key from a hostile process occupying its port. Enterprise-secure live write-back cannot be claimed without protected provider transport or explicit governance acceptance of that remaining risk, followed by approved write, partial-failure and rollback evidence.

ConceptWeave remains the owner of research intake and semantic-model generation. `semantic-data-portal` owns catalog/consumption, `context-graph-contracts` owns versioned interop contracts, and `contextual-orchestrator` owns model calls. These are the existing Context Map boundaries, not claims of released integration. ConceptWeave has no immutable release or verified released consumer adoption yet. A separate Utility Repository has no evidenced independent consumer or deployment contract at this snapshot.

## September 6 source-scope admission checkpoint

PR #12 successor runtime `fc0465e` now uses the shared inventory/audit validator and required v2 scope binding before independent duplicate governance. The existing exact candidate-membership receipt already covers duplicate selection; it is not an unprotected authority gap in this consumer. RED `4656d6b` reproduced three retained-source admission bypasses, now rejected. Current local result: 87 tests/17 suites, independent duplicate review 12/12, pinned coverage 153/153 functions, 1,294/1,294 normalized regions and 220/220 normalized branches. Raw coverage remains 1,649/1,658 lines, 2,516/2,536 regions, 200/220 branches. See the Proposed ADR 0006 amendment for history, compatibility and remaining gates. Later restored-report/worksheet/write integration is still required; current root PR #39 has not adopted these changes.

PR #11 local successor checkpoint `23178a9768aa216692d77918d56357ae1269535c` normally inherits #10 `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4` while preserving prior #11 `1dc032598b41a35d52c09d8690c871e07365d7e3`. The stable isolated baseline passed 60 tests/15 suites; an earlier run contaminated by merge timing is invalid evidence. RED `ebdd852` reproduced forged derived audit and ambiguous provenance totals. Extracted audit computation now counts unique parent/child identities and is recomputed before governance. Final local tests passed 75/15 suites including two doctests; independent integrity review passed 17 tests with no new regression. Strict Clippy passed at unchanged runtime `935e035`. Pinned coverage is still running; no inherited coverage claim or remote PR update is made here.

The pinned coverage run subsequently finished with exit 0: 140/140 functions, 1,101/1,101 normalized owned regions, 182/182 normalized branches. Raw lines 1,502/1,505, regions 2,332/2,343 and branches 177/182 remain below 100%. The coverage script and exclusions are unchanged; the earlier pending sentence records chronology, not current execution state.

Residual duplicate-owner gap: changing duplicate candidates and their matching audit count is not bound by the v2 proposal receipt; derived count consistency does not authenticate duplicate proposals. No duplicate merge/write authority is granted. Fresh native screenshot and accessibility state both showed 3,719 items in the library view, with list rows and attachment icons rendered. The earlier Mac-lock limitation is no longer current. No screenshot or bibliographic identities are committed; this view check does not establish full-library reclassification.

Follow-up runtime `8ccb0d5b3d7705786b6c40c3bcf5a10ff32046d9` adds explicit legacy receipt/empty/orphan/cycle/blank-identity regressions and removes only checks proven redundant after shared admission. Local workspace tests: 71/14 unfiltered suites including two doctests; strict Clippy, warnings-denied rustdoc, release build, format and diff checks passed. Independent read-only review reran 15 integrity tests and found no actionable defect. Unchanged pinned coverage gate passed: functions 136/136, normalized owned regions 1,037/1,037 and branches 174/174. Raw lines 1,448/1,451, regions 2,268/2,279 and branches 169/174 remain below 100%; no raw full-coverage claim is made. Logs: `/tmp/conceptweave-admission-coverage-baseline.log`, `/tmp/conceptweave-admission-coverage-green.log`, `/tmp/conceptweave-admission-boundary-tests.log`.

PR #10 source `f6735b585022aac1c8ceff86c150d9b64fd77ec2` repairs evaluation admission and independently approved scope binding after normal integration of producer `51c7df6d03f072449422fd58ca24b2f9d6026f07`. RED `3f2cf55` failed three new integrity tests; GREEN passed 68 tests/14 unfiltered suites and strict Clippy. See the Proposed [ADR 0006 amendment](adr/0006-zotero-research-intake.md). This local result does not establish hosted GREEN, protection-compliant merge, release, downstream adoption or full coverage.

Remaining work: mandatory adoption by restoration, worksheet, duplicate and write consumers without empty defaults or weakened full-text binding. Genuine reviewed decisions and approvals remain 0/3,715 bibliographic proposals, with four additional standalone sources unresolved. Native visual inspection remains unverified at this checkpoint because the Mac was locked. No UI change or new utility repository was needed.

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local registry/credential resolution; explicit read-only session/transaction; exact schema allowlist; total operation and statement deadlines; cancellation plus row/byte/concurrency budgets; complete immutable snapshot or fail closed; source-disappearance handling; deterministic replay against a frozen anonymized GRC-shaped fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; do not infer business authority from relational structure alone.
5. **LLM Proposal** — every production model call through a released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
12. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

### PR #23 private report path repair (2026-09-06)

Untouched `2a3619f52e1d3e4f699c91be1fc2d0e9a6e234c8` passed 121 tests/23 suites.
Normal merge `01d6e3f` preserves that delta and PR #22 `51d1682`; integration
passed 145/23. Unix creation-time `0600`, handle-based permission enforcement,
exclusive creation, and non-Unix rejection remain intact.

Independent review found two existing defects. RED `7cfc7fb` compiled and failed
two of five CLI tests: raw `/tmp` was returned instead of the checked canonical
parent, and permission failure deleted an unrelated replacement at the output
path. Existing canonical owner fix `86288cdf5959040a95221c2ca2d99e243d25dc27`
was reused with provenance as `25154b6`. `48d7068` propagates permission errors
without unlinking a pathname that may have changed. Report serialization never
starts on that failure; an empty private file may remain for deliberate cleanup.
The regression verifies no group/other mode bits before the injected setter and
preserves the replacement sentinel. Only synthetic temporary files were used.

`c0db7ff` corrects the old raw-system-temp test expectation. Coverage then exposed
the reachable nameless `..` case; `522e46b` reuses the later-owner rejection test
and removes incidental branches from test cleanup, with no coverage exclusions
or weakened runtime checks. Independent review found no further production issue.
The later shared private-output writer still needs this no-unlink failure policy.

Final 147 tests/23 suites including three doctests, strict Clippy, warnings-denied
rustdoc, format/CI-contract/diff and unchanged coverage pass. Coverage: 278/278
functions, 2404/2404 normalized regions, 404/404 normalized branches; raw LLVM
3183/3244 lines, 4795/4896 regions, 360/404 branches remain below 100%.
Logs use `/tmp/conceptweave-pr23-private-` with `red.log`, `final.log` (old path
expectation failure), `verified.log`, `clippy-verified.log`, `rustdoc-verified.log`
and `coverage-verified.log`; baseline/integration use `pr23-scope-` instead.
README and Proposed ADR record the empty-file downside and rejected racy cleanup.

No fresh visual evidence was collected; latest native attempt encountered the
locked Mac. Historical 3,719 displayed items are not reclassification evidence.
Real decisions/approvals remain 0/3,715 plus four unresolved sources; no actual
authorization, mutation, recovery, protected merge or release occurred. Next
verified successor is PR #24 complete review evaluation
`1e73e1545de32ae9a349c469a7794c5c3fc2ae9b`; root and shared-writer adoption remain open.

### PR #22 steward-context binding verification (2026-09-06)

Untouched `7179d13b45d160682e4cce1473c145d465fe657b` passed 120 tests/23 suites.
Normal merge `49daf4c` retains that delta and parent PR #21 `425d8df`; integrated
tests passed 143/23. Documentation conflicts preserve both private abstract
minimization and corrected source-version/uncertainty contracts. The original
PR #22 production implementation is unchanged: an abstention retains a nonblank
abstract only when it is not already in matched evidence; decided items omit it.

Independent review identified missing binding regression evidence, not a runtime
defect. Test `ebcf2a3` covers insertion, replacement and removal of review context:
each changes the inherited v2 proposal digest and rejects the original approval
before contacting governance. `b44d603` also rejects locally rewritten digests
against the independent original receipt, preserves whitespace omission and
verbatim Japanese context, and retains all four original minimization cases.
Evaluation aggregates exclude synthetic item/reviewer/title/abstract values.
This is context retention, not multilingual classification or real approval.

Final 144 tests/23 suites including three doctests, strict Clippy, warnings-denied
rustdoc, format/CI-contract/diff and unchanged coverage pass. Coverage is 269/269
functions, 2369/2369 normalized regions, 404/404 normalized branches; raw LLVM is
3083/3143 lines, 4625/4722 regions, 360/404 branches, not 100%. Logs use
`/tmp/conceptweave-pr22-scope-` with `baseline.log`, `integration.log`,
`complete.log`, `clippy-complete.log`, `rustdoc.log`, `coverage-complete.log`.
Independent final review found no remaining issue in the reviewed delta.

Sensitive report context stays private; no fresh real-data capture or visual
evidence was collected. Latest native visual attempt encountered the locked Mac;
historical 3,719 displayed items are not classification evidence. Real decisions
and approvals remain 0/3,715 plus four unresolved sources. No real mutation,
authorization, recovery, protected merge or release occurred. Next successor is
PR #23 owner-only report permissions `2a3619f52e1d3e4f699c91be1fc2d0e9a6e234c8`;
root and authoritative full-text envelope adoption remain outstanding.

### PR #21 delayed observation authority repair (2026-09-06)

Untouched `09c84e4cdb1393a5e450f5200b87f292eeea956f` passed 119 tests/22 suites.
Normal merge `ec2baac` preserves that delta and repaired PR #20 `000b37b`;
integration passed 142 tests/22 suites. Conflicts retain the delayed observer
while preserving parent forward/inverse uncertainty and complete request fields.

Behavioral RED `f2cf4ae` compiled and failed one of two reconciliation tests:
metadata-only observation returned `Restored` instead of `Indeterminate`.
Owner fix `f9c2c03` retains local operation validation, the single callback read,
complete operation and complete observation, but never infers completion or
termination and never emits a retry operation. The obsolete state classifier is
removed. All eight metadata cases remain; the adapter test compares the complete
state and proves its three HTTP requests are GETs. Legacy enum variants and the
optional retry slot remain compatible, not authority-producing outputs.

At `f9c2c03`, 142 tests/22 suites including three doctests, strict Clippy,
warnings-denied rustdoc, format/CI-contract/diff and unchanged coverage pass:
268/268 functions, 2360/2360 normalized regions and 400/400 normalized branches.
Raw LLVM remains 3077/3137 lines, 4616/4713 regions and 356/400 branches, not 100%.
Logs use `/tmp/conceptweave-pr21-causal-` with `red.log`, `final.log`, `clippy.log`,
`rustdoc.log`, `coverage.log`; doc/test-name-only follow-up uses `complete.log`
and `rustdoc-complete.log`. Independent review found no blocking issue.

PRD/TRD/Proposed ADR retain the distinct downstream gap: opaque full-text wrappers
must retain the entire prior rollback receipt, exact submitted request, binding
and untouched tail. An operation-only observation cannot substitute that envelope.
No new approval or capability layer was introduced. Next verified successor is
PR #22 review context `7179d13b45d160682e4cce1473c145d465fe657b`.
No fresh visual evidence was collected; the latest native attempt encountered
the locked Mac. Historical 3,719 displayed items are not classification evidence.
Actual decisions/approvals stay 0/3,715 plus four pending sources. No real
authorization, mutation, recovery, protected merge or release occurred.

### PR #20 rollback uncertainty repair (2026-09-06)

Untouched `a03a7248c894a1e0765968ddf58514d98c517da3` passed 116 tests/21 suites.
Normal merge `c53e36b` retains it and PR #19 `6dc6176`; conflicts preserve the
original rollback functionality and corrected forward-write uncertainty. Missing
required review proposal binding was repaired in test fixture `e5f583a`.

Independent review found the inverse executor repeated the forward-write causal
error: matching restored metadata implied completion, while unchanged metadata
cleared uncertainty and re-enqueued the failed inverse. Initial `318b370` RED had
an empty-vector type error and is not behavioral evidence. Independent test-only
`c0fc17ba5262882662b487114d24c75e7ffb5d3d` against unchanged `e5f583a` compiled,
then failed one of six tests: observed restored keys `[B]` versus expected empty.
Its worktree `/private/tmp/conceptweave-rollback-red.O5zdby` and log
`/tmp/conceptweave-pr20-causal-independent-red.log` are preserved.

Owner fix `4e28613` always retains failed or invalid inverse attempts as unknown,
with complete operation, exact submitted request and optional readback. Only
directly verified earlier restorations remain restored; remaining operations
contain untouched work only and confer no retry authority. `ff66a54` compares
complete request/observation in all three retained readback scenarios; `006efd0`
corrects the six invalid-response cases. Coverage found an obsolete self-comparison
after inference removal; `876cbfe` removes it without weakening shared-library,
server, metadata or item-revision preflight. Independent final review confirmed
these checks remain intact. `b238bc6` corrects misleading authority docstrings.

At `876cbfe`, 139 tests/21 suites including three doctests, strict Clippy,
warnings-denied rustdoc and unchanged coverage gate pass: 262/262 functions,
2332/2332 normalized regions, 394/394 normalized branches. Raw LLVM remains
3007/3067 lines, 4525/4622 regions and 350/394 branches, not 100%. Logs use
`/tmp/conceptweave-pr20-causal-` with `verified.log`, `clippy-verified.log`,
`rustdoc-verified.log` and `coverage-verified.log`; docstring-only follow-up uses
`release-check.log` and `rustdoc-release-check.log` (not a release claim).

PRD/TRD/Proposed ADR explicitly keep detached operation DTO authority and complete
original-write scope open. Empty inverse lists do not prove recovery of an unknown
original write; existing authoritative successor wrappers must be adopted rather
than inventing a competing capability layer. PR #21 delayed reconciliation
`09c84e4cdb1393a5e450f5200b87f292eeea956f` repeats the causal inference and is next.
No fresh native visual evidence: reinspection again encountered the locked Mac.
Historical 3,719 displayed items are not classification proof. Decisions/approvals
remain 0/3,715 plus four unresolved sources; no real authorization, mutation,
recovery, protected merge or release occurred. Root adoption remains outstanding.

### PR #19 approved execution successor verification (2026-09-06)

Baseline `62c19ee23e3c827bc7db15c79f4755ff040489e9` passed 109 tests/20 suites.
Normal merge `bf4b2e561fdbd8e085c0d98dfa76ca97ad4d6ade` retains that head and
parent `b68d21aa24f355608c4673ad037cc2ff8af031f6`. Integration failed visibly
because the synthetic direct-plan constructor omitted required `proposal_digest`.
`785cdf5` replaces it with the existing report/review builder and checks the
output digest. This exposed a stale synthetic before-state; `5693f12` fixes the
source fixture, preserving the original nonempty-to-nonempty metadata scenario.
Neither required binding nor before-state validation was weakened.

The existing failed-POST/matching-observation test now runs both the generic
executor and public adapter boundary. It retains the exact transport-error check
on the generic route and complete receipt/request/observation/digest, one POST,
and no inferred applied/inverse assertions on both. No second runtime execution
path or approval issuer was added. Independent review found no runtime bypass;
its fixture finding was addressed before final validation.

Final source passed 132 tests/20 suites including three doctests, strict Clippy,
warnings-denied rustdoc, format/CI-contract/diff and unchanged coverage gates.
Functions 252/252, normalized regions 2164/2164, normalized branches 370/370;
raw LLVM 2900/2944 lines, 4393/4466 regions and 330/370 branches remain below 100%.
Logs use `/tmp/conceptweave-pr19-scope-` with `baseline.log`, `integration.log`,
`final.log` (stale fixture failure), `complete.log`, `clippy-complete.log`,
`rustdoc-complete.log` and `coverage-complete.log` suffixes.

PRD's inherited unexpected-mutation inverse claim is removed; ADR 0007 remains
Proposed and records observation-only semantics and the rejected duplicate path.
No fresh visual evidence was collected; the latest attempt was blocked by the
locked Mac, and historical 3,719 displayed items do not establish reclassification.
Real decisions/independent approvals stay 0/3,715 plus four unresolved sources.
No real authorization, mutation, recovery, protected merge or release occurred.
Next verified successor is PR #20 `a03a7248c894a1e0765968ddf58514d98c517da3`;
rollback must retain unknown original-write scope rather than derive authority
from serialized audit data. Root checkout adoption remains outstanding.

### PR #18 local authorization successor verification (2026-09-06)

Baseline `fe2cff4f9fc40496bbb4339ba4242543beacea9b` passed 108 tests/20 suites.
Normal merge `2dfec77ab32d7dde5a9c92c6c29bc91df4058294` retains that head and
parent `06d836a07fdb434683f88a31b45150a8a06f27f7`. Independent review verified
the authorization/transport block is identical to original PR #18 and the
write-plan/executor block identical to the repaired PR #17. No new runtime,
credential storage, revocation API or authority issuer was introduced.

The inherited HTTP 500 fixture lacked server identity, so PR #18 rejected it
before interpreting status. RED `d5bc6e3` proves `InvalidResponse` differed from
the intended `RequestFailed`; `5790358` adds the matching server header while
retaining the explicit error assertion. The executor still keeps the exact
request, complete matching observation and proposal binding as indeterminate,
with exactly one POST and no inferred applied/inverse operations. Independent
read-only re-review found no further inheritance issue; this is not approval.

Final source passed 131 tests/20 result suites including three doctests, strict
Clippy, warnings-denied rustdoc, format/CI-contract/diff and the unchanged coverage
gate: 247/247 functions, 2154/2154 normalized regions, 370/370 normalized branches.
Raw LLVM remains 2821/2874 lines, 4265/4349 regions and 330/370 branches, not 100%.
Logs use `/tmp/conceptweave-pr18-scope-` with `baseline.log`, `red.log`, `final.log`,
`clippy-final.log`, `rustdoc-final.log` and `coverage-final.log` suffixes.

Native visual reinspection was attempted after verification but the Mac was
locked; no fresh screenshot was obtained. Previously verified 3,719 displayed
items remain historical display evidence only. Actual decisions/independent
approvals remain 0/3,715, plus four unresolved standalone sources. No real
authorization, write, recovery, protected merge or release occurred. PR #19
approved execution must inherit this chain next; the root checkout remains older.

### PR #17 authenticated transport successor verification (2026-09-06)

Baseline `c88f9a34c1fc4e72e38cf66b1d2f3fcb305e560a` passed 100 tests/19 suites.
Normal merge `a2768ae` retains that transport and parent `84b27fb`; documentation
conflicts retain the original wire contract while replacing unsafe post-read
completion/rollback inference. No original transport implementation is replaced.
Independent review found missing executor-plus-HTTP regression coverage, added
in `97cce5a` and strengthened in `29a3771` to compare the entire observed state.

The synthetic fixture returns a failed POST followed by fully matching metadata.
Exactly one POST is observed; the receipt preserves the exact submitted request,
complete observation and proposal binding, remains indeterminate, and emits no
applied or inverse operations. It uses an ephemeral loopback port and synthetic
credentials only. Existing proxy, conditional-write and provider-response tests
remain intact. This does not prove peer authentication, live mutation or recovery.

Final source passed 123 tests/19 result suites including three doctests, strict
Clippy, warnings-denied rustdoc, format/CI-contract/diff and unchanged coverage
gate: 226/226 functions, 2022/2022 normalized regions, 354/354 normalized branches.
Raw LLVM: 2508/2560 lines, 3799/3880 regions, 314/354 branches, not 100%.
Logs use `/tmp/conceptweave-pr17-scope-` with `baseline.log`, `complete.log`,
`clippy-complete.log`, `rustdoc.log` and `coverage-complete.log`.

Protected merge, release, authentic decisions and real library reclassification
remain open. Next verified dependencies are PR #18 local authorization then PR #19
approved execution; they must preserve scope and uncertainty without reissuing
authority from audit JSON. The root checkout still lacks this cascade.

### PR #16 multilingual successor verification (2026-09-06)

Baseline `044018cef4e5d3e919b278a1cfebe56857863601` passed 87 tests/19 suites.
Normal merge `4f3ac52a27ff1090b94c63ff807ddb5bd9ce872f` retains that entire delta
and parent `42ddd81adf8d994f67a30f0c6b8383d637073c72`. Production source is identical
to the repaired parent; `review_contract.rs` is identical to the prior PR #16.
An independent static review confirmed all seven non-English cases, English
match, blank metadata and unmatched metadata remain without reduced assertions.
This is abstention safety, not multilingual classification or completed research.

Exact merged source passed 109 tests/19 result suites including three doctests,
strict all-target Clippy, warnings-denied rustdoc, format, CI-contract and diff
checks, plus the unchanged coverage gate: 185/185 functions, 1770/1770 normalized
regions and 320/320 normalized branches. Raw LLVM remains 1998/2050 lines,
2934/3014 regions and 280/320 branches, not 100%. Logs use the
`/tmp/conceptweave-pr16-scope-` prefix (`baseline`, `final`, `clippy`, `rustdoc`,
`coverage`, each with `.log`). No new production code, live write, paper decision,
approval, protected merge or release is claimed. Next owner is PR #17 authenticated
transport; it must preserve required bindings and indeterminate-request semantics.

### PR #15 execution-scope and uncertainty repair (2026-09-06)

Baseline `45e9c493` passed 87 tests/19 result suites; normal merge `2887f70`
preserves that executor and parent `eb8eaa4`, passing 109 tests/19 suites.
RED `e8b4c06` failed three receipt-binding tests, repaired by `b91ad9f` retaining
the proposal/source digest across dry-run, applied, preflight and partial failure.

Independent review then found a root causal-completion defect: matching before
metadata could hide a delayed request, while matching after or unrelated newer
metadata could invent applied/rollback status. Existing scenarios were preserved
with corrected expectations in RED `646a10c` (two failures). Runtime `c09d101`
removes those inferences and retains the exact submitted request plus optional
observation. Only earlier directly verified operations retain applied/inverse
status. Test `e169630` compares the complete captured request and missing fields.
Independent static review found no remaining actionable owner defect; it is not
GitHub approval. Original and later executor paths both require propagation.

Final source: 109 passing tests/19 result suites including three doctests, strict
all-target Clippy, warnings-denied rustdoc and unchanged coverage gate passed.
Coverage: 185/185 functions, 1770/1770 source-normalized regions, 320/320 normalized
branches. Raw LLVM is 1998/2050 lines, 2934/3014 regions and 280/320 branches, not
100%. Evidence logs: `/tmp/conceptweave-pr15-causal-final.log`, `-clippy-final.log`,
`-rustdoc.log`, and `-coverage.log` share the `conceptweave-pr15-causal` prefix.

No real library write, paper decision, approval or release occurred. Subsequent
execution/recovery/full-text consumers must inherit unknown-request semantics,
retain exact request and earlier receipt fields, and reject retry/rollback of an
unknown original write even when its inverse list is empty. Protected integration,
descendant adoption, live write/rollback and actual reclassification remain open.

### PR #13 source-scope integration checkpoint (2026-09-06)

Normal merge `5df57a7` preserves write-plan head `b41217b` and source-scope
parent `3d2c252`. Two inherited test inputs lacked the optional tag type added
by write planning; explicitly retaining `None` restores their original manual-tag
semantics. Rust 1.98.0 workspace verification passed 94 tests across 18 result
suites, including two documentation tests. The initial shell selected Rust 1.97.1;
the verified invocation is `cargo +1.98.0 test --workspace`.

Native Zotero visual inspection again showed 3,719 items and attachment icons.
This is display evidence, not approved reclassification or write evidence.
Write-plan source-scope validation and proposal-bound approval remain open;
this local integration is not a hosted-check, protected-merge, or release claim.

### PR #13 write-scope repair checkpoint (2026-09-06)

The preceding integration-only gap is locally repaired by `c348278` with tests
`dcde49e` (two RED failures) and `1fe3d7d` (binding/independent-receipt coverage).
Shared report admission now precedes authority, and the required v2 proposal
binding is retained in the plan. Legacy item/metadata errors retain precedence.
Independent static review found no blocking issue; it is not GitHub approval.

Rust 1.98.0 workspace: 97 passing tests, 18 result suites including two doctests;
strict all-target Clippy, warnings-denied rustdoc and unchanged coverage gate pass.
Coverage: 169/169 functions, 1533/1533 source-normalized regions and 292/292
normalized branches. Raw LLVM remains 1831/1852 lines, 2739/2777 regions and
255/292 branches, not 100%. Logs use `/tmp/conceptweave-pr13-write-scope-` with
`final.log`, `clippy.log`, `rustdoc.log` and `coverage.log` suffixes.

PRD/TRD/ADR 0007 and DDD views retain separate metadata, duplicate-membership,
full-text and execution authority. No real decisions, approvals or Zotero writes
occurred. Protected merge, release, descendant adoption and actual reclassification
remain open; no local evidence is transferred to a remote or later head.

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
