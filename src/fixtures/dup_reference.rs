use ::map_macro::hash_map_e;

use crate::entities::inputs::{Field, PrimitiveTypes, Root, Structure};

pub fn dup_references() -> Root {
  return Structure::new()
    .members(hash_map_e! {
      "reference1".to_string() => Field::Primitive(
        PrimitiveTypes::Use("SimpleStructure".to_string())
      ),
      "reference2".to_string() => Field::Primitive(
        PrimitiveTypes::Use("SimpleStructure".to_string())
      ),
    })
    .into();
}
