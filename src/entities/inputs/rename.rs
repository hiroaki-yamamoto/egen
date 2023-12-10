use ::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(Clone))]
#[serde(rename_all = "camelCase")]
pub enum Rename {
  /// PascalCase
  PascalCase,
  /// camelCase
  CamelCase,
  /// snake_case
  SnakeCase,
  /// kebab-case
  KebabCase,
}

impl Default for Rename {
  fn default() -> Self {
    return Self::SnakeCase;
  }
}
