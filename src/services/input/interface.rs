use crate::entities::inputs::Root;

pub trait IDecode {
  fn decode(&self, input: &str) -> Root;
}
