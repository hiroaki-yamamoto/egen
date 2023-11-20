use ::std::io::Error as IOErr;

use ::err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "Invalid file name: {}", _0)]
  InvalidFileName(String),
  #[error(display = "IO Err: {}", _0)]
  IO(#[source] IOErr),
}

pub type CMDResult<T> = Result<T, Error>;
