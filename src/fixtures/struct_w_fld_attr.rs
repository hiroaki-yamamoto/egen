use ::map_macro::hash_map_e;
use ::std::sync::Arc;

use crate::entities::inputs::{
  Field, FieldInner, PrimitiveTypes, Root, Rust, Structure,
};

pub fn struct_w_fld_attr() -> Root {
  return Structure::new()
    .members(hash_map_e! {
      "test".to_string() => Field::Inner(FieldInner{
        f_type: PrimitiveTypes::String,
        rust: Arc::new(Some(Rust {
          derive: vec!["Debug".to_string()].into(),
          attrs: vec!["serde(rename = \"test2\")".to_string()].into(),
        })),
        optional: false,
      })
    })
    .into();
}
