use ::map_macro::hash_map_e;

use crate::entities::inputs::{
  ArrayProperty, Field, FieldInner, PrimitiveTypes, Root, Rust, Structure,
};

pub fn complex() -> Root {
  return Structure::new()
    .rust(Some(
      Rust::new()
        .derive(Some(vec![
          "Debug".to_string(),
          "::serde::Serialize".to_string(),
        ]))
        .attrs(Some(vec!["serde(rename_all = \"camelCase\")".to_string()])),
    ))
    .members(hash_map_e! {
      "code".to_string() => Field::Primitive(PrimitiveTypes::U16),
      "simpleText".to_string() => Field::Primitive(PrimitiveTypes::String),
      "detailedText".to_string() => Field::Inner(FieldInner {
        f_type: PrimitiveTypes::String,
        rust: Some(Rust::new().attrs(Some(vec!["serde(rename = \"detail\")".to_string()]))),
        optional: false,
      }),
      "simpleArray".to_string() => Field::Inner(FieldInner {
        f_type: PrimitiveTypes::Array(ArrayProperty::new(PrimitiveTypes::String)),
        rust: Some(Rust::new().attrs(Some(vec!["serde(rename = \"lst\")".to_string()]))),
        optional: false,
      }),
      "referenceArray".to_string() => Field::Primitive(PrimitiveTypes::Array(
        ArrayProperty::new(PrimitiveTypes::Use("SimpleStructure".to_string()))
      )),
      "selfReferenceArray".to_string() => Field::Primitive(PrimitiveTypes::Array(
        ArrayProperty::new(PrimitiveTypes::Use("Complex".to_string()))
      )),
    }).into();
}
