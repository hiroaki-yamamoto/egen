use ::std::io::Read;
use ::std::marker::PhantomData;

use ::serde_yaml::from_reader;

use crate::entities::inputs::Root;

use super::error::Result;
use super::interface::IDecode;

pub struct Yaml<Reader>
where
  Reader: Read,
{
  _reader: PhantomData<Reader>,
}

impl<Reader> Yaml<Reader>
where
  Reader: Read,
{
  pub fn new() -> Self {
    Self {
      _reader: PhantomData,
    }
  }
}

impl<Reader> IDecode for Yaml<Reader>
where
  Reader: Read,
{
  type Reader = Reader;
  fn decode(&self, input: Reader) -> Result<Root> {
    return Ok(from_reader(input)?);
  }
}

#[cfg(test)]
mod tests {
  use super::super::interface::IDecode;
  use super::Yaml;
  use crate::fixtures::{
    complex::complex, enumeration::enumeration, reference::reference,
    self_reference::self_reference, simple_struct::struct_simple,
    struct_array::struct_array, struct_w_fld_attr::struct_w_fld_attr,
  };

  #[test]
  fn test_simple() {
    let structure = struct_simple();
    let doc = include_str!("../../fixtures/simple_structure.yml");
    let struct_from_fixture = Yaml::new().decode(doc.as_bytes()).unwrap();
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
    let from_doc = Yaml::new().decode(doc.as_bytes()).unwrap();
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
    let doc = Yaml::new().decode(doc.as_bytes()).unwrap();
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
    let doc = Yaml::new().decode(doc.as_bytes()).unwrap();
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
    let doc = Yaml::new().decode(doc.as_bytes()).unwrap();
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
    let doc = Yaml::new().decode(doc.as_bytes()).unwrap();
    assert!(
      doc == structure,
      "fixture = {:?}, structure = {:?}",
      doc,
      structure
    );
  }

  #[test]
  fn test_enumeration() {
    let structure = enumeration();
    let doc = include_str!("../../fixtures/enumeration.yml");
    let doc = Yaml::new().decode(doc.as_bytes()).unwrap();
    assert!(
      doc == structure,
      "fixture = {:?}, structure = {:?}",
      doc,
      structure
    );
  }
}
