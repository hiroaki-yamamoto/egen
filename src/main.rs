mod cmd;
mod entities;
mod macros;
mod services;

use ::std::fs::File;
use ::std::sync::Arc;

use ::clap::Parser;
use ::glob::glob;

use crate::cmd::CMD;
use crate::entities::intermediate::{ITag, Tag};
use crate::services::input::IDecode;
use crate::services::processor::InputProcessor;

#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod test_utils;

// Check available classes in the input directory and create tags.
fn get_tags(in_glob: &[String]) -> Vec<Arc<dyn ITag>> {
  let in_glob: Vec<_> = in_glob
    .into_iter()
    .filter_map(|glob_txt| glob(&glob_txt).ok())
    .collect();
  let mut in_tags: Vec<Arc<dyn ITag>> = Vec::new();
  for paths in in_glob {
    in_tags.extend(paths.filter_map(|path| {
      let path = path.ok()?;
      let stem = path.file_stem()?.to_str()?;
      Tag::new(stem.to_string())
        .map(|tag| Arc::new(tag) as Arc<dyn ITag>)
        .ok()
    }));
  }
  return in_tags;
}

fn main() {
  let cfg = CMD::parse();
  let input_name = cfg.input;
  let input_dir = input_name.parent().map(|p| p.to_str()).flatten().unwrap();
  let in_glob: Vec<String> = match cfg.in_format {
    cmd::Input::Yaml => vec![
      format!("{}/*.yml", input_dir),
      format!("{}/*.yaml", input_dir),
    ],
  };
  let tags = get_tags(&in_glob);
}
