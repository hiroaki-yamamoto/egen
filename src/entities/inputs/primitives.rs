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
    let conv_type = match self {
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
    return conv_type.to_string();
  }
}

impl PrimitiveTypes {
  fn min_max(&self) -> Option<(String, String)> {
    return match self {
      PrimitiveTypes::F32 => {
        Some((f32::MIN.to_string(), f32::MAX.to_string()))
      }
      PrimitiveTypes::F64 => {
        Some((f64::MIN.to_string(), f64::MAX.to_string()))
      }
      PrimitiveTypes::I8 => Some((i8::MIN.to_string(), i8::MAX.to_string())),
      PrimitiveTypes::I16 => {
        Some((i16::MIN.to_string(), i16::MAX.to_string()))
      }
      PrimitiveTypes::I32 => {
        Some((i32::MIN.to_string(), i32::MAX.to_string()))
      }
      PrimitiveTypes::I64 => {
        Some((i64::MIN.to_string(), i64::MAX.to_string()))
      }
      PrimitiveTypes::I128 => {
        Some((i128::MIN.to_string(), i128::MAX.to_string()))
      }
      PrimitiveTypes::U8 => Some((u8::MIN.to_string(), u8::MAX.to_string())),
      PrimitiveTypes::U16 => {
        Some((u16::MIN.to_string(), u16::MAX.to_string()))
      }
      PrimitiveTypes::U32 => {
        Some((u32::MIN.to_string(), u32::MAX.to_string()))
      }
      PrimitiveTypes::U64 => {
        Some((u64::MIN.to_string(), u64::MAX.to_string()))
      }
      PrimitiveTypes::U128 => {
        Some((u128::MIN.to_string(), u128::MAX.to_string()))
      }
      _ => None,
    };
  }
  pub fn to_zod_ts(&self) -> String {
    let (min, max) =
      self.min_max().unwrap_or(("".to_string(), "".to_string()));
    let conv_type = match self {
      PrimitiveTypes::Bool => "z.boolean()".to_string(),
      PrimitiveTypes::String => "z.string()".to_string(),
      PrimitiveTypes::F32
      | PrimitiveTypes::F64
      | PrimitiveTypes::I8
      | PrimitiveTypes::I16
      | PrimitiveTypes::I32
      | PrimitiveTypes::I64
      | PrimitiveTypes::I128
      | PrimitiveTypes::U8
      | PrimitiveTypes::U16
      | PrimitiveTypes::U32
      | PrimitiveTypes::U64
      | PrimitiveTypes::U128 => {
        format!("z.number().max({}).min({})", max, min)
      }
      PrimitiveTypes::Use(s) => s.to_string(),
      PrimitiveTypes::Array(arr) => {
        format!("z.array({})", arr.item.to_zod_ts())
      }
    };
    return conv_type.to_string();
  }
}
