use ::std::sync::Arc;

use super::field::FieldInner;
use super::rs::Rust;

pub trait IRustAttributes {
  fn rust(&self) -> Arc<Option<Rust>>;
}

pub trait IMembers {
  fn members(&self) -> Vec<(String, Option<FieldInner>)>;
}
