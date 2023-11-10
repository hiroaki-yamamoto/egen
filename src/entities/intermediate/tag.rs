use crate::entities::inputs::Root;
use ::std::sync::Arc;

use super::itag::ITag;

#[derive(Debug)]
pub struct Tag {
  raw_name: String,
  root: Root,
}

impl Tag {
  pub fn new(name: String, root: Root) -> Self {
    return Self {
      raw_name: name,
      root,
    };
  }
}

impl ITag for Tag {
  fn class_name(&self) -> Arc<String> {
    let mut name = self.raw_name.to_string();
    name = name.replace("-", "_");
    let split_capitalized_names = name.split("_").map(|name| {
      let mut name = name.to_string();
      name = name.remove(0).to_uppercase().to_string() + &name;
      return name;
    });
    name = split_capitalized_names.collect::<Vec<String>>().join("");
    return Arc::new(name);
  }
}

#[cfg(test)]
mod test {
  use crate::entities::inputs::{Root, Structure};

  use super::super::itag::ITag;
  use super::Tag;

  #[test]
  pub fn test_class_name() {
    let tag = Tag::new(
      "class_name-test".to_string(),
      Root::Struct(Structure::new()),
    );
    let name = tag.class_name();
    assert!(name.as_ref() == "ClassNameTest", "name: {:?}", name);
  }
}
