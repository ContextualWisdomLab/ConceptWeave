# Pinned workspace test baseline

- Tested source: `6180a3cc9f19c24de05d0ed1205fac3eb545d9f0`.
- Command: `rustup run 1.98.0 cargo test --workspace --locked --quiet`.
- Execution session: `86857`; observed terminal exit code: 0.
- The Zotero library unit-test suite reported 108 passed, zero failed, zero ignored, zero filtered, in 859.77 seconds. Subsequent workspace suites and doctests completed before the terminal zero exit. No aggregate test count is asserted because intermediate displayed output was truncated.
- During execution, a one-second process sample located active work in capture verification, JSON serialization and SHA-256 hashing. The large escaped-text tests exercise the real 16 MiB review boundary and reserved decision growth; input reduction would change their coverage. This sample identifies an investigation path, not a measured optimization result.
- Local execution proves neither hosted required checks nor release, external approval or Zotero classification. Earlier complete-test claims without terminal execution evidence must not substitute for this observation.
