use crate::entities::inputs::{Field, PrimitiveTypes, Root, Rust, Structure};
use ::map_macro::hash_map_e;

pub fn struct_simple() -> Root {
  return Structure::new()
    .rust(Some(Rust {
      derive: Some(vec![
        "Debug".to_string(),
        "Clone".to_string(),
        "::serde:Serialize".to_string(),
      ]),
      attrs: Some(vec!["serde(rename_all = \"camelCase\")".to_string()]),
    }))
    .members(hash_map_e! {
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
      "optFloat32".to_string() => Field::Primitive(PrimitiveTypes::F32),
      "optFloat64".to_string() => Field::Primitive(PrimitiveTypes::F64),
      "optInt8".to_string() => Field::Primitive(PrimitiveTypes::I8),
      "optInt16".to_string() => Field::Primitive(PrimitiveTypes::I16),
      "optInt32".to_string() => Field::Primitive(PrimitiveTypes::I32),
      "optInt64".to_string() => Field::Primitive(PrimitiveTypes::I64),
      "optInt128".to_string() => Field::Primitive(PrimitiveTypes::I128),
      "optUint8".to_string() => Field::Primitive(PrimitiveTypes::U8),
      "optUint16".to_string() => Field::Primitive(PrimitiveTypes::U16),
      "optUint32".to_string() => Field::Primitive(PrimitiveTypes::U32),
      "optUint64".to_string() => Field::Primitive(PrimitiveTypes::U64),
      "optUint128".to_string() => Field::Primitive(PrimitiveTypes::U128),
      "optBoolean".to_string() => Field::Primitive(PrimitiveTypes::Bool),
      "optText".to_string() => Field::Primitive(PrimitiveTypes::String),
    })
    .optional(Some(vec![
      "optFloat32".to_string(),
      "optFloat64".to_string(),
      "optInt8".to_string(),
      "optInt16".to_string(),
      "optInt32".to_string(),
      "optInt64".to_string(),
      "optInt128".to_string(),
      "optUint8".to_string(),
      "optUint16".to_string(),
      "optUint32".to_string(),
      "optUint64".to_string(),
      "optUint128".to_string(),
      "optBoolean".to_string(),
      "optText".to_string(),
    ]))
    .into();
}
