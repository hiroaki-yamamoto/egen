use ::std::io::Write;
use ::std::sync::Arc;

use ::minijinja::value::Serde;
use ::minijinja::{context, Template};

use crate::entities::inputs::{
  IMembers, IRustAttributes, ITSAttributes, Root,
};
use crate::entities::intermediate::ITag;

use super::error::OutputResult;

pub trait ITemplate {
  fn struct_template(&self) -> OutputResult<Template<'_, '_>>;
  fn enum_template(&self) -> OutputResult<Template<'_, '_>>;
}

pub trait IOutput: ITemplate {
  type Writer: Write;
  fn render(
    &self,
    writer: &mut Self::Writer,
    root: &Root,
    root_tag: Arc<dyn ITag>,
  ) -> OutputResult<()> {
    let (template, rust_attrs, ts_attrs, members): (
      Template,
      Arc<dyn IRustAttributes>,
      Arc<dyn ITSAttributes>,
      Arc<dyn IMembers>,
    ) = match root {
      Root::Struct(s) => (
        self.struct_template()?,
        Arc::new(s),
        Arc::new(s),
        Arc::new(s),
      ),
      Root::Enum(e) => {
        (self.enum_template()?, Arc::new(e), Arc::new(e), Arc::new(e))
      }
    };
    template.render_captured_to(
      context! {
        class_name => root_tag.class_name().to_string(),
        rust => Serde(rust_attrs.rust()),
        typescript => Serde(ts_attrs.typescript()),
        members => Serde(members.members()),
      },
      writer,
    )?;
    return Ok(());
  }
}
