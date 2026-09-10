import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { sha256Digest, verifyManifestSourceProvenance } from "../verify_semantic_engineering_provenance.mjs";

function initializeRepository(prefix) {
  const root = mkdtempSync(join(tmpdir(), prefix));
  const git = (...args) => execFileSync("git", ["-C", root, ...args], { encoding: "utf8" }).trim();
  git("init", "-q", "-b", "main");
  git("config", "user.email", "provenance-test@example.invalid");
  git("config", "user.name", "ConceptWeave provenance test");
  return { root, git };
}

test("manifest provenance rejects a Git symlink blob as source content", () => {
  const { root, git } = initializeRepository("conceptweave-provenance-symlink-");
  try {
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

test("manifest provenance admits regular Git file modes 100644 and 100755", () => {
  const { root, git } = initializeRepository("conceptweave-provenance-regular-");
  try {
    writeFileSync(join(root, "design.md"), "regular design source\n");
    writeFileSync(join(root, "generator.sh"), "#!/bin/sh\nexit 0\n");
    const designBlob = git("hash-object", "-w", "design.md");
    const generatorBlob = git("hash-object", "-w", "generator.sh");
    git("update-index", "--add", "--cacheinfo", `100644,${designBlob},design.md`);
    git("update-index", "--add", "--cacheinfo", `100755,${generatorBlob},generator.sh`);
    git("commit", "-q", "-m", "regular source fixtures");
    const commit = git("rev-parse", "HEAD");

    const manifest = {
      sources: [
        {
          source_id: "regular_design",
          repository: "ContextualWisdomLab/ConceptWeave",
          commit_sha: commit,
          path: "design.md",
          git_blob_sha: designBlob,
          content_digest: sha256Digest(readFileSync(join(root, "design.md"))),
        },
        {
          source_id: "regular_generator",
          repository: "ContextualWisdomLab/ConceptWeave",
          commit_sha: commit,
          path: "generator.sh",
          git_blob_sha: generatorBlob,
          content_digest: sha256Digest(readFileSync(join(root, "generator.sh"))),
        },
      ],
    };

    const result = verifyManifestSourceProvenance(root, manifest);
    assert.equal(result.sources_verified, 2);
    assert.ok(result.source_bytes_verified > 0);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
