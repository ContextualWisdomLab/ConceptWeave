//! PostgreSQL ordinary exclusion-constraint transform-converter planner-support evidence.
//!
//! Converter definition and execution properties do not determine `pg_proc.prosupport`. This
//! successor preserves support absence or the exact resolved support-function identity while
//! retaining the immutable raw converter root needed by later lineage checks.

use std::collections::BTreeSet;
use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};
use super::{IndexExclusionConstraintCoordinate, QualifiedProcedureSignature, IndexExclusionConstraintOperatorProcedureTransformConverterDirection, IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_PLANNER_SUPPORT_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.planner_support.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_proc.prosupport` state for one nonzero transform converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    planner_support: Option<QualifiedProcedureSignature>,
}
impl IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation {
    /// Records support absence or the exact support function resolved from converter `prosupport`.
    pub fn new(coordinate: IndexExclusionConstraintCoordinate, key_position: u32, transform_type: QualifiedTypeName, direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection, converter_schema_name: impl Into<String>, converter_function_name: impl Into<String>, planner_support: Option<QualifiedProcedureSignature>) -> Result<Self, ObservationError> {
        if key_position == 0 { return Err(ObservationError::InvalidOrdinalPosition); }
        let converter_schema_name = converter_schema_name.into(); let converter_function_name = converter_function_name.into();
        validate_nonblank(&converter_schema_name, "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_function_schema")?;
        validate_nonblank(&converter_function_name, "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_function_name")?;
        Ok(Self { coordinate, key_position, transform_type, direction, converter_schema_name, converter_function_name, planner_support })
    }
    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use] pub const fn coordinate(&self)->&IndexExclusionConstraintCoordinate{&self.coordinate}
    /// Returns the one-based exclusion-key position.
    #[must_use] pub const fn key_position(&self)->u32{self.key_position}
    /// Returns the selected transform type.
    #[must_use] pub const fn transform_type(&self)->&QualifiedTypeName{&self.transform_type}
    /// Returns the converter direction.
    #[must_use] pub const fn direction(&self)->IndexExclusionConstraintOperatorProcedureTransformConverterDirection{self.direction}
    /// Returns the exact converter-function schema.
    #[must_use] pub fn converter_schema_name(&self)->&str{&self.converter_schema_name}
    /// Returns the exact converter-function name.
    #[must_use] pub fn converter_function_name(&self)->&str{&self.converter_function_name}
    /// Returns the exact planner support function, or `None` when `prosupport = 0`.
    #[must_use] pub const fn planner_support(&self)->Option<&QualifiedProcedureSignature>{self.planner_support.as_ref()}
    /// Returns the collision-safe evidence location.
    #[must_use] pub fn canonical_location(&self)->String{procedure_transform_converter_planner_support_location(&self.coordinate,self.key_position,&self.transform_type,self.direction)}
}

/// Immutable provenance receipt for one exact converter planner-support observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSourceReceipt { source_id:String, connection_policy_binding:String, source_digest:String, extractor_revision:String, observed_at_utc:String, location:IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation }
impl IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSourceReceipt {
    /// Returns the stable source registry key.
    #[must_use] pub fn source_id(&self)->&str{&self.source_id}
    /// Returns the immutable source-policy binding.
    #[must_use] pub fn connection_policy_binding(&self)->&str{&self.connection_policy_binding}
    /// Returns the owner-computed successor digest.
    #[must_use] pub fn source_digest(&self)->&str{&self.source_digest}
    /// Returns the exact extractor revision.
    #[must_use] pub fn extractor_revision(&self)->&str{&self.extractor_revision}
    /// Returns the exact canonical UTC observation time.
    #[must_use] pub fn observed_at_utc(&self)->&str{&self.observed_at_utc}
    /// Returns the validated planner-support observation.
    #[must_use] pub const fn location(&self)->&IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation{&self.location}
}

/// Complete converter `pg_proc.prosupport` evidence over one exact parallel-safety predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot {
    source_connection_key:String, connection_policy_binding:String, snapshot_digest:String, converter_snapshot_digest:String, extractor_revision:String, observed_at_utc:String, observations:Vec<IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation>,
}
impl IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot {
    /// Creates complete planner-support evidence for every converter direction in the predecessor.
    pub fn new(parallel_safety_snapshot:&IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot, mut observations:Vec<IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation>)->Result<Self,ObservationError>{
        observations.sort_by_key(planner_support_key);
        let expected=parallel_safety_snapshot.observations().iter().map(|o|planner_support_coordinate_key(o.coordinate(),o.key_position(),o.transform_type(),o.direction())).collect::<BTreeSet<_>>();
        let observed=observations.iter().map(planner_support_key).collect::<BTreeSet<_>>();
        if observed.len()!=observations.len(){return Err(invalid("index_exclusion_constraint_operator_procedure_transform_converter_planner_support_coordinate"));}
        if observed!=expected{return Err(invalid("index_exclusion_constraint_operator_procedure_transform_converter_planner_support_completeness"));}
        for observation in &observations { let predecessor=parallel_safety_snapshot.observations().iter().find(|c|c.coordinate()==observation.coordinate()&&c.key_position()==observation.key_position()&&c.transform_type()==observation.transform_type()&&c.direction()==observation.direction()).ok_or_else(||invalid("index_exclusion_constraint_operator_procedure_transform_converter_planner_support_completeness"))?; if predecessor.converter_schema_name()!=observation.converter_schema_name()||predecessor.converter_function_name()!=observation.converter_function_name(){return Err(invalid("index_exclusion_constraint_operator_procedure_transform_converter_planner_support_binding"));}}
        let snapshot_digest=compute_transform_converter_planner_support_digest(parallel_safety_snapshot.snapshot_digest(),&observations);
        Ok(Self{source_connection_key:parallel_safety_snapshot.source_connection_key().to_owned(),connection_policy_binding:parallel_safety_snapshot.connection_policy_binding().to_owned(),snapshot_digest,converter_snapshot_digest:parallel_safety_snapshot.converter_snapshot_digest().to_owned(),extractor_revision:parallel_safety_snapshot.extractor_revision().to_owned(),observed_at_utc:parallel_safety_snapshot.observed_at_utc().to_owned(),observations})
    }
    /// Returns the stable source-connection registry key.
    #[must_use] pub fn source_connection_key(&self)->&str{&self.source_connection_key}
    /// Returns the immutable source-policy binding.
    #[must_use] pub fn connection_policy_binding(&self)->&str{&self.connection_policy_binding}
    /// Returns the domain-separated planner-support successor digest.
    #[must_use] pub fn snapshot_digest(&self)->&str{&self.snapshot_digest}
    /// Returns the immutable raw transform-converter root digest.
    #[must_use] pub fn converter_snapshot_digest(&self)->&str{&self.converter_snapshot_digest}
    /// Returns the exact extractor revision.
    #[must_use] pub fn extractor_revision(&self)->&str{&self.extractor_revision}
    /// Returns the exact canonical UTC observation time.
    #[must_use] pub fn observed_at_utc(&self)->&str{&self.observed_at_utc}
    /// Returns complete observations in deterministic converter-direction order.
    #[must_use] pub fn observations(&self)->&[IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation]{&self.observations}
    /// Issues exact provenance for one observed converter planner-support identity.
    pub fn source_receipt(&self,coordinate:IndexExclusionConstraintCoordinate,key_position:u32,transform_type:QualifiedTypeName,direction:IndexExclusionConstraintOperatorProcedureTransformConverterDirection)->Result<IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSourceReceipt,ObservationError>{let observation=self.observations.iter().find(|o|o.coordinate()==&coordinate&&o.key_position()==key_position&&o.transform_type()==&transform_type&&o.direction()==direction).ok_or_else(||ObservationError::UnknownObservationLocation{location:procedure_transform_converter_planner_support_location(&coordinate,key_position,&transform_type,direction)})?;Ok(IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSourceReceipt{source_id:self.source_connection_key.clone(),connection_policy_binding:self.connection_policy_binding.clone(),source_digest:self.snapshot_digest.clone(),extractor_revision:self.extractor_revision.clone(),observed_at_utc:self.observed_at_utc.clone(),location:observation.clone()})}
}

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)] struct ConverterPlannerSupportCoordinateKey{schema_name:String,relation_name:String,relation_kind:String,constraint_name:String,key_position:u32,transform_type_schema_name:String,transform_type_name:String,direction:IndexExclusionConstraintOperatorProcedureTransformConverterDirection}
fn compute_transform_converter_planner_support_digest(predecessor_digest:&str,observations:&[IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation])->String{let mut hasher=Sha256::new();hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_PLANNER_SUPPORT_DIGEST_DOMAIN_V1);encode_str(&mut hasher,predecessor_digest);encode_len(&mut hasher,observations.len());for o in observations{encode_coordinate(&mut hasher,o.coordinate());hasher.update(o.key_position().to_be_bytes());encode_type(&mut hasher,o.transform_type());encode_str(&mut hasher,direction_token(o.direction()));encode_str(&mut hasher,o.converter_schema_name());encode_str(&mut hasher,o.converter_function_name());match o.planner_support(){None=>hasher.update([0]),Some(p)=>{hasher.update([1]);encode_procedure(&mut hasher,p);}}}format!("{SHA256_DIGEST_PREFIX}{:x}",hasher.finalize())}
fn planner_support_key(o:&IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation)->ConverterPlannerSupportCoordinateKey{planner_support_coordinate_key(o.coordinate(),o.key_position(),o.transform_type(),o.direction())}
fn planner_support_coordinate_key(c:&IndexExclusionConstraintCoordinate,k:u32,t:&QualifiedTypeName,d:IndexExclusionConstraintOperatorProcedureTransformConverterDirection)->ConverterPlannerSupportCoordinateKey{ConverterPlannerSupportCoordinateKey{schema_name:c.schema_name().to_owned(),relation_name:c.relation_name().to_owned(),relation_kind:c.relation_kind().token().to_owned(),constraint_name:c.constraint_name().to_owned(),key_position:k,transform_type_schema_name:t.schema_name().to_owned(),transform_type_name:t.type_name().to_owned(),direction:d}}
fn procedure_transform_converter_planner_support_location(c:&IndexExclusionConstraintCoordinate,k:u32,t:&QualifiedTypeName,d:IndexExclusionConstraintOperatorProcedureTransformConverterDirection)->String{let s=encode_location_component(t.schema_name());let n=encode_location_component(t.type_name());format!("{}/exclusion-operators/{k}/procedure-transform-converters/{s}.{n}/{}/planner-support",c.canonical_location(),direction_token(d))}
fn encode_location_component(value:&str)->String{const HEX:&[u8;16]=b"0123456789ABCDEF";let mut encoded=String::with_capacity(value.len());for byte in value.bytes(){match byte{b'A'..=b'Z'|b'a'..=b'z'|b'0'..=b'9'|b'_'|b'-'=>encoded.push(char::from(byte)),_=>{encoded.push('%');encoded.push(char::from(HEX[usize::from(byte>>4)]));encoded.push(char::from(HEX[usize::from(byte&0x0f)]));}}}encoded}
const fn direction_token(d:IndexExclusionConstraintOperatorProcedureTransformConverterDirection)->&'static str{match d{IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql=>"from_sql",IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql=>"to_sql"}}
fn encode_coordinate(h:&mut Sha256,c:&IndexExclusionConstraintCoordinate){encode_str(h,c.schema_name());encode_str(h,c.relation_name());encode_str(h,c.relation_kind().token());encode_str(h,c.constraint_name());}
fn encode_procedure(h:&mut Sha256,p:&QualifiedProcedureSignature){encode_str(h,p.schema_name());encode_str(h,p.procedure_name());encode_len(h,p.argument_types().len());for t in p.argument_types(){encode_type(h,t);}}
fn encode_type(h:&mut Sha256,t:&QualifiedTypeName){encode_str(h,t.schema_name());encode_str(h,t.type_name());} fn encode_len(h:&mut Sha256,v:usize){let v=u64::try_from(v).expect("Rust target usize must fit into canonical u64 length");h.update(v.to_be_bytes());} fn encode_str(h:&mut Sha256,v:&str){encode_len(h,v.len());h.update(v.as_bytes());}
fn validate_nonblank(v:&str,f:&'static str)->Result<(),ObservationError>{if v.trim().is_empty(){return Err(invalid(f));}Ok(())} fn invalid(field:&'static str)->ObservationError{ObservationError::InvalidObservationField{field}}
