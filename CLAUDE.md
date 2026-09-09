# CLAUDE.md — ConceptWeave

Follow `AGENTS.md`, `ARCHITECTURE.md`, accepted ADRs, and the organization master context before making changes.

ConceptWeave's core invariant is: **inference is not authority**. Every generated concept, relation, constraint, dimension, measure, or physical mapping must retain evidence and pass the explicit governance lifecycle before publication.

Keep domain logic in bounded domain modules, LLM/provider logic behind ports/adapters, and source/consumer systems independent. Prefer deterministic validation and explicit abstention over plausible unsupported output.

## Research Intake verification runbook

- Use the repository-pinned compiler explicitly when `RUSTUP_TOOLCHAIN` overrides `rust-toolchain.toml`: `rustup run 1.98.0 cargo ...`.
- Run `./scripts/check_coverage.sh` unchanged. A native function miss or a nonempty normalized function/region/branch miss is RED; never add `coverage(off)` or a test seam to the measured production artifact to make it pass.
- If cfg-specific builds emit several mangled records for one function, diagnose exact symbols and source coordinates first. The normalizer may join only equal declaration origins plus crate-hash-normalized function identity, and its synthetic fixture must still keep a different function at the same origin separate.
- For report-write failures, inspect the final path and the exact operation-specific temporary path. Preserve both the primary failure and cleanup failure; after hard-link publication, keep the final report and identify cleanup as post-publication.
- Before updating a stacked PR, re-fetch both branch heads, merge the owner normally, rerun the frozen gates on the new exact head, then inspect the rendered GitHub PR page. A clean local run, Draft bot status, or stored Markdown does not establish protected merge or release.
- Inspect baseline Mermaid output in GitHub, not only its source. If viewer controls cover a node or the viewport clips a tall graph, use `flowchart TB` with bounded `rankSpacing`, reload the exact revision, and confirm every node and edge label remains visible.

Detailed rationale and stable invariants live in `AGENTS.md`; dated SHAs, measurements, failed attempts, and remaining gates live in `docs/product-technical-gap-baseline.md` and the linked doctoring records.
