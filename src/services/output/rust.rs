use ::minijinja::{Environment, Template};

use super::OutputResult;

pub struct Rust<'env> {
  env: Environment<'env>,
}

impl<'env> Rust<'env> {
  pub fn new() -> OutputResult<Self> {
    let mut env: Environment<'env> = Environment::new();
    env.add_template(
      "struct",
      include_str!("../../templates/struct.rs.jinja"),
    )?;
    env.add_template("enum", include_str!("../../templates/enum.rs.jinja"))?;
    return Ok(Self { env });
  }

  fn struct_template(&self) -> OutputResult<Template> {
    return Ok(self.env.get_template("struct")?);
  }

  fn enum_template(&self) -> OutputResult<Template> {
    return Ok(self.env.get_template("enum")?);
  }
}
