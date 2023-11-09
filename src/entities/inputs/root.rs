pub use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Root {
  #[serde(rename = "type")]
  pub struct_type: PrimitiveTypes,
}
