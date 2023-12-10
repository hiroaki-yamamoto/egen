use ::regex::{Error as RegexError, Regex};

pub trait ICaseModifier {
  fn remove_non_alnum(&self) -> Result<String, RegexError>;
  fn camel_case(&self) -> String;
  fn pascal_case(&self) -> String;
  fn snake_case(&self) -> String;
}

impl ICaseModifier for &str {
  fn remove_non_alnum(&self) -> Result<String, RegexError> {
    let re = Regex::new(r"[^0-9a-zA-Z\-\_]")?;
    return Ok(re.replace_all(self, "").to_string());
  }

  fn camel_case(&self) -> String {
    let mut text = self.pascal_case();
    text = text.remove(0).to_lowercase().to_string() + &text;
    return text;
  }

  fn pascal_case(&self) -> String {
    let mut text = self.replace("-", "_");
    let split_capitalized_text = text.split("_").map(|text| {
      let mut text = text.to_string();
      let text =
        text.remove(0).to_uppercase().to_string() + &text.to_lowercase();
      return text;
    });
    text = split_capitalized_text.collect::<Vec<String>>().join("");
    return text;
  }

  fn snake_case(&self) -> String {
    let mut text = self.replace("-", "_");
    let split_capitalized_text = text.split("_").map(|text| {
      let text = text.to_lowercase();
      return text;
    });
    text = split_capitalized_text.collect::<Vec<String>>().join("_");
    return text;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_remove_non_alnum() {
    let text = "@ cla\tss_na||☺Me-te\nst|\\]';".remove_non_alnum().unwrap();
    assert!(text.as_str() == "class_naMe-test", "text: {:?}", text);
  }

  #[test]
  fn test_camel_case() {
    let text = "cla\tss_na||☺Me-te\nst|\\]';".camel_case();
    assert!(
      text.as_str() == "cla\tssNa||☺meTe\nst|\\]';",
      "text: {:?}",
      text
    );
  }

  #[test]
  fn test_pascal_case() {
    let text = "cla\tss_na||☺Me-te\nst|\\]';".pascal_case();
    assert!(
      text.as_str() == "Cla\tssNa||☺meTe\nst|\\]';",
      "text: {:?}",
      text
    );
  }

  #[test]
  fn test_snake_case() {
    let text = "cla\tss_na||☺Me-te\nst|\\]';".snake_case();
    assert!(
      text.as_str() == "cla\tss_na||☺me_te\nst|\\]';",
      "text: {:?}",
      text
    );
  }
}
