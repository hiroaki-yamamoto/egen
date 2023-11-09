use ::serde::Deserialize;

use super::array::ArrayProperty;
use super::structure::Structure;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CompositeTypes {
  Array(ArrayProperty),
  Struct(Structure),
  Use(String),
}
