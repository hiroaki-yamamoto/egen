use ::map_macro::hash_map_e;

use crate::entities::inputs::{Field, PrimitiveTypes, Root, Structure};

pub fn reference() -> Root {
  return Structure::new()
    .members(hash_map_e! {
      "reference".to_string() => Field::Primitive(
        PrimitiveTypes::Use("SimpleStructure".to_string())
      )
    })
    .into();
}
