use ::std::sync::Arc;

use crate::entities::inputs::Rename;
use ::regex::{Error as RegexError, Regex};

#[derive(Debug, Clone)]
pub struct CaseManipulator {
  non_alnum_regex: Regex,
  text: Arc<String>,
}

impl CaseManipulator {
  pub fn new<T>(text: T) -> Result<Self, RegexError>
  where
    T: ToString,
  {
    return Ok(Self {
      non_alnum_regex: Regex::new(r"[^0-9a-zA-Z\-\_]")?,
      text: Arc::new(text.to_string()),
    });
  }

  pub fn remove_non_alnum(mut self) -> Self {
    self.text = Arc::new(
      self
        .non_alnum_regex
        .replace_all(self.text.as_ref(), "")
        .to_string(),
    );
    return self;
  }

  pub fn camel_case(self) -> Self {
    let regex = self.non_alnum_regex.clone();
    let mut text = self.pascal_case().text.to_string();
    text = text.remove(0).to_lowercase().to_string() + &text;
    return Self {
      text: Arc::new(text),
      non_alnum_regex: regex,
    };
  }

  pub fn pascal_case(mut self) -> Self {
    let text = self.text.replace("-", "_");
    let split_capitalized_text = text.split("_").map(|text| {
      let mut text = text.to_string();
      let text =
        text.remove(0).to_uppercase().to_string() + &text.to_lowercase();
      return text;
    });
    self.text =
      Arc::new(split_capitalized_text.collect::<Vec<String>>().join(""));
    return self;
  }

  pub fn snake_case(mut self) -> Self {
    let text = self.text.replace("-", "_");
    let split_capitalized_text = text.split("_").map(|text| {
      let text = text.to_lowercase();
      return text;
    });
    self.text =
      Arc::new(split_capitalized_text.collect::<Vec<String>>().join("_"));
    return self;
  }

  pub fn kebab_case(self) -> Self {
    let regex = self.non_alnum_regex.clone();
    let text = self.snake_case().build().replace("_", "-");
    return Self {
      text: Arc::new(text),
      non_alnum_regex: regex,
    };
  }

  pub fn with_rename(self, rename: &Rename) -> Self {
    return match rename {
      Rename::CamelCase => self.camel_case(),
      Rename::PascalCase => self.pascal_case(),
      Rename::SnakeCase => self.snake_case(),
      Rename::KebabCase => self.kebab_case(),
    };
  }

  pub fn build(self) -> Arc<String> {
    return self.text;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_remove_non_alnum() {
    let text = CaseManipulator::new("@ cla\tss_na||☺Me-te\nst|\\]';")
      .unwrap()
      .remove_non_alnum()
      .build();
    assert!(text.as_str() == "class_naMe-test", "text: {:?}", text);
  }

  #[test]
  fn test_camel_case() {
    let text = CaseManipulator::new("cla\tss_na||☺Me-te\nst|\\]';")
      .unwrap()
      .camel_case()
      .build();
    assert!(
      text.as_str() == "cla\tssNa||☺meTe\nst|\\]';",
      "text: {:?}",
      text
    );
  }

  #[test]
  fn test_pascal_case() {
    let text = CaseManipulator::new("cla\tss_na||☺Me-te\nst|\\]';")
      .unwrap()
      .pascal_case()
      .build();
    assert!(
      text.as_str() == "Cla\tssNa||☺meTe\nst|\\]';",
      "text: {:?}",
      text
    );
  }

  #[test]
  fn test_snake_case() {
    let text = CaseManipulator::new("cla\tss_na||☺Me-te\nst|\\]';")
      .unwrap()
      .snake_case()
      .build();
    assert!(
      text.as_str() == "cla\tss_na||☺me_te\nst|\\]';",
      "text: {:?}",
      text
    );
  }

  #[test]
  fn test_snake_case_from_camel_case() {
    let text = CaseManipulator::new("classNameTest")
      .unwrap()
      .snake_case()
      .build();
    assert!(text.as_str() == "class_name_test", "text: {:?}", text);
  }

  #[test]
  fn test_kebab_case() {
    let text = CaseManipulator::new("cla\tss_na||☺Me-te\nst|\\]';")
      .unwrap()
      .kebab_case()
      .build();
    assert!(
      text.as_str() == "cla\tss-na||☺me-te\nst|\\]';",
      "text: {:?}",
      text
    );
  }
}
