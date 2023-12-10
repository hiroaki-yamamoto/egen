mod cmd;
mod entities;
mod macros;
mod manipulators;
mod services;

use ::std::fs::File;
use ::std::io::Write;
use ::std::sync::Arc;

use ::clap::Parser;

use crate::cmd::CMD;
use crate::entities::intermediate::ITag;
use crate::services::input::IDecode;
use crate::services::processor::{
  IImportExtractor, IInputProcessor, ImportExtractor, InputProcessor,
};

#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod test_utils;

fn main() {
  let cfg = CMD::parse();
  // Input Process
  let input_name = cfg.in_format.check_file_name(&cfg.input).unwrap();
  let input_dir = input_name.parent().unwrap();
  let tags = cfg.in_format.glob(input_dir);
  let decoder: Arc<dyn IDecode<Reader = File> + Send + Sync> =
    cfg.in_format.parse();
  // Intermediate Process
  let processor = InputProcessor::new(decoder);
  let (root_tag, root) = processor.process(&input_name).unwrap();
  let import_extractor = ImportExtractor::new(tags);
  let modules: Vec<Arc<dyn ITag>> = import_extractor
    .extract(&root_tag, &root)
    .unwrap()
    .into_iter()
    .map(|module| {
      let tag: Arc<dyn ITag> = Arc::new(module);
      return tag;
    })
    .collect();
  // Output Process
  let root_tag: Arc<dyn ITag> = Arc::new(root_tag);
  let mut out_file = cfg
    .out_format
    .create(&cfg.outdir, root_tag.clone())
    .unwrap();
  let output = cfg.out_format.parse::<File>(&modules).unwrap();
  output.render(&mut out_file, &root, root_tag).unwrap();
  writeln!(&mut out_file, "").unwrap();
}
