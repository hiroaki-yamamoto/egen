use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::serde::Deserialize;

#[cfg(test)]
use crate::setter;

use super::field::Field;
use super::interface::IRustAttributes;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
  pub members: HashMap<String, Field>,
  pub rust: Option<Rust>,
}

impl Structure {
  #[cfg(test)]
  pub fn new() -> Self {
    return Self::default();
  }
  #[cfg(test)]
  setter!(members, HashMap<String, Field>);
  #[cfg(test)]
  setter!(rust, Option<Rust>);
}

impl IRustAttributes for Structure {
  fn rust(&self) -> Arc<Option<Rust>> {
    return Arc::new(self.rust);
  }
}
