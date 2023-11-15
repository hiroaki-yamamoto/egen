use ::std::sync::Arc;

use ::serde::Deserialize;

use super::field::FieldInner;
use super::interface::{IMembers, IRustAttributes};
use super::rs::Rust;

#[cfg(test)]
use crate::setter;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Enumeration {
  rust: Arc<Option<Rust>>,
  members: Vec<String>,
}

#[cfg(test)]
impl Enumeration {
  pub fn new() -> Self {
    return Self::default();
  }
  setter!(rust, Arc<Option<Rust>>);
  setter!(members, Vec<String>);
}

impl IRustAttributes for &Enumeration {
  fn rust(&self) -> Arc<Option<Rust>> {
    return self.rust.clone();
  }
}

impl IMembers for &Enumeration {
  fn members(&self) -> Vec<(String, Option<FieldInner>)> {
    let members: Vec<(String, Option<FieldInner>)> =
      self.members.iter().map(|key| (key.clone(), None)).collect();
    return members;
  }
}
