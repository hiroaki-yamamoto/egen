use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::minijinja::{Environment, Template};

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
    let modules: Vec<&str> = modules
      .iter()
      .map(|tag| tag.rs_module_name().as_str())
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

  fn render(&self, writer: &mut Self::Writer) -> OutputResult<()> {}
}
