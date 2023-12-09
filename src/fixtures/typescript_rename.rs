use ::map_macro::hash_map_e;

use crate::entities::inputs::{
  Field, FieldInner, PrimitiveTypes, Rename, Root, Structure, TypeScript,
};

pub fn typescript_rename() -> Root {
  return Structure::new()
    .typescript(Some(TypeScript::new().rename(Rename::CamelCase)).into())
    .members(hash_map_e! {
      "test_field1".to_string() => Field::Primitive(PrimitiveTypes::String),
      "test_field2".to_string() => Field::Inner(
        FieldInner::new(PrimitiveTypes::String).typescript(
          Some(TypeScript::new().rename(Rename::SnakeCase)).into()
        )
      ),
    })
    .into();
}
