import { createHash } from "node:crypto";

const JSON_WHITESPACE = new Set([" ", "\t", "\r", "\n"]);
const MAX_JSON_BYTES = 2 * 1024 * 1024;
const MAX_JSON_DEPTH = 128;

function duplicateKeyError(label, key) {
  const keyBytes = Buffer.byteLength(key, "utf8");
  const fingerprint = createHash("sha256").update(key, "utf8").digest("hex").slice(0, 16);
  return new Error(`duplicate_json_key:${label}:key_sha256:${fingerprint}:bytes=${keyBytes}`);
}

/**
 * Parses JSON while rejecting duplicate decoded object member names.
 *
 * JSON.parse alone cannot preserve duplicate-member evidence: many runtimes keep the
 * last value. This scanner walks object boundaries first and compares decoded key
 * values, so `"a"` and `"\\u0061"` are treated as the same member name. Duplicate
 * diagnostics expose only a bounded fingerprint and byte length, never the member
 * value itself. JSON.parse remains the syntax/value authority after this pass.
 */
export function parseJsonRejectDuplicateKeys(text, label = "json") {
  if (typeof text !== "string") throw new TypeError("json_text_must_be_string");
  if (typeof label !== "string" || label.length === 0 || label.length > 256) {
    throw new TypeError("json_label_invalid");
  }

  if (Buffer.byteLength(text, "utf8") > MAX_JSON_BYTES) throw new RangeError(`json_size_limit_exceeded:${label}`);

  let index = 0;
  const length = text.length;

  const skipWhitespace = () => {
    while (index < length && JSON_WHITESPACE.has(text[index])) index += 1;
  };

  const parseStringToken = () => {
    if (text[index] !== '"') throw new SyntaxError(`json_object_key_expected:${label}`);
    const start = index;
    index += 1;
    while (index < length) {
      const character = text[index];
      if (character === '"') {
        index += 1;
        return JSON.parse(text.slice(start, index));
      }
      if (character === "\\") {
        index += 2;
        continue;
      }
      index += 1;
    }
    throw new SyntaxError(`json_unterminated_string:${label}`);
  };

  const parsePrimitive = () => {
    const start = index;
    while (index < length && !JSON_WHITESPACE.has(text[index]) && !",]}".includes(text[index])) {
      index += 1;
    }
    if (index === start) throw new SyntaxError(`json_value_expected:${label}`);
  };

  const parseValue = depth => {
    if (depth > MAX_JSON_DEPTH) throw new RangeError(`json_depth_limit_exceeded:${label}`);
    skipWhitespace();
    if (index >= length) throw new SyntaxError(`json_value_expected:${label}`);
    if (text[index] === "{") return parseObject(depth);
    if (text[index] === "[") return parseArray(depth);
    if (text[index] === '"') {
      parseStringToken();
      return;
    }
    parsePrimitive();
  };

  const parseObject = depth => {
    index += 1;
    skipWhitespace();
    const keys = new Set();
    if (text[index] === "}") {
      index += 1;
      return;
    }
    while (index < length) {
      skipWhitespace();
      const key = parseStringToken();
      if (keys.has(key)) throw duplicateKeyError(label, key);
      keys.add(key);
      skipWhitespace();
      if (text[index] !== ":") throw new SyntaxError(`json_colon_expected:${label}`);
      index += 1;
      parseValue(depth + 1);
      skipWhitespace();
      if (text[index] === "}") {
        index += 1;
        return;
      }
      if (text[index] !== ",") throw new SyntaxError(`json_object_separator_expected:${label}`);
      index += 1;
    }
    throw new SyntaxError(`json_unterminated_object:${label}`);
  };

  const parseArray = depth => {
    index += 1;
    skipWhitespace();
    if (text[index] === "]") {
      index += 1;
      return;
    }
    while (index < length) {
      parseValue(depth + 1);
      skipWhitespace();
      if (text[index] === "]") {
        index += 1;
        return;
      }
      if (text[index] !== ",") throw new SyntaxError(`json_array_separator_expected:${label}`);
      index += 1;
    }
    throw new SyntaxError(`json_unterminated_array:${label}`);
  };

  skipWhitespace();
  parseValue(0);
  skipWhitespace();
  if (index !== length) throw new SyntaxError(`json_trailing_content:${label}`);
  return JSON.parse(text);
}
