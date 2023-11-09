use super::error::Result;
use crate::entities::inputs::Root;

pub trait IDecode {
  fn decode(&self, input: &str) -> Result<Root>;
}
