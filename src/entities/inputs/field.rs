use ::serde::Deserialize;
use ::std::sync::Arc;

#[cfg(test)]
use crate::setter;

use super::interface::IRustAttributes;
use super::primitives::PrimitiveTypes;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase")]
pub struct FieldInner {
  #[serde(rename = "type")]
  pub f_type: PrimitiveTypes,
  pub rust: Arc<Option<Rust>>,
  #[serde(default = "bool::default")]
  pub optional: bool,
}

impl IRustAttributes for &FieldInner {
  fn rust(&self) -> Arc<Option<Rust>> {
    return self.rust.clone();
  }
}

impl IRustAttributes for FieldInner {
  fn rust(&self) -> Arc<Option<Rust>> {
    return self.rust.clone();
  }
}

#[cfg(test)]
impl FieldInner {
  pub fn new(f_type: PrimitiveTypes) -> Self {
    return Self {
      f_type,
      rust: Arc::new(None),
      optional: false,
    };
  }
  // setter!(f_type, PrimitiveTypes);
  // setter!(rust, Arc<Option<Rust>>);
  setter!(optional, bool);
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase", untagged)]
pub enum Field {
  Inner(FieldInner),
  Primitive(PrimitiveTypes),
}

impl From<Field> for FieldInner {
  fn from(field: Field) -> Self {
    match field {
      Field::Inner(inner) => inner,
      Field::Primitive(primitive) => FieldInner {
        f_type: primitive,
        optional: false,
        rust: Arc::new(None),
      },
    }
  }
}

#[cfg(test)]
mod test {
  use super::PrimitiveTypes;
  use super::{Field, FieldInner};

  #[test]
  fn test_field_inner_from_field() {
    let correct = FieldInner::new(PrimitiveTypes::String);
    let field = Field::Inner(correct.clone());
    let field_inner: FieldInner = field.into();
    assert_eq!(field_inner, correct);
  }

  #[test]
  fn test_field_inner_from_primitive() {
    let correct = FieldInner::new(PrimitiveTypes::String);
    let field = Field::Primitive(PrimitiveTypes::String);
    let field_inner: FieldInner = field.into();
    assert_eq!(field_inner, correct);
  }
}
