"""Behavioral tests for exact-base Rust workspace adoption in Product CI."""

from __future__ import annotations

import contextlib
import io
import os
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import check_rust_workspace_adoption as adoption


class RustWorkspaceAdoptionTests(unittest.TestCase):
    """Exercise first adoption, absence, deletion, and invalid-base boundaries."""

    def setUp(self) -> None:
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.repository_root = Path(self.temporary_directory.name)
        self._git("init", "--quiet")
        self._git("config", "user.name", "ConceptWeave CI")
        self._git("config", "user.email", "ci@example.invalid")
        self._git("commit", "--allow-empty", "--quiet", "-m", "base")
        self.empty_base_sha = self._git("rev-parse", "HEAD").stdout.strip()
        self.original_repository_root = adoption.REPOSITORY_ROOT
        adoption.REPOSITORY_ROOT = self.repository_root

    def tearDown(self) -> None:
        adoption.REPOSITORY_ROOT = self.original_repository_root
        self.temporary_directory.cleanup()

    def _git(self, *arguments: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["git", *arguments],
            cwd=self.repository_root,
            check=True,
            capture_output=True,
            text=True,
        )

    def _run(self, base_sha: str) -> tuple[int, str]:
        output = io.StringIO()
        with patch.dict(os.environ, {"BASE_SHA": base_sha}, clear=False):
            with contextlib.redirect_stdout(output):
                result = adoption.main()
        return result, output.getvalue()

    def _write_complete_workspace(self) -> None:
        (self.repository_root / "Cargo.toml").write_text(
            '[workspace]\nmembers = []\nresolver = "2"\n', encoding="utf-8"
        )
        (self.repository_root / "Cargo.lock").write_text(
            "# generated\nversion = 4\n", encoding="utf-8"
        )

    def test_absent_workspace_is_not_adopted(self) -> None:
        self.assertEqual(self._run(self.empty_base_sha), (0, "adopted=false\n"))

    def test_first_complete_workspace_adoption_is_enabled(self) -> None:
        self._write_complete_workspace()
        self.assertEqual(self._run(self.empty_base_sha), (0, "adopted=true\n"))

    def test_explicit_candidate_root_controls_adoption(self) -> None:
        self._write_complete_workspace()
        with patch.dict(
            os.environ,
            {"PRODUCT_CANDIDATE_ROOT": str(self.repository_root)},
            clear=False,
        ):
            self.assertEqual(self._run(self.empty_base_sha), (0, "adopted=true\n"))

    def test_partial_first_adoption_fails_closed(self) -> None:
        (self.repository_root / "Cargo.toml").write_text(
            '[workspace]\nmembers = []\n', encoding="utf-8"
        )
        with self.assertRaisesRegex(SystemExit, "rust_workspace_incomplete:Cargo.lock"):
            self._run(self.empty_base_sha)

    def test_deleting_adopted_lockfile_fails_closed(self) -> None:
        self._write_complete_workspace()
        self._git("add", "Cargo.toml", "Cargo.lock")
        self._git("commit", "--quiet", "-m", "adopt workspace")
        adopted_base_sha = self._git("rev-parse", "HEAD").stdout.strip()
        (self.repository_root / "Cargo.lock").unlink()
        with self.assertRaisesRegex(SystemExit, "rust_workspace_incomplete:Cargo.lock"):
            self._run(adopted_base_sha)

    def test_invalid_base_sha_fails_closed(self) -> None:
        with self.assertRaisesRegex(SystemExit, "rust_workspace_base_sha_invalid"):
            self._run("main")

    def test_unavailable_base_commit_fails_closed(self) -> None:
        unavailable_sha = "0" * 40
        with self.assertRaisesRegex(SystemExit, "rust_workspace_base_commit_unavailable"):
            self._run(unavailable_sha)


if __name__ == "__main__":
    unittest.main()
