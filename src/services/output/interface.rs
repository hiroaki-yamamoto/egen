use ::std::io::Write;

use crate::entities::inputs::Root;

use super::error::OutputResult;

pub trait IOutput {
  type Writer: Write;
  fn render(&self, writer: &mut Self::Writer, root: &Root)
    -> OutputResult<()>;
}
