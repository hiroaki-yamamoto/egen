use ::serde::Deserialize;

use super::enumeration::Enumeration;
use super::structure::Structure;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Root {
  Enum(Enumeration),
  Struct(Structure),
}

#[cfg(test)]
impl From<Structure> for Root {
  fn from(structure: Structure) -> Self {
    return Root::Struct(structure);
  }
}

#[cfg(test)]
impl From<Enumeration> for Root {
  fn from(enumeration: Enumeration) -> Self {
    return Root::Enum(enumeration);
  }
}
