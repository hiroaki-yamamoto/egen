use ::std::path::Path;

use crate::entities::inputs::Root;
use crate::entities::intermediate::Tag;

use super::error::ImportExtractorResult as ExtractionResult;
use super::error::InputProcessResult as InputResult;

pub trait IInputProcessor {
  /// Process a file into a tag.
  fn process(&self, path: impl AsRef<Path>) -> InputResult<(Tag, Root)>;
}

pub trait IImportExtractor {
  fn extract(&self, root_tag: &Tag, root: &Root)
    -> ExtractionResult<Vec<Tag>>;
}
