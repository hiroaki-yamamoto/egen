use ::std::io::Error as IOErr;

use ::err_derive::Error;

use ::minijinja::Error as TemplateError;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "IO error: {}", _0)]
  IOErr(#[source] IOErr),
  #[error(display = "Template Error: {}", _0)]
  TemplateError(#[source] TemplateError),
}

pub type OutputResult<T> = Result<T, Error>;
