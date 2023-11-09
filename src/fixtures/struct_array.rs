use ::map_macro::hash_map_e;

use crate::entities::inputs::{
  ArrayProperty, Field, PrimitiveTypes, Root, Structure,
};

pub fn struct_array() -> Root {
  return Structure::new()
    .members(hash_map_e! {
      "lst".to_string() => Field::Primitive(PrimitiveTypes::Array(
        ArrayProperty::new(PrimitiveTypes::String)
      )),
    })
    .into();
}
