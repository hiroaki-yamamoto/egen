use ::std::fs::{DirBuilder, File};
use ::std::io::Write;
use ::std::path::Path;
use ::std::sync::Arc;

use ::clap::ValueEnum;

use crate::entities::intermediate::ITag;
use crate::services::output::{IOutput, OutputResult, Rust, ZodTS};

use super::error::CMDResult;

#[derive(Debug, Clone, ValueEnum)]
pub enum Output {
  Rust,
  ZodTS,
}

impl Output {
  pub fn parse<Writer>(
    &self,
    modules: &[Arc<dyn ITag>],
  ) -> OutputResult<Arc<dyn IOutput<Writer = Writer>>>
  where
    Writer: Write + Send + Sync + 'static,
  {
    return Ok(match self {
      Self::Rust => Arc::new(Rust::<'static, Writer>::new(modules)?),
      Self::ZodTS => Arc::new(ZodTS::<'static, Writer>::new(modules)?),
    });
  }

  pub fn create(
    &self,
    prefix_dir: &Path,
    tag: Arc<dyn ITag>,
  ) -> CMDResult<File> {
    DirBuilder::new().recursive(true).create(prefix_dir)?;
    let file_name = match self {
      Output::Rust => format!("{}.rs", tag.rs_module_name()),
      Output::ZodTS => format!("{}.zod.ts", tag.ts_module_name()),
    };
    let to_store = prefix_dir.join(file_name);
    let file = File::create(to_store)?;
    return Ok(file);
  }
}
