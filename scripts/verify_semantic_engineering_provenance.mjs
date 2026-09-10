import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { realpathSync } from "node:fs";

const SOURCE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const HEX40 = /^[0-9a-f]{40}$/;
const SHA256 = /^sha256:[0-9a-f]{64}$/;
const SAFE_PATH = /^[A-Za-z0-9._/-]{1,512}$/;
const DEFAULT_MAX_SOURCE_BYTES = 2 * 1024 * 1024;
const DEFAULT_GIT_TIMEOUT_MS = 5000;

function validateSourceShape(source, seen) {
  if (!SOURCE_ID.test(source.source_id ?? "") || seen.has(source.source_id)) {
    throw new Error(`source_id_invalid_or_duplicate:${source.source_id ?? "unknown"}`);
  }
  seen.add(source.source_id);
  if (source.repository !== "ContextualWisdomLab/ConceptWeave") {
    throw new Error(`source_repository_not_owned:${source.source_id}`);
  }
  if (!HEX40.test(source.commit_sha ?? "")) {
    throw new Error(`source_commit_invalid:${source.source_id}`);
  }
  if (!HEX40.test(source.git_blob_sha ?? "")) {
    throw new Error(`source_git_blob_invalid:${source.source_id}`);
  }
  if (!SHA256.test(source.content_digest ?? "")) {
    throw new Error(`source_content_digest_invalid:${source.source_id}`);
  }
  const path = source.path ?? "";
  const parts = path.split("/");
  if (!SAFE_PATH.test(path) || path.startsWith("/") || path.endsWith("/") || path.includes("//")
      || parts.some(part => part === "." || part === "..")) {
    throw new Error(`source_path_invalid:${source.source_id}`);
  }
}

function cleanGitEnvironment() {
  const environment = {};
  for (const [key, value] of Object.entries(process.env)) {
    if (!key.startsWith("GIT_")) environment[key] = value;
  }
  environment.GIT_OPTIONAL_LOCKS = "0";
  environment.GIT_CONFIG_NOSYSTEM = "1";
  environment.GIT_TERMINAL_PROMPT = "0";
  return environment;
}

function git(repositoryRoot, args, { maxBuffer = 256 * 1024, timeout = DEFAULT_GIT_TIMEOUT_MS } = {}) {
  return execFileSync(
    "git",
    ["--no-replace-objects", "-C", repositoryRoot, ...args],
    {
      encoding: null,
      env: cleanGitEnvironment(),
      maxBuffer,
      timeout,
      stdio: ["ignore", "pipe", "pipe"],
    },
  );
}

function gitText(repositoryRoot, args, errorCode, options) {
  try {
    return git(repositoryRoot, args, options).toString("utf8").trim();
  } catch {
    throw new Error(errorCode);
  }
}

export function sha256Digest(bytes) {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

export function verifyManifestSourceProvenance(repositoryRoot, manifest, options = {}) {
  if (!Array.isArray(manifest.sources) || manifest.sources.length === 0 || manifest.sources.length > 64) {
    throw new Error("source_collection_limit");
  }
  const maxSourceBytes = options.maxSourceBytes ?? DEFAULT_MAX_SOURCE_BYTES;
  const gitTimeoutMs = options.gitTimeoutMs ?? DEFAULT_GIT_TIMEOUT_MS;
  if (!Number.isSafeInteger(maxSourceBytes) || maxSourceBytes < 1 || maxSourceBytes > DEFAULT_MAX_SOURCE_BYTES) {
    throw new Error("source_size_limit_invalid");
  }
  if (!Number.isSafeInteger(gitTimeoutMs) || gitTimeoutMs < 100 || gitTimeoutMs > DEFAULT_GIT_TIMEOUT_MS) {
    throw new Error("source_git_timeout_invalid");
  }

  const root = realpathSync(repositoryRoot);
  const gitTopLevel = realpathSync(gitText(root, ["rev-parse", "--show-toplevel"], "source_repository_unavailable", { timeout: gitTimeoutMs }));
  if (gitTopLevel !== root) throw new Error("source_repository_root_mismatch");
  const seen = new Set();
  let sourceBytesVerified = 0;

  for (const source of manifest.sources) {
    validateSourceShape(source, seen);
    const id = source.source_id;
    gitText(root, ["cat-file", "-e", `${source.commit_sha}^{commit}`], `source_commit_unavailable:${id}`, { timeout: gitTimeoutMs });
    try {
      git(root, ["merge-base", "--is-ancestor", source.commit_sha, "HEAD"], { timeout: gitTimeoutMs });
    } catch {
      throw new Error(`source_commit_not_in_head_history:${id}`);
    }
    const blob = gitText(root, ["rev-parse", "--verify", `${source.commit_sha}:${source.path}`], `source_path_unavailable:${id}`, { timeout: gitTimeoutMs });
    if (!HEX40.test(blob)) throw new Error(`source_git_blob_invalid:${id}`);
    const objectType = gitText(root, ["cat-file", "-t", blob], `source_git_blob_unavailable:${id}`, { timeout: gitTimeoutMs });
    if (objectType !== "blob") throw new Error(`source_object_not_blob:${id}`);
    if (blob !== source.git_blob_sha) throw new Error(`source_git_blob_mismatch:${id}`);

    const sizeText = gitText(root, ["cat-file", "-s", blob], `source_git_blob_unavailable:${id}`, { timeout: gitTimeoutMs });
    const size = Number.parseInt(sizeText, 10);
    if (!Number.isSafeInteger(size) || size < 0) throw new Error(`source_size_invalid:${id}`);
    if (size > maxSourceBytes) throw new Error(`source_size_limit_exceeded:${id}`);
    if (sourceBytesVerified > DEFAULT_MAX_SOURCE_BYTES * 64 - size) throw new Error("source_total_size_limit_exceeded");

    let bytes;
    try {
      bytes = git(root, ["cat-file", "blob", blob], { maxBuffer: maxSourceBytes + 1, timeout: gitTimeoutMs });
    } catch {
      throw new Error(`source_git_blob_unavailable:${id}`);
    }
    if (bytes.length !== size) throw new Error(`source_size_changed:${id}`);
    if (sha256Digest(bytes) !== source.content_digest) throw new Error(`source_content_digest_mismatch:${id}`);
    sourceBytesVerified += bytes.length;
  }

  return {
    sources_verified: manifest.sources.length,
    source_bytes_verified: sourceBytesVerified,
    verification_scope: "local_git_commit_path_blob_and_sha256",
    source_authentication_established: true,
  };
}
