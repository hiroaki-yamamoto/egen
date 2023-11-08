use super::rs::Rust;
use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Field {
  pub name: String,
  pub rust: Option<Rust>,
}
