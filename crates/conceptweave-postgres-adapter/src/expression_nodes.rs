//! Bounded reader for PostgreSQL 18's catalog expression serialization.
//!
//! Catalog OIDs are transient lookup keys, never governed semantic identities. This reader only
//! separates node fields and locates dependency OIDs; catalog validation remains the adapter's
//! responsibility. Grammar follows PostgreSQL 18 `outfuncs.c`, including escaped tokens and Datum
//! byte arrays. Unknown coercion argument types fail rather than borrowing a nested child's type.

use std::collections::{BTreeMap, BTreeSet};

use super::{CaptureMeter, bounded, field};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use tokio_postgres::{Transaction, types::ToSql};

// Catalog-name policy is conservative; expression fields themselves are read structurally.
const FORBIDDEN_FUNCTION_NAMES: &str = "^(nextval|currval|setval|lastval|pg_.*|to_reg.*|reg.*in|obj_description|col_description|shobj_description|format_type|oidvectortypes|has_.*_privilege|row_security_active)$";

/// Validates parsed dependency fields against the same transaction's catalog generation.
pub(super) async fn validate_dependencies(
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
         ) SELECT CASE WHEN octet_length(tree)::bigint <= $2 THEN tree END FROM trees",
            vec![&schema_oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut dependencies = ExpressionDependencies::default();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let tree = field::<Option<String>>(&row, 0)?.ok_or(
            SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            },
        )?;
        meter.add(request, tree.len())?;
        let captured = expression_dependencies(&tree)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        dependencies.functions.extend(captured.functions);
        dependencies.types.extend(captured.types);
        dependencies
            .subscript_containers
            .extend(captured.subscript_containers);
        dependencies.operators.extend(captured.operators);
        dependencies.io_coercions.extend(captured.io_coercions);
    }
    let functions: Vec<u32> = dependencies.functions.into_iter().collect();
    let types: Vec<u32> = dependencies.types.into_iter().collect();
    let containers: Vec<u32> = dependencies.subscript_containers.into_iter().collect();
    let (operators, cached_functions): (Vec<u32>, Vec<u32>) =
        dependencies.operators.into_iter().unzip();
    let (sources, targets): (Vec<u32>, Vec<u32>) = dependencies.io_coercions.into_iter().unzip();
    // Function ACL material has no source receipt in the current aggregate. Refuse it for
    // every explicit, operator-derived and I/O reference until a versioned observation binds it.
    let row = bounded(
        request,
        cancellation,
        transaction.query_one(
            "WITH subscript_procedure AS ( \
           SELECT t.typsubscript AS oid FROM unnest($8::oid[]) referenced(oid) \
           LEFT JOIN pg_catalog.pg_type t ON t.oid = referenced.oid \
         ), referenced_procedure AS ( \
           SELECT unnest($1::oid[]) AS oid \
           UNION SELECT oid FROM subscript_procedure \
           UNION SELECT o.oprcode FROM unnest($3::oid[]) referenced(oid) \
             JOIN pg_catalog.pg_operator o ON o.oid = referenced.oid \
           UNION SELECT t.typoutput FROM unnest($5::oid[]) referenced(oid) \
             JOIN pg_catalog.pg_type t ON t.oid = referenced.oid \
           UNION SELECT t.typinput FROM unnest($6::oid[]) referenced(oid) \
             JOIN pg_catalog.pg_type t ON t.oid = referenced.oid \
         ) SELECT EXISTS( \
           SELECT 1 FROM referenced_procedure referenced \
           JOIN pg_catalog.pg_proc p ON p.oid = referenced.oid \
           WHERE p.proacl IS NOT NULL \
         ) OR EXISTS( \
           SELECT 1 FROM subscript_procedure referenced \
           LEFT JOIN pg_catalog.pg_proc p ON p.oid = referenced.oid \
           WHERE p.oid IS NULL OR p.provolatile <> 'i' \
             OR p.pronamespace <> 'pg_catalog'::regnamespace OR p.oid >= 16384::oid \
             OR p.proname ~ $7 \
         ) OR EXISTS( \
           SELECT 1 FROM unnest($1::oid[]) f(oid) \
           LEFT JOIN pg_catalog.pg_proc p ON p.oid = f.oid \
           WHERE p.oid IS NULL OR p.provolatile <> 'i' \
             OR p.pronamespace <> 'pg_catalog'::regnamespace OR p.oid >= 16384::oid \
             OR p.proname ~ $7 \
         ) OR EXISTS( \
           SELECT 1 FROM unnest($2::oid[]) referenced(oid) \
           LEFT JOIN pg_catalog.pg_type t ON t.oid = referenced.oid \
           LEFT JOIN pg_catalog.pg_type element ON element.oid = t.typelem \
           WHERE t.oid IS NULL \
             OR (t.typnamespace = 'pg_catalog'::regnamespace AND t.typname ~ '^reg') \
             OR (element.typnamespace = 'pg_catalog'::regnamespace AND element.typname ~ '^reg') \
         ) OR EXISTS( \
           SELECT 1 FROM unnest($3::oid[], $4::oid[]) pair(operator_oid, cached_function_oid) \
           LEFT JOIN pg_catalog.pg_operator o ON o.oid = pair.operator_oid \
           LEFT JOIN pg_catalog.pg_proc p ON p.oid = o.oprcode \
           WHERE o.oid IS NULL OR o.oprnamespace <> 'pg_catalog'::regnamespace \
             OR o.oid >= 16384::oid OR p.oid IS NULL OR p.provolatile <> 'i' \
             OR p.pronamespace <> 'pg_catalog'::regnamespace OR p.oid >= 16384::oid \
             OR p.proname ~ $7 \
             OR (pair.cached_function_oid <> 0::oid AND pair.cached_function_oid <> o.oprcode) \
         ) OR EXISTS( \
           SELECT 1 FROM unnest($5::oid[], $6::oid[]) pair(source_oid, target_oid) \
           LEFT JOIN pg_catalog.pg_type source_type ON source_type.oid = pair.source_oid \
           LEFT JOIN pg_catalog.pg_type target_type ON target_type.oid = pair.target_oid \
           LEFT JOIN pg_catalog.pg_proc output_fn ON output_fn.oid = source_type.typoutput \
           LEFT JOIN pg_catalog.pg_proc input_fn ON input_fn.oid = target_type.typinput \
           WHERE source_type.oid IS NULL OR target_type.oid IS NULL \
             OR output_fn.oid IS NULL OR input_fn.oid IS NULL \
             OR source_type.oid >= 16384::oid OR target_type.oid >= 16384::oid \
             OR source_type.typnamespace <> 'pg_catalog'::regnamespace \
             OR target_type.typnamespace <> 'pg_catalog'::regnamespace \
             OR output_fn.oid >= 16384::oid OR input_fn.oid >= 16384::oid \
             OR output_fn.pronamespace <> 'pg_catalog'::regnamespace \
             OR input_fn.pronamespace <> 'pg_catalog'::regnamespace \
             OR output_fn.provolatile <> 'i' OR input_fn.provolatile <> 'i')",
            &[
                &functions,
                &types,
                &operators,
                &cached_functions,
                &sources,
                &targets,
                &FORBIDDEN_FUNCTION_NAMES,
                &containers,
            ],
        ),
    )
    .await?;
    meter.add(request, 1)?;
    if field::<bool>(&row, 0)? {
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

#[derive(Debug, Default, Eq, PartialEq)]
struct ExpressionDependencies {
    functions: BTreeSet<u32>,
    types: BTreeSet<u32>,
    subscript_containers: BTreeSet<u32>,
    operators: BTreeSet<(u32, u32)>,
    io_coercions: BTreeSet<(u32, u32)>,
}

/// The caller bounds bytes; nesting is bounded here. OIDs are same-transaction lookup keys.
fn expression_dependencies(tree: &str) -> Result<ExpressionDependencies, InvalidExpressionTree> {
    let mut reader = Reader {
        input: tree,
        offset: 0,
    };
    let root = reader.value(0)?;
    if reader.peek().is_some() {
        return Err(InvalidExpressionTree);
    }
    let mut dependencies = ExpressionDependencies::default();
    collect_dependencies(&root, &mut dependencies)?;
    Ok(dependencies)
}

fn collect_dependencies(
    value: &Value<'_>,
    dependencies: &mut ExpressionDependencies,
) -> Result<(), InvalidExpressionTree> {
    match value {
        Value::Node(kind, fields) => {
            if *kind == "SQLVALUEFUNCTION" {
                return Err(InvalidExpressionTree);
            }
            if *kind == "SUBSCRIPTINGREF" {
                dependencies
                    .subscript_containers
                    .insert(oid(fields, "refcontainertype")?);
                scalar_oid(fields.get("refelemtype").ok_or(InvalidExpressionTree)?)?;
                oid(fields, "refrestype")?;
            }
            if *kind == "COERCEVIAIO" {
                dependencies.io_coercions.insert((
                    result_type(fields.get("arg").ok_or(InvalidExpressionTree)?)?,
                    oid(fields, "resulttype")?,
                ));
            }
            let operator_node = matches!(
                *kind,
                "OPEXPR" | "DISTINCTEXPR" | "NULLIFEXPR" | "SCALARARRAYOPEXPR"
            );
            if operator_node {
                dependencies.operators.insert((
                    oid(fields, "opno")?,
                    scalar_oid(fields.get("opfuncid").ok_or(InvalidExpressionTree)?)?,
                ));
            }
            if *kind == "SCALARARRAYOPEXPR" {
                for slot in ["hashfuncid", "negfuncid"] {
                    scalar_oid(fields.get(slot).ok_or(InvalidExpressionTree)?)?;
                }
            }
            if *kind == "FUNCEXPR" {
                oid(fields, "funcid")?;
            }
            if *kind == "ROWCOMPAREEXPR" {
                let Some(Value::List(opnos)) = fields.get("opnos") else {
                    return Err(InvalidExpressionTree);
                };
                if !matches!(opnos.first(), Some(Value::Atom("o"))) || opnos.len() < 2 {
                    return Err(InvalidExpressionTree);
                }
                for value in &opnos[1..] {
                    let operator = scalar_oid(value)?;
                    if operator == 0 {
                        return Err(InvalidExpressionTree);
                    }
                    dependencies.operators.insert((operator, 0));
                }
            }
            for (name, child) in fields {
                if name.ends_with("funcid") {
                    let function = scalar_oid(child)?;
                    let optional = (operator_node && *name == "opfuncid")
                        || (*kind == "SCALARARRAYOPEXPR"
                            && matches!(*name, "hashfuncid" | "negfuncid"));
                    if function == 0 && !optional {
                        return Err(InvalidExpressionTree);
                    }
                    if function != 0 {
                        dependencies.functions.insert(function);
                    }
                }
                if matches!(
                    *name,
                    "consttype"
                        | "vartype"
                        | "resulttype"
                        | "funcresulttype"
                        | "opresulttype"
                        | "casetype"
                        | "coalescetype"
                        | "array_typeid"
                        | "element_typeid"
                        | "minmaxtype"
                        | "row_typeid"
                        | "typeId"
                        | "refcontainertype"
                        | "refelemtype"
                        | "refrestype"
                ) {
                    let type_oid = scalar_oid(child)?;
                    // Non-array containers such as jsonb have no separate element type.
                    if type_oid == 0 && !(*kind == "SUBSCRIPTINGREF" && *name == "refelemtype") {
                        return Err(InvalidExpressionTree);
                    }
                    if type_oid != 0 {
                        dependencies.types.insert(type_oid);
                    }
                }
                collect_dependencies(child, dependencies)?;
            }
        }
        Value::List(values) => {
            for child in values {
                collect_dependencies(child, dependencies)?;
            }
        }
        Value::Atom(_) | Value::Datum => {}
    }
    Ok(())
}

fn scalar_oid(value: &Value<'_>) -> Result<u32, InvalidExpressionTree> {
    match value {
        Value::Atom(token) => token.parse::<u32>().map_err(|_| InvalidExpressionTree),
        _ => Err(InvalidExpressionTree),
    }
}

fn result_type(value: &Value<'_>) -> Result<u32, InvalidExpressionTree> {
    let Value::Node(kind, fields) = value else {
        return Err(InvalidExpressionTree);
    };
    let field = match *kind {
        "VAR" => "vartype",
        "CONST" => "consttype",
        "SUBSCRIPTINGREF" => "refrestype",
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
    let value = scalar_oid(fields.get(field).ok_or(InvalidExpressionTree)?)?;
    if value == 0 {
        return Err(InvalidExpressionTree);
    }
    Ok(value)
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

    fn io_coercion_pairs(tree: &str) -> Result<Vec<(u32, u32)>, InvalidExpressionTree> {
        Ok(expression_dependencies(tree)?
            .io_coercions
            .into_iter()
            .collect())
    }

    #[test]
    fn optional_procedure_slots_require_their_own_operator_context() {
        let unfilled = expression_dependencies(
            "{SCALARARRAYOPEXPR :opno 96 :opfuncid 0 :hashfuncid 0 :negfuncid 0 :args ({VAR :vartype 23})}"
        ).unwrap();
        assert!(unfilled.functions.is_empty());
        assert_eq!(unfilled.operators, BTreeSet::from([(96, 0)]));
        assert_eq!(unfilled.types, BTreeSet::from([23]));
        let populated = expression_dependencies(
            "{SCALARARRAYOPEXPR :opno 96 :opfuncid 65 :hashfuncid 450 :negfuncid 65}",
        )
        .unwrap();
        assert_eq!(populated.functions, BTreeSet::from([65, 450]));
        assert_eq!(populated.operators, BTreeSet::from([(96, 65)]));
        let row = expression_dependencies("{ROWCOMPAREEXPR :opnos (o 97 97)}").unwrap();
        assert_eq!(row.operators, BTreeSet::from([(97, 0)]));
        assert!(expression_dependencies("{NODE :label SQLVALUEFUNCTION}").is_ok());
        for malformed in [
            "{FUNCEXPR :funcid 0}",
            "{FUNCEXPR}",
            "{NODE :hashfuncid 0}",
            "{SQLVALUEFUNCTION :type 1082}",
            "{SCALARARRAYOPEXPR :opno 0 :opfuncid 0 :hashfuncid 0 :negfuncid 0}",
            "{SCALARARRAYOPEXPR :opno 96 :opfuncid 65 :hashfuncid -1 :negfuncid 0}",
            "{SCALARARRAYOPEXPR :opno 96 :opfuncid 65 :hashfuncid 0}",
            "{OPEXPR :opno 96 :opfuncid <>}",
            "{ROWCOMPAREEXPR :opnos (i 97)}",
            "{ROWCOMPAREEXPR :opnos (o)}",
            "{ROWCOMPAREEXPR :opnos (o 0)}",
        ] {
            assert_eq!(
                expression_dependencies(malformed),
                Err(InvalidExpressionTree),
                "{malformed}"
            );
        }
    }

    #[test]
    fn coercion_result_comes_from_the_declared_node_not_a_nested_variable() {
        for (kind, result_field, other_fields) in [
            ("FUNCEXPR", "funcresulttype", ":funcid 123"),
            ("DISTINCTEXPR", "opresulttype", ":opno 96 :opfuncid 0"),
            ("NULLIFEXPR", "opresulttype", ":opno 96 :opfuncid 0"),
            ("RELABELTYPE", "resulttype", ""),
            ("ARRAYCOERCEEXPR", "resulttype", ""),
            ("COERCETODOMAIN", "resulttype", ""),
            ("CASEEXPR", "casetype", ""),
            ("COALESCEEXPR", "coalescetype", ""),
            ("MINMAXEXPR", "minmaxtype", ""),
            ("ARRAYEXPR", "array_typeid", ""),
            ("ROWEXPR", "row_typeid", ""),
            ("CASETESTEXPR", "typeId", ""),
            ("COERCETODOMAINVALUE", "typeId", ""),
        ] {
            let tree = format!(
                "{{COERCEVIAIO :arg {{{kind} :{result_field} 20 {other_fields} :arg {{VAR :vartype 23}}}} :resulttype 25}}"
            );
            assert_eq!(io_coercion_pairs(&tree), Ok(vec![(20, 25)]), "{kind}");
            let missing = tree.replace(&format!(":{result_field} 20"), "");
            assert_eq!(
                io_coercion_pairs(&missing),
                Err(InvalidExpressionTree),
                "{kind}"
            );
        }
        assert_eq!(
            io_coercion_pairs(
                "{COERCEVIAIO :arg {COLLATEEXPR :arg {VAR :vartype 23}} :resulttype 25}"
            ),
            Ok(vec![(23, 25)])
        );
        for kind in ["BOOLEXPR", "NULLTEST", "BOOLEANTEST"] {
            let tree =
                format!("{{COERCEVIAIO :arg {{{kind} :arg {{VAR :vartype 23}}}} :resulttype 25}}");
            assert_eq!(io_coercion_pairs(&tree), Ok(vec![(16, 25)]), "{kind}");
        }
    }

    #[test]
    fn nested_coercion_uses_its_arguments_own_type_and_rejects_ambiguous_input() {
        let composed = "{COERCEVIAIO :arg {OPEXPR :opno 551 :opfuncid 177 :opresulttype 23 :args ({VAR :vartype 20} {CONST :consttype 23 :constvalue 4 [ 1 0 0 0 0 0 0 0 ]})} :resulttype 25}";
        assert_eq!(io_coercion_pairs(composed), Ok(vec![(23, 25)]));
        let subscript = expression_dependencies(
            "{COERCEVIAIO :arg {SUBSCRIPTINGREF :refcontainertype 1007 :refelemtype 23 :refrestype 23 :refexpr {VAR :vartype 1007}} :resulttype 25}",
        ).unwrap();
        assert_eq!(subscript.io_coercions, BTreeSet::from([(23, 25)]));
        assert_eq!(subscript.subscript_containers, BTreeSet::from([1007]));
        assert_eq!(subscript.types, BTreeSet::from([23, 25, 1007]));
        let json = expression_dependencies(
            "{COERCEVIAIO :arg {SUBSCRIPTINGREF :refcontainertype 3802 :refelemtype 0 :refrestype 3802} :resulttype 25}",
        ).unwrap();
        assert_eq!(json.io_coercions, BTreeSet::from([(3802, 25)]));
        assert_eq!(json.types, BTreeSet::from([25, 3802]));
        assert_eq!(
            io_coercion_pairs(
                "{COERCEVIAIO :arg {COERCEVIAIO :arg {VAR :vartype 23} :resulttype 25} :resulttype 1043}"
            ),
            Ok(vec![(23, 25), (25, 1043)])
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
            "{SUBSCRIPTINGREF :refcontainertype 1007 :refelemtype 23}",
            "{SUBSCRIPTINGREF :refcontainertype 0 :refelemtype 23 :refrestype 23}",
            "{SUBSCRIPTINGREF :refcontainertype 1007 :refelemtype -1 :refrestype 23}",
            "{SUBSCRIPTINGREF :refcontainertype 1007 :refelemtype 23 :refrestype 0}",
            "{SUBSCRIPTINGREF :refcontainertype 1007 :refelemtype 23 :refrestype -1}",
            "{:NODE}",
            "{NODE : value}",
            "{NODE field 23}",
            "{NODE :value )}",
            "{NODE :value (}",
            "{ROWCOMPAREEXPR :opnos 97}",
            "{ROWCOMPAREEXPR :opnos (o -1)}",
            "{COERCEVIAIO :arg <> :resulttype 25}",
            "{COERCEVIAIO :arg {VAR :vartype (o 23)} :resulttype 25}",
            "{COERCEVIAIO :arg {COLLATEEXPR} :resulttype 25}",
            "{CONST :constvalue nope}",
            "{CONST :constvalue 1 0}",
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
