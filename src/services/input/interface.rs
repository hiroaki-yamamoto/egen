use ::std::io::Read;

use super::error::Result;
use crate::entities::inputs::Root;

pub trait IDecode {
  type Reader: Read;
  fn decode(&self, input: Self::Reader) -> Result<Root>;
}
