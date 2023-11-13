use ::err_derive::Error;
use ::std::io::Error as IOErr;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "IO error: {}", _0)]
  IOErr(#[source] IOErr),
}

pub type OutputResult<T> = Result<T, Error>;
