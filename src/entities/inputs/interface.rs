use ::std::sync::Arc;

use super::field::FieldInner;
use super::rs::Rust;
use super::ts::TypeScript;

pub trait IRustAttributes {
  fn rust(&self) -> Arc<Option<Rust>>;
}

pub trait ITSAttributes {
  fn typescript(&self) -> Arc<Option<TypeScript>>;
}

pub trait IMembers {
  fn members(&self) -> Vec<(String, Option<FieldInner>)>;
}
