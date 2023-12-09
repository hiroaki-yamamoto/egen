use ::serde::{Deserialize, Serialize};

use super::rename::Rename;

#[cfg(test)]
use crate::setter;

#[derive(Debug, Deserialize, PartialEq, Eq, Default, Serialize)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase")]
pub struct TypeScript {
  pub rename: Rename,
}

#[cfg(test)]
impl TypeScript {
  pub fn new() -> Self {
    return Self::default();
  }

  setter!(rename, Rename);
}
