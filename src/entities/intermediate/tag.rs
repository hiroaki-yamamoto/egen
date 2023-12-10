use ::std::cmp::Ordering;
use ::std::hash::Hash;
use ::std::sync::Arc;

use super::error::Result as IntermediateResult;
use super::itag::ITag;

use crate::manipulators::CaseManipulator;

#[derive(Debug, Clone)]
pub struct Tag {
  raw_name: String,
  case_manip: CaseManipulator,
}

impl Tag {
  pub fn new(name: String) -> IntermediateResult<Self> {
    return Ok(Self {
      raw_name: name.clone(),
      case_manip: CaseManipulator::new(name)?,
    });
  }
}

impl PartialEq for Tag {
  fn eq(&self, other: &Self) -> bool {
    return self.raw_name == other.raw_name;
  }
}
impl Eq for Tag {}

impl PartialOrd for Tag {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return self.raw_name.partial_cmp(&other.raw_name);
  }
}
impl Ord for Tag {
  fn cmp(&self, other: &Self) -> Ordering {
    return self.raw_name.cmp(&other.raw_name);
  }
}

impl Hash for Tag {
  fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
    self.raw_name.hash(state);
  }
}

impl ITag for Tag {
  fn class_name(&self) -> Arc<String> {
    let manip = self.case_manip.clone();
    return manip.remove_non_alnum().pascal_case().build();
  }

  fn rs_module_name(&self) -> Arc<String> {
    let manip = self.case_manip.clone();
    return manip.remove_non_alnum().snake_case().build();
  }

  fn ts_module_name(&self) -> Arc<String> {
    let manip = self.case_manip.clone();
    return manip.remove_non_alnum().kebab_case().build();
  }
}

#[cfg(test)]
mod test {
  use ::std::sync::Arc;

  use super::super::itag::ITag;
  use super::Tag;

  #[test]
  fn test_class_name() {
    let tag = Tag::new("@ cla\tss_na||☺Me-te\nst|\\]';".to_string()).unwrap();
    let name = tag.class_name();
    assert!(name.as_ref() == "ClassNameTest", "name: {:?}", name);
  }

  #[test]
  fn test_rs_tag() {
    let tag = Tag::new("@ cla\tss_na||☺Me-te\nst|\\]';".to_string()).unwrap();
    let result = tag.rs_module_name();
    let correct = Arc::new("class_name_test".to_string());
    assert_eq!(result, correct);
  }

  #[test]
  fn test_ts_module_name() {
    let tag = Tag::new("@ cla\tss_na||☺Me-te\nst|\\]';".to_string()).unwrap();
    let name = tag.ts_module_name();
    assert!(name.as_ref() == "class-name-test", "name: {:?}", name);
  }
}
