pub use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;

#[derive(Debug, Deserialize)]
pub struct Structure {
  #[serde(rename = "type")]
  pub struct_type: PrimitiveTypes,
}
