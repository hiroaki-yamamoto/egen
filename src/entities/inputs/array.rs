use super::primitives::PrimitiveTypes;
use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArrayProperty {
  pub item: Box<PrimitiveTypes>,
}
