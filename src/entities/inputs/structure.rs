pub use ::serde::Deserialize;
pub use ::std::collections::HashMap;

use super::primitives::PrimitiveTypes;

#[derive(Debug, Deserialize)]
pub struct Structure {
  pub members: HashMap<String, PrimitiveTypes>,
}
