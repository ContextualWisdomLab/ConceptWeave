import { createHash } from "node:crypto";

const SOURCE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const HEX40 = /^[0-9a-f]{40}$/;
const SHA256 = /^sha256:[0-9a-f]{64}$/;
const SAFE_PATH = /^[A-Za-z0-9._/-]{1,512}$/;

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

export function sha256Digest(bytes) {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

export function verifyManifestSourceProvenance(_repositoryRoot, manifest, _options = {}) {
  if (!Array.isArray(manifest.sources) || manifest.sources.length === 0 || manifest.sources.length > 64) {
    throw new Error("source_collection_limit");
  }
  const seen = new Set();
  for (const source of manifest.sources) validateSourceShape(source, seen);
  return {
    sources_verified: manifest.sources.length,
    source_bytes_verified: 0,
    verification_scope: "manifest_shape_only",
    source_authentication_established: false,
  };
}
