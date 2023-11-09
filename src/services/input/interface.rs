use ::std::io::Read;

use super::error::Result;
use crate::entities::inputs::CompositeTypes;

pub trait IDecode {
  fn decode(&self, input: impl Read) -> Result<CompositeTypes>;
}
