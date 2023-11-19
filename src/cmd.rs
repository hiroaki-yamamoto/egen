use ::std::path::PathBuf;

use ::clap::{value_parser, Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Input {
  Yaml,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Output {
  Rust,
  ZodTS,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Entity Generator")]
pub struct CMD {
  /// Input file name
  #[arg(short, long, value_parser = value_parser!(PathBuf))]
  input: PathBuf,
  /// Output directory
  #[arg(short, long, value_parser = value_parser!(PathBuf))]
  outdir: PathBuf,

  /// Input format
  #[arg(short, long, value_enum, default_value_t = Input::Yaml)]
  from_format: Input,

  /// Output format
  #[arg(short, long, value_enum)]
  to_format: Output,
}
