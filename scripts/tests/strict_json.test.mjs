import assert from "node:assert/strict";
import { test } from "node:test";
import { parseJsonRejectDuplicateKeys } from "../parse_strict_json.mjs";

test("valid JSON retains normal JSON.parse semantics", () => {
  const value = parseJsonRejectDuplicateKeys('{"model_id":"model_one","nested":{"a":1},"items":[true,null,"x"]}', "valid");
  assert.deepEqual(value, {model_id: "model_one", nested: {a: 1}, items: [true, null, "x"]});
});

test("top-level duplicate member is rejected before last-wins parsing", () => {
  assert.throws(
    () => parseJsonRejectDuplicateKeys('{"model_id":"trusted","model_id":"shadow"}', "profile"),
    /duplicate_json_key:profile:/,
  );
});

test("nested duplicate member is rejected", () => {
  assert.throws(
    () => parseJsonRejectDuplicateKeys('{"scope":{"tenant_ref":"trusted","tenant_ref":"shadow"}}', "profile"),
    /duplicate_json_key:profile:/,
  );
});

test("escaped-equivalent member names are one decoded identity", () => {
  assert.throws(
    () => parseJsonRejectDuplicateKeys('{"scope":{"a":1,"\\u0061":2}}', "profile"),
    /duplicate_json_key:profile:/,
  );
});

test("duplicate-key diagnostics do not echo attacker-sized member names", () => {
  const key = "s".repeat(900_000);
  const input = `{"${key}":1,"${key}":2}`;
  let failure;
  try {
    parseJsonRejectDuplicateKeys(input, "profile");
  } catch (error) {
    failure = error;
  }
  assert.ok(failure instanceof Error);
  assert.match(failure.message, /^duplicate_json_key:profile:key_sha256:[0-9a-f]{16}:bytes=900000$/);
  assert.ok(Buffer.byteLength(failure.message, "utf8") <= 128);
  assert.equal(failure.message.includes(key.slice(0, 128)), false);
});

test("ordinary JSON syntax errors still fail closed", () => {
  assert.throws(() => parseJsonRejectDuplicateKeys('{"a":1} trailing', "profile"));
  assert.throws(() => parseJsonRejectDuplicateKeys('{"a":}', "profile"));
});

test("excessive nesting and input size fail before parsing", () => {
  const tooDeep = "[".repeat(130) + "0" + "]".repeat(130);
  assert.throws(() => parseJsonRejectDuplicateKeys(tooDeep, "profile"), /json_depth_limit_exceeded:profile/);
  assert.throws(() => parseJsonRejectDuplicateKeys(`"${"x".repeat(2 * 1024 * 1024)}"`, "profile"), /json_size_limit_exceeded:profile/);
});
