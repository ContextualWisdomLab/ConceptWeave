"""Fail closed when Product CI regresses on runner, Draft execution, or coverage identity."""

from __future__ import annotations

from pathlib import Path


WORKFLOW_PATH = Path(".github/workflows/product.yml")


def main() -> int:
    """Validate queue-admission, Draft execution, supersession, and coverage invariants."""
    workflow = WORKFLOW_PATH.read_text(encoding="utf-8")

    required_fragments = (
        "runs-on: ubuntu-24.04",
        "types: [opened, synchronize, reopened, ready_for_review, converted_to_draft, closed]",
        "group: ${{ github.workflow }}-${{ github.repository }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}",
        "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
        "if: ${{ github.event_name != 'pull_request' || github.event.action != 'closed' }}",
        "actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
        "COVERAGE_TOOLCHAIN: nightly-2026-08-20",
        "image: postgres:18.4@sha256:",
        "- /tmp/conceptweave-pg18:/var/run/postgresql",
        "- /tmp/conceptweave-pg18-tablespaces:/tmp/conceptweave-pg18-tablespaces",
        "CONCEPTWEAVE_PG18_TEST_DSN: host=/tmp/conceptweave-pg18",
        "CONCEPTWEAVE_PG18_TEST_TABLESPACE_DIR: /tmp/conceptweave-pg18-tablespaces/fixture",
        'docker exec --user root "$POSTGRES_CONTAINER_ID" install -d -m 0700 -o postgres -g postgres /tmp/conceptweave-pg18-tablespaces/fixture',
        "cargo clippy --workspace --all-targets --all-features --locked -- -D warnings",
        "cargo test --workspace --all-features --locked -- --test-threads=1",
        "cargo doc --workspace --all-features --no-deps --locked",
        'rustup toolchain install "$COVERAGE_TOOLCHAIN" --profile minimal --component llvm-tools-preview',
        "cargo metadata --locked --format-version 1 > /dev/null",
    )
    missing = [fragment for fragment in required_fragments if fragment not in workflow]
    if missing:
        raise SystemExit(
            "Product CI contract missing required fragment(s): " + ", ".join(missing)
        )

    if "-- --test-threads=1" not in Path("scripts/check_coverage.sh").read_text(encoding="utf-8"):
        raise SystemExit("Product coverage must serialize shared PostgreSQL fixtures")

    if "runs-on: ubuntu-latest" in workflow:
        raise SystemExit(
            "Product CI must not use ubuntu-latest while current organization "
            "evidence demonstrates selective floating-image starvation"
        )

    if "github.event.pull_request.number || github.ref" in workflow:
        raise SystemExit("Product CI must isolate non-PR runs by run_id")

    if "github.event.pull_request.draft == false" in workflow:
        raise SystemExit(
            "Product CI must execute quality gates on Draft pull requests; "
            "Ready is downstream of exact-head GREEN"
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
