use crate::entities::inputs::{
  CompositeTypes, Field, PrimitiveTypes, Rust, Structure,
};
use ::map_macro::hash_map_e;

pub fn struct_simple() -> CompositeTypes {
  return CompositeTypes::Struct(Structure {
    rust: Some(Rust {
      derive: Some(vec![
        "Debug".to_string(),
        "Clone".to_string(),
        "::serde:Serialize".to_string(),
      ]),
      attrs: Some(vec!["serde(rename_all = \"camelCase\")".to_string()]),
    }),
    members: hash_map_e! {
      "float32".to_string() => Field::Primitive(PrimitiveTypes::F32),
      "float64".to_string() => Field::Primitive(PrimitiveTypes::F64),
      "int8".to_string() => Field::Primitive(PrimitiveTypes::I8),
      "int16".to_string() => Field::Primitive(PrimitiveTypes::I16),
      "int32".to_string() => Field::Primitive(PrimitiveTypes::I32),
      "int64".to_string() => Field::Primitive(PrimitiveTypes::I64),
      "int128".to_string() => Field::Primitive(PrimitiveTypes::I128),
      "uint8".to_string() => Field::Primitive(PrimitiveTypes::U8),
      "uint16".to_string() => Field::Primitive(PrimitiveTypes::U16),
      "uint32".to_string() => Field::Primitive(PrimitiveTypes::U32),
      "uint64".to_string() => Field::Primitive(PrimitiveTypes::U64),
      "uint128".to_string() => Field::Primitive(PrimitiveTypes::U128),
      "boolean".to_string() => Field::Primitive(PrimitiveTypes::Bool),
      "text".to_string() => Field::Primitive(PrimitiveTypes::String),
      "opt_float32".to_string() => Field::Primitive(PrimitiveTypes::F32),
      "opt_float64".to_string() => Field::Primitive(PrimitiveTypes::F64),
      "opt_int8".to_string() => Field::Primitive(PrimitiveTypes::I8),
      "opt_int16".to_string() => Field::Primitive(PrimitiveTypes::I16),
      "opt_int32".to_string() => Field::Primitive(PrimitiveTypes::I32),
      "opt_int64".to_string() => Field::Primitive(PrimitiveTypes::I64),
      "opt_int128".to_string() => Field::Primitive(PrimitiveTypes::I128),
      "opt_uint8".to_string() => Field::Primitive(PrimitiveTypes::U8),
      "opt_uint16".to_string() => Field::Primitive(PrimitiveTypes::U16),
      "opt_uint32".to_string() => Field::Primitive(PrimitiveTypes::U32),
      "opt_uint64".to_string() => Field::Primitive(PrimitiveTypes::U64),
      "opt_uint128".to_string() => Field::Primitive(PrimitiveTypes::U128),
      "opt_boolean".to_string() => Field::Primitive(PrimitiveTypes::Bool),
      "opt_text".to_string() => Field::Primitive(PrimitiveTypes::String),
    },
    optional: Some(vec![
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
  });
}
