mod error;
mod input;
mod interface;

pub use self::error::{InputProcessError, InputProcessResult};
pub use self::input::Processor as InputProcessor;
pub use self::interface::IInputProcessor;
