use ::serde::Deserialize;

use super::enumeration::Enumeration;
use super::structure::Structure;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Root {
  Enumeration(Enumeration),
  Structure(Structure),
}

#[cfg(test)]
impl From<Structure> for Root {
  fn from(structure: Structure) -> Self {
    return Root::Structure(structure);
  }
}

#[cfg(test)]
impl From<Enumeration> for Root {
  fn from(enumeration: Enumeration) -> Self {
    return Root::Enumeration(enumeration);
  }
}
