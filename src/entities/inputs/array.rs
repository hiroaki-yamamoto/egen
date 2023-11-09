use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayProperty {
  pub rust: Option<Rust>,
  pub item: Box<PrimitiveTypes>,
}

impl ArrayProperty {
  #[cfg(test)]
  pub fn new(item_type: PrimitiveTypes) -> Self {
    return Self {
      rust: None,
      item: Box::new(item_type),
    };
  }
}
