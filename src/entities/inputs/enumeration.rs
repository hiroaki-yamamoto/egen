use ::serde::Deserialize;

use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Enumeration {
  rust: Option<Rust>,
  members: Vec<String>,
}
