use ::serde::Deserialize;

use super::rs::Rust;

#[cfg(test)]
use crate::setter;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Enumeration {
  rust: Option<Rust>,
  members: Vec<String>,
}

#[cfg(test)]
impl Enumeration {
  pub fn new() -> Self {
    return Self::default();
  }
  setter!(rust, Option<Rust>);
  setter!(members, Vec<String>);
}
