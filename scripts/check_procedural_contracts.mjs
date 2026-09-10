import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const forbiddenKeys = new Set(["__proto__", "prototype", "constructor"]);
const schemaFiles = {
  model: "procedural-model-draft.schema.json",
  revision: "procedural-revision-proposal.schema.json",
};
const requireFixture = predicate => {
  if (!predicate) throw new Error("invalid_fixture_protocol");
};

/** Test-only corpus expansion; it never authorizes graph changes or handles product requests. */
export function materializeContractCases(caseManifest, fixtureBases) {
  requireFixture(caseManifest?.validation_scope === "input_shape_only");
  requireFixture(caseManifest.fixture_data_policy === "synthetic_unit_test_only");
  requireFixture(Array.isArray(caseManifest.case_rows) && caseManifest.case_rows.length > 0 && caseManifest.case_rows.length <= 256);
  const caseNames = new Set();
  return caseManifest.case_rows.map(caseRow => {
    requireFixture(typeof caseRow.case_name === "string" && /^[a-z][a-z0-9_]{1,95}$/.test(caseRow.case_name));
    requireFixture(!caseNames.has(caseRow.case_name));
    caseNames.add(caseRow.case_name);
    requireFixture(Object.hasOwn(schemaFiles, caseRow.base_name) && Object.hasOwn(fixtureBases, caseRow.base_name));
    requireFixture(typeof caseRow.shape_valid === "boolean");
    requireFixture(typeof caseRow.semantic_expectation === "string" && caseRow.semantic_expectation.length > 0);
    requireFixture(Array.isArray(caseRow.changes) && caseRow.changes.length <= 16);
    const payload = structuredClone(fixtureBases[caseRow.base_name]);
    for (const changeRecord of caseRow.changes) {
      requireFixture(changeRecord.operation === "set" || changeRecord.operation === "remove");
      requireFixture(Array.isArray(changeRecord.field_path) && changeRecord.field_path.length > 0 && changeRecord.field_path.length <= 16);
      for (const fieldKey of changeRecord.field_path) {
        requireFixture((typeof fieldKey === "string" && fieldKey.length > 0 && !forbiddenKeys.has(fieldKey)) || (Number.isSafeInteger(fieldKey) && fieldKey >= 0));
      }
      let fieldParent = payload;
      for (const fieldKey of changeRecord.field_path.slice(0, -1)) {
        requireFixture(fieldParent !== null && typeof fieldParent === "object" && Object.hasOwn(fieldParent, fieldKey));
        fieldParent = fieldParent[fieldKey];
      }
      requireFixture(fieldParent !== null && typeof fieldParent === "object");
      const finalKey = changeRecord.field_path.at(-1);
      if (changeRecord.operation === "remove") {
        requireFixture(Object.hasOwn(fieldParent, finalKey));
        delete fieldParent[finalKey];
      } else {
        requireFixture(Object.hasOwn(changeRecord, "field_value"));
        Object.defineProperty(fieldParent, finalKey, {
          value: structuredClone(changeRecord.field_value), enumerable: true, writable: true, configurable: true,
        });
      }
    }
    return {...caseRow, payload};
  });
}

/** Uses the existing pinned AJV CLI; expected-invalid cases must be rejected without coercion. */
export function createAjvInvocations(repositoryRoot, temporaryRoot, outputCases) {
  const existingSchema = resolve(repositoryRoot, "contracts/semantic-candidate.schema.json");
  const draftSchema = resolve(repositoryRoot, "contracts", schemaFiles.model);
  const invocations = [];
  for (const baseName of Object.keys(schemaFiles)) {
    const schemaPath = resolve(repositoryRoot, "contracts", schemaFiles[baseName]);
    const commonArguments = ["--spec=draft2020", "-s", schemaPath, "-r", existingSchema];
    if (baseName === "revision") commonArguments.push("-r", draftSchema);
    invocations.push(["compile", ...commonArguments]);
    for (const shapeValid of [true, false]) {
      if (!outputCases.some(item => item.base_name === baseName && item.shape_valid === shapeValid)) continue;
      const groupName = `${baseName}_${shapeValid ? "valid" : "invalid"}`;
      invocations.push(["test", ...commonArguments, "-d", join(temporaryRoot, `${groupName}_*.json`), shapeValid ? "--valid" : "--invalid"]);
    }
  }
  return invocations;
}

function runFixtureChecks() {
  const repositoryRoot = fileURLToPath(new URL("../", import.meta.url));
  const readFixture = filename => JSON.parse(readFileSync(resolve(repositoryRoot, "contracts/fixtures", filename), "utf8"));
  const fixtureBases = {model: readFixture("procedural-model.base.json"), revision: readFixture("procedural-revision.base.json")};
  const outputCases = materializeContractCases(readFixture("procedural-authoring.cases.json"), fixtureBases);
  const temporaryRoot = mkdtempSync(join(tmpdir(), "conceptweave_procedural_fixtures_"));
  try {
    for (const caseRow of outputCases) {
      const groupName = `${caseRow.base_name}_${caseRow.shape_valid ? "valid" : "invalid"}`;
      writeFileSync(join(temporaryRoot, `${groupName}_${caseRow.case_name}.json`), JSON.stringify(caseRow.payload), {flag: "wx", mode: 0o600});
    }
    for (const commandArguments of createAjvInvocations(repositoryRoot, temporaryRoot, outputCases)) {
      const result = spawnSync("npx", ["--yes", "ajv-cli@5.0.0", ...commandArguments], {
        cwd: repositoryRoot, stdio: "inherit", shell: false,
      });
      if (result.error || result.status !== 0) throw new Error("procedural_contract_validation_failed");
    }
    console.log(JSON.stringify({validation_scope: "input_shape_only", checked_cases: outputCases.length,
      semantic_gap_witnesses: outputCases.filter(item => item.semantic_expectation !== "not_evaluated").length,
      publication_authorized: false, activation_authorized: false}));
  } finally {
    rmSync(temporaryRoot, {recursive: true, force: true});
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) runFixtureChecks();
