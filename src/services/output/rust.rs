use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::minijinja::value::ViaDeserialize;
use ::minijinja::{context, Environment, Template};

use crate::entities::inputs::{FieldInner, IMembers, IRustAttributes, Root};
use crate::entities::intermediate::ITag;

use super::interface::IOutput;
use super::OutputResult;

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
    let modules: Vec<String> = modules
      .iter()
      .map(|tag| tag.rs_module_name().to_string())
      .collect();
    let mut env: Environment<'env> = Environment::new();
    env.add_global("tags", modules);
    env.add_template(
      "struct",
      include_str!("../../templates/struct.rs.jinja"),
    )?;
    env.add_filter("rust_type", convert_type);
    env.add_template("enum", include_str!("../../templates/enum.rs.jinja"))?;
    return Ok(Self {
      env,
      _w: PhantomData,
    });
  }

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

  fn render(
    &self,
    writer: &mut Self::Writer,
    root: &Root,
    root_tag: Arc<dyn ITag>,
  ) -> OutputResult<()> {
    let (template, rust_attrs, members): (
      Template,
      Arc<dyn IRustAttributes>,
      Arc<dyn IMembers>,
    ) = match root {
      Root::Struct(s) => (self.struct_template()?, Arc::new(s), Arc::new(s)),
      Root::Enum(e) => (self.enum_template()?, Arc::new(e), Arc::new(e)),
    };
    template.render_to_write(
      context! {
        class_name => root_tag.class_name().to_string(),
        rust => rust_attrs.rust().as_ref(),
        members => members.members(),
      },
      writer,
    )?;
    return Ok(());
  }
}

#[cfg(test)]
pub mod test {
  use ::std::sync::Arc;

  use ::bytes::buf::BufMut;

  use crate::entities::inputs::Root;
  use crate::entities::intermediate::{ITag, Tag};
  use crate::fixtures::reference::reference;
  use crate::fixtures::simple_struct::struct_simple;
  use crate::fixtures::struct_array::struct_array;
  use crate::fixtures::struct_w_fld_attr::struct_w_fld_attr;
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
    let correct = include_str!("../../fixtures/simple_struct.rs.out")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_attr_field() {
    let root = struct_w_fld_attr();
    let tag = Tag::new("struct_has_field_attr".to_string()).unwrap();
    let correct = include_str!("../../fixtures/struct_w_field_attr.rs.out")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_array() {
    let root = struct_array();
    let tag = Tag::new("struct_array".to_string()).unwrap();
    let correct = include_str!("../../fixtures/struct_array.rs.out")
      .trim()
      .to_string();

    process(root, tag, &[], correct);
  }

  #[test]
  fn test_reference() {
    let root = reference();
    let tag = Tag::new("reference".to_string()).unwrap();
    let correct = include_str!("../../fixtures/reference.rs.out")
      .trim()
      .to_string();

    process(
      root,
      tag,
      &[Arc::new(Tag::new("simple_structure".to_string()).unwrap())],
      correct,
    );
  }
}
