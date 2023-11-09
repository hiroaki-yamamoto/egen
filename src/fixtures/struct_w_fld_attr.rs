use ::map_macro::hash_map_e;

use crate::entities::inputs::{
  CompositeTypes, Field, FieldInner, PrimitiveTypes, Rust, Structure,
};

pub fn struct_w_fld_attr() -> CompositeTypes {
  return CompositeTypes::Struct(Structure::new().members(hash_map_e! {
    "test".to_string() => Field::Inner(FieldInner{
      f_type: PrimitiveTypes::String,
      rust: Some(Rust {
        derive: vec!["Debug".to_string()].into(),
        attrs: vec!["serde(rename = \"test2\")".to_string()].into(),
      }),})
  }));
}
