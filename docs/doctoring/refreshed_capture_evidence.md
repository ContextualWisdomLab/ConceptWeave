# Refreshed private capture evidence

September 8, 2026; source `4324400733f262a0d575e3bd6d5c40258fc44f18`. No Zotero writes, labels or approvals were issued.

The complete metadata read (session 70446, exit 0) produced 8,326 observed records, 3,715 paper proposals and four pending sources. Report SHA-256: `8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`. The owner-only report is 8,132,756 bytes, outside the repository. Its snapshot digest is `sha256:0666dbebfb0c5aa99deb5a6dda1fc02d84bc46d08aaaddf25f5526a18eceef6d`.

Restoring the September 5 capture against this report failed full verification (session 75306, exit 1), despite matching snapshot identifiers. The exact failing predicate was not isolated. No capture digest was rewritten and no validation was relaxed.

Debug recapture (session 40328) exited 1 with the legacy shared budget error. Release recapture (session 25930) completed with exit 0 using `rustup run 1.98.0 cargo run --release --locked -p conceptweave-zotero -- --capture-full-text REPORT CAPTURE`. Both retained the existing size/time limits. This is an operational outcome under host contention, not a controlled speed benchmark or proof of the debug failure's specific budget.

New capture digest: `sha256:86f9c13043a504c9a6ad8f7ecb6dc6bcf207e9e98f19c5573dd39980844d87c1`. Private file size 235,602,798 bytes, mode 0600. Offline `--full-text-worksheet REPORT CAPTURE WORKSHEET` completed with exit 0 (session 68894), revalidating the binding and creating an owner-only 1,776,452-byte worksheet with 3,715 blank decisions. Capture availability is not semantic review, independent approval, immutable release or publication authority.

Private artifacts use the `/private/tmp/conceptweave-live-refresh-u4T9cx-` prefix and `report.json`, `release-capture.json`, `release-worksheet.json` suffixes. Keep them private; do not publish paper contents or identifiers.

The diagnostic follow-up separates deadline failure from byte-budget failure without changing either limit. Existing exact-boundary test first failed (12870), then passed after the one-line fix (18940); the clock-failure/late-result test passed (51963). Full-suite and independent-review evidence must be collected before claiming complete verification.

Follow-up verification completed: release library suite 18076 exited 0, 108 passed, zero failed/ignored/measured/filtered, 7.45 seconds after compilation. Independent read-only review found no concrete defect and confirmed byte-exhaustion precedence remains unchanged. Formatting and diff checks pass. This verifies the Zotero library suite, not all workspace integration tests, hosted required checks or release admission.

Full workspace verification subsequently completed at `b0f7d5b8bb139d627cec37b598331464c14b60ab`: `rustup run 1.98.0 cargo test --release --workspace --locked --quiet`, session 10400, terminal exit 0. This includes CLI/integration suites and doctests; no aggregate count is asserted because intermediate displayed output was truncated. Hosted required checks, approval and deployment remain separate.

The new 25-row bound review view also completed (session 19376, exit 0), retaining the new capture digest and all 3,715 remaining decisions. The 1,590,859-byte output is mode 0600. No decision was supplied or applied.
