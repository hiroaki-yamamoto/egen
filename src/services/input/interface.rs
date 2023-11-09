use ::std::io::Read;

use super::error::Result;
use crate::entities::inputs::Root;

pub trait IDecode {
  fn decode(&self, input: impl Read) -> Result<Root>;
}
