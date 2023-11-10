use ::std::sync::Arc;

use ::regex::Regex;

use crate::entities::inputs::Root;

use super::error::Result as IntermediateResult;
use super::itag::ITag;

#[derive(Debug)]
pub struct Tag {
  raw_name: String,
  root: Root,
  re: Regex,
}

impl Tag {
  pub fn new(name: String, root: Root) -> IntermediateResult<Self> {
    return Ok(Self {
      raw_name: name,
      root,
      re: Regex::new(r"[[:^alnum:]]")?,
    });
  }
}

impl ITag for Tag {
  fn class_name(&self) -> Arc<String> {
    let mut name = self.raw_name.to_string().replace("-", "_");
    let split_capitalized_names = name.split("_").map(|name| {
      let mut name = self.re.replace_all(name, "").to_string();
      name = name.remove(0).to_uppercase().to_string() + &name.to_lowercase();
      return name;
    });
    name = split_capitalized_names.collect::<Vec<String>>().join("");
    return Arc::new(name);
  }

  fn rs_module_name(&self) -> Arc<String> {
    let mut name = self.raw_name.to_string().replace("-", "_");
    let split_capitalized_names = name.split("_").map(|name| {
      let name = self.re.replace_all(&name, "").to_lowercase();
      return name;
    });
    name = split_capitalized_names.collect::<Vec<String>>().join("_");
    return Arc::new(name);
  }

  fn ts_module_name(&self) -> Arc<String> {
    let name = self.rs_module_name().replace("_", "-");
    return Arc::new(name);
  }
}

#[cfg(test)]
mod test {
  use crate::entities::inputs::{Root, Structure};

  use super::super::itag::ITag;
  use super::Tag;

  #[test]
  fn test_class_name() {
    let tag = Tag::new(
      "@ cla\tss_na||☺Me-te\nst|\\]';".to_string(),
      Root::Struct(Structure::new()),
    )
    .unwrap();
    let name = tag.class_name();
    assert!(name.as_ref() == "ClassNameTest", "name: {:?}", name);
  }

  #[test]
  fn test_rs_module_name() {
    let tag = Tag::new(
      "@ cla\tss_na||☺Me-te\nst|\\]';".to_string(),
      Root::Struct(Structure::new()),
    )
    .unwrap();
    let name = tag.rs_module_name();
    assert!(name.as_ref() == "class_name_test", "name: {:?}", name);
  }

  #[test]
  fn test_ts_module_name() {
    let tag = Tag::new(
      "@ cla\tss_na||☺Me-te\nst|\\]';".to_string(),
      Root::Struct(Structure::new()),
    )
    .unwrap();
    let name = tag.ts_module_name();
    assert!(name.as_ref() == "class-name-test", "name: {:?}", name);
  }
}
