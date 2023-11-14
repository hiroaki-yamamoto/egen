use crate::entities::inputs::{Enumeration, Root, Rust};
use ::std::sync::Arc;

pub fn enumeration() -> Root {
  return Root::Enum(
    Enumeration::new()
      .rust(Arc::new(Some(
        Rust::new()
          .attrs(Some(vec!["serde(rename_all = \"camelCase\")".to_string()]))
          .derive(Some(vec![
            "Debug".to_string(),
            "::serde::Serialize".to_string(),
          ])),
      )))
      .members(vec![
        "this".to_string(),
        "is".to_string(),
        "a".to_string(),
        "test".to_string(),
      ]),
  );
}
