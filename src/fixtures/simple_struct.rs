use ::std::sync::Arc;

use crate::entities::inputs::{
  Field, FieldInner, PrimitiveTypes, Root, Rust, Structure,
};
use ::map_macro::hash_map_e;

pub fn struct_simple() -> Root {
  return Structure::new()
    .rust(Arc::new(Some(Rust {
      derive: Some(vec![
        "Debug".to_string(),
        "Clone".to_string(),
        "::serde:Serialize".to_string(),
      ]),
      attrs: Some(vec!["serde(rename_all = \"camelCase\")".to_string()]),
    })))
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
      "optFloat32".to_string() => Field::Inner(
        FieldInner::new(PrimitiveTypes::F32).optional(true)
      ),
      "optFloat64".to_string() => Field::Inner(
        FieldInner::new(PrimitiveTypes::F64).optional(true)
      ),
      "optInt8".to_string() =>    Field::Inner(
        FieldInner::new(PrimitiveTypes::I8).optional(true)
      ),
      "optInt16".to_string() =>   Field::Inner(
        FieldInner::new(PrimitiveTypes::I16).optional(true)
      ),
      "optInt32".to_string() =>   Field::Inner(
        FieldInner::new(PrimitiveTypes::I32).optional(true)
      ),
      "optInt64".to_string() =>   Field::Inner(
        FieldInner::new(PrimitiveTypes::I64).optional(true)
      ),
      "optInt128".to_string() =>  Field::Inner(
        FieldInner::new(PrimitiveTypes::I128).optional(true)
      ),
      "optUint8".to_string() =>   Field::Inner(
        FieldInner::new(PrimitiveTypes::U8).optional(true)
      ),
      "optUint16".to_string() =>  Field::Inner(
        FieldInner::new(PrimitiveTypes::U16).optional(true)
      ),
      "optUint32".to_string() =>  Field::Inner(
        FieldInner::new(PrimitiveTypes::U32).optional(true)
      ),
      "optUint64".to_string() =>  Field::Inner(
        FieldInner::new(PrimitiveTypes::U64).optional(true)
      ),
      "optUint128".to_string() => Field::Inner(
        FieldInner::new(PrimitiveTypes::U128).optional(true)
      ),
      "optBoolean".to_string() => Field::Inner(
        FieldInner::new(PrimitiveTypes::Bool).optional(true)
      ),
      "optText".to_string() =>    Field::Inner(
        FieldInner::new(PrimitiveTypes::String).optional(true)
      ),
    })
    .into();
}
