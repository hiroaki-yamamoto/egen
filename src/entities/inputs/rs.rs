use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rust {
  pub derive: Option<Vec<String>>,
  pub attrs: Option<Vec<String>>,
}
