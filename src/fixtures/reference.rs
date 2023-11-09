use ::map_macro::hash_map_e;

use crate::entities::inputs::{Field, PrimitiveTypes, Structure};

pub fn reference() -> Structure {
  return Structure::new().members(hash_map_e! {
    "reference".to_string() => Field::Primitive(
      PrimitiveTypes::Use("SimpleStructure".to_string())
    )
  });
}
