use ::std::ffi::OsStr;
use ::std::io::Read;
use ::std::path::PathBuf;
use ::std::sync::Arc;

use ::clap::ValueEnum;
use ::glob::glob;

use crate::entities::intermediate::Tag;
use crate::services::input::{IDecode, Yaml};

use super::error::{CMDResult, Error as CMDErr};

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

  fn get_tags(&self, in_glob: &[String]) -> Vec<Tag> {
    let in_glob: Vec<_> = in_glob
      .into_iter()
      .filter_map(|glob_txt| glob(&glob_txt).ok())
      .collect();
    let mut in_tags: Vec<Tag> = Vec::new();
    for paths in in_glob {
      in_tags.extend(paths.filter_map(|path| {
        let path = path.ok()?;
        let stem = path.file_stem()?.to_str()?;
        Tag::new(stem.to_string()).ok()
      }));
    }
    return in_tags;
  }

  pub fn check_file_name(&self, file_name: &PathBuf) -> CMDResult<PathBuf> {
    let ext =
      file_name
        .as_path()
        .extension()
        .ok_or(CMDErr::InvalidFileName(
          file_name.to_str().unwrap().to_string(),
        ))?;

    let match_ext = match self {
      Self::Yaml => [OsStr::new("yaml"), OsStr::new("yml")].contains(&ext),
    };

    if match_ext {
      return Err(CMDErr::InvalidFileName(
        file_name.to_str().unwrap().to_string(),
      ));
    }
    return Ok(file_name.clone());
  }

  pub fn glob(&self, dir: &str) -> Vec<Tag> {
    let in_glob: Vec<String> = match self {
      Input::Yaml => vec![format!("{}/*.yml", dir), format!("{}/*.yaml", dir)],
    };
    return self.get_tags(&in_glob);
  }
}
