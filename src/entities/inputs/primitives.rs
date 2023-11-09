use ::serde::Deserialize;

use super::array::ArrayProperty;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveTypes {
  Bool,
  String,
  F32,
  F64,
  I8,
  I16,
  I32,
  I64,
  I128,
  U16,
  U8,
  U32,
  U64,
  U128,
  Use(String),
  Array(ArrayProperty),
}
