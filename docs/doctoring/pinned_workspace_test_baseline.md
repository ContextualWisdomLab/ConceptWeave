# Pinned workspace test baseline

- Tested source: `6180a3cc9f19c24de05d0ed1205fac3eb545d9f0`.
- Command: `rustup run 1.98.0 cargo test --workspace --locked --quiet`.
- Execution session: `86857`; observed terminal exit code: 0.
- The Zotero library unit-test suite reported 108 passed, zero failed, zero ignored, zero filtered, in 859.77 seconds. Subsequent workspace suites and doctests completed before the terminal zero exit. No aggregate test count is asserted because intermediate displayed output was truncated.
- During execution, a one-second process sample located active work in capture verification, JSON serialization and SHA-256 hashing. The large escaped-text tests exercise the real 16 MiB review boundary and reserved decision growth; input reduction would change their coverage. This sample identifies an investigation path, not a measured optimization result.
- Local execution proves neither hosted required checks nor release, external approval or Zotero classification. Earlier complete-test claims without terminal execution evidence must not substitute for this observation.

## Discarded bounded-buffer experiment

An uncommitted experiment on `f3099c4` wrapped the shared JSON digest writer in an 8 KiB standard-library buffer and explicitly flushed before finalization. A regression test compared hashes with independently serialized JSON containing escapes and Unicode across multiple buffer writes. Independent static review found no concrete correctness defect; repetition counts were not exact serialized-byte boundary lengths.

`rustup run 1.98.0 cargo test --locked -p conceptweave-zotero --lib --quiet` completed in session `95131` with exit 0: 109 passed, zero failed/ignored/measured/filtered, 988.78 seconds. The previous library observation was 859.77 seconds with 108 tests. Host contention and the added test prevent a controlled causal comparison; no performance improvement is established.

Disposition: discarded both experimental source changes with a targeted patch. The worktree was confirmed clean afterward. No fixture was reduced, public contract changed, or buffered implementation published. This attempt was not committed before measurement, contrary to the experiment protocol; future experiments must record a candidate commit before running. The next performance experiment requires a repeatable paired measurement rather than another full-suite timing inference.
