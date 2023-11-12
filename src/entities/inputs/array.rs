use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase")]
pub struct ArrayProperty {
  pub item: Box<PrimitiveTypes>,
}

impl ArrayProperty {
  #[cfg(test)]
  pub fn new(item_type: PrimitiveTypes) -> Self {
    return Self {
      item: Box::new(item_type),
    };
  }
}
