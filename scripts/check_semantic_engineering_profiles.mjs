import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createProceduralValidators } from "./check_procedural_contracts.mjs";

// CI-only reuse of the parent's locked validator; no new validation engine or registry install.
const root = new URL("../", import.meta.url);
const { model: validate } = createProceduralValidators(fileURLToPath(root));
for (const file of ["semantic_authoring.draft.json", "procedural_refinement.draft.json"]) {
  const input = JSON.parse(readFileSync(new URL(`profiles/semantic_engineering/${file}`, root), "utf8"));
  if (!validate(input)) throw new Error(`invalid_semantic_engineering_profile:${file}`);
}
console.log(JSON.stringify({profiles_checked: 2, validation_scope: "input_shape_only", activation_authorized: false}));
