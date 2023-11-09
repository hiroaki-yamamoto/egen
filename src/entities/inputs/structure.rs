use ::std::collections::HashMap;

use ::serde::Deserialize;

use super::field::Field;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
  pub members: HashMap<String, Field>,
  pub rust: Option<Rust>,
  pub optional: Option<Vec<String>>,
}
