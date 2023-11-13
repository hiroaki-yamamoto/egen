use ::std::io::Write;
use ::std::sync::Arc;

use crate::entities::inputs::Root;
use crate::entities::intermediate::ITag;

use super::error::OutputResult;

pub trait IOutput {
  type Writer: Write;
  fn render(
    &self,
    writer: &mut Self::Writer,
    root: &Root,
    root_tag: Arc<dyn ITag>,
  ) -> OutputResult<()>;
}
