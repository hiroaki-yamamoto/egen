use ::std::io::Write;
use ::std::sync::Arc;

use ::minijinja::{context, Template};

use crate::entities::inputs::{IMembers, IRustAttributes, Root};
use crate::entities::intermediate::ITag;

use super::error::OutputResult;

pub trait ITemplate {
  fn struct_template(&self) -> OutputResult<Template>;
  fn enum_template(&self) -> OutputResult<Template>;
}

pub trait IOutput: ITemplate {
  type Writer: Write;
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
