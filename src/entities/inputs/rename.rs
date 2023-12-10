use ::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
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
