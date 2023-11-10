use ::std::io::Error as IOErr;

use ::err_derive::Error;

use crate::entities::intermediate::Error as IntermediateError;

use super::super::input::Error as InputError;

#[derive(Debug, Error)]
pub enum InputProcessError {
  #[error(display = "IO error: {}", _0)]
  Io(#[source] IOErr),
  #[error(display = "Invalid File Name: {}", _0)]
  InvalidFileName(String),
  #[error(display = "Decode error: {}", _0)]
  InputError(#[source] InputError),
  #[error(display = "Intermediate error: {}", _0)]
  IntermediateError(#[source] IntermediateError),
}

pub type InputProcessResult<T> = Result<T, InputProcessError>;
