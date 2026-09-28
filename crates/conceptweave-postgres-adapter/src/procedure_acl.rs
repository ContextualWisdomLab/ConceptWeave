//! Original ACL item extraction, including items omitted by aclexplode.

use super::{CaptureMeter, bounded, field};
use conceptweave_observation::{
    ProcedureAccessControlObservation, ProcedureAclItem, ProcedureInitialPrivilegeOrigin,
    ProcedureInitialPrivileges, ProcedureInitialPrivilegesObservation,
    ReferencedProcedureDefinitionObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use std::collections::BTreeMap;
use tokio_postgres::{Transaction, types::ToSql};

pub(super) async fn capture(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    oids: &[u32],
    definitions: &BTreeMap<u32, ReferencedProcedureDefinitionObservation>,
) -> Result<Vec<ProcedureAccessControlObservation>, SourceObservationFailure> {
    capture_catalog(
        tx,
        request,
        cancellation,
        meter,
        oids,
        definitions,
        AclSource::Current,
    )
    .await?
    .into_iter()
    .map(|(oid, item)| {
        let definition = definitions
            .get(&oid)
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        Ok(ProcedureAccessControlObservation::new(
            definition.location().clone(),
            item.raw_acl,
        ))
    })
    .collect()
}

pub(super) async fn capture_initial_privileges(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    oids: &[u32],
    definitions: &BTreeMap<u32, ReferencedProcedureDefinitionObservation>,
) -> Result<Vec<ProcedureInitialPrivilegesObservation>, SourceObservationFailure> {
    capture_catalog(
        tx,
        request,
        cancellation,
        meter,
        oids,
        definitions,
        AclSource::Initial,
    )
    .await?
    .into_iter()
    .map(|(oid, item)| {
        let definition = definitions
            .get(&oid)
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        let material = match (item.origin, item.raw_acl) {
            (None, None) => None,
            (Some(origin), Some(acl)) => Some(ProcedureInitialPrivileges::new(origin, acl)),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        };
        Ok(ProcedureInitialPrivilegesObservation::new(
            definition.location().clone(),
            material,
        ))
    })
    .collect()
}

#[derive(Clone, Copy)]
enum AclSource {
    Current,
    Initial,
}
struct CapturedAcl {
    raw_acl: Option<Vec<ProcedureAclItem>>,
    origin: Option<ProcedureInitialPrivilegeOrigin>,
}

async fn capture_catalog(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    oids: &[u32],
    definitions: &BTreeMap<u32, ReferencedProcedureDefinitionObservation>,
    source: AclSource,
) -> Result<BTreeMap<u32, CapturedAcl>, SourceObservationFailure> {
    let initial = matches!(source, AclSource::Initial);
    if oids.is_empty() {
        return Ok(BTreeMap::new());
    }
    let max_rows = request.request().limits().max_rows().min(i64::MAX as u64) as i64;
    let headers = bounded(request, cancellation, tx.query_raw(
        "WITH material AS (SELECT ref.oid, p.oid IS NULL AS missing, \
         CASE WHEN $3::boolean THEN ip.initprivs ELSE p.proacl END AS raw_acl, ip.privtype::text AS origin, ip.objsubid \
         FROM unnest($1::oid[]) ref(oid) LEFT JOIN pg_catalog.pg_proc p ON p.oid=ref.oid \
         LEFT JOIN pg_catalog.pg_init_privs ip ON $3::boolean AND ip.classoid='pg_catalog.pg_proc'::regclass AND ip.objoid=p.oid) \
         SELECT oid, missing, raw_acl IS NULL, COALESCE(cardinality(raw_acl),0), array_ndims(raw_acl), \
         CASE WHEN cardinality(raw_acl)::bigint <= $2 THEN \
         EXISTS(SELECT 1 FROM unnest(raw_acl) a WHERE a IS NULL) ELSE false END, origin, objsubid \
         FROM material ORDER BY oid",
        vec![&oids as &(dyn ToSql + Sync), &max_rows, &initial],
    )).await?;
    tokio::pin!(headers);
    let mut items: BTreeMap<u32, CapturedAcl> = BTreeMap::new();
    let mut counts = BTreeMap::new();
    while let Some(row) = bounded(request, cancellation, headers.try_next()).await? {
        let oid: u32 = field(&row, 0)?;
        let count: i32 = field(&row, 3)?;
        if i64::from(count) > max_rows {
            return Err(SourceObservationFailure::RowLimitExceeded {
                max_rows: request.request().limits().max_rows(),
            });
        }
        if field::<bool>(&row, 1)?
            || count < 0
            || (count > 0 && field::<Option<i32>>(&row, 4)? != Some(1))
            || field::<bool>(&row, 5)?
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let is_null: bool = field(&row, 2)?;
        let origin = match field::<Option<String>>(&row, 6)?.as_deref() {
            None => None,
            Some("i") => Some(ProcedureInitialPrivilegeOrigin::Initialization),
            Some("e") => Some(ProcedureInitialPrivilegeOrigin::Extension),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        };
        let subobject: Option<i32> = field(&row, 7)?;
        if initial
            && ((origin.is_some() && (is_null || subobject != Some(0)))
                || (origin.is_none() && (!is_null || subobject.is_some())))
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        meter.add(request, if initial && origin.is_some() { 14 } else { 9 })?;
        if items
            .insert(
                oid,
                CapturedAcl {
                    raw_acl: if is_null { None } else { Some(Vec::new()) },
                    origin,
                },
            )
            .is_some()
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        counts.insert(oid, count as usize);
    }
    if items.len() != definitions.len() {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    let stream = bounded(request, cancellation, tx.query_raw(
        "WITH material AS (SELECT p.oid, CASE WHEN $2::boolean THEN ip.initprivs ELSE p.proacl END AS raw_acl \
         FROM unnest($1::oid[]) ref(oid) JOIN pg_catalog.pg_proc p ON p.oid=ref.oid \
         LEFT JOIN pg_catalog.pg_init_privs ip ON $2::boolean AND ip.classoid='pg_catalog.pg_proc'::regclass AND ip.objoid=p.oid) \
         SELECT p.oid, item.position, acl.grantee, gr.rolname::text, acl.grantor, rr.rolname::text, \
         acl.privilege_type, acl.is_grantable, CASE WHEN acl.privilege_type IS NULL THEN item.value::text END \
         FROM material p CROSS JOIN LATERAL unnest(p.raw_acl) WITH ORDINALITY item(value, position) \
         LEFT JOIN LATERAL pg_catalog.aclexplode(ARRAY[item.value]) acl ON true \
         LEFT JOIN pg_catalog.pg_roles gr ON gr.oid = acl.grantee \
         LEFT JOIN pg_catalog.pg_roles rr ON rr.oid = acl.grantor ORDER BY p.oid, item.position",
        vec![&oids as &(dyn ToSql + Sync), &initial],
    )).await?;
    tokio::pin!(stream);
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let oid: u32 = field(&row, 0)?;
        let position: i64 = field(&row, 1)?;
        let entry = match field::<Option<String>>(&row, 6)?.as_deref() {
            Some("EXECUTE") => {
                let grantee_name: Option<String> = field(&row, 3)?;
                let grantor_name: Option<String> = field(&row, 5)?;
                meter.add(
                    request,
                    10 + grantee_name.as_ref().map_or(0, String::len)
                        + grantor_name.as_ref().map_or(0, String::len),
                )?;
                ProcedureAclItem::new(
                    field(&row, 2)?,
                    grantee_name,
                    field(&row, 4)?,
                    grantor_name,
                    true,
                    field(&row, 7)?,
                )
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
            }
            None => {
                let text: String = field(&row, 8)?;
                meter.add(request, text.len() + 10)?;
                empty_item(
                    tx,
                    request,
                    cancellation,
                    meter,
                    source,
                    (oid, position),
                    &text,
                )
                .await?
            }
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        };
        let grants = items
            .get_mut(&oid)
            .and_then(|item| item.raw_acl.as_mut())
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        if position != grants.len() as i64 + 1 {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        grants.push(entry);
    }
    for (oid, item) in &items {
        if item.raw_acl.as_ref().map_or(0, Vec::len) != counts[oid] {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
    }
    Ok(items)
}

async fn empty_item(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    source: AclSource,
    coordinate: (u32, i64),
    text: &str,
) -> Result<ProcedureAclItem, SourceObservationFailure> {
    let (oid, position) = coordinate;
    let initial = matches!(source, AclSource::Initial);
    let (grantee, grantor) =
        empty_roles(text).ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
    let grantees = candidates(tx, request, cancellation, grantee.as_deref()).await?;
    let grantors = candidates(tx, request, cancellation, Some(&grantor)).await?;
    // The original item has no privilege bits. Native containment verifies BOTH original role
    // OIDs and excludes malformed grant-option-only bits; names never establish identity.
    let matches = bounded(
        request,
        cancellation,
        tx.query(
            "SELECT g.oid, r.oid, gr.rolname::text, rr.rolname::text \
         FROM unnest($3::oid[]) g(oid) CROSS JOIN unnest($4::oid[]) r(oid) \
         JOIN pg_catalog.pg_proc p ON p.oid = $1 \
         LEFT JOIN pg_catalog.pg_init_privs ip ON $5::boolean AND ip.classoid='pg_catalog.pg_proc'::regclass AND ip.objoid=p.oid \
         CROSS JOIN LATERAL unnest(CASE WHEN $5::boolean THEN ip.initprivs ELSE p.proacl END) WITH ORDINALITY item(value, position) \
         LEFT JOIN pg_catalog.pg_roles gr ON gr.oid = g.oid \
         LEFT JOIN pg_catalog.pg_roles rr ON rr.oid = r.oid \
         WHERE item.position = $2 \
         AND ARRAY[pg_catalog.makeaclitem(g.oid, r.oid, 'EXECUTE', false)] @> item.value",
            &[&oid, &position, &grantees, &grantors, &initial],
        ),
    )
    .await?;
    let [row] = matches.as_slice() else {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    };
    let grantee_name: Option<String> = field(row, 2)?;
    let grantor_name: Option<String> = field(row, 3)?;
    meter.add(
        request,
        grantee_name.as_ref().map_or(0, String::len) + grantor_name.as_ref().map_or(0, String::len),
    )?;
    ProcedureAclItem::new(
        field(row, 0)?,
        grantee_name,
        field(row, 1)?,
        grantor_name,
        false,
        false,
    )
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)
}

async fn candidates(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    name: Option<&str>,
) -> Result<Vec<u32>, SourceObservationFailure> {
    let Some(name) = name else {
        return Ok(vec![0]);
    };
    let mut ids = Vec::new();
    if let Some(row) = bounded(
        request,
        cancellation,
        tx.query_opt(
            "SELECT oid FROM pg_catalog.pg_roles WHERE rolname::text = $1",
            &[&name],
        ),
    )
    .await?
    {
        ids.push(field::<u32>(&row, 0)?);
    }
    if let Ok(oid) = name.parse::<u32>() {
        ids.push(oid);
    }
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    Ok(ids)
}

fn empty_roles(text: &str) -> Option<(Option<String>, String)> {
    let mut quoted = false;
    let mut separators = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((index, ch)) = chars.next() {
        if ch == '"' {
            if quoted && chars.peek().is_some_and(|(_, next)| *next == '"') {
                chars.next();
            } else {
                quoted = !quoted;
            }
        } else if !quoted && (ch == '=' || ch == '/') {
            separators.push((index, ch));
        }
    }
    let [(equals, '='), (slash, '/')] = separators.as_slice() else {
        return None;
    };
    if quoted || *slash != equals + 1 {
        return None;
    }
    let grantee = if *equals == 0 {
        None
    } else {
        Some(role_name(&text[..*equals])?)
    };
    Some((grantee, role_name(&text[slash + 1..])?))
}

fn role_name(token: &str) -> Option<String> {
    let name = if let Some(rest) = token.strip_prefix('"') {
        let mut chars = rest.chars().peekable();
        let mut value = String::new();
        loop {
            match chars.next()? {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    value.push('"');
                }
                '"' if chars.peek().is_none() => break,
                '"' => return None,
                ch => value.push(ch),
            }
        }
        value
    } else {
        if !token
            .bytes()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == b'_')
        {
            return None;
        }
        token.to_owned()
    };
    if name.is_empty() || name.len() > 63 || name.contains('\0') {
        return None;
    }
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_acl_roles_keep_quoted_delimiters_and_numeric_names() {
        assert_eq!(
            empty_roles("=/pg_read_all_data"),
            Some((None, "pg_read_all_data".into()))
        );
        assert_eq!(
            empty_roles("123=/456"),
            Some((Some("123".into()), "456".into()))
        );
        assert_eq!(
            empty_roles("\"a=/b\"=/\"x\"\"y\""),
            Some((Some("a=/b".into()), "x\"y".into()))
        );
        assert_eq!(
            empty_roles("\" \"=/\"이름\""),
            Some((Some(" ".into()), "이름".into()))
        );
        for bad in [
            "",
            "=/",
            "a=X/b",
            "a=*/b",
            "a=/b/extra",
            "a=/\"b",
            "a=/\"b\"tail",
            "a=/\"\"",
            "a=/b\0",
        ] {
            assert!(empty_roles(bad).is_none(), "{bad:?}");
        }
    }
}
