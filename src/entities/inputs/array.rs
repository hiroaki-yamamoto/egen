use ::serde::{Deserialize, Serialize};

use super::primitives::PrimitiveTypes;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
