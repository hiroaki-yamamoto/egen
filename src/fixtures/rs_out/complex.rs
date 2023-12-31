use super::reference::Reference;
use super::simple_structure::SimpleStructure;

#[derive(Debug, ::serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Complex {
  pub code: u16,
  #[serde(rename = "detail")]
  pub detailedText: String,
  pub referenceArray: Vec<Box<SimpleStructure>>,
  pub secondReference: Box<Reference>,
  pub selfReferenceArray: Vec<Box<Complex>>,
  #[serde(rename = "lst")]
  pub simpleArray: Vec<String>,
  pub simpleText: String,
}
