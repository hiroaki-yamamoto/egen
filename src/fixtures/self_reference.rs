use crate::entities::inputs::{Field, PrimitiveTypes, Structure};
use map_macro::hash_map_e;

pub fn self_reference() -> Structure {
  return Structure::new().members(hash_map_e! {
    "reference".to_string() => Field::Primitive(PrimitiveTypes::Use(
      "SelReference".to_string()
    ))
  });
}
