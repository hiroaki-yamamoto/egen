use ::std::string::ToString;

use ::serde::{Deserialize, Serialize};

use super::array::ArrayProperty;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveTypes {
  Bool,
  String,
  F32,
  F64,
  I8,
  I16,
  I32,
  I64,
  I128,
  U8,
  U16,
  U32,
  U64,
  U128,
  Use(String),
  Array(ArrayProperty),
}

impl ToString for PrimitiveTypes {
  fn to_string(&self) -> String {
    return match self {
      PrimitiveTypes::Bool => "bool".to_string(),
      PrimitiveTypes::String => "String".to_string(),
      PrimitiveTypes::F32 => "f32".to_string(),
      PrimitiveTypes::F64 => "f64".to_string(),
      PrimitiveTypes::I8 => "i8".to_string(),
      PrimitiveTypes::I16 => "i16".to_string(),
      PrimitiveTypes::I32 => "i32".to_string(),
      PrimitiveTypes::I64 => "i64".to_string(),
      PrimitiveTypes::I128 => "i128".to_string(),
      PrimitiveTypes::U8 => "u8".to_string(),
      PrimitiveTypes::U16 => "u16".to_string(),
      PrimitiveTypes::U32 => "u32".to_string(),
      PrimitiveTypes::U64 => "u64".to_string(),
      PrimitiveTypes::U128 => "u128".to_string(),
      PrimitiveTypes::Use(s) => format!("Box<{}>", s.to_string()),
      PrimitiveTypes::Array(arr) => format!("Vec<{}>", arr.item.to_string()),
    };
  }
}
