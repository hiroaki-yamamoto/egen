use ::std::io::{Read, Write};
use ::std::path::PathBuf;
use ::std::sync::Arc;

use ::clap::{value_parser, Parser, ValueEnum};

use crate::entities::intermediate::ITag;
use crate::services::input::{IDecode, Yaml};
use crate::services::output::{IOutput, OutputResult, Rust, ZodTS};

#[derive(Debug, Clone, ValueEnum)]
pub enum Input {
  Yaml,
}

impl Input {
  pub fn parse<Reader>(
    &self,
  ) -> Arc<dyn IDecode<Reader = Reader> + Send + Sync>
  where
    Reader: Read + Send + Sync + 'static,
  {
    return match self {
      Self::Yaml => Arc::new(Yaml::new()),
    };
  }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Output {
  Rust,
  ZodTS,
}

impl Output {
  pub fn parse<Writer>(
    &self,
    modules: &[Arc<dyn ITag>],
  ) -> OutputResult<Arc<dyn IOutput<Writer = Writer> + Send + Sync>>
  where
    Writer: Write + Send + Sync + 'static,
  {
    return Ok(match self {
      Self::Rust => Arc::new(Rust::<'static, Writer>::new(modules)?),
      Self::ZodTS => Arc::new(ZodTS::<'static, Writer>::new(modules)?),
    });
  }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Entity Generator")]
pub struct CMD {
  /// Input file name
  #[arg(short, long, value_parser = value_parser!(PathBuf))]
  pub input: PathBuf,
  /// Output directory
  #[arg(short, long, value_parser = value_parser!(PathBuf))]
  pub outdir: PathBuf,

  /// Input format
  #[arg(short = 'j', long, value_enum, default_value_t = Input::Yaml)]
  pub in_format: Input,

  /// Output format
  #[arg(short = 'p', long, value_enum)]
  pub out_format: Output,
}
