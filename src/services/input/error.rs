use ::err_derive::Error;
use ::serde_yaml::Error as YaMLDecodeError;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "YAML Decode Error: {}", _0)]
  YaMLDecodeError(#[source] YaMLDecodeError),
  #[error(display = "Invalid File Name: {}", _0)]
  InvalidFileName(String),
}

pub type Result<T> = ::std::result::Result<T, Error>;
