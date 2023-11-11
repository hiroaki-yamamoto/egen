use crate::entities::intermediate::Tag;

use super::interface::IImportExtractor;

pub struct ImportExtractor {
  tags: Vec<Tag>,
}

impl ImportExtractor {
  pub fn new(tags: Vec<Tag>) -> Self {
    Self { tags }
  }
}

impl IImportExtractor for ImportExtractor {
  fn extract(&self, root: &crate::entities::inputs::Root) -> Vec<Tag> {
    let mut result = Vec::new();

    return result;
  }
}

#[cfg(test)]
mod test {
  use super::ImportExtractor;
  use crate::entities::intermediate::Tag;
  use crate::fixtures::reference::reference;

  #[test]
  fn test_simple_extraction() {
    let correct = vec![Tag::new("simple_structure".to_string())];
    let doc = reference();

    let extractor = ImportExtractor::new(vec![
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("complex_structure".to_string()).unwrap(),
      Tag::new("reference".to_string()).unwrap(),
    ]);
    let result = extractor.extract(&doc);
    assert!(result == correct, "result: {:?}", result);
  }
}
