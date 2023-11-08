use super::primitives::PrimitiveTypes;
use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayProperty {
  pub item: Box<PrimitiveTypes>,
}
