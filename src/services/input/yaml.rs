use super::interface::IDecode;

use serde_yaml::Result;

pub struct Yaml;

#[cfg(test)]
mod tests {
  use crate::entities::inputs::{
    Field, FieldInner, PrimitiveTypes, Root, Rust, Structure,
  };
  use ::map_macro::hash_map_e;
  const simple: &'static str = include_str!("../../fixtures/simple.yml");

  #[test]
  fn test_simple() {
    todo!("members");
    let decoded_struct = Root {
      struct_type: PrimitiveTypes::Struct(Structure {
        derive: Some(vec![
          "Debug".to_string(),
          "Clone".to_string(),
          "::serde:Serialize".to_string(),
        ]),
        attrs: Some(vec!["serde(rename = \"test_array1\")".to_string()]),
        members: hash_map_e! {
          "arr1".to_string() => Field::Inner(FieldInner {
            f_type: PrimitiveTypes::String,
            rust: Some(Rust{
              derive: Some(vec!["Debug".to_string(), "Clone".to_string()]),
              attrs: Some(vec!["serde(rename = \"test_array1\")".to_string()])
            })
          })
        },
        optional: Some(vec![
          "opt_arr1".to_string(),
          "opt_arr2".to_string(),
          "opt_test_array3".to_string(),
          "opt_float32".to_string(),
          "opt_float64".to_string(),
          "opt_int8".to_string(),
          "opt_int16".to_string(),
          "opt_int32".to_string(),
          "opt_int64".to_string(),
          "opt_int128".to_string(),
          "opt_uint8".to_string(),
          "opt_uint16".to_string(),
          "opt_uint32".to_string(),
          "opt_uint64".to_string(),
          "opt_uint128".to_string(),
          "opt_boolean".to_string(),
          "opt_text".to_string(),
        ]),
      }),
    };
  }
}
