use ::std::fs::File;
use ::std::path::Path;
use ::std::sync::Arc;

use crate::entities::inputs::Root;
use crate::entities::intermediate::Tag;

use super::super::input::IDecode;
use super::error::{InputProcessError, InputProcessResult as Result};
use super::interface::IInputProcessor;

pub struct Processor {
  decoder: Arc<dyn IDecode<Reader = File> + Send + Sync>,
}

impl Processor {
  pub fn new(decoder: Arc<dyn IDecode<Reader = File> + Send + Sync>) -> Self {
    Self { decoder }
  }

  fn name(&self, path: impl AsRef<Path>) -> Result<String> {
    let name = path
      .as_ref()
      .file_name()
      .map(|fname| fname.to_str())
      .flatten()
      .map(|fname| fname.split(".").nth(0))
      .flatten()
      .map(|fname| fname.to_string())
      .ok_or(InputProcessError::InvalidFileName(
        path.as_ref().to_string_lossy().to_string(),
      ))?;
    return Ok(name);
  }

  #[cfg(test)]
  pub fn __name__(&self, path: impl AsRef<Path>) -> Result<String> {
    return self.name(path);
  }
}

impl IInputProcessor for Processor {
  fn process(&self, path: impl AsRef<Path>) -> Result<(Tag, Root)> {
    let name = self.name(&path)?;
    let reader = File::open(path)?;
    let root = self.decoder.decode(reader)?;
    return Ok((Tag::new(name)?, root));
  }
}

#[cfg(test)]
mod test {
  use ::std::fs::File;
  use ::std::sync::Arc;

  use super::Processor;
  use crate::entities::inputs::{Root, Structure};
  use crate::services::input::{IDecode, Result as InputResult};

  struct MockDecoder;

  impl IDecode for MockDecoder {
    type Reader = File;
    fn decode(&self, _: Self::Reader) -> InputResult<Root> {
      return Ok(Root::Struct(Structure::new()));
    }
  }

  #[test]
  fn test_name() {
    let proc = Processor::new(Arc::new(MockDecoder));
    let name = proc
      .__name__(
        "/home/user/0/the quick brown💩 fox jumps over the lazy dog.yml.tar.gz",
      )
      .unwrap();
    assert!(
      name == "the quick brown💩 fox jumps over the lazy dog",
      "name = {}",
      name
    );
  }
}
