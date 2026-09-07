//! Bounded JSON admission before a review object can discard duplicate keys.

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::fmt;

pub(super) const MAX_REVIEW_JSON_BYTES: usize = 16 * 1024 * 1024;
const INVALID_REVIEW_JSON: &str = "review JSON is invalid";

/// Parses one complete review value without accepting ambiguous object keys.
///
/// The byte limit is inclusive. Invalid syntax, duplicate keys, excessive depth,
/// trailing data, and oversize inputs return one error without source content.
pub(crate) fn parse_review_json(bytes: &[u8]) -> Result<Value, &'static str> {
    if bytes.len() > MAX_REVIEW_JSON_BYTES {
        return Err(INVALID_REVIEW_JSON);
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = UniqueReviewValue
        .deserialize(&mut deserializer)
        .map_err(|_| INVALID_REVIEW_JSON)?;
    deserializer.end().map_err(|_| INVALID_REVIEW_JSON)?;
    Ok(value)
}

/// Builds values recursively while retaining each object's decoded key boundary.
struct UniqueReviewValue;

impl<'de> DeserializeSeed<'de> for UniqueReviewValue {
    type Value = Value;

    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for UniqueReviewValue {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value with unique object keys")
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::from(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::from(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::from(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(UniqueReviewValue)? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = object.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(serde::de::Error::custom("duplicate review key"));
            }
            values.insert(key, object.next_value_seed(UniqueReviewValue)?);
        }
        Ok(Value::Object(values))
    }
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
        let expectation = <serde_json::Error as serde::de::Error>::invalid_type(
            serde::de::Unexpected::Bytes(b""),
            &UniqueReviewValue,
        );
        assert!(
            expectation
                .to_string()
                .contains("a JSON value with unique object keys")
        );
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
