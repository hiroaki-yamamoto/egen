use ::serde::Deserialize;

use super::array::ArrayProperty;
use super::structure::Structure;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveTypes {
  Array(ArrayProperty),
  Struct(Structure),
  Use(String),
  Bool,
  String,
  F32,
  F64,
  I8,
  I32,
  I64,
  I128,
  U8,
  U32,
  U64,
  U128,
}
