use ::serde::Deserialize;

use super::rs::Rust;
use super::PrimitiveTypes;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
  pub name: String,
  #[serde(rename = "type")]
  pub f_type: PrimitiveTypes,
  pub rust: Option<Rust>,
}
