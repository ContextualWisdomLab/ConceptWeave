import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { parseJsonRejectDuplicateKeys } from "./parse_strict_json.mjs";

const forbiddenKeys = new Set(["__proto__", "prototype", "constructor"]);
const schemaFiles = {
  model: "procedural-model-draft.schema.json",
  revision: "procedural-revision-proposal.schema.json",
};
const requireFixture = predicate => {
  if (!predicate) throw new Error("invalid_fixture_protocol");
};
const readRepositoryJson = (repositoryRoot, path) => parseJsonRejectDuplicateKeys(
  readFileSync(resolve(repositoryRoot, path), "utf8"),
  path,
);

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

/** Builds in-process Draft 2020-12 validators from repository-owned schemas. */
export function createProceduralValidators(repositoryRoot) {
  const semanticCandidateSchema = readRepositoryJson(repositoryRoot, "contracts/semantic-candidate.schema.json");
  const proceduralModelSchema = readRepositoryJson(repositoryRoot, "contracts/procedural-model-draft.schema.json");
  const proceduralRevisionSchema = readRepositoryJson(repositoryRoot, "contracts/procedural-revision-proposal.schema.json");
  const ajv = new Ajv2020({allErrors: true, strict: true});
  ajv.addSchema(semanticCandidateSchema);
  ajv.addSchema(proceduralModelSchema);
  ajv.addSchema(proceduralRevisionSchema);
  const validators = {
    model: ajv.getSchema(proceduralModelSchema.$id),
    revision: ajv.getSchema(proceduralRevisionSchema.$id),
  };
  requireFixture(typeof validators.model === "function" && typeof validators.revision === "function");
  return validators;
}

/** Evaluates every materialized shape case without coercion, defaults, or external command execution. */
export function validateContractCases(validators, outputCases) {
  for (const caseRow of outputCases) {
    const validator = validators[caseRow.base_name];
    requireFixture(typeof validator === "function");
    const actualValid = validator(caseRow.payload);
    if (actualValid !== caseRow.shape_valid) {
      const details = validator.errors ? JSON.stringify(validator.errors) : "[]";
      throw new Error(`procedural_contract_validation_failed:${caseRow.case_name}:${details}`);
    }
  }
  return {
    validation_scope: "input_shape_only",
    checked_cases: outputCases.length,
    semantic_gap_witnesses: outputCases.filter(item => item.semantic_expectation !== "not_evaluated").length,
    publication_authorized: false,
    activation_authorized: false,
  };
}

function runFixtureChecks() {
  const repositoryRoot = fileURLToPath(new URL("../", import.meta.url));
  const readFixture = filename => readRepositoryJson(repositoryRoot, `contracts/fixtures/${filename}`);
  const fixtureBases = {model: readFixture("procedural-model.base.json"), revision: readFixture("procedural-revision.base.json")};
  const outputCases = materializeContractCases(readFixture("procedural-authoring.cases.json"), fixtureBases);
  const validators = createProceduralValidators(repositoryRoot);
  console.log(JSON.stringify(validateContractCases(validators, outputCases)));
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) runFixtureChecks();
