use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Types {
    Array,
    Bool,
    String,
    F32,
    F64,
    I8,
    I32,
    I64,
    I128,
    U8,
    U32,
    U64,
    U128,
}
