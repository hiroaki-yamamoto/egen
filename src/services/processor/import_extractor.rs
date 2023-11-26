use ::std::collections::HashSet;

use crate::entities::inputs::{Field, PrimitiveTypes, Root};
use crate::entities::intermediate::{ITag, Tag};

use super::error::{ImportExtractorError, ImportExtractorResult};
use super::interface::IImportExtractor;

pub struct ImportExtractor {
  tags: Vec<Tag>,
}

impl ImportExtractor {
  pub fn new(tags: Vec<Tag>) -> Self {
    Self { tags }
  }

  fn extract_by_type(
    &self,
    tag_root: &Tag,
    field_type: &PrimitiveTypes,
  ) -> ImportExtractorResult<HashSet<Tag>> {
    let mut result: HashSet<Tag> = HashSet::new();
    match field_type {
      PrimitiveTypes::Use(name) => {
        let matched = self
          .tags
          .iter()
          .find(|&tag| tag.class_name().as_ref() == name);
        if let Some(matched) = matched {
          if matched != tag_root {
            result.insert(matched.clone());
          }
        } else {
          return Err(ImportExtractorError::ModuleNotFound(name.clone()));
        }
      }
      PrimitiveTypes::Array(arr) => {
        let arr_result = self.extract_by_type(tag_root, &arr.item)?;
        result = &result | &arr_result;
      }
      _ => {}
    };
    return Ok(result);
  }
}

impl IImportExtractor for ImportExtractor {
  fn extract(
    &self,
    tag_root: &Tag,
    root: &Root,
  ) -> ImportExtractorResult<Vec<Tag>> {
    let mut result: HashSet<Tag> = HashSet::new();
    match root {
      Root::Struct(root) => {
        for (_, field) in root.members.iter() {
          let fld_type = match field {
            Field::Inner(inner) => &inner.f_type,
            Field::Primitive(primitive) => primitive,
          };
          result = &result | &self.extract_by_type(tag_root, fld_type)?;
        }
      }
      Root::Enum(_) => {}
    }
    let result: Vec<Tag> = result.into_iter().collect();
    return Ok(result);
  }
}

#[cfg(test)]
mod test {
  use ::map_macro::hash_set;
  use ::std::collections::HashSet;

  use super::IImportExtractor;
  use super::ImportExtractor;
  use super::ImportExtractorError;
  use crate::entities::intermediate::Tag;
  use crate::fixtures::complex::complex;
  use crate::fixtures::dup_reference::dup_references;
  use crate::fixtures::not_found::not_found;
  use crate::fixtures::reference::reference;
  use crate::fixtures::self_reference::self_reference;

  #[test]
  fn test_simple_extraction() {
    let correct = vec![Tag::new("simple_structure".to_string()).unwrap()];
    let doc = reference();

    let me = Tag::new("reference".to_string()).unwrap();
    let extractor = ImportExtractor::new(vec![
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("complex_structure".to_string()).unwrap(),
      me.clone(),
    ]);
    let result = extractor.extract(&me, &doc).unwrap();
    assert!(result == correct, "result: {:?}", result);
  }

  #[test]
  fn test_2reference_extraction() {
    let correct = vec![Tag::new("simple_structure".to_string()).unwrap()];
    let doc = dup_references();

    let me = Tag::new("dup_reference".to_string()).unwrap();
    let extractor = ImportExtractor::new(vec![
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("complex_structure".to_string()).unwrap(),
      Tag::new("reference".to_string()).unwrap(),
      me.clone(),
    ]);

    let result = extractor.extract(&me, &doc).unwrap();
    assert!(result == correct, "result: {:?}", result);
  }

  #[test]
  fn test_self_reference_extraction() {
    let correct: Vec<Tag> = vec![];
    let doc = self_reference();

    let me = Tag::new("self_reference".to_string()).unwrap();
    let extractor = ImportExtractor::new(vec![
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("complex_structure".to_string()).unwrap(),
      Tag::new("reference".to_string()).unwrap(),
      Tag::new("two_reference".to_string()).unwrap(),
      me.clone(),
    ]);

    let result = extractor.extract(&me, &doc).unwrap();
    assert!(result == correct, "result: {:?}", result)
  }

  #[test]
  fn test_array_with_reference() {
    let correct = hash_set! {
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("reference".to_string()).unwrap(),
    };
    let doc = complex();

    let me = Tag::new("complex".to_string()).unwrap();
    let extractor = ImportExtractor::new(vec![
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("complex_structure".to_string()).unwrap(),
      Tag::new("reference".to_string()).unwrap(),
      Tag::new("two_reference".to_string()).unwrap(),
      Tag::new("self_reference".to_string()).unwrap(),
      me.clone(),
    ]);

    let result: HashSet<_> =
      extractor.extract(&me, &doc).unwrap().into_iter().collect();
    assert!(result == correct, "result: {:?}", result)
  }

  #[test]
  fn test_not_found() {
    let doc = not_found();
    let me = Tag::new("not_found".to_string()).unwrap();
    let extractor = ImportExtractor::new(vec![
      Tag::new("simple_structure".to_string()).unwrap(),
      Tag::new("complex_structure".to_string()).unwrap(),
      Tag::new("reference".to_string()).unwrap(),
      Tag::new("two_reference".to_string()).unwrap(),
      Tag::new("self_reference".to_string()).unwrap(),
      me.clone(),
    ]);

    let result = extractor.extract(&me, &doc).unwrap_err();
    assert!(
      result
        == ImportExtractorError::ModuleNotFound(
          "__NotFoundStructure__".to_string(),
        ),
    )
  }
}
