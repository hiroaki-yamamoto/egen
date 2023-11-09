use ::serde_yaml::from_reader;
use ::std::io::Read;

use crate::entities::inputs::Structure;

use super::error::Result;
use super::interface::IDecode;

pub struct Yaml;

impl IDecode for Yaml {
  fn decode(&self, input: impl Read) -> Result<Structure> {
    return Ok(from_reader(input)?);
  }
}

#[cfg(test)]
mod tests {
  use super::super::interface::IDecode;
  use super::Yaml;
  use crate::fixtures::{
    complex::complex, reference::reference, self_reference::self_reference,
    simple_struct::struct_simple, struct_array::struct_array,
    struct_w_fld_attr::struct_w_fld_attr,
  };

  #[test]
  fn test_simple() {
    let structure = struct_simple();
    let doc = include_str!("../../fixtures/simple_structure.yml");
    let struct_from_fixture = Yaml.decode(doc.as_bytes()).unwrap();
    assert!(
      struct_from_fixture == structure,
      "fixture = {:?}, structure = {:?}",
      struct_from_fixture,
      structure
    );
  }

  #[test]
  fn test_field_has_attr() {
    let structure = struct_w_fld_attr();
    let doc = include_str!("../../fixtures/struct_has_field_attr.yml");
    let from_doc = Yaml.decode(doc.as_bytes()).unwrap();
    assert!(
      from_doc == structure,
      "fixture = {:?}, structure = {:?}",
      from_doc,
      structure
    );
  }

  #[test]
  fn test_struct_array() {
    let structure = struct_array();
    let doc = include_str!("../../fixtures/struct_array.yml");
    let doc = Yaml.decode(doc.as_bytes()).unwrap();
    assert!(
      doc == structure,
      "fixture = {:?}, structure = {:?}",
      doc,
      structure
    );
  }

  #[test]
  fn test_reference() {
    let structure = reference();
    let doc = include_str!("../../fixtures/reference.yml");
    let doc = Yaml.decode(doc.as_bytes()).unwrap();
    assert!(
      doc == structure,
      "fixture = {:?}, structure = {:?}",
      doc,
      structure
    );
  }

  #[test]
  fn test_self_reference() {
    let structure = self_reference();
    let doc = include_str!("../../fixtures/self_reference.yml");
    let doc = Yaml.decode(doc.as_bytes()).unwrap();
    assert!(
      doc == structure,
      "fixture = {:?}, structure = {:?}",
      doc,
      structure
    );
  }

  #[test]
  fn test_complex_structure() {
    let structure = complex();
    let doc = include_str!("../../fixtures/complex.yml");
    let doc = Yaml.decode(doc.as_bytes()).unwrap();
    assert!(
      doc == structure,
      "fixture = {:?}, structure = {:?}",
      doc,
      structure
    );
  }
}
