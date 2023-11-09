use crate::entities::inputs::{Enumeration, Root, Rust};

pub fn enumeration() -> Root {
  return Root::Enum(
    Enumeration::new()
      .rust(Some(
        Rust::new()
          .attrs(Some(vec!["serde(rename_all = \"camelCase\")".to_string()]))
          .derive(Some(vec![
            "Debug".to_string(),
            "::serde::Serialize".to_string(),
          ])),
      ))
      .members(vec![
        "this".to_string(),
        "is".to_string(),
        "a".to_string(),
        "test".to_string(),
      ]),
  );
}
