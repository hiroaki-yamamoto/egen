use ::serde::Deserialize;

#[cfg(test)]
use crate::setter;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase")]
pub struct Rust {
  pub derive: Option<Vec<String>>,
  pub attrs: Option<Vec<String>>,
}

#[cfg(test)]
impl Rust {
  pub fn new() -> Self {
    return Self::default();
  }

  setter!(derive, Option<Vec<String>>);
  setter!(attrs, Option<Vec<String>>);
}
