//! Bounded JSON admission before a review object can discard duplicate keys.

use serde_json::Value;

const MAX_REVIEW_JSON_BYTES: usize = 16 * 1024 * 1024;
const INVALID_REVIEW_JSON: &str = "review JSON is invalid";

/// Parses one complete review value without accepting ambiguous object keys.
///
/// The byte limit is inclusive. Invalid syntax, duplicate keys, excessive depth,
/// trailing data, and oversize inputs return one error without source content.
pub(crate) fn parse_review_json(bytes: &[u8]) -> Result<Value, &'static str> {
    if bytes.len() > MAX_REVIEW_JSON_BYTES {
        return Err(INVALID_REVIEW_JSON);
    }
    serde_json::from_slice(bytes).map_err(|_| INVALID_REVIEW_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_json_rejects_duplicate_decoded_keys_at_every_depth() {
        for bytes in [
            br#"{"decision":null,"decision":true}"#.as_slice(),
            br#"{"decision":null,"\u0064ecision":true}"#,
            br#"{"outer":{"decision":1,"decision":2}}"#,
            br#"[{"decision":1,"decision":2}]"#,
            br#"{"outer":[[{"decision":1,"decision":2}]]}"#,
            br#"{"":null,"":false}"#,
            r#"{"\ud834\udd1e":1,"𝄞":2}"#.as_bytes(),
        ] {
            assert_eq!(parse_review_json(bytes), Err(INVALID_REVIEW_JSON));
        }
    }

    #[test]
    fn review_json_preserves_normal_scalar_array_and_object_semantics() {
        for text in [
            "null",
            "true",
            "false",
            "0",
            "-1",
            "-9223372036854775808",
            "18446744073709551615",
            "1.25",
            "-0.0",
            "6.02e23",
            r#""plain text""#,
            r#""escaped \" text \n \uD834\uDD1E""#,
            r#"{"first":{},"second":[],"third":[null,true,false,-2,3.5,"text"]}"#,
            r#"[{"same":1},{"same":2}]"#,
            r#"{"same":{"same":1},"Same":2}"#,
        ] {
            let expected: Value = serde_json::from_str(text).unwrap();
            assert_eq!(parse_review_json(text.as_bytes()).unwrap(), expected);
        }
    }

    #[test]
    fn review_json_rejects_malformed_trailing_and_excessively_deep_inputs() {
        for bytes in [
            b"".as_slice(),
            b" ",
            b"{}{}",
            b"{} private-sentinel",
            b"[0,]",
            b"{0:true}",
            b"{\"key\":}",
            b"[",
            b"\xff",
            b"NaN",
            b"1e9999",
            br#""\uD800""#,
        ] {
            assert_eq!(parse_review_json(bytes), Err(INVALID_REVIEW_JSON));
        }
        let within_depth = format!("{}null{}", "[".repeat(64), "]".repeat(64));
        assert_eq!(
            parse_review_json(within_depth.as_bytes()).unwrap(),
            serde_json::from_str::<Value>(&within_depth).unwrap(),
        );
        let excessive_depth = format!("{}null{}", "[".repeat(256), "]".repeat(256));
        assert_eq!(
            parse_review_json(excessive_depth.as_bytes()),
            Err(INVALID_REVIEW_JSON)
        );
    }

    #[test]
    fn review_json_accepts_exact_size_limit_and_rejects_one_extra_byte() {
        let mut bytes = b"null".to_vec();
        bytes.resize(MAX_REVIEW_JSON_BYTES, b' ');
        assert_eq!(parse_review_json(&bytes), Ok(Value::Null));
        bytes.push(b' ');
        assert_eq!(parse_review_json(&bytes), Err(INVALID_REVIEW_JSON));
    }
}
