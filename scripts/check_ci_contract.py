"""Fail closed when Product CI regresses queue admission or pinned validation."""

from __future__ import annotations

from pathlib import Path


WORKFLOW_PATH = Path(".github/workflows/product.yml")
PACKAGE_JSON_PATH = Path("package.json")
PACKAGE_LOCK_PATH = Path("package-lock.json")


def main() -> int:
    """Validate Product CI ownership, Draft execution, and deterministic tooling."""
    workflow = WORKFLOW_PATH.read_text(encoding="utf-8")

    required_fragments = (
        "runs-on: ubuntu-24.04",
        "types: [opened, synchronize, reopened, ready_for_review, converted_to_draft, closed]",
        "group: ${{ github.workflow }}-${{ github.repository }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}",
        "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
        "if: ${{ github.event_name != 'pull_request' || github.event.action != 'closed' }}",
        "actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
        "RUSTUP_TOOLCHAIN: 1.98.1",
        'rustup toolchain install "$RUSTUP_TOOLCHAIN" --profile minimal --component rustfmt --component clippy',
        'test "$(rustc --version | awk \'{print $2}\')" = "$RUSTUP_TOOLCHAIN"',
        'rustup show active-toolchain | grep -Eq "^${RUSTUP_TOOLCHAIN}-"',
        "actions/setup-node@820762786026740c76f36085b0efc47a31fe5020",
        "node-version: '24.21.0'",
        "package-manager-cache: false",
        "COVERAGE_TOOLCHAIN: nightly-2026-08-20",
        'rustup toolchain install "$COVERAGE_TOOLCHAIN" --profile minimal --component llvm-tools-preview',
        "npm ci --ignore-scripts --no-audit --no-fund",
        "npm run check:json-contracts",
        "git ls-files --error-unmatch package-lock.json",
    )
    missing = [fragment for fragment in required_fragments if fragment not in workflow]
    if missing:
        raise SystemExit(
            "Product CI contract missing required fragment(s): " + ", ".join(missing)
        )

    if not PACKAGE_JSON_PATH.is_file() or not PACKAGE_LOCK_PATH.is_file():
        raise SystemExit("Product CI requires tracked package.json and package-lock.json")

    forbidden_fragments = (
        "runs-on: ubuntu-latest",
        "github.event.pull_request.number || github.ref",
        "github.event.pull_request.draft == false",
        "npx --yes",
        "uses: actions/setup-node@v",
        "node-version: lts/",
        "node-version: latest",
        "RUSTUP_TOOLCHAIN: stable",
        "RUSTUP_TOOLCHAIN: 1.98.0",
        "rustup default stable",
    )
    forbidden = [fragment for fragment in forbidden_fragments if fragment in workflow]
    if forbidden:
        raise SystemExit(
            "Product CI contract contains forbidden fragment(s): " + ", ".join(forbidden)
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
