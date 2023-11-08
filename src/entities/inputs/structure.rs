use ::std::collections::HashMap;

use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;

#[derive(Debug, Deserialize)]
pub struct Structure {
  pub members: HashMap<String, PrimitiveTypes>,
  pub derive: Option<Vec<String>>,
}
