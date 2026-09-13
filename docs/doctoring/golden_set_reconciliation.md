# Golden-set stale-parent reconciliation record

Date: 2026-09-14

## Evidence boundary

Historical golden-set head `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4` had merge base `51c7df6d03f072449422fd58ca24b2f9d6026f07`. Canonical Research Intake #9 is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`. Before repair, comparison was 35 commits ahead / 81 behind and GitHub reported #10 non-mergeable.

Review `5192181682` restated the causal compatibility finding: keep current #9's private `ClassificationReport` and public three-field `ZoteroItem`, move raw provider evidence to an explicit capture/wire boundary, and bind golden approval to every material current-#9 report field without reopening trusted state.

Ordinary merge `c9954e286041c80d08655e092f10007a684ceab1` has parents historical #10 `fdf8b8d70...` and current #9 `a67d9d66...`. Fresh ancestry after that integration is ahead of #9 with zero commits behind; GitHub recomputed #10 as mechanically mergeable. This proves parent adoption only. It does not transfer predecessor checks or authorize merge.

## Semantic reconciliation

The old `ZoteroItem.source_record` design was rejected. Current #9's source shape stays `{ key, version, data }`. `CapturedZoteroItem` instead owns the raw provider JSON and its decoded typed item privately. Provider and typed-fixture receipts use separate SHA-256 domains so a fixture receipt cannot be presented as provider-authenticated evidence.

The old evaluator's direct `ClassificationReport` field access was rejected. `GoldenSnapshot`, `classification_proposal_digest`, structural validation and evaluation all consume current #9 read-only accessors. Existing compile-fail/private aggregate behavior therefore remains in the adopted parent rather than being counteracted by the child.

Historical corruption tests that mutated public report internals were replaced by externally reachable RED cases: duplicate source identity, changed same-revision source content, changed unreviewed proposal/source evidence, stale or locally rewritten receipts, invalid labels, and malformed provider capture. States that only exist after bypassing the trusted constructor are no longer made reachable solely for testing.

## Receipt compatibility

Provider snapshot domain: `conceptweave-zotero-provider-snapshot-v3`.

Typed fixture snapshot domain: `conceptweave-zotero-typed-snapshot-v3`.

Proposal domain: `conceptweave-classification-proposals-v3`.

v1/v2 proposal and v2 snapshot receipts are historical evidence. They require fresh report generation and independent approval; local digest recomputation is not approval migration.

## Acceptance status

The ordinary integration and source repair are not exact-head GREEN. The execution environment used for this reconciliation has no usable Rust toolchain, and no repository-owned PR workflow generation was present on the first reconciled head. CodeRabbit status alone is not sufficient acceptance. Keep #10 Draft until unchanged-head Rust 1.98 and hosted/review evidence are established.
