use ::err_derive::Error;
use ::serde_yaml::Error as YaMLDecodeError;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "YAML Decode Error: {}", _0)]
  YaMLDecodeError(#[source] YaMLDecodeError),
}

pub type Result<T> = ::std::result::Result<T, Error>;
