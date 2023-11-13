use ::std::io::Write;

use super::error::OutputResult;

pub trait IOutput {
  type Writer: Write;
  fn render(&self, writer: &mut Self::Writer) -> OutputResult<()>;
}
