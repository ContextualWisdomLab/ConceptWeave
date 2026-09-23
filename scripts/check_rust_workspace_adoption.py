"""Stage Rust Product checks on the exact pull-request base workspace contract."""

from __future__ import annotations

import os
import re
import subprocess
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
REQUIRED_WORKSPACE_PATHS = ("Cargo.toml", "Cargo.lock")


def _candidate_repository_root() -> Path:
    """Return the candidate checkout selected by the trusted Product workflow."""
    candidate_root = os.environ.get("PRODUCT_CANDIDATE_ROOT")
    return Path(candidate_root).resolve() if candidate_root else REPOSITORY_ROOT


def _git_object_exists(repository_root: Path, object_name: str) -> bool:
    """Return whether an exact Git object/path exists without reading mutable refs."""
    result = subprocess.run(
        ["git", "cat-file", "-e", object_name],
        cwd=repository_root,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    return result.returncode == 0


def main() -> int:
    """Emit a GitHub Actions output for exact-base Rust workspace adoption."""
    base_sha = os.environ.get("BASE_SHA")
    if not base_sha or re.fullmatch(r"[0-9a-f]{40}", base_sha) is None:
        raise SystemExit("rust_workspace_base_sha_invalid")

    repository_root = _candidate_repository_root()
    if not _git_object_exists(repository_root, f"{base_sha}^{{commit}}"):
        raise SystemExit("rust_workspace_base_commit_unavailable")

    base_has_workspace_path = any(
        _git_object_exists(repository_root, f"{base_sha}:{path}")
        for path in REQUIRED_WORKSPACE_PATHS
    )
    head_has_workspace_path = any(
        (repository_root / path).exists() for path in REQUIRED_WORKSPACE_PATHS
    )

    if not base_has_workspace_path and not head_has_workspace_path:
        print("adopted=false")
        return 0

    missing_head_paths = [
        path
        for path in REQUIRED_WORKSPACE_PATHS
        if not (repository_root / path).is_file()
    ]
    if missing_head_paths:
        raise SystemExit(
            "rust_workspace_incomplete:" + ",".join(missing_head_paths)
        )

    print("adopted=true")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
