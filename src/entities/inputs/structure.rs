use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::serde::Deserialize;

#[cfg(test)]
use crate::setter;

use super::field::{Field, FieldInner};
use super::interface::{IMembers, IRustAttributes, ITSAttributes};
use super::rs::Rust;
use super::ts::TypeScript;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
  pub members: HashMap<String, Field>,
  pub rust: Arc<Option<Rust>>,
  pub typescript: Arc<Option<TypeScript>>,
}

#[cfg(test)]
impl Structure {
  pub fn new() -> Self {
    return Self::default();
  }
  setter!(members, HashMap<String, Field>);
  setter!(rust, Arc<Option<Rust>>);
  setter!(typescript, Arc<Option<TypeScript>>);
}

impl IRustAttributes for &Structure {
  fn rust(&self) -> Arc<Option<Rust>> {
    return self.rust.clone();
  }
}

impl ITSAttributes for &Structure {
  fn typescript(&self) -> Arc<Option<TypeScript>> {
    return self.typescript.clone();
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
