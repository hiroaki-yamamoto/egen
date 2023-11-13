use ::std::sync::Arc;

use super::rs::Rust;

pub trait IRustAttributes {
  fn rust(&self) -> Arc<Option<Rust>>;
}
