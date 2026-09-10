import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { sha256Digest, verifyManifestSourceProvenance } from "../verify_semantic_engineering_provenance.mjs";

test("manifest provenance rejects a Git symlink blob as source content", () => {
  const root = mkdtempSync(join(tmpdir(), "conceptweave-provenance-symlink-"));
  const git = (...args) => execFileSync("git", ["-C", root, ...args], { encoding: "utf8" }).trim();
  try {
    git("init", "-q", "-b", "main");
    git("config", "user.email", "provenance-test@example.invalid");
    git("config", "user.name", "ConceptWeave provenance test");
    writeFileSync(join(root, "target.md"), "trusted design source\n");
    git("add", "target.md");

    const linkBytes = Buffer.from("target.md");
    const linkBlob = execFileSync("git", ["-C", root, "hash-object", "-w", "--stdin"], { input: linkBytes, encoding: "utf8" }).trim();
    git("update-index", "--add", "--cacheinfo", `120000,${linkBlob},source.md`);
    git("commit", "-q", "-m", "symlink source fixture");
    const commit = git("rev-parse", "HEAD");

    const manifest = {
      sources: [{
        source_id: "symlink_source",
        repository: "ContextualWisdomLab/ConceptWeave",
        commit_sha: commit,
        path: "source.md",
        git_blob_sha: linkBlob,
        content_digest: sha256Digest(linkBytes),
      }],
    };

    assert.throws(
      () => verifyManifestSourceProvenance(root, manifest),
      /source_path_not_regular:symlink_source/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
