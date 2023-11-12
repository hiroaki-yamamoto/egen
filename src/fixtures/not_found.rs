use crate::entities::inputs::{Field, PrimitiveTypes, Root, Structure};
use ::map_macro::hash_map_e;

pub fn not_found() -> Root {
  return Structure::new()
    .members(hash_map_e! {
      "reference".to_string() => Field::Primitive(PrimitiveTypes::Use(
        "__NotFoundStructure__".to_string()
      ).into()),
    })
    .into();
}
