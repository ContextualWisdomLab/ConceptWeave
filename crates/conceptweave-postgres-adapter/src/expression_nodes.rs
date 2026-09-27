//! Bounded reader for PostgreSQL 18's catalog expression serialization.
//!
//! Catalog OIDs are transient lookup keys, never governed semantic identities. This reader only
//! separates node fields and locates I/O coercion types; catalog validation remains the adapter's
//! responsibility. Grammar follows PostgreSQL 18 `outfuncs.c`, including escaped tokens and Datum
//! byte arrays. Unknown coercion argument types fail rather than borrowing a nested child's type.

use std::collections::BTreeMap;

use super::{CaptureMeter, bounded, field};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use tokio_postgres::{Transaction, types::ToSql};

/// Validates parsed I/O coercions against the same transaction's catalog generation.
pub(super) async fn validate_io_coercions(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    schema_oid: u32,
) -> Result<(), SourceObservationFailure> {
    let max_bytes = request.request().limits().max_bytes().min(i64::MAX as u64) as i64;
    let stream = bounded(
        request,
        cancellation,
        transaction.query_raw(
            "WITH trees AS ( \
         SELECT ad.adbin::text AS tree FROM pg_catalog.pg_attrdef ad \
         JOIN pg_catalog.pg_class c ON c.oid = ad.adrelid WHERE c.relnamespace = $1 \
         UNION ALL SELECT t.typdefaultbin::text FROM pg_catalog.pg_type t \
         WHERE t.typnamespace = $1 AND t.typtype = 'd' AND t.typdefaultbin IS NOT NULL \
         UNION ALL SELECT c.conbin::text FROM pg_catalog.pg_constraint c \
         WHERE c.connamespace = $1 AND c.conbin IS NOT NULL \
         UNION ALL SELECT i.indexprs::text FROM pg_catalog.pg_index i \
         JOIN pg_catalog.pg_class c ON c.oid = i.indexrelid \
         WHERE c.relnamespace = $1 AND i.indexprs IS NOT NULL \
         UNION ALL SELECT i.indpred::text FROM pg_catalog.pg_index i \
         JOIN pg_catalog.pg_class c ON c.oid = i.indexrelid \
         WHERE c.relnamespace = $1 AND i.indpred IS NOT NULL \
         ) SELECT CASE WHEN octet_length(tree)::bigint <= $2 THEN tree END FROM trees \
         WHERE pg_catalog.strpos(tree, '{COERCEVIAIO ') > 0",
            vec![&schema_oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut pairs = std::collections::BTreeSet::new();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let tree = field::<Option<String>>(&row, 0)?.ok_or(
            SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            },
        )?;
        meter.add(request, tree.len())?;
        pairs.extend(
            io_coercion_pairs(&tree)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
    }
    if pairs.is_empty() {
        return Ok(());
    }
    let (sources, targets): (Vec<u32>, Vec<u32>) = pairs.into_iter().unzip();
    let row = bounded(
        request,
        cancellation,
        transaction.query_one(
            "SELECT count(*) FROM unnest($1::oid[], $2::oid[]) AS pair(source_oid, target_oid) \
         JOIN pg_catalog.pg_type source_type ON source_type.oid = pair.source_oid \
         JOIN pg_catalog.pg_type target_type ON target_type.oid = pair.target_oid \
         JOIN pg_catalog.pg_proc output_fn ON output_fn.oid = source_type.typoutput \
         JOIN pg_catalog.pg_proc input_fn ON input_fn.oid = target_type.typinput \
         WHERE source_type.oid < 16384::oid AND target_type.oid < 16384::oid \
         AND source_type.typnamespace = 'pg_catalog'::regnamespace \
         AND target_type.typnamespace = 'pg_catalog'::regnamespace \
         AND output_fn.oid < 16384::oid AND input_fn.oid < 16384::oid \
         AND output_fn.pronamespace = 'pg_catalog'::regnamespace \
         AND input_fn.pronamespace = 'pg_catalog'::regnamespace \
         AND output_fn.provolatile = 'i' AND input_fn.provolatile = 'i'",
            &[&sources, &targets],
        ),
    )
    .await?;
    meter.add(request, 8)?;
    if field::<i64>(&row, 0)? as usize != sources.len() {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    Ok(())
}

const MAX_DEPTH: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InvalidExpressionTree;

#[derive(Debug)]
enum Value<'a> {
    Atom(&'a str),
    List(Vec<Self>),
    Node(&'a str, BTreeMap<&'a str, Self>),
    Datum,
}

/// Reads all I/O coercions and the exact type of each coercion's immediate argument.
///
/// The caller must bound input bytes before invoking this function. Nesting is independently
/// bounded here. Returned OIDs must be resolved within the same captured catalog transaction.
fn io_coercion_pairs(tree: &str) -> Result<Vec<(u32, u32)>, InvalidExpressionTree> {
    let mut reader = Reader {
        input: tree,
        offset: 0,
    };
    let root = reader.value(0)?;
    if reader.peek().is_some() {
        return Err(InvalidExpressionTree);
    }
    let mut pairs = Vec::new();
    collect_pairs(&root, &mut pairs)?;
    Ok(pairs)
}

fn collect_pairs(
    value: &Value<'_>,
    pairs: &mut Vec<(u32, u32)>,
) -> Result<(), InvalidExpressionTree> {
    match value {
        Value::Node(kind, fields) => {
            if *kind == "COERCEVIAIO" {
                pairs.push((
                    result_type(fields.get("arg").ok_or(InvalidExpressionTree)?)?,
                    oid(fields, "resulttype")?,
                ));
            }
            for child in fields.values() {
                collect_pairs(child, pairs)?;
            }
        }
        Value::List(values) => {
            for child in values {
                collect_pairs(child, pairs)?;
            }
        }
        Value::Atom(_) | Value::Datum => {}
    }
    Ok(())
}

fn result_type(value: &Value<'_>) -> Result<u32, InvalidExpressionTree> {
    let Value::Node(kind, fields) = value else {
        return Err(InvalidExpressionTree);
    };
    let field = match *kind {
        "VAR" => "vartype",
        "CONST" => "consttype",
        "FUNCEXPR" => "funcresulttype",
        "OPEXPR" | "DISTINCTEXPR" | "NULLIFEXPR" => "opresulttype",
        "RELABELTYPE" | "COERCEVIAIO" | "ARRAYCOERCEEXPR" | "COERCETODOMAIN" => "resulttype",
        "CASEEXPR" => "casetype",
        "COALESCEEXPR" => "coalescetype",
        "MINMAXEXPR" => "minmaxtype",
        "ARRAYEXPR" => "array_typeid",
        "ROWEXPR" => "row_typeid",
        "CASETESTEXPR" | "COERCETODOMAINVALUE" => "typeId",
        "COLLATEEXPR" => return result_type(fields.get("arg").ok_or(InvalidExpressionTree)?),
        "BOOLEXPR" | "NULLTEST" | "BOOLEANTEST" | "SCALARARRAYOPEXPR" => return Ok(16),
        _ => return Err(InvalidExpressionTree),
    };
    oid(fields, field)
}

fn oid(fields: &BTreeMap<&str, Value<'_>>, field: &str) -> Result<u32, InvalidExpressionTree> {
    match fields.get(field) {
        Some(Value::Atom(token)) => token
            .parse::<u32>()
            .ok()
            .filter(|value| *value != 0)
            .ok_or(InvalidExpressionTree),
        _ => Err(InvalidExpressionTree),
    }
}

struct Reader<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> Reader<'a> {
    fn peek(&mut self) -> Option<u8> {
        while self
            .input
            .as_bytes()
            .get(self.offset)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.offset += 1;
        }
        self.input.as_bytes().get(self.offset).copied()
    }

    fn token(&mut self) -> Result<&'a str, InvalidExpressionTree> {
        self.peek().ok_or(InvalidExpressionTree)?;
        let start = self.offset;
        let bytes = self.input.as_bytes();
        while let Some(&byte) = bytes.get(self.offset) {
            if byte == b'\\' {
                self.offset += 1;
                if bytes.get(self.offset).is_none() {
                    return Err(InvalidExpressionTree);
                }
                self.offset += 1;
            } else if byte.is_ascii_whitespace() || b"{}()".contains(&byte) {
                break;
            } else {
                self.offset += 1;
            }
        }
        if self.offset == start {
            return Err(InvalidExpressionTree);
        }
        self.input
            .get(start..self.offset)
            .ok_or(InvalidExpressionTree)
    }

    fn value(&mut self, depth: usize) -> Result<Value<'a>, InvalidExpressionTree> {
        if depth > MAX_DEPTH {
            return Err(InvalidExpressionTree);
        }
        match self.peek().ok_or(InvalidExpressionTree)? {
            b'{' => {
                self.offset += 1;
                let kind = self.token()?;
                if kind.starts_with(':') {
                    return Err(InvalidExpressionTree);
                }
                let mut fields = BTreeMap::new();
                while self.peek() != Some(b'}') {
                    let field = self
                        .token()?
                        .strip_prefix(':')
                        .filter(|name| !name.is_empty())
                        .ok_or(InvalidExpressionTree)?;
                    let value = if field == "constvalue" {
                        self.datum()?
                    } else {
                        self.value(depth + 1)?
                    };
                    if fields.insert(field, value).is_some() {
                        return Err(InvalidExpressionTree);
                    }
                }
                self.offset += 1;
                Ok(Value::Node(kind, fields))
            }
            b'(' => {
                self.offset += 1;
                let mut values = Vec::new();
                while self.peek() != Some(b')') {
                    values.push(self.value(depth + 1)?);
                }
                self.offset += 1;
                Ok(Value::List(values))
            }
            b'}' | b')' => Err(InvalidExpressionTree),
            _ => Ok(Value::Atom(self.token()?)),
        }
    }

    fn datum(&mut self) -> Result<Value<'a>, InvalidExpressionTree> {
        let length = self.token()?;
        if length == "<>" {
            return Ok(Value::Datum);
        }
        length.parse::<usize>().map_err(|_| InvalidExpressionTree)?;
        if self.peek() != Some(b'[') {
            return Err(InvalidExpressionTree);
        }
        self.offset += 1;
        while self.peek() != Some(b']') {
            let byte = self
                .token()?
                .parse::<i16>()
                .map_err(|_| InvalidExpressionTree)?;
            if !(-128..=255).contains(&byte) {
                return Err(InvalidExpressionTree);
            }
        }
        self.offset += 1;
        Ok(Value::Datum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_coercion_uses_its_arguments_own_type_and_rejects_ambiguous_input() {
        let composed = "{COERCEVIAIO :arg {OPEXPR :opresulttype 23 :args ({VAR :vartype 20} {CONST :consttype 23 :constvalue 4 [ 1 0 0 0 0 0 0 0 ]})} :resulttype 25}";
        assert_eq!(io_coercion_pairs(composed), Ok(vec![(23, 25)]));
        assert_eq!(
            io_coercion_pairs(
                "{COERCEVIAIO :arg {COERCEVIAIO :arg {VAR :vartype 23} :resulttype 25} :resulttype 1043}"
            ),
            Ok(vec![(25, 1043), (23, 25)])
        );
        assert_eq!(
            io_coercion_pairs(
                "{NODE :label escaped\\ \u{c774}\u{b984}\\(\\)\\{\\}\\\\ :value :literal :args (o 1 2)}"
            ),
            Ok(vec![])
        );
        assert_eq!(io_coercion_pairs("{NODE :label bracket[name]}"), Ok(vec![]));
        for malformed in [
            "",
            "{VAR :vartype 23",
            "{VAR :vartype 23} trailing",
            "{VAR :vartype 23 :vartype 25}",
            "{COERCEVIAIO :arg {UNKNOWN :args ({VAR :vartype 23})} :resulttype 25}",
            "{COERCEVIAIO :arg {VAR :vartype 0} :resulttype 25}",
            "{COERCEVIAIO :arg {VAR :vartype 4294967296} :resulttype 25}",
            "{CONST :constvalue 1 [ 256 ]}",
            "{CONST :constvalue 1 [ 0}",
            "{NODE :label bad\\",
        ] {
            assert_eq!(
                io_coercion_pairs(malformed),
                Err(InvalidExpressionTree),
                "{malformed}"
            );
        }
        let too_deep = format!(
            "{}<> {}",
            "(".repeat(MAX_DEPTH + 2),
            ")".repeat(MAX_DEPTH + 2)
        );
        assert_eq!(io_coercion_pairs(&too_deep), Err(InvalidExpressionTree));
    }
}
