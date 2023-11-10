use ::err_derive::Error;
use ::regex::Error as RegexError;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "Regex Compilation Err: {}", _0)]
  RegexCompileError(#[source] RegexError),
}

pub type Result<T> = ::std::result::Result<T, Error>;
