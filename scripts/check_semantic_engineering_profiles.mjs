import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createProceduralValidators } from "./check_procedural_contracts.mjs";
import { parseJsonRejectDuplicateKeys } from "./parse_strict_json.mjs";
import { verifyManifestSourceProvenance } from "./verify_semantic_engineering_provenance.mjs";

// CI-only reuse of the parent's locked validator; no new validation engine or registry install.
const root = new URL("../", import.meta.url);
const repositoryRoot = fileURLToPath(root);
const manifestPath = "profiles/semantic_engineering/profile_manifest.json";
const manifest = parseJsonRejectDuplicateKeys(
  readFileSync(new URL(manifestPath, root), "utf8"),
  manifestPath,
);
const provenance = verifyManifestSourceProvenance(repositoryRoot, manifest);
const { model: validate } = createProceduralValidators(repositoryRoot);
for (const file of ["semantic_authoring.draft.json", "procedural_refinement.draft.json"]) {
  const path = `profiles/semantic_engineering/${file}`;
  const input = parseJsonRejectDuplicateKeys(readFileSync(new URL(path, root), "utf8"), path);
  if (!validate(input)) throw new Error(`invalid_semantic_engineering_profile:${file}`);
}
console.log(JSON.stringify({
  profiles_checked: 2,
  git_sources_verified: provenance.sources_verified,
  source_bytes_verified: provenance.source_bytes_verified,
  validation_scope: "strict_json_input_shape_plus_local_git_source_provenance",
  activation_authorized: false,
}));
