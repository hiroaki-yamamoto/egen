use ::serde_yaml::from_reader;
use ::std::io::Read;

use crate::entities::inputs::CompositeTypes;

use super::error::Result;
use super::interface::IDecode;

pub struct Yaml;

impl IDecode for Yaml {
  fn decode(&self, input: impl Read) -> Result<CompositeTypes> {
    return Ok(from_reader(input)?);
  }
}

#[cfg(test)]
mod tests {
  use super::super::interface::IDecode;
  use crate::fixtures::simple_struct::struct_simple;
  const SIMPLE_STRUCTURE_DOC: &'static str =
    include_str!("../../fixtures/simple_structure.yml");

  #[test]
  fn test_simple() {
    let structure = struct_simple();
    let struct_from_fixture =
      super::Yaml.decode(SIMPLE_STRUCTURE_DOC.as_bytes()).unwrap();
    assert!(
      struct_from_fixture == structure,
      "fixture = {:?}, structure = {:?}",
      struct_from_fixture,
      structure
    );
  }
}
