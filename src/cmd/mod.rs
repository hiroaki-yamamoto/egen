mod error;
mod input;
mod output;

use ::std::path::PathBuf;

use ::clap::{value_parser, Parser};

use self::input::Input;
use self::output::Output;

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
