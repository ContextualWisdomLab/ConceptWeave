use std::collections::BTreeSet;

use conceptweave_observation::{
    ConstraintDeferrability, ConstraintTimingObservation, ForeignKeyAction,
    ForeignKeyCatalogObservation, ForeignKeyDeferrability, ForeignKeyMatchType,
    ForeignKeyObservation, ForeignKeyOperatorObservation, ForeignKeyReferenceBehavior,
    QualifiedTypeName, RelationObservation, TableConstraintObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use tokio_postgres::{Transaction, types::ToSql};

use super::{CaptureMeter, bounded, field};

pub(super) struct ForeignKeyTrigger {
    pub(super) oid: u32,
    pub(super) local_oid: u32,
    pub(super) referenced_oid: u32,
    pub(super) index_oid: u32,
    pub(super) enforced: bool,
    pub(super) update_action: ForeignKeyAction,
    pub(super) delete_action: ForeignKeyAction,
    pub(super) deferrable: bool,
    pub(super) initially_deferred: bool,
}

struct RawForeignKey {
    oid: u32,
    local_oid: u32,
    name: String,
    local_key: Vec<i16>,
    referenced_key: Vec<i16>,
    referenced_oid: u32,
    referenced_schema: String,
    referenced_relation: String,
    index_oid: u32,
    index_name: String,
    validated: bool,
    enforced: bool,
    deferrable: bool,
    initially_deferred: bool,
    update_action: ForeignKeyAction,
    delete_action: ForeignKeyAction,
    match_type: ForeignKeyMatchType,
    delete_target_key: Option<Vec<i16>>,
    primary_foreign_operators: Vec<u32>,
    primary_primary_operators: Vec<u32>,
    foreign_foreign_operators: Vec<u32>,
}

fn action(code: &str) -> Result<ForeignKeyAction, SourceObservationFailure> {
    match code {
        "a" => Ok(ForeignKeyAction::NoAction),
        "r" => Ok(ForeignKeyAction::Restrict),
        "c" => Ok(ForeignKeyAction::Cascade),
        "n" => Ok(ForeignKeyAction::SetNull),
        "d" => Ok(ForeignKeyAction::SetDefault),
        _ => Err(SourceObservationFailure::InvalidCapturedMetadata),
    }
}

fn names(
    positions: &[i16],
    relation: &RelationObservation,
) -> Result<Vec<String>, SourceObservationFailure> {
    if positions.is_empty() {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    positions
        .iter()
        .map(|position| {
            relation
                .columns()
                .iter()
                .find(|column| column.ordinal_position() == *position as u32)
                .map(|column| column.column_name().to_owned())
                .ok_or(SourceObservationFailure::InvalidCapturedMetadata)
        })
        .collect()
}

async fn operators(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    oids: &[u32],
) -> Result<Vec<ForeignKeyOperatorObservation>, SourceObservationFailure> {
    let mut result = Vec::with_capacity(oids.len());
    for oid in oids {
        let row = bounded(
            request,
            cancellation,
            transaction.query_opt(
                "SELECT n.nspname::text, o.oprname::text, ln.nspname::text, lt.typname::text, \
                 rn.nspname::text, rt.typname::text FROM pg_catalog.pg_operator o \
                 JOIN pg_catalog.pg_namespace n ON n.oid = o.oprnamespace \
                 JOIN pg_catalog.pg_type lt ON lt.oid = o.oprleft \
                 JOIN pg_catalog.pg_namespace ln ON ln.oid = lt.typnamespace \
                 JOIN pg_catalog.pg_type rt ON rt.oid = o.oprright \
                 JOIN pg_catalog.pg_namespace rn ON rn.oid = rt.typnamespace \
                 WHERE o.oid = $1 AND o.oprkind = 'b'",
                &[oid],
            ),
        )
        .await?
        .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        let fields = (0..6)
            .map(|index| field::<String>(&row, index))
            .collect::<Result<Vec<_>, _>>()?;
        meter.add(request, fields.iter().map(String::len).sum())?;
        result.push(
            ForeignKeyOperatorObservation::new(
                &fields[0],
                &fields[1],
                QualifiedTypeName::new(&fields[2], &fields[3])
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                QualifiedTypeName::new(&fields[4], &fields[5])
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
    }
    Ok(result)
}

pub(super) async fn capture(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    relation_oids: &[u32],
    relations: &mut [RelationObservation],
) -> Result<(Vec<ForeignKeyCatalogObservation>, Vec<ForeignKeyTrigger>), SourceObservationFailure> {
    let mut raw = Vec::new();
    for local_oid in relation_oids {
        let stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT c.oid, c.conname::text, c.conkey, c.confkey, \
                 rn.nspname::text, rt.relname::text, c.confrelid, c.conindid, ix.relname::text, \
                 c.convalidated, c.conenforced, c.condeferrable, c.condeferred, \
                 c.confupdtype::text, c.confdeltype::text, c.confmatchtype::text, \
                 c.confdelsetcols, c.conpfeqop, c.conppeqop, c.conffeqop, \
                 c.connoinherit, c.conislocal, c.coninhcount, c.conparentid = 0, \
                 NOT c.conperiod, c.conbin IS NULL, c.conexclop IS NULL, c.contypid = 0, \
                 c.connamespace = t.relnamespace, \
                 NOT EXISTS(SELECT 1 FROM pg_catalog.pg_description \
                   WHERE classoid = 'pg_constraint'::regclass AND objoid = c.oid), \
                 NOT EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel \
                   WHERE classoid = 'pg_constraint'::regclass AND objoid = c.oid) \
                 FROM pg_catalog.pg_constraint c \
                 JOIN pg_catalog.pg_class t ON t.oid = c.conrelid \
                 LEFT JOIN pg_catalog.pg_class rt ON rt.oid = c.confrelid \
                 LEFT JOIN pg_catalog.pg_namespace rn ON rn.oid = rt.relnamespace \
                 LEFT JOIN pg_catalog.pg_class ix ON ix.oid = c.conindid \
                 WHERE c.conrelid = $1 AND c.contype = 'f' ORDER BY c.conname, c.oid",
                vec![local_oid as &(dyn ToSql + Sync)],
            ),
        )
        .await?;
        tokio::pin!(stream);
        while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
            let name: String = field(&row, 1)?;
            let local_key: Vec<i16> = field(&row, 2)?;
            let referenced_key: Vec<i16> = field(&row, 3)?;
            let referenced_schema: Option<String> = field(&row, 4)?;
            let referenced_relation: Option<String> = field(&row, 5)?;
            let index_name: Option<String> = field(&row, 8)?;
            let delete_target_key: Option<Vec<i16>> = field(&row, 16)?;
            let primary_foreign_operators: Vec<u32> = field(&row, 17)?;
            let primary_primary_operators: Vec<u32> = field(&row, 18)?;
            let foreign_foreign_operators: Vec<u32> = field(&row, 19)?;
            meter.add(
                request,
                96 + name.len()
                    + referenced_schema.as_ref().map_or(0, String::len)
                    + referenced_relation.as_ref().map_or(0, String::len)
                    + index_name.as_ref().map_or(0, String::len)
                    + 2 * (local_key.len()
                        + referenced_key.len()
                        + delete_target_key.as_ref().map_or(0, Vec::len))
                    + 4 * (primary_foreign_operators.len()
                        + primary_primary_operators.len()
                        + foreign_foreign_operators.len()),
            )?;
            if field::<i16>(&row, 22)? != 0
                || !(20..=21).all(|index| field::<bool>(&row, index) == Ok(true))
                || !(23..=30).all(|index| field::<bool>(&row, index) == Ok(true))
                || local_key.len() != referenced_key.len()
                || local_key.is_empty()
                || primary_foreign_operators.len() != local_key.len()
                || primary_primary_operators.len() != local_key.len()
                || foreign_foreign_operators.len() != local_key.len()
                || (field::<bool>(&row, 12)? && !field::<bool>(&row, 11)?)
            {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            let update_action = action(&field::<String>(&row, 13)?)?;
            let delete_action = action(&field::<String>(&row, 14)?)?;
            let match_type = match field::<String>(&row, 15)?.as_str() {
                "s" => ForeignKeyMatchType::Simple,
                "f" => ForeignKeyMatchType::Full,
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            if delete_target_key.is_some()
                && !matches!(
                    delete_action,
                    ForeignKeyAction::SetNull | ForeignKeyAction::SetDefault
                )
            {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            raw.push(RawForeignKey {
                oid: field(&row, 0)?,
                local_oid: *local_oid,
                name,
                local_key,
                referenced_key,
                referenced_oid: field(&row, 6)?,
                referenced_schema: referenced_schema
                    .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?,
                referenced_relation: referenced_relation
                    .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?,
                index_oid: field(&row, 7)?,
                index_name: index_name.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?,
                validated: field(&row, 9)?,
                enforced: field(&row, 10)?,
                deferrable: field(&row, 11)?,
                initially_deferred: field(&row, 12)?,
                update_action,
                delete_action,
                match_type,
                delete_target_key,
                primary_foreign_operators,
                primary_primary_operators,
                foreign_foreign_operators,
            });
        }
    }
    let mut catalogs = Vec::with_capacity(raw.len());
    let mut triggers = Vec::with_capacity(raw.len());
    for key in raw {
        let local_index = relation_oids
            .iter()
            .position(|oid| *oid == key.local_oid)
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        let referenced_index = relation_oids
            .iter()
            .position(|oid| *oid == key.referenced_oid)
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        let local = relations[local_index].clone();
        let referenced = &relations[referenced_index];
        if referenced.schema_name() != key.referenced_schema
            || referenced.relation_name() != key.referenced_relation
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let local_names = names(&key.local_key, &local)?;
        let referenced_names = names(&key.referenced_key, referenced)?;
        let mut behavior = ForeignKeyReferenceBehavior::new(
            key.update_action,
            key.delete_action,
            key.match_type,
            match (key.deferrable, key.initially_deferred) {
                (false, false) => ForeignKeyDeferrability::NotDeferrable,
                (true, false) => ForeignKeyDeferrability::InitiallyImmediate,
                (true, true) => ForeignKeyDeferrability::InitiallyDeferred,
                (false, true) => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            },
        );
        if let Some(delete_target_key) = &key.delete_target_key {
            behavior = behavior
                .with_delete_target_columns(names(delete_target_key, &local)?)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        }
        let observation = ForeignKeyObservation::with_reference_behavior(
            &key.name,
            local_names,
            &key.referenced_schema,
            &key.referenced_relation,
            referenced_names,
            behavior,
        )
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
        .with_validation_and_enforcement(key.validated, key.enforced);
        let mut constraints = local.constraints().to_vec();
        constraints.push(TableConstraintObservation::ForeignKey(observation));
        relations[local_index] = local
            .clone()
            .with_constraints(constraints)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let primary_foreign = operators(
            transaction,
            request,
            cancellation,
            meter,
            &key.primary_foreign_operators,
        )
        .await?;
        let primary_primary = operators(
            transaction,
            request,
            cancellation,
            meter,
            &key.primary_primary_operators,
        )
        .await?;
        let foreign_foreign = operators(
            transaction,
            request,
            cancellation,
            meter,
            &key.foreign_foreign_operators,
        )
        .await?;
        catalogs.push(
            ForeignKeyCatalogObservation::new(
                local.schema_name(),
                local.relation_name(),
                &key.name,
                &key.index_name,
                primary_foreign,
                primary_primary,
                foreign_foreign,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
        triggers.push(ForeignKeyTrigger {
            oid: key.oid,
            local_oid: key.local_oid,
            referenced_oid: key.referenced_oid,
            index_oid: key.index_oid,
            enforced: key.enforced,
            update_action: key.update_action,
            delete_action: key.delete_action,
            deferrable: key.deferrable,
            initially_deferred: key.initially_deferred,
        });
    }
    Ok((catalogs, triggers))
}

fn action_name(action: ForeignKeyAction) -> &'static str {
    match action {
        ForeignKeyAction::NoAction => "noaction",
        ForeignKeyAction::Restrict => "restrict",
        ForeignKeyAction::Cascade => "cascade",
        ForeignKeyAction::SetNull => "setnull",
        ForeignKeyAction::SetDefault => "setdefault",
    }
}

pub(super) async fn validate_triggers(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    relation_oids: &[u32],
    key_timings: &[Vec<ConstraintTimingObservation>],
    foreign_keys: &[ForeignKeyTrigger],
) -> Result<(), SourceObservationFailure> {
    for (relation_oid, timings) in relation_oids.iter().zip(key_timings) {
        let expected_keys = timings
            .iter()
            .filter(|timing| timing.deferrability() != ConstraintDeferrability::NotDeferrable)
            .map(|timing| timing.constraint_name().to_owned())
            .collect::<BTreeSet<_>>();
        let mut expected_fk = BTreeSet::new();
        for key in foreign_keys {
            if !key.enforced {
                continue;
            }
            if key.local_oid == *relation_oid {
                expected_fk.insert((key.oid, "RI_FKey_check_ins".to_owned()));
                expected_fk.insert((key.oid, "RI_FKey_check_upd".to_owned()));
            }
            if key.referenced_oid == *relation_oid {
                expected_fk.insert((
                    key.oid,
                    format!("RI_FKey_{}_del", action_name(key.delete_action)),
                ));
                expected_fk.insert((
                    key.oid,
                    format!("RI_FKey_{}_upd", action_name(key.update_action)),
                ));
            }
        }
        let stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT c.oid, c.conname::text, p.proname::text, t.tgtype, \
                 t.tgisinternal, t.tgenabled::text, t.tgnargs, octet_length(t.tgargs), \
                 t.tgattr::text, t.tgqual IS NULL, t.tgoldtable IS NULL, \
                 t.tgnewtable IS NULL, t.tgparentid = 0, t.tgconstrrelid, \
                 t.tgconstrindid, t.tgdeferrable, t.tginitdeferred, pn.nspname::text, \
                 c.contype::text, c.conrelid, c.confrelid, c.conindid, \
                 c.condeferrable, c.condeferred \
                 FROM pg_catalog.pg_trigger t \
                 LEFT JOIN pg_catalog.pg_constraint c ON c.oid = t.tgconstraint \
                 LEFT JOIN pg_catalog.pg_proc p ON p.oid = t.tgfoid \
                 LEFT JOIN pg_catalog.pg_namespace pn ON pn.oid = p.pronamespace \
                 WHERE t.tgrelid = $1 ORDER BY t.tgname, t.oid",
                vec![relation_oid as &(dyn ToSql + Sync)],
            ),
        )
        .await?;
        tokio::pin!(stream);
        let mut observed_keys = BTreeSet::new();
        let mut observed_fk = BTreeSet::new();
        while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
            let name: Option<String> = field(&row, 1)?;
            let proc_name: Option<String> = field(&row, 2)?;
            meter.add(
                request,
                64 + name.as_ref().map_or(0, String::len)
                    + proc_name.as_ref().map_or(0, String::len),
            )?;
            let name = name.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
            let proc_name = proc_name.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
            if !field::<bool>(&row, 4)?
                || field::<String>(&row, 5)? != "O"
                || field::<i16>(&row, 6)? != 0
                || field::<i32>(&row, 7)? != 0
                || !field::<String>(&row, 8)?.is_empty()
                || !(9..=12).all(|index| field::<bool>(&row, index) == Ok(true))
                || field::<String>(&row, 17)? != "pg_catalog"
            {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            let constraint_oid: u32 = field(&row, 0)?;
            let trigger_type: i16 = field(&row, 3)?;
            let opposite_oid: u32 = field(&row, 13)?;
            let index_oid: u32 = field(&row, 14)?;
            let deferrable: bool = field(&row, 15)?;
            let deferred: bool = field(&row, 16)?;
            let kind: String = field(&row, 18)?;
            if kind == "f" {
                let key = foreign_keys
                    .iter()
                    .find(|key| key.oid == constraint_oid)
                    .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                if field::<u32>(&row, 19)? != key.local_oid
                    || field::<u32>(&row, 20)? != key.referenced_oid
                    || field::<u32>(&row, 21)? != key.index_oid
                    || field::<bool>(&row, 22)? != key.deferrable
                    || field::<bool>(&row, 23)? != key.initially_deferred
                    || index_oid != key.index_oid
                {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
                let expected_shape =
                    if key.local_oid == *relation_oid && proc_name == "RI_FKey_check_ins" {
                        Some((
                            5,
                            key.referenced_oid,
                            key.deferrable,
                            key.initially_deferred,
                        ))
                    } else if key.local_oid == *relation_oid && proc_name == "RI_FKey_check_upd" {
                        Some((
                            17,
                            key.referenced_oid,
                            key.deferrable,
                            key.initially_deferred,
                        ))
                    } else if key.referenced_oid == *relation_oid
                        && proc_name == format!("RI_FKey_{}_del", action_name(key.delete_action))
                    {
                        let deferred_action = key.delete_action == ForeignKeyAction::NoAction;
                        Some((
                            9,
                            key.local_oid,
                            key.deferrable && deferred_action,
                            key.initially_deferred && deferred_action,
                        ))
                    } else if key.referenced_oid == *relation_oid
                        && proc_name == format!("RI_FKey_{}_upd", action_name(key.update_action))
                    {
                        let deferred_action = key.update_action == ForeignKeyAction::NoAction;
                        Some((
                            17,
                            key.local_oid,
                            key.deferrable && deferred_action,
                            key.initially_deferred && deferred_action,
                        ))
                    } else {
                        None
                    };
                if expected_shape != Some((trigger_type, opposite_oid, deferrable, deferred))
                    || !expected_fk.contains(&(constraint_oid, proc_name.clone()))
                    || !observed_fk.insert((constraint_oid, proc_name))
                {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
            } else if matches!(kind.as_str(), "p" | "u") {
                if field::<u32>(&row, 19)? != *relation_oid
                    || field::<u32>(&row, 20)? != 0
                    || field::<u32>(&row, 21)? != index_oid
                    || !field::<bool>(&row, 22)?
                    || field::<bool>(&row, 23)? != deferred
                    || !deferrable
                    || opposite_oid != 0
                    || proc_name != "unique_key_recheck"
                    || trigger_type != 21
                    || !expected_keys.contains(&name)
                    || !observed_keys.insert(name)
                {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
            } else {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
        }
        if observed_keys != expected_keys || observed_fk != expected_fk {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
    }
    Ok(())
}
