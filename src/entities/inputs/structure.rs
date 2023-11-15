use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::serde::Deserialize;

#[cfg(test)]
use crate::setter;

use super::field::{Field, FieldInner};
use super::interface::{IMembers, IRustAttributes};
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
  pub members: HashMap<String, Field>,
  pub rust: Arc<Option<Rust>>,
}

impl Structure {
  #[cfg(test)]
  pub fn new() -> Self {
    return Self::default();
  }
  #[cfg(test)]
  setter!(members, HashMap<String, Field>);
  #[cfg(test)]
  setter!(rust, Arc<Option<Rust>>);
}

impl IRustAttributes for &Structure {
  fn rust(&self) -> Arc<Option<Rust>> {
    return self.rust.clone();
  }
}

impl IMembers for &Structure {
  fn members(&self) -> Vec<(String, Option<FieldInner>)> {
    let members: Vec<(String, Option<FieldInner>)> = self
      .members
      .iter()
      .map(|(key, field)| (key.clone(), Some(field.into())))
      .collect();
    return members;
  }
}
