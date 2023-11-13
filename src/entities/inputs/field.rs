use ::serde::Deserialize;

use super::primitives::PrimitiveTypes;
use super::rs::Rust;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase")]
pub struct FieldInner {
  #[serde(rename = "type")]
  pub f_type: PrimitiveTypes,
  pub rust: Option<Rust>,
  #[serde(default = "bool::default")]
  pub optional: bool,
}

#[cfg(test)]
impl FieldInner {
  pub fn new(f_type: PrimitiveTypes) -> Self {
    return Self {
      f_type,
      rust: None,
      optional: false,
    };
  }
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
        rust: None,
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
