use crate::entities::inputs::{Field, PrimitiveTypes, Root};
use crate::entities::intermediate::{ITag, Tag};

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
  fn extract(&self, root: &Root) -> Vec<Tag> {
    let mut result = Vec::new();
    match root {
      Root::Struct(root) => {
        for (_, field) in root.members.iter() {
          let fld_type = match field {
            Field::Inner(inner) => &inner.f_type,
            Field::Primitive(primitive) => primitive,
          };
          match fld_type {
            PrimitiveTypes::Use(name) => {
              let matched = self
                .tags
                .iter()
                .find(|tag| tag.class_name().as_ref() == name);
              if let Some(matched) = matched {
                result.push(matched.clone());
              }
            }
            _ => {}
          }
        }
      }
      Root::Enum(_) => {}
    }
    return result;
  }
}

#[cfg(test)]
mod test {
  use super::IImportExtractor;
  use super::ImportExtractor;
  use crate::entities::intermediate::Tag;
  use crate::fixtures::reference::reference;

  #[test]
  fn test_simple_extraction() {
    let correct = vec![Tag::new("simple_structure".to_string()).unwrap()];
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
