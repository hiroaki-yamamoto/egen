use ::map_macro::hash_map_e;

use crate::entities::inputs::{
  ArrayProperty, Field, PrimitiveTypes, Structure,
};

pub fn struct_array() -> Structure {
  return Structure::new().members(hash_map_e! {
    "lst".to_string() => Field::Primitive(PrimitiveTypes::Array(
      ArrayProperty::new(PrimitiveTypes::String)
    )),
  });
}
