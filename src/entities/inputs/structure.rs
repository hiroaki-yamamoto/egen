use ::std::collections::HashMap;

use ::serde::Deserialize;

use super::field::Field;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
  pub members: HashMap<String, Field>,
  pub derive: Option<Vec<String>>,
  pub attrs: Option<Vec<String>>,
  pub optional: Option<Vec<String>>,
}
