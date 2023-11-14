use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::minijinja::{context, Environment, Template};

use crate::entities::inputs::{IRustAttributes, Root};
use crate::entities::intermediate::ITag;

use super::interface::IOutput;
use super::OutputResult;

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
  pub fn new(modules: Vec<Arc<dyn ITag>>) -> OutputResult<Self> {
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
    let template = match root {
      Root::Struct(_) => self.struct_template()?,
      Root::Enum(_) => self.enum_template()?,
    };
    let rust_attrs: Arc<dyn IRustAttributes> = match root {
      Root::Struct(s) => Arc::new(s),
      Root::Enum(e) => Arc::new(e),
    };
    template.render_to_write(
      context! {
        class_name => root_tag.class_name().to_string(),
        rust => rust_attrs.rust().as_ref(),
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

  use crate::entities::intermediate::Tag;
  use crate::fixtures::simple_struct::struct_simple;

  use super::IOutput;
  use super::Rust;

  #[test]
  fn test_simple_rendering() {
    let root = struct_simple();
    let tag = Tag::new("simple_structure".to_string()).unwrap();
    let correct =
      include_str!("../../fixtures/simple_struct.rs.out").to_string();

    let proc = Rust::new(vec![]).unwrap();
    let result = Vec::<u8>::new();
    let mut writer = result.writer();
    proc.render(&mut writer, &root, Arc::new(tag)).unwrap();
    let result = writer.get_ref();
    let result = String::from_utf8(result.to_vec()).unwrap();
    assert_eq!(
      result,
      correct,
      "{}",
      format!(
        "\n\nResult:\n{result}\n\nCorrect:\n{correct}\n",
        result = result,
        correct = correct
      )
    );
  }
}
