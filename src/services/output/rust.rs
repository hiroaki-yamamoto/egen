use ::std::collections::HashMap;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::map_macro::hash_map_e;

use ::minijinja::value::{Value, ViaDeserialize};
use ::minijinja::{Environment, Template};

use crate::entities::inputs::FieldInner;
use crate::entities::intermediate::ITag;

use super::interface::{IOutput, ITemplate};
use super::OutputResult;

#[allow(dead_code)]
fn convert_type(fld_inner: ViaDeserialize<FieldInner>) -> String {
  let inner = fld_inner.f_type.to_string();
  if fld_inner.optional {
    return format!("Option<{}>", inner);
  }
  return inner;
}

pub struct Rust<'env, Writer>
where
  Writer: Write,
{
  env: Environment<'env>,
  _w: PhantomData<Writer>,
}

impl<'env, Writer> Rust<'env, Writer>
where
  Writer: Write,
{
  /// Initialize a new instance.
  ///
  /// ### Parameters:
  /// * `modules` - The list of modules to be imported. Note that this should be
  ///  the list of modules that is proceeded by ImportExtractor.
  pub fn new(modules: &[Arc<dyn ITag>]) -> OutputResult<Self> {
    let modules: Vec<HashMap<&str, String>> = modules
      .iter()
      .map(|tag| {
        hash_map_e! {
          "module" => tag.rs_module_name().to_string(),
          "class" => tag.class_name().to_string(),
        }
      })
      .collect();
    let mut env: Environment<'env> = Environment::new();
    env.add_global("tags", Value::from(modules));
    env.add_template(
      "struct",
      include_str!("../../templates/struct.rs.jinja"),
    )?;
    env.add_filter("type", convert_type);
    env.add_template("enum", include_str!("../../templates/enum.rs.jinja"))?;
    return Ok(Self {
      env,
      _w: PhantomData,
    });
  }
}

impl<'env, Writer> ITemplate for Rust<'env, Writer>
where
  Writer: Write,
{
  fn struct_template(&self) -> OutputResult<Template> {
    return Ok(self.env.get_template("struct")?);
  }

  fn enum_template(&self) -> OutputResult<Template> {
    return Ok(self.env.get_template("enum")?);
  }
}

impl<'env, Writer> IOutput for Rust<'env, Writer>
where
  Writer: Write,
{
  type Writer = Writer;
}

#[cfg(test)]
pub mod test {
  use ::std::sync::Arc;

  use ::bytes::buf::BufMut;

  use crate::entities::inputs::Root;
  use crate::entities::intermediate::{ITag, Tag};
  use crate::fixtures::complex::complex;
  use crate::fixtures::enumeration::enumeration;
  use crate::fixtures::reference::reference;
  use crate::fixtures::self_reference::self_reference;
  use crate::fixtures::simple_struct::struct_simple;
  use crate::fixtures::struct_array::struct_array;
  use crate::fixtures::struct_w_fld_attr::struct_w_fld_attr;
  use crate::fixtures::typescript_rename::typescript_rename;
  use crate::test_utils::assert_txt_eq;

  use super::IOutput;
  use super::Rust;

  fn process(root: Root, tag: Tag, refs: &[Arc<dyn ITag>], correct: String) {
    let proc = Rust::new(refs).unwrap();
    let result = Vec::<u8>::new();
    let mut writer = result.writer();
    proc.render(&mut writer, &root, Arc::new(tag)).unwrap();
    let result = writer.get_ref();
    let result = String::from_utf8(result.to_vec())
      .unwrap()
      .trim()
      .to_string();
    assert_txt_eq(&result, &correct);
  }

  #[test]
  fn test_simple_rendering() {
    let root = struct_simple();
    let tag = Tag::new("simple_structure".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/simple_structure.rs")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_attr_field() {
    let root = struct_w_fld_attr();
    let tag = Tag::new("struct_has_field_attr".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/struct_w_field_attr.rs")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_array() {
    let root = struct_array();
    let tag = Tag::new("struct_array".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/struct_array.rs")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_reference() {
    let root = reference();
    let tag = Tag::new("reference".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/reference.rs")
      .trim()
      .to_string();

    process(
      root,
      tag,
      &[Arc::new(Tag::new("simple_structure".to_string()).unwrap())],
      correct,
    );
  }

  #[test]
  fn test_self_reference() {
    let root = self_reference();
    let tag = Tag::new("self_reference".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/self_reference.rs")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_complex() {
    let root = complex();
    let tag = Tag::new("complex".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/complex.rs")
      .trim()
      .to_string();
    process(
      root,
      tag,
      &[
        Arc::new(Tag::new("simple_structure".to_string()).unwrap()),
        Arc::new(Tag::new("reference".to_string()).unwrap()),
      ],
      correct,
    );
  }

  #[test]
  fn test_enum() {
    let root = enumeration();
    let tag = Tag::new("enumeration".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/enumeration.rs")
      .trim()
      .to_string();
    process(root, tag, &[], correct);
  }

  #[test]
  fn test_ts_rename() {
    let root = typescript_rename();
    let tag = Tag::new("typescript_rename".to_string()).unwrap();
    let correct = include_str!("../../fixtures/rs_out/typescript_rename.rs")
      .trim()
      .to_string();
    process(root, tag, &[], correct);
  }
}
