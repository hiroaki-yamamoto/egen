use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FieldInner {
  #[serde(rename = "type")]
  pub f_type: PrimitiveTypes,
  pub rust: Option<Rust>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Field {
  Inner(FieldInner),
  Primitive(PrimitiveTypes),
}
