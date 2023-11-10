use ::std::path::Path;

use crate::entities::inputs::Root;
use crate::entities::intermediate::Tag;

use super::error::InputProcessResult as Result;

pub trait IInputProcessor {
  /// Process a file into a tag.
  fn process(&self, path: impl AsRef<Path>) -> Result<(Tag, Root)>;
}
