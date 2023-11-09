use super::interface::IDecode;
use crate::entities::inputs::Root;

use super::error::Result;

pub struct Yaml;

impl IDecode for Yaml {
  fn decode(&self, input: &str) -> Result<Root> {
    todo!("decode");
  }
}

#[cfg(test)]
mod tests {
  use super::super::interface::IDecode;
  use crate::fixtures::simple::struct_simple;
  const SIMPLE_DOC: &'static str = include_str!("../../fixtures/simple.yml");

  #[test]
  fn test_simple() {
    let structure = struct_simple();
    let struct_from_fixture = super::Yaml.decode(SIMPLE_DOC).unwrap();
    assert!(struct_from_fixture == structure);
  }
}
