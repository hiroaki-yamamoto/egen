use ::serde::Deserialize;

use super::composite::CompositeTypes;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayProperty {
  pub rust: Option<Rust>,
  pub item: Box<CompositeTypes>,
}
