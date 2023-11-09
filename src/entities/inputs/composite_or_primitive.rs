use ::serde::Deserialize;

use super::composite::CompositeTypes;
use super::primitives::PrimitiveTypes;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum CompositeOrPrimitive {
  Composite(CompositeTypes),
  PrimitiveTypes(PrimitiveTypes),
}
